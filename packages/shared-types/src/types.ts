export type ThemeMode = "system" | "light" | "dark";

export type ProviderKind = "openai-compatible";

export type AuthScheme = "bearer" | "none";

export type RoutingKind = "bidirectional" | "branching" | "fixed";

export type LanguageCode = string;

export interface KeyValuePair {
  name: string;
  value: string;
}

export interface TranslationProfile {
  outputMode: "single" | "multi";
  preserveParagraphs: boolean;
  preserveCodeAndIdentifiers: boolean;
}

export interface TranslationUsage {
  promptTokens?: number;
  completionTokens?: number;
  totalTokens?: number;
}

export interface TranslationItem {
  targetLang: LanguageCode;
  text: string;
}

export interface TranslationRequest {
  requestId?: string;
  sourceText: string;
  sourceLang?: LanguageCode;
  targetLangs: LanguageCode[];
  providerId?: string;
  model: string;
  temperature: number;
  topP: number;
  maxTokens: number | null;
  timeoutSecs: number;
  systemProfile: TranslationProfile;
  userRules: string | null;
}

export interface TranslationResponse {
  requestId?: string;
  detectedSourceLang: LanguageCode;
  translations: TranslationItem[];
  provider: string;
  model: string;
  latencyMs: number;
  rawUsage: TranslationUsage | null;
}

export interface BidirectionalLanguageRoutingRule {
  kind: "bidirectional";
  primarySourceLanguage: LanguageCode;
  primaryTargetLanguages: readonly LanguageCode[];
  secondarySourceLanguage: LanguageCode;
  secondaryTargetLanguages: readonly LanguageCode[];
}

export interface BranchingLanguageRoutingRule {
  kind: "branching";
  englishSourceLanguage: LanguageCode;
  englishTargetLanguages: readonly LanguageCode[];
  chineseSourceLanguage: LanguageCode;
  chineseTargetLanguages: readonly LanguageCode[];
  fallbackTargetLanguages: readonly LanguageCode[];
}

export interface FixedLanguageRoutingRule {
  kind: "fixed";
  targetLanguages: readonly LanguageCode[];
}

export type LanguageRoutingRule =
  | BidirectionalLanguageRoutingRule
  | BranchingLanguageRoutingRule
  | FixedLanguageRoutingRule;

export interface ProviderConfig {
  id: string;
  name: string;
  kind: ProviderKind;
  baseUrl: string;
  path: string;
  authScheme: AuthScheme;
  apiKeyRef: string | null;
  organization: string | null;
  model: string;
  temperature: number;
  topP: number;
  maxTokens: number | null;
  timeoutSecs: number;
  customHeaders: KeyValuePair[];
  enabled: boolean;
  verifiedAt: number | null;
}

export interface ProviderDirectory {
  providers: ProviderConfig[];
  activeProviderId: string | null;
}

export interface UiConfig {
  themeMode: ThemeMode;
  showTrayIcon: boolean;
}

export interface TriggerConfig {
  doubleCopyWindowMs: number;
  fallbackShortcut: string;
  replacePopupOnNewTrigger: boolean;
}

export interface TranslationConfig {
  routingRule: LanguageRoutingRule;
  userRules: string | null;
  preserveParagraphs: boolean;
}

export interface HistoryConfig {
  enabled: boolean;
  maxItems: number;
  storeFullText: boolean;
}

export interface DebugConfig {
  enabled: boolean;
  logRawNetworkErrors: boolean;
}

export interface AppConfig {
  schemaVersion: 1;
  ui: UiConfig;
  trigger: TriggerConfig;
  providers: ProviderConfig[];
  activeProviderId: string | null;
  translation: TranslationConfig;
  history: HistoryConfig;
  debug: DebugConfig;
}

export type DeepPartial<T> = T extends readonly (infer U)[]
  ? readonly DeepPartial<U>[]
  : T extends object
    ? {
        [K in keyof T]?: DeepPartial<T[K]>;
      }
    : T;

export type AppConfigInput = DeepPartial<AppConfig>;
export type ProviderConfigInput = DeepPartial<ProviderConfig>;
export type TranslationRequestInput = DeepPartial<TranslationRequest>;
