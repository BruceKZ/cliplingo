import type {
  AppConfig,
  AppConfigInput,
  DeepPartial,
  BranchingLanguageRoutingRule,
  BidirectionalLanguageRoutingRule,
  DebugConfig,
  FixedLanguageRoutingRule,
  HistoryConfig,
  KeyValuePair,
  LanguageRoutingRule,
  ProviderConfig,
  ProviderConfigInput,
  TranslationConfig,
  TranslationProfile,
  TranslationRequest,
  TranslationRequestInput,
  TranslationResponse,
  TranslationUsage,
  TriggerConfig,
  UiConfig,
} from "./types";

export const MIN_DOUBLE_COPY_WINDOW_MS = 250;
export const MAX_DOUBLE_COPY_WINDOW_MS = 800;
export const DEFAULT_DOUBLE_COPY_WINDOW_MS = 450;

export const MIN_TIMEOUT_SECS = 3;
export const MAX_TIMEOUT_SECS = 120;
export const DEFAULT_TIMEOUT_SECS = 30;

export const MIN_TOP_P = 0;
export const MAX_TOP_P = 1;
export const DEFAULT_TOP_P = 1;

export const MIN_TEMPERATURE = 0;
export const MAX_TEMPERATURE = 2;
export const DEFAULT_TEMPERATURE = 0.2;

export const MIN_MAX_TOKENS = 1;
export const MAX_MAX_TOKENS = 32_000;
export const DEFAULT_MAX_TOKENS = 4_096;

export const DEFAULT_HISTORY_LIMIT = 100;
export const MAX_HISTORY_LIMIT = 5_000;

export const DEFAULT_PROVIDER_PATH = "/chat/completions";

export const DEFAULT_TRANSLATION_PROFILE: TranslationProfile = {
  outputMode: "multi",
  preserveParagraphs: true,
  preserveCodeAndIdentifiers: true,
};

export const DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE: BranchingLanguageRoutingRule =
  {
    kind: "branching",
    englishSourceLanguage: "en",
    englishTargetLanguages: ["zh-CN"],
    chineseSourceLanguage: "zh-CN",
    chineseTargetLanguages: ["en"],
    fallbackTargetLanguages: ["en", "zh-CN"],
  };

export const DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE: BidirectionalLanguageRoutingRule =
  {
    kind: "bidirectional",
    primarySourceLanguage: "en",
    primaryTargetLanguages: ["zh-CN"],
    secondarySourceLanguage: "zh-CN",
    secondaryTargetLanguages: ["en"],
  };

export const DEFAULT_FIXED_LANGUAGE_ROUTING_RULE: FixedLanguageRoutingRule = {
  kind: "fixed",
  targetLanguages: ["en", "zh-CN"],
};

export const DEFAULT_LANGUAGE_ROUTING_RULE: LanguageRoutingRule =
  DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE;

export const DEFAULT_UI_CONFIG: UiConfig = {
  themeMode: "system",
  showTrayIcon: true,
};

export const DEFAULT_TRIGGER_CONFIG: TriggerConfig = {
  doubleCopyWindowMs: DEFAULT_DOUBLE_COPY_WINDOW_MS,
  fallbackShortcut: "CmdOrCtrl+Shift+Y",
  replacePopupOnNewTrigger: true,
};

export const DEFAULT_TRANSLATION_CONFIG: TranslationConfig = {
  routingRule: DEFAULT_LANGUAGE_ROUTING_RULE,
  userRules: null,
  preserveParagraphs: true,
};

export const DEFAULT_HISTORY_CONFIG: HistoryConfig = {
  enabled: true,
  maxItems: DEFAULT_HISTORY_LIMIT,
  storeFullText: false,
};

export const DEFAULT_DEBUG_CONFIG: DebugConfig = {
  enabled: true,
  logRawNetworkErrors: false,
};

export const DEFAULT_PROVIDER_CONFIG: ProviderConfig = {
  id: "provider-default",
  name: "OpenAI-compatible",
  kind: "openai-compatible",
  baseUrl: "",
  path: DEFAULT_PROVIDER_PATH,
  authScheme: "bearer",
  apiKeyRef: null,
  organization: null,
  model: "",
  temperature: DEFAULT_TEMPERATURE,
  topP: DEFAULT_TOP_P,
  maxTokens: DEFAULT_MAX_TOKENS,
  timeoutSecs: DEFAULT_TIMEOUT_SECS,
  customHeaders: [],
  enabled: true,
  verifiedAt: null,
};

