import { computed, reactive, ref } from "vue";
import { defineStore } from "pinia";
import {
  createDefaultAppConfig,
  createDefaultProviderConfig,
  DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE,
  DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE,
  DEFAULT_FIXED_LANGUAGE_ROUTING_RULE,
  DEFAULT_MAX_TOKENS,
  MAX_DOUBLE_COPY_WINDOW_MS,
  MAX_HISTORY_LIMIT,
  MAX_MAX_TOKENS,
  MAX_TIMEOUT_SECS,
  MAX_TOP_P,
  MAX_TEMPERATURE,
  MIN_DOUBLE_COPY_WINDOW_MS,
  MIN_MAX_TOKENS,
  MIN_TIMEOUT_SECS,
  MIN_TOP_P,
  MIN_TEMPERATURE,
  type AppConfig,
  type AuthScheme,
  type LanguageRoutingRule,
  type ProviderConfig,
  type ProviderKind,
  type RoutingKind,
  type ThemeMode,
} from "@cliplingo/shared-types";

export interface ProviderDraft extends Omit<ProviderConfig, "organization"> {
  apiKeyDraft: string;
  organization: string;
}

export interface FieldValidation {
  valid: boolean;
  message: string | null;
}

export interface ProviderValidation {
  id: FieldValidation;
  name: FieldValidation;
  baseUrl: FieldValidation;
  path: FieldValidation;
  apiKey: FieldValidation;
  model: FieldValidation;
  temperature: FieldValidation;
  topP: FieldValidation;
  maxTokens: FieldValidation;
  timeoutSecs: FieldValidation;
  headers: FieldValidation;
  hasErrors: boolean;
}

export interface SettingsValidation {
  ui: {
    themeMode: FieldValidation;
  };
  trigger: {
    doubleCopyWindowMs: FieldValidation;
    fallbackShortcut: FieldValidation;
  };
  translation: {
    routingRule: FieldValidation;
    userRules: FieldValidation;
  };
  history: {
    maxItems: FieldValidation;
  };
  debug: {
    logRawNetworkErrors: FieldValidation;
  };
  providers: ProviderValidation[];
  hasErrors: boolean;
}

export interface SettingsSummary {
  providerCount: number;
  activeProviderName: string;
  themeMode: ThemeMode;
  doubleCopyWindowMs: number;
  routingKind: RoutingKind;
  hasErrors: boolean;
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
  };
}

function createRoutingRule(kind: RoutingKind): LanguageRoutingRule {
  switch (kind) {
    case "bidirectional":
      return { ...DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE };
    case "fixed":
      return { ...DEFAULT_FIXED_LANGUAGE_ROUTING_RULE };
    case "branching":
    default:
      return { ...DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE };
  }
}

function createInitialDraft(): AppConfig {
  return createDefaultAppConfig();
}

function createEmptyField(message: string): FieldValidation {
  return { valid: false, message };
}

function createValidField(): FieldValidation {
  return { valid: true, message: null };
}

function trimList(values: string[]): string[] {
  return values.map((value) => value.trim()).filter((value) => value.length > 0);
}

function splitList(value: string): string[] {
  return trimList(
    value
      .split(",")
      .map((part) => part.trim())
      .filter((part) => part.length > 0),
  );
}

function joinList(values: readonly string[]): string {
  return values.join(", ");
}

function validateLanguageList(value: readonly string[], fieldName: string): FieldValidation {
  if (!value.length) {
    return createEmptyField(`${fieldName} needs at least one language code.`);
  }

  if (value.some((item) => !item.trim())) {
    return createEmptyField(`${fieldName} cannot contain empty items.`);
  }

  return createValidField();
}

