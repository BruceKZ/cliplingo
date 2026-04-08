import { computed, reactive, ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import {
  createDefaultAppConfig,
  DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE,
  DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE,
  DEFAULT_FIXED_LANGUAGE_ROUTING_RULE,
  normalizeDoubleCopyWindowMs,
  normalizeHistoryLimit,
  type AppConfig,
  type RoutingKind,
} from "@cliplingo/shared-types";
import { useUiStore, type AppLocale } from "@/stores/ui";

export type SettingsPersistState = "idle" | "saving" | "saved" | "error";

export interface SettingsStatusLine {
  tone: "info" | "success" | "warning" | "error";
  message: string;
  at: number;
}

function setStatus(
  statusLine: { value: SettingsStatusLine | null },
  tone: SettingsStatusLine["tone"],
  message: string,
) {
  statusLine.value = { tone, message, at: Date.now() };
}

function copyConfigInto(target: AppConfig, source: AppConfig) {
  target.schemaVersion = source.schemaVersion;
  target.ui = { ...source.ui };
  target.trigger = { ...source.trigger };
  target.providers = source.providers.map((provider) => ({
    ...provider,
    customHeaders: provider.customHeaders.map((header) => ({ ...header })),
  }));
  target.activeProviderId = source.activeProviderId;
  target.translation = {
    ...source.translation,
    routingRule: structuredClone(source.translation.routingRule),
  };
  target.history = { ...source.history };
  target.debug = { ...source.debug };
}

function createRoutingRule(kind: RoutingKind): AppConfig["translation"]["routingRule"] {
  switch (kind) {
    case "bidirectional":
      return structuredClone(DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE);
    case "fixed":
      return structuredClone(DEFAULT_FIXED_LANGUAGE_ROUTING_RULE);
    case "branching":
    default:
      return structuredClone(DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE);
  }
}

function splitList(value: string): string[] {
  return value
    .split(",")
    .map((part) => part.trim())
    .filter((part) => part.length > 0);
}

function joinList(values: readonly string[]): string {
  return values.join(", ");
}

export const useSettingsStore = defineStore("settings", () => {
  const uiStore = useUiStore();
  const draft = reactive(createDefaultAppConfig());
  const loaded = ref(false);
  const loading = ref(false);
  const persistState = ref<SettingsPersistState>("idle");
  const error = ref<string | null>(null);
  const statusLine = ref<SettingsStatusLine | null>(null);

  const routingKind = computed<RoutingKind>({
    get: () => draft.translation.routingRule.kind,
    set: (kind) => {
      draft.translation.routingRule = createRoutingRule(kind);
    },
  });

  async function load(force = false) {
    if (!force && (loaded.value || loading.value)) {
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      const config = await invoke<AppConfig>("load_app_config");
      copyConfigInto(draft, createDefaultAppConfig(config));
      uiStore.applyTheme(draft.ui.themeMode);
      loaded.value = true;
      setStatus(statusLine, "success", "Settings loaded.");
    } catch (cause) {
      error.value = cause instanceof Error ? cause.message : String(cause);
      setStatus(statusLine, "error", error.value);
      throw cause;
    } finally {
      loading.value = false;
    }
  }

  async function save() {
    persistState.value = "saving";
    error.value = null;

    const payload = createDefaultAppConfig({
      ...draft,
      trigger: {
        ...draft.trigger,
        doubleCopyWindowMs: normalizeDoubleCopyWindowMs(draft.trigger.doubleCopyWindowMs),
      },
      history: {
        ...draft.history,
        maxItems: normalizeHistoryLimit(draft.history.maxItems),
      },
    });

    try {
      const saved = await invoke<AppConfig>("save_app_config", { config: payload });
      copyConfigInto(draft, createDefaultAppConfig(saved));
      uiStore.applyTheme(draft.ui.themeMode);
      persistState.value = "saved";
      setStatus(statusLine, "success", "Settings saved.");
    } catch (cause) {
      error.value = cause instanceof Error ? cause.message : String(cause);
      persistState.value = "error";
      setStatus(statusLine, "error", error.value);
      throw cause;
    }
  }

  function resetDraft() {
    copyConfigInto(draft, createDefaultAppConfig());
    uiStore.applyTheme(draft.ui.themeMode);
    setStatus(statusLine, "info", "Settings reset to defaults.");
  }

  function setThemeMode(themeMode: AppConfig["ui"]["themeMode"]) {
    draft.ui.themeMode = themeMode;
    uiStore.applyTheme(themeMode);
  }

  function setLocale(locale: AppLocale) {
    uiStore.applyLocale(locale);
  }

  function updateList(
    value: string,
    apply: (next: string[]) => void,
  ) {
    apply(splitList(value));
  }

  return {
    draft,
    loaded,
    loading,
    persistState,
    error,
    statusLine,
    routingKind,
    load,
    save,
    resetDraft,
    setThemeMode,
    setLocale,
    updateList,
    splitList,
    joinList,
  };
});