export const DEFAULT_APP_CONFIG: AppConfig = {
  schemaVersion: 1,
  ui: DEFAULT_UI_CONFIG,
  trigger: DEFAULT_TRIGGER_CONFIG,
  providers: [],
  activeProviderId: null,
  translation: DEFAULT_TRANSLATION_CONFIG,
  history: DEFAULT_HISTORY_CONFIG,
  debug: DEFAULT_DEBUG_CONFIG,
};

export function clampNumber(
  value: number,
  minimum: number,
  maximum: number,
): number {
  if (!Number.isFinite(value)) {
    return minimum;
  }

  return Math.min(maximum, Math.max(minimum, value));
}

export function normalizeDoubleCopyWindowMs(value: number | null | undefined) {
  return clampNumber(
    Math.trunc(value ?? DEFAULT_DOUBLE_COPY_WINDOW_MS),
    MIN_DOUBLE_COPY_WINDOW_MS,
    MAX_DOUBLE_COPY_WINDOW_MS,
  );
}

export function normalizeTimeoutSecs(value: number | null | undefined) {
  return clampNumber(
    Math.trunc(value ?? DEFAULT_TIMEOUT_SECS),
    MIN_TIMEOUT_SECS,
    MAX_TIMEOUT_SECS,
  );
}

export function normalizeTopP(value: number | null | undefined) {
  return clampNumber(value ?? DEFAULT_TOP_P, MIN_TOP_P, MAX_TOP_P);
}

export function normalizeTemperature(value: number | null | undefined) {
  return clampNumber(
    value ?? DEFAULT_TEMPERATURE,
    MIN_TEMPERATURE,
    MAX_TEMPERATURE,
  );
}

export function normalizeMaxTokens(value: number | null | undefined) {
  return clampNumber(
    Math.trunc(value ?? DEFAULT_MAX_TOKENS),
    MIN_MAX_TOKENS,
    MAX_MAX_TOKENS,
  );
}

export function normalizeHistoryLimit(value: number | null | undefined) {
  return clampNumber(
    Math.trunc(value ?? DEFAULT_HISTORY_LIMIT),
    1,
    MAX_HISTORY_LIMIT,
  );
}

export function normalizeCustomHeader(input: KeyValuePair): KeyValuePair {
  return {
    name: input.name.trim(),
    value: input.value.trim(),
  };
}

export function normalizeLanguageRoutingRule(
  value: DeepPartial<LanguageRoutingRule> | undefined,
): LanguageRoutingRule {
  if (!value) {
    return DEFAULT_LANGUAGE_ROUTING_RULE;
  }

  if (value.kind === "fixed" || "targetLanguages" in value) {
    const fixedValue = value as DeepPartial<FixedLanguageRoutingRule>;

    return {
      kind: "fixed",
      targetLanguages: fixedValue.targetLanguages
        ? [...fixedValue.targetLanguages]
        : [...DEFAULT_FIXED_LANGUAGE_ROUTING_RULE.targetLanguages],
    };
  }

  if (value.kind === "bidirectional" || "primarySourceLanguage" in value) {
    const bidirectionalValue =
      value as DeepPartial<BidirectionalLanguageRoutingRule>;

    return {
      kind: "bidirectional",
      primarySourceLanguage:
        bidirectionalValue.primarySourceLanguage ??
        DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE.primarySourceLanguage,
      primaryTargetLanguages: bidirectionalValue.primaryTargetLanguages
        ? [...bidirectionalValue.primaryTargetLanguages]
        : [...DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE.primaryTargetLanguages],
      secondarySourceLanguage:
        bidirectionalValue.secondarySourceLanguage ??
        DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE.secondarySourceLanguage,
      secondaryTargetLanguages: bidirectionalValue.secondaryTargetLanguages
        ? [...bidirectionalValue.secondaryTargetLanguages]
        : [
            ...DEFAULT_BIDIRECTIONAL_LANGUAGE_ROUTING_RULE.secondaryTargetLanguages,
          ],
    };
  }

  const branchingValue = value as DeepPartial<BranchingLanguageRoutingRule>;

  return {
    kind: "branching",
    englishSourceLanguage:
      branchingValue.englishSourceLanguage ??
      DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE.englishSourceLanguage,
    englishTargetLanguages: branchingValue.englishTargetLanguages
      ? [...branchingValue.englishTargetLanguages]
      : [...DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE.englishTargetLanguages],
    chineseSourceLanguage:
      branchingValue.chineseSourceLanguage ??
      DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE.chineseSourceLanguage,
    chineseTargetLanguages: branchingValue.chineseTargetLanguages
      ? [...branchingValue.chineseTargetLanguages]
      : [...DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE.chineseTargetLanguages],
    fallbackTargetLanguages: branchingValue.fallbackTargetLanguages
      ? [...branchingValue.fallbackTargetLanguages]
      : [...DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE.fallbackTargetLanguages],
  };
}

