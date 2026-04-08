import { computed, reactive, ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import {
  createDefaultProviderConfig,
  MAX_MAX_TOKENS,
  MAX_TIMEOUT_SECS,
  MAX_TOP_P,
  MAX_TEMPERATURE,
  MIN_MAX_TOKENS,
  MIN_TIMEOUT_SECS,
  MIN_TOP_P,
  MIN_TEMPERATURE,
  type ProviderConfig,
  type ProviderDirectory,
} from "@cliplingo/shared-types";
import { getDefaultProviderDraft } from "@/config/providerDefaults";
import type { TranslationCommandOutput } from "@/components/translation/types";
import {
  validateHeaderNameInput,
  validateHeaderValueInput,
  validateProviderBaseUrl,
} from "@/utils/security";

export interface ProviderDraft extends Omit<ProviderConfig, "organization"> {
  apiKeyDraft: string;
  organization: string;
  persistedId: string | null;
  hasSecret: boolean;
}

export type PersistState = "idle" | "saving" | "saved" | "error";
export type ConnectionTestState = "idle" | "running" | "success" | "error";

export interface StatusLine {
  tone: "info" | "success" | "warning" | "error";
  message: string;
  at: number;
}

function createProviderDraft(
  overrides: Partial<ProviderDraft> = {},
): ProviderDraft {
  return {
    ...createDefaultProviderConfig(),
    ...overrides,
    customHeaders: overrides.customHeaders
      ? overrides.customHeaders.map((header) => ({ ...header }))
      : overrides.customHeaders ?? [],
    apiKeyDraft: overrides.apiKeyDraft ?? "",
    organization: overrides.organization ?? "",
    persistedId: overrides.persistedId ?? null,
    hasSecret: overrides.hasSecret ?? false,
  };
}

function setStatus(
  statusLine: { value: StatusLine | null },
  tone: StatusLine["tone"],
  message: string,
) {
  statusLine.value = { tone, message, at: Date.now() };
}

function validateProvider(provider: ProviderDraft): string | null {
  if (!provider.id.trim()) {
    return "Provider id is required.";
  }
  if (!provider.name.trim()) {
    return "Provider name is required.";
  }
  const baseUrlIssue = validateProviderBaseUrl(provider.baseUrl);
  if (baseUrlIssue) {
    return baseUrlIssue;
  }
  if (!provider.path.trim()) {
    return "Request path is required.";
  }
  if (
    provider.authScheme === "bearer" &&
    !provider.apiKeyDraft.trim() &&
    !provider.hasSecret
  ) {
    return "API key is required for bearer auth.";
  }
  if (!provider.model.trim()) {
    return "Model is required.";
  }
  if (
    !Number.isFinite(provider.temperature) ||
    provider.temperature < MIN_TEMPERATURE ||
    provider.temperature > MAX_TEMPERATURE
  ) {
    return `Temperature must be between ${MIN_TEMPERATURE} and ${MAX_TEMPERATURE}.`;
  }
  if (
    !Number.isFinite(provider.topP) ||
    provider.topP < MIN_TOP_P ||
    provider.topP > MAX_TOP_P
  ) {
    return `Top P must be between ${MIN_TOP_P} and ${MAX_TOP_P}.`;
  }

  const maxTokens = provider.maxTokens ?? MIN_MAX_TOKENS;
  if (
    !Number.isFinite(maxTokens) ||
    maxTokens < MIN_MAX_TOKENS ||
    maxTokens > MAX_MAX_TOKENS
  ) {
    return `Max tokens must be between ${MIN_MAX_TOKENS} and ${MAX_MAX_TOKENS}.`;
  }

  if (
    !Number.isFinite(provider.timeoutSecs) ||
    provider.timeoutSecs < MIN_TIMEOUT_SECS ||
    provider.timeoutSecs > MAX_TIMEOUT_SECS
  ) {
    return `Timeout must be between ${MIN_TIMEOUT_SECS} and ${MAX_TIMEOUT_SECS}.`;
  }

  for (const header of provider.customHeaders) {
    const invalidName = validateHeaderNameInput(header.name);
    if (invalidName) {
      return invalidName;
    }
    const invalidValue = validateHeaderValueInput(header.value);
    if (invalidValue) {
      return invalidValue;
    }
  }

  return null;
}

function normalizeDirectory(
  directory: ProviderDirectory,
  preservedDrafts: Map<string, string> = new Map(),
): ProviderDraft[] {
  return directory.providers.map((provider) =>
    createProviderDraft({
      ...provider,
      apiKeyDraft: preservedDrafts.get(provider.id) ?? "",
      organization: provider.organization ?? "",
      persistedId: provider.id,
    }),
  );
}

export const useProvidersStore = defineStore("providers", () => {
  const providers = reactive<ProviderDraft[]>([]);
  const activeProviderId = ref<string | null>(null);
  const selectedProviderId = ref<string | null>(null);
  const initialized = ref(false);
  const initializing = ref(false);
  const persistState = ref<PersistState>("idle");
  const testState = ref<ConnectionTestState>("idle");
  const error = ref<string | null>(null);
  const testMessage = ref<string | null>(null);
  const statusLine = ref<StatusLine | null>(null);

  const selectedProvider = computed(
    () =>
      providers.find((provider) => provider.id === selectedProviderId.value) ??
      providers[0] ??
      null,
  );

  function applyDirectory(
    directory: ProviderDirectory,
    options: {
      preservedDrafts?: Map<string, string>;
      selectedId?: string | null;
    } = {},
  ) {
    const nextProviders = normalizeDirectory(
      directory,
      options.preservedDrafts,
    );

    providers.splice(0, providers.length, ...nextProviders);
    activeProviderId.value = directory.activeProviderId;

    const selectedId =
      options.selectedId ??
      selectedProviderId.value ??
      directory.activeProviderId ??
      nextProviders[0]?.id ??
      null;

    selectedProviderId.value = nextProviders.some((provider) => provider.id === selectedId)
      ? selectedId
      : nextProviders[0]?.id ?? null;
  }

  async function invokeWithTimeout<T>(
    command: string,
    payload?: Record<string, unknown>,
    timeoutMs = 8_000,
  ): Promise<T> {
    let timer: number | null = null;

    try {
      return await Promise.race([
        invoke<T>(command, payload),
        new Promise<T>((_, reject) => {
          timer = window.setTimeout(() => {
            reject(new Error(`Command ${command} timed out after ${timeoutMs} ms`));
          }, timeoutMs);
        }),
      ]);
    } finally {
      if (timer !== null) {
        window.clearTimeout(timer);
      }
    }
  }

  async function refreshSecretStatus(providerId: string) {
    const provider = providers.find((entry) => entry.id === providerId);
    if (!provider) {
      return;
    }

    if (!provider.persistedId) {
      provider.hasSecret = provider.apiKeyDraft.trim().length > 0;
      return;
    }

    const status = await invokeWithTimeout<{ providerId: string; hasSecret: boolean }>(
      "get_provider_api_key_status",
      { providerId: provider.persistedId },
    );
    provider.hasSecret = status.hasSecret;
  }

  async function clearProviderSecret(providerId: string) {
    const provider = providers.find((entry) => entry.id === providerId);
    if (!provider) {
      return;
    }

    provider.apiKeyDraft = "";

    if (!provider.persistedId) {
      provider.hasSecret = false;
      setStatus(statusLine, "success", `Cleared unsaved API key for ${providerId}.`);
      return;
    }

    await invokeWithTimeout("delete_provider_api_key", {
      providerId: provider.persistedId,
    });
    provider.hasSecret = false;
    setStatus(statusLine, "success", `Cleared saved API key for ${providerId}.`);
  }

  async function refresh(force = false) {
    if (!force && (initialized.value || initializing.value)) {
      return;
    }

    initializing.value = true;
    error.value = null;

    try {
      const directory = await invokeWithTimeout<ProviderDirectory>("list_providers");
      applyDirectory(directory);
      await Promise.all(providers.map((provider) => refreshSecretStatus(provider.id)));
      initialized.value = true;
      setStatus(statusLine, "success", "Providers refreshed.");
    } catch (cause) {
      error.value = cause instanceof Error ? cause.message : String(cause);
      setStatus(statusLine, "warning", `Failed to refresh providers: ${error.value}`);
      initialized.value = true;
    } finally {
      initializing.value = false;
    }
  }

  async function reload(selectedId?: string | null) {
    const preservedDrafts = new Map(
      providers
        .filter((provider) => provider.apiKeyDraft.trim())
        .map((provider) => [provider.id, provider.apiKeyDraft.trim()] as const),
    );
    const directory = await invokeWithTimeout<ProviderDirectory>("list_providers");
    applyDirectory(directory, { preservedDrafts, selectedId });
    await Promise.all(providers.map((provider) => refreshSecretStatus(provider.id)));
  }

  function addProvider() {
    const suffix = `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 7)}`;
    const defaults = getDefaultProviderDraft();
    const nextId = providers.some((provider) => provider.id === defaults.id)
      ? `${defaults.id}-${suffix}`
      : defaults.id;

    const nextProvider = createProviderDraft({
      ...defaults,
      id: nextId,
      name: defaults.name || `Provider ${providers.length + 1}`,
      persistedId: null,
      hasSecret: Boolean(defaults.apiKeyDraft),
    });

    providers.push(nextProvider);
    selectedProviderId.value = nextProvider.id;
  }

  function duplicateProvider(providerId: string) {
    const existing = providers.find((provider) => provider.id === providerId);
    if (!existing) {
      return;
    }

    const suffix = `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 5)}`;
    const duplicate = createProviderDraft({
      ...existing,
      id: `${existing.id || "provider"}-${suffix}`,
      name: `${existing.name || existing.id || "Provider"} Copy`,
      persistedId: null,
    });

    providers.push(duplicate);
    selectedProviderId.value = duplicate.id;
    setStatus(statusLine, "success", `Duplicated provider ${existing.id}.`);
  }

  async function removeProvider(providerId: string) {
    const provider = providers.find((entry) => entry.id === providerId);
    if (!provider) {
      return;
    }

    const nextSelection = providers.find((entry) => entry.id !== providerId)?.id ?? null;

    if (!provider.persistedId) {
      const index = providers.findIndex((entry) => entry.id === providerId);
      providers.splice(index, 1);
      selectedProviderId.value = nextSelection;
      if (activeProviderId.value === providerId) {
        activeProviderId.value = nextSelection;
      }
      return;
    }

    const directory = await invokeWithTimeout<ProviderDirectory>("remove_provider_config", {
      providerId: provider.persistedId,
    });
    applyDirectory(directory, { selectedId: nextSelection });
    await Promise.all(providers.map((entry) => refreshSecretStatus(entry.id)));
    setStatus(statusLine, "success", `Removed provider ${providerId}.`);
  }

  function selectProvider(providerId: string) {
    if (providers.some((provider) => provider.id === providerId)) {
      selectedProviderId.value = providerId;
    }
  }

  async function makeProviderActive(providerId: string) {
    const directory = await invokeWithTimeout<ProviderDirectory>("set_active_provider", {
      providerId,
    });
    applyDirectory(directory, { selectedId: providerId });
    await Promise.all(providers.map((provider) => refreshSecretStatus(provider.id)));
    setStatus(statusLine, "success", `Active provider set to ${providerId}.`);
  }

  function addProviderHeader(providerId: string) {
    const provider = providers.find((entry) => entry.id === providerId);
    if (!provider) {
      return;
    }
    provider.customHeaders.push({ name: "", value: "" });
  }

  function removeProviderHeader(providerId: string, index: number) {
    const provider = providers.find((entry) => entry.id === providerId);
    if (!provider || index < 0 || index >= provider.customHeaders.length) {
      return;
    }
    provider.customHeaders.splice(index, 1);
  }

  async function persistCurrent(options: { preserveApiKeyDraft?: boolean } = {}) {
    const provider = selectedProvider.value;
    if (!provider) {
      throw new Error("No provider selected.");
    }

    const validationMessage = validateProvider(provider);
    if (validationMessage) {
      persistState.value = "error";
      error.value = validationMessage;
      setStatus(statusLine, "error", validationMessage);
      throw new Error(validationMessage);
    }

    const nextId = provider.id.trim();
    const originalId = provider.persistedId;
    const renaming = Boolean(originalId && originalId !== nextId);
    const nextApiKey = provider.apiKeyDraft.trim();

    if (renaming && provider.hasSecret && !nextApiKey && provider.authScheme === "bearer") {
      const message = "Re-enter the API key before changing a saved provider id.";
      persistState.value = "error";
      error.value = message;
      setStatus(statusLine, "error", message);
      throw new Error(message);
    }

    persistState.value = "saving";
    error.value = null;

    try {
      const directory = await invokeWithTimeout<ProviderDirectory>("upsert_provider_config", {
        provider: {
          id: nextId,
          name: provider.name.trim(),
          kind: provider.kind,
          baseUrl: provider.baseUrl.trim(),
          path: provider.path.trim(),
          authScheme: provider.authScheme,
          apiKeyRef: renaming ? null : provider.apiKeyRef,
          organization: provider.organization.trim() || null,
          model: provider.model.trim(),
          temperature: provider.temperature,
          topP: provider.topP,
          maxTokens: provider.maxTokens,
          timeoutSecs: provider.timeoutSecs,
          customHeaders: provider.customHeaders.map((header) => ({
            name: header.name.trim(),
            value: header.value.trim(),
          })),
          enabled: provider.enabled,
        },
      });

      applyDirectory(directory, {
        preservedDrafts:
          options.preserveApiKeyDraft && nextApiKey
            ? new Map([[nextId, nextApiKey]])
            : new Map(),
        selectedId: nextId,
      });

      if (provider.authScheme === "bearer" && nextApiKey) {
        await invokeWithTimeout("set_provider_api_key", {
          providerId: nextId,
          apiKey: nextApiKey,
        });
      } else if (provider.authScheme === "none" && (provider.hasSecret || renaming)) {
        await invokeWithTimeout("delete_provider_api_key", {
          providerId: nextId,
        }).catch(() => undefined);
      }

      if (renaming && originalId) {
        const updatedDirectory = await invokeWithTimeout<ProviderDirectory>(
          "remove_provider_config",
          {
            providerId: originalId,
          },
        );
        applyDirectory(updatedDirectory, {
          preservedDrafts:
            options.preserveApiKeyDraft && nextApiKey
              ? new Map([[nextId, nextApiKey]])
              : new Map(),
          selectedId: nextId,
        });
      }

      await refreshSecretStatus(nextId);
      const savedProvider = providers.find((entry) => entry.id === nextId);
      if (savedProvider && !options.preserveApiKeyDraft) {
        savedProvider.apiKeyDraft = "";
      }

      persistState.value = "saved";
      setStatus(statusLine, "success", `Saved provider ${nextId}.`);
    } catch (cause) {
      error.value = cause instanceof Error ? cause.message : String(cause);
      persistState.value = "error";
      setStatus(statusLine, "error", `Failed to save: ${error.value}`);
      throw cause;
    }
  }

  async function testConnection() {
    const provider = selectedProvider.value;
    if (!provider) {
      return;
    }

    testState.value = "running";
    testMessage.value = null;

    try {
      await persistCurrent({ preserveApiKeyDraft: true });
      const providerId = provider.id.trim();
      const result = await invokeWithTimeout<TranslationCommandOutput>("translate_text", {
        input: {
          text: "hello",
          providerId,
          targetLanguages: ["zh-CN"],
        },
      });

      if (result.error) {
        throw new Error(result.error.message);
      }

      testState.value = "success";
      testMessage.value = `${result.providerName} responded in ${result.latencyMs} ms.`;
      setStatus(statusLine, "success", "Provider test succeeded.");
    } catch (cause) {
      testState.value = "error";
      testMessage.value = cause instanceof Error ? cause.message : String(cause);
      setStatus(statusLine, "error", `Test failed: ${testMessage.value}`);
    }
  }

  return {
    providers,
    activeProviderId,
    selectedProviderId,
    selectedProvider,
    initialized,
    initializing,
    persistState,
    testState,
    error,
    testMessage,
    statusLine,
    refresh,
    reload,
    addProvider,
    duplicateProvider,
    removeProvider,
    selectProvider,
    makeProviderActive,
    addProviderHeader,
    removeProviderHeader,
    clearProviderSecret,
    persistCurrent,
    testConnection,
  };
});