function validateUrl(value: string): FieldValidation {
  const trimmed = value.trim();
  if (!trimmed) {
    return createEmptyField("Base URL is required.");
  }

  try {
    const parsed = new URL(trimmed);
    if (!["https:", "http:"].includes(parsed.protocol)) {
      return createEmptyField("Use http or https for the Base URL.");
    }

    if (parsed.protocol === "http:" && !["localhost", "127.0.0.1", "::1"].includes(parsed.hostname)) {
      return createEmptyField("HTTP is only allowed for localhost providers.");
    }
  } catch {
    return createEmptyField("Base URL must be a valid URL.");
  }

  return createValidField();
}

function validateHeaderName(value: string): FieldValidation {
  const trimmed = value.trim();
  if (!trimmed) {
    return createEmptyField("Header name is required.");
  }

  if (/[\r\n]/.test(trimmed)) {
    return createEmptyField("Header name cannot contain line breaks.");
  }

  return createValidField();
}

function validateHeaderValue(value: string): FieldValidation {
  const trimmed = value.trim();
  if (!trimmed) {
    return createEmptyField("Header value is required.");
  }

  if (/[\r\n]/.test(trimmed)) {
    return createEmptyField("Header value cannot contain line breaks.");
  }

  return createValidField();
}

function validateShortcut(value: string): FieldValidation {
  if (!value.trim()) {
    return createEmptyField("Fallback shortcut is required.");
  }

  if (!/[+]/.test(value)) {
    return createEmptyField("Shortcut should include modifiers, such as CmdOrCtrl+Shift+Y.");
  }

  return createValidField();
}

function validateNumberInRange(
  value: number,
  minimum: number,
  maximum: number,
  label: string,
): FieldValidation {
  if (!Number.isFinite(value)) {
    return createEmptyField(`${label} must be a number.`);
  }

  if (value < minimum || value > maximum) {
    return createEmptyField(`${label} must be between ${minimum} and ${maximum}.`);
  }

  return createValidField();
}

function providerSummary(provider: ProviderDraft): string {
  return provider.name.trim() || provider.id.trim() || "Untitled provider";
}

