import {
  DEFAULT_APP_CONFIG,
  DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE,
  DEFAULT_PROVIDER_CONFIG,
  createDefaultAppConfig,
  createDefaultProviderConfig,
  createDefaultTranslationRequest,
  normalizeDoubleCopyWindowMs,
  normalizeMaxTokens,
  normalizeTemperature,
  normalizeTimeoutSecs,
  normalizeTopP,
  type AppConfig,
  type ProviderConfig,
  type TranslationRequest,
} from "../src";

const appConfig: AppConfig = createDefaultAppConfig();
const providerConfig: ProviderConfig = createDefaultProviderConfig({
  name: "Example",
});
const translationRequest: TranslationRequest = createDefaultTranslationRequest({
  sourceText: "hello",
  targetLangs: ["zh-CN"],
  model: "gpt-4.1-mini",
});

void appConfig;
void providerConfig;
void translationRequest;

DEFAULT_APP_CONFIG satisfies AppConfig;
DEFAULT_PROVIDER_CONFIG satisfies ProviderConfig;
DEFAULT_BRANCHING_LANGUAGE_ROUTING_RULE satisfies AppConfig["translation"]["routingRule"];

normalizeDoubleCopyWindowMs(500);
normalizeTimeoutSecs(30);
normalizeTemperature(0.5);
normalizeTopP(0.9);
normalizeMaxTokens(2048);

// @ts-expect-error Provider kind is fixed to OpenAI-compatible in phase 1.
createDefaultProviderConfig({ kind: "anthropic" });

// @ts-expect-error Temperature must be a number.
normalizeTemperature("0.5");
