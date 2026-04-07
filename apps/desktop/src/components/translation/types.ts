export const TRANSLATION_TRIGGER_EVENT = "trigger:translation-requested";
export const POPUP_COPY_FEEDBACK_MS = 1_400;

export type TranslationTriggerSource = "doubleCopy" | "fallbackShortcut";

export interface TranslationTriggerPayload {
  source: TranslationTriggerSource;
  text: string;
  characterCount: number;
}

export interface TranslationRequestInput {
  text: string;
  providerId?: string;
  sourceLanguage?: string;
  targetLanguages?: string[];
  userRules?: string | null;
}

export interface TranslationSectionOutput {
  targetLanguage: string;
  text: string;
}

export interface TranslationCommandError {
  code: string;
  message: string;
  retryable: boolean;
}

export interface TranslationCommandOutput {
  requestId: string;
  sourceText: string;
  sourceLanguage: string;
  targetLanguages: string[];
  translations: TranslationSectionOutput[];
  providerId: string;
  providerName: string;
  model: string;
  latencyMs: number;
  fallbackUsed: boolean;
  error: TranslationCommandError | null;
}