function createValidationSnapshot(draft: AppConfig, providers: ProviderDraft[]): SettingsValidation {
  const routingRule = draft.translation.routingRule;
  const routingValidation = (() => {
    switch (routingRule.kind) {
      case "bidirectional":
        return [
          validateLanguageList(
            [routingRule.primarySourceLanguage],
            "Primary source language",
          ),
          validateLanguageList(routingRule.primaryTargetLanguages, "Primary target languages"),
          validateLanguageList(
            [routingRule.secondarySourceLanguage],
            "Secondary source language",
          ),
          validateLanguageList(
            routingRule.secondaryTargetLanguages,
            "Secondary target languages",
          ),
        ];
      case "fixed":
        return [validateLanguageList(routingRule.targetLanguages, "Fixed target languages")];
      case "branching":
      default:
        return [
          validateLanguageList([routingRule.englishSourceLanguage], "English source language"),
          validateLanguageList(routingRule.englishTargetLanguages, "English target languages"),
          validateLanguageList([routingRule.chineseSourceLanguage], "Chinese source language"),
          validateLanguageList(routingRule.chineseTargetLanguages, "Chinese target languages"),
          validateLanguageList(routingRule.fallbackTargetLanguages, "Fallback target languages"),
        ];
    }
  })();

  const providerValidation = providers.map((provider) => {
    const customHeaderIssues = provider.customHeaders.flatMap((header) => [
      validateHeaderName(header.name),
      validateHeaderValue(header.value),
    ]);

    return {
      id: provider.id.trim()
        ? createValidField()
        : createEmptyField("Provider id is required."),
      name: provider.name.trim()
        ? createValidField()
        : createEmptyField("Provider name is required."),
      baseUrl: validateUrl(provider.baseUrl),
      path: provider.path.trim()
        ? createValidField()
        : createEmptyField("Request path is required."),
      apiKey:
        provider.authScheme === "none" || provider.apiKeyDraft.trim()
          ? createValidField()
          : createEmptyField("API key is required for bearer auth."),
      model: provider.model.trim()
        ? createValidField()
        : createEmptyField("Model is required."),
      temperature: validateNumberInRange(
        provider.temperature,
        MIN_TEMPERATURE,
        MAX_TEMPERATURE,
        "Temperature",
      ),
      topP: validateNumberInRange(provider.topP, MIN_TOP_P, MAX_TOP_P, "Top P"),
      maxTokens: validateNumberInRange(
        provider.maxTokens ?? DEFAULT_MAX_TOKENS,
        MIN_MAX_TOKENS,
        MAX_MAX_TOKENS,
        "Max tokens",
      ),
      timeoutSecs: validateNumberInRange(
        provider.timeoutSecs,
        MIN_TIMEOUT_SECS,
        MAX_TIMEOUT_SECS,
        "Timeout",
      ),
      headers:
        customHeaderIssues.find((issue) => !issue.valid) ?? createValidField(),
      hasErrors:
        [
          provider.id.trim(),
          provider.name.trim(),
          provider.baseUrl.trim(),
          provider.path.trim(),
          provider.model.trim(),
        ].some((value) => !value) ||
        (provider.authScheme === "bearer" && !provider.apiKeyDraft.trim()) ||
        customHeaderIssues.some((issue) => !issue.valid) ||
        !validateUrl(provider.baseUrl).valid ||
        !validateNumberInRange(provider.temperature, MIN_TEMPERATURE, MAX_TEMPERATURE, "Temperature").valid ||
        !validateNumberInRange(provider.topP, MIN_TOP_P, MAX_TOP_P, "Top P").valid ||
        !validateNumberInRange(provider.maxTokens ?? DEFAULT_MAX_TOKENS, MIN_MAX_TOKENS, MAX_MAX_TOKENS, "Max tokens").valid ||
        !validateNumberInRange(provider.timeoutSecs, MIN_TIMEOUT_SECS, MAX_TIMEOUT_SECS, "Timeout").valid,
    } satisfies ProviderValidation;
  });

  const validation: SettingsValidation = {
    ui: {
      themeMode:
        ["system", "light", "dark"].includes(draft.ui.themeMode)
          ? createValidField()
          : createEmptyField("Choose a supported theme mode."),
    },
    trigger: {
      doubleCopyWindowMs: validateNumberInRange(
        draft.trigger.doubleCopyWindowMs,
        MIN_DOUBLE_COPY_WINDOW_MS,
        MAX_DOUBLE_COPY_WINDOW_MS,
        "Double copy window",
      ),
      fallbackShortcut: validateShortcut(draft.trigger.fallbackShortcut),
    },
    translation: {
      routingRule:
        routingValidation.every((item) => item.valid)
          ? createValidField()
          : createEmptyField("Check the language routing fields."),
      userRules:
        draft.translation.userRules && draft.translation.userRules.trim()
          ? createValidField()
          : createValidField(),
    },
    history: {
      maxItems: validateNumberInRange(
        draft.history.maxItems,
        1,
        MAX_HISTORY_LIMIT,
        "History limit",
      ),
    },
    debug: {
      logRawNetworkErrors: createValidField(),
    },
    providers: providerValidation,
    hasErrors: false,
  };

  validation.hasErrors =
    !validation.ui.themeMode.valid ||
    !validation.trigger.doubleCopyWindowMs.valid ||
    !validation.trigger.fallbackShortcut.valid ||
    !validation.translation.routingRule.valid ||
    !validation.history.maxItems.valid ||
    providerValidation.some((provider) => provider.hasErrors);

  return validation;
}

