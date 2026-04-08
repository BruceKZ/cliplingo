import {
  DEFAULT_MAX_TOKENS,
  DEFAULT_PROVIDER_PATH,
  DEFAULT_TEMPERATURE,
  DEFAULT_TIMEOUT_SECS,
  DEFAULT_TOP_P,
} from "@cliplingo/shared-types";

export interface ProviderDraftDefaults {
  id: string;
  name: string;
  kind: "openai-compatible";
  authScheme: "bearer" | "none";
  baseUrl: string;
  path: string;
  apiKeyDraft: string;
  organization: string;
  model: string;
  temperature: number;
  topP: number;
  maxTokens: number;
  timeoutSecs: number;
  customHeaders: [];
  enabled: boolean;
}

function parseInteger(
  value: string | undefined,
  fallback: number,
): number {
  if (!value) {
    return fallback;
  }

  const parsed = Number.parseInt(value, 10);
  return Number.isFinite(parsed) ? parsed : fallback;
}

function parseFloatNumber(
  value: string | undefined,
  fallback: number,
): number {
  if (!value) {
    return fallback;
  }

  const parsed = Number.parseFloat(value);
  return Number.isFinite(parsed) ? parsed : fallback;
}

export function getDefaultProviderDraft(): ProviderDraftDefaults {
  return {
    id: import.meta.env.VITE_DEFAULT_PROVIDER_ID?.trim() || "openai",
    name:
      import.meta.env.VITE_DEFAULT_PROVIDER_NAME?.trim() || "OpenAI-compatible",
    kind: "openai-compatible",
    authScheme: "bearer",
    baseUrl: import.meta.env.VITE_DEFAULT_PROVIDER_BASE_URL?.trim() || "",
    path:
      import.meta.env.VITE_DEFAULT_PROVIDER_PATH?.trim() ||
      DEFAULT_PROVIDER_PATH,
    apiKeyDraft: import.meta.env.VITE_DEFAULT_PROVIDER_API_KEY?.trim() || "",
    organization:
      import.meta.env.VITE_DEFAULT_PROVIDER_ORGANIZATION?.trim() || "",
    model: import.meta.env.VITE_DEFAULT_PROVIDER_MODEL?.trim() || "",
    temperature: parseFloatNumber(
      import.meta.env.VITE_DEFAULT_PROVIDER_TEMPERATURE,
      DEFAULT_TEMPERATURE,
    ),
    topP: parseFloatNumber(
      import.meta.env.VITE_DEFAULT_PROVIDER_TOP_P,
      DEFAULT_TOP_P,
    ),
    maxTokens: parseInteger(
      import.meta.env.VITE_DEFAULT_PROVIDER_MAX_TOKENS,
      DEFAULT_MAX_TOKENS,
    ),
    timeoutSecs: parseInteger(
      import.meta.env.VITE_DEFAULT_PROVIDER_TIMEOUT_SECS,
      DEFAULT_TIMEOUT_SECS,
    ),
    customHeaders: [],
    enabled: true,
  };
}
