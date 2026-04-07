import { computed, ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type {
  TranslationCommandOutput,
  TranslationRequestInput,
  TranslationSectionOutput,
  TranslationTriggerPayload,
} from "@/components/translation/types";

const COPY_FEEDBACK_MS = 1_400;

interface TranslationErrorState {
  code: string;
  message: string;
  retryable: boolean;
}

function normalizeText(text: string): string {
  return text
    .replace(/\r\n?/g, "\n")
    .split("")
    .filter((character) => {
      const code = character.charCodeAt(0);
      return (
        character === "\n" ||
        character === "\t" ||
        (code >= 0x20 && code !== 0x7f)
      );
    })
    .join("")
    .trim();
}

function copyToClipboard(text: string): Promise<void> {
  if (!navigator.clipboard?.writeText) {
    return Promise.reject(new Error("Clipboard API unavailable"));
  }

  return navigator.clipboard.writeText(text);
}

function toSafeError(error: unknown): TranslationErrorState {
  if (error instanceof Error) {
    return {
      code: "translation_failed",
      message: error.message,
      retryable: true,
    };
  }

  return {
    code: "translation_failed",
    message: "Translation failed. Please try again.",
    retryable: true,
  };
}

export const useTranslationStore = defineStore("translation", () => {
  const visible = ref(false);
  const pinned = ref(true);
  const loading = ref(false);
  const sourceExpanded = ref(false);
  const copiedSource = ref(false);
  const copiedTargetLanguage = ref<string | null>(null);
  const requestSource = ref<TranslationTriggerPayload["source"] | null>(null);
  const sourceText = ref("");
  const sourceCharacterCount = ref(0);
  const currentResult = ref<TranslationCommandOutput | null>(null);
  const error = ref<TranslationErrorState | null>(null);
  const requestSequence = ref(0);

  let copiedSourceTimer: number | null = null;
  let copiedTargetTimer: number | null = null;

  const translations = computed<TranslationSectionOutput[]>(
    () => currentResult.value?.translations ?? [],
  );

  const hasResult = computed(() => translations.value.length > 0);
  const sourceLanguage = computed(
    () => currentResult.value?.sourceLanguage ?? "und",
  );
  const detectedLabel = computed(() => {
    if (!currentResult.value) {
      return "Waiting for translation";
    }

    const label = currentResult.value.sourceLanguage || "und";
    return currentResult.value.fallbackUsed
      ? `${label} · routed fallback`
      : label;
  });
  const statusLabel = computed(() => {
    if (loading.value) {
      return "Translating";
    }

    if (error.value) {
      return "Needs attention";
    }

    return hasResult.value ? "Ready" : "Idle";
  });

  function clearCopyFeedback() {
    if (copiedSourceTimer !== null) {
      window.clearTimeout(copiedSourceTimer);
      copiedSourceTimer = null;
    }

    if (copiedTargetTimer !== null) {
      window.clearTimeout(copiedTargetTimer);
      copiedTargetTimer = null;
    }
  }

  function resetTransientState() {
    clearCopyFeedback();
    copiedSource.value = false;
    copiedTargetLanguage.value = null;
    sourceExpanded.value = false;
    error.value = null;
  }

  function applyError(nextError: unknown) {
    currentResult.value = null;
    loading.value = false;
    error.value = toSafeError(nextError);
    visible.value = true;
  }

  async function translateCurrent() {
    const text = normalizeText(sourceText.value);
    if (!text) {
      applyError(new Error("No source text available"));
      return;
    }

    const sequence = ++requestSequence.value;
    loading.value = true;
    error.value = null;

    try {
      const result = await invoke<TranslationCommandOutput>("translate_text", {
        input: {
          text,
        } satisfies TranslationRequestInput,
      });

      if (sequence !== requestSequence.value) {
        return;
      }

      currentResult.value = result;
      sourceText.value = normalizeText(result.sourceText);
      sourceCharacterCount.value = sourceText.value.length;
      visible.value = true;
      loading.value = false;
      error.value = result.error ? { ...result.error } : null;
      copiedSource.value = false;
      copiedTargetLanguage.value = null;
      sourceExpanded.value = false;
    } catch (caught) {
      if (sequence !== requestSequence.value) {
        return;
      }

      applyError(caught);
    }
  }

  async function handleTrigger(payload: TranslationTriggerPayload) {
    resetTransientState();
    requestSource.value = payload.source;
    sourceText.value = normalizeText(payload.text);
    sourceCharacterCount.value = payload.characterCount;
    visible.value = true;
    currentResult.value = null;
    await translateCurrent();
  }

  async function retryTranslation() {
    await translateCurrent();
  }

  async function copySource() {
    if (!sourceText.value) {
      return;
    }

    try {
      await copyToClipboard(sourceText.value);
      copiedSource.value = true;
      if (copiedSourceTimer !== null) {
        window.clearTimeout(copiedSourceTimer);
      }
      copiedSourceTimer = window.setTimeout(() => {
        copiedSource.value = false;
        copiedSourceTimer = null;
      }, COPY_FEEDBACK_MS);
    } catch {
      // Copy failures should not break the popup flow.
    }
  }

  async function copyTarget(targetLanguage: string) {
    const translation = translations.value.find(
      (item) => item.targetLanguage === targetLanguage,
    );

    if (!translation) {
      return;
    }

    try {
      await copyToClipboard(translation.text);
      copiedTargetLanguage.value = targetLanguage;
      if (copiedTargetTimer !== null) {
        window.clearTimeout(copiedTargetTimer);
      }
      copiedTargetTimer = window.setTimeout(() => {
        copiedTargetLanguage.value = null;
        copiedTargetTimer = null;
      }, COPY_FEEDBACK_MS);
    } catch {
      // Copy failures should not break the popup flow.
    }
  }

  async function copyAllTranslations() {
    if (translations.value.length === 0) {
      return;
    }

    const combined = translations.value
      .map((item) => `[${item.targetLanguage}]\n${item.text}`)
      .join("\n\n");
    try {
      await copyToClipboard(combined);
    } catch {
      // Copy failures should not break the popup flow.
    }
  }

  function toggleSourceExpanded() {
    sourceExpanded.value = !sourceExpanded.value;
  }

  async function setPinned(nextPinned: boolean) {
    pinned.value = nextPinned;
    try {
      await getCurrentWindow().setAlwaysOnTop(nextPinned);
    } catch {
      // Keep the UI responsive even if the platform rejects the chrome change.
    }
  }

  async function togglePinned() {
    await setPinned(!pinned.value);
  }

  async function closePopup() {
    visible.value = false;
    try {
      await getCurrentWindow().hide();
    } catch {
      // The Rust close handler already falls back to hide-on-close behavior.
    }
  }

  return {
    visible,
    pinned,
    loading,
    sourceExpanded,
    copiedSource,
    copiedTargetLanguage,
    requestSource,
    sourceText,
    sourceCharacterCount,
    currentResult,
    error,
    translations,
    hasResult,
    sourceLanguage,
    detectedLabel,
    statusLabel,
    handleTrigger,
    retryTranslation,
    copySource,
    copyTarget,
    copyAllTranslations,
    toggleSourceExpanded,
    togglePinned,
    closePopup,
  };
});