export const useSettingsStore = defineStore("settings", () => {
  const draft = reactive(createInitialDraft());
  const providers = reactive<ProviderDraft[]>([
    createProviderDraft({
      id: "provider-default",
      name: "OpenAI-compatible",
      baseUrl: "",
      model: "",
      customHeaders: [],
      apiKeyDraft: "",
    }),
  ]);
  const activeProviderId = ref<string | null>(providers[0]?.id ?? null);
  const selectedProviderId = ref<string | null>(providers[0]?.id ?? null);
  const validation = computed(() => createValidationSnapshot(draft, providers));

  const summary = computed<SettingsSummary>(() => {
    const activeProvider = providers.find((provider) => provider.id === activeProviderId.value);
    return {
      providerCount: providers.length,
      activeProviderName: activeProvider ? providerSummary(activeProvider) : "No active provider",
      themeMode: draft.ui.themeMode,
      doubleCopyWindowMs: draft.trigger.doubleCopyWindowMs,
      routingKind: draft.translation.routingRule.kind,
      hasErrors: validation.value.hasErrors,
    };
  });

  function resetAll() {
    const fresh = createInitialDraft();
    draft.ui = fresh.ui;
    draft.trigger = fresh.trigger;
    draft.translation = fresh.translation;
    draft.history = fresh.history;
    draft.debug = fresh.debug;
    providers.splice(
      0,
      providers.length,
      createProviderDraft({
        id: "provider-default",
        name: "OpenAI-compatible",
        baseUrl: "",
        model: "",
        customHeaders: [],
        apiKeyDraft: "",
      }),
    );
    activeProviderId.value = providers[0]?.id ?? null;
    selectedProviderId.value = providers[0]?.id ?? null;
  }

  function setRoutingKind(kind: RoutingKind) {
    draft.translation.routingRule = createRoutingRule(kind);
  }

  function addProvider() {
    const suffix = `${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 7)}`;
    const nextProvider = createProviderDraft({
      id: `provider-${suffix}`,
      name: `Provider ${providers.length + 1}`,
      baseUrl: "",
      model: "",
      customHeaders: [],
      apiKeyDraft: "",
    });
    providers.push(nextProvider);
    selectedProviderId.value = nextProvider.id;
    if (!activeProviderId.value) {
      activeProviderId.value = nextProvider.id;
    }
  }

  function removeProvider(providerId: string) {
    const index = providers.findIndex((provider) => provider.id === providerId);
    if (index < 0) {
      return;
    }

    providers.splice(index, 1);
    if (activeProviderId.value === providerId) {
      activeProviderId.value = providers[0]?.id ?? null;
    }
    if (selectedProviderId.value === providerId) {
      selectedProviderId.value = providers[0]?.id ?? null;
    }
  }

  function selectProvider(providerId: string) {
    if (providers.some((provider) => provider.id === providerId)) {
      selectedProviderId.value = providerId;
      activeProviderId.value = providerId;
    }
  }

  function makeProviderActive(providerId: string) {
    if (providers.some((provider) => provider.id === providerId)) {
      activeProviderId.value = providerId;
      selectedProviderId.value = providerId;
    }
  }

  function addProviderHeader(providerId: string) {
    const provider = providers.find((item) => item.id === providerId);
    if (!provider) {
      return;
    }

    provider.customHeaders.push({ name: "", value: "" });
  }

  function removeProviderHeader(providerId: string, index: number) {
    const provider = providers.find((item) => item.id === providerId);
    if (!provider || index < 0 || index >= provider.customHeaders.length) {
      return;
    }

    provider.customHeaders.splice(index, 1);
  }

  function updateProviderKind(providerId: string, kind: ProviderKind) {
    const provider = providers.find((item) => item.id === providerId);
    if (!provider) {
      return;
    }

    provider.kind = kind;
  }

  function updateProviderAuthScheme(providerId: string, authScheme: AuthScheme) {
    const provider = providers.find((item) => item.id === providerId);
    if (!provider) {
      return;
    }

    provider.authScheme = authScheme;
  }

  return {
    draft,
    providers,
    activeProviderId,
    selectedProviderId,
    validation,
    summary,
    resetAll,
    setRoutingKind,
    addProvider,
    removeProvider,
    selectProvider,
    makeProviderActive,
    addProviderHeader,
    removeProviderHeader,
    updateProviderKind,
    updateProviderAuthScheme,
    splitList,
    joinList,
  };
});