export function createDefaultProviderConfig(
  overrides: ProviderConfigInput = {},
): ProviderConfig {
  return {
    ...DEFAULT_PROVIDER_CONFIG,
    ...overrides,
    customHeaders: overrides.customHeaders
      ? overrides.customHeaders.map((header) =>
          normalizeCustomHeader({
            name: header?.name ?? "",
            value: header?.value ?? "",
          }),
        )
      : [],
    temperature: normalizeTemperature(overrides.temperature),
    topP: normalizeTopP(overrides.topP),
    maxTokens: overrides.maxTokens === null
      ? null
      : normalizeMaxTokens(overrides.maxTokens),
    timeoutSecs: normalizeTimeoutSecs(overrides.timeoutSecs),
  };
}

export function createDefaultTranslationProfile(
  overrides: Partial<TranslationProfile> = {},
): TranslationProfile {
  return {
    ...DEFAULT_TRANSLATION_PROFILE,
    ...overrides,
  };
}

export function createDefaultTranslationRequest(
  overrides: TranslationRequestInput = {},
): TranslationRequest {
  return {
    requestId: overrides.requestId,
    sourceText: overrides.sourceText ?? "",
    sourceLang: overrides.sourceLang,
    targetLangs: overrides.targetLangs ? [...overrides.targetLangs] : [],
    providerId: overrides.providerId,
    model: overrides.model ?? "",
    temperature: normalizeTemperature(overrides.temperature),
    topP: normalizeTopP(overrides.topP),
    maxTokens: overrides.maxTokens === null
      ? null
      : normalizeMaxTokens(overrides.maxTokens),
    timeoutSecs: normalizeTimeoutSecs(overrides.timeoutSecs),
    systemProfile: createDefaultTranslationProfile(
      overrides.systemProfile as Partial<TranslationProfile> | undefined,
    ),
    userRules: overrides.userRules ?? null,
  };
}

export function createDefaultTranslationResponse(
  overrides: Partial<TranslationResponse> = {},
): TranslationResponse {
  return {
    requestId: overrides.requestId,
    detectedSourceLang: overrides.detectedSourceLang ?? "",
    translations: overrides.translations ? [...overrides.translations] : [],
    provider: overrides.provider ?? "",
    model: overrides.model ?? "",
    latencyMs: Math.trunc(overrides.latencyMs ?? 0),
    rawUsage: overrides.rawUsage ?? null,
  };
}

export function createDefaultAppConfig(
  overrides: AppConfigInput = {},
): AppConfig {
  return {
    schemaVersion: 1,
    ui: {
      ...DEFAULT_UI_CONFIG,
      ...overrides.ui,
    },
    trigger: {
      ...DEFAULT_TRIGGER_CONFIG,
      ...overrides.trigger,
      doubleCopyWindowMs: normalizeDoubleCopyWindowMs(
        overrides.trigger?.doubleCopyWindowMs,
      ),
    },
    providers: overrides.providers
      ? overrides.providers.map((provider) =>
          createDefaultProviderConfig(provider),
        )
      : [],
    activeProviderId: overrides.activeProviderId ?? null,
    translation: {
      ...DEFAULT_TRANSLATION_CONFIG,
      ...overrides.translation,
      routingRule: normalizeLanguageRoutingRule(
        overrides.translation?.routingRule,
      ),
    },
    history: {
      ...DEFAULT_HISTORY_CONFIG,
      ...overrides.history,
      maxItems: normalizeHistoryLimit(overrides.history?.maxItems),
    },
    debug: {
      ...DEFAULT_DEBUG_CONFIG,
      ...overrides.debug,
    },
  };
}

export const DEFAULT_TOKEN_USAGE: TranslationUsage = {
  promptTokens: 0,
  completionTokens: 0,
  totalTokens: 0,
};
