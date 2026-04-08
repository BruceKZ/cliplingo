import { computed, reactive } from "vue";
import { useTranslationStore } from "@/stores/translation";

export function useTranslationPopupViewModel() {
  const store = useTranslationStore();

  const headerTitle = computed(() => {
    if (store.loading) {
      return "Preparing translation";
    }

    if (store.error) {
      return "Translation needs attention";
    }

    if (store.hasResult) {
      return "Translation ready";
    }

    return "Waiting for a copy trigger";
  });

  const headerCopy = computed(() => {
    if (store.loading) {
      return "The popup is waiting on the OpenAI-compatible provider and will update as soon as the response arrives.";
    }

    if (store.error) {
      return "The request failed safely. Source text remains visible, and the retry action reuses the latest clipboard content.";
    }

    if (store.hasResult) {
      return "The source text stays foldable, each target translation is copyable on its own, and the popup can be pinned while you work.";
    }

    return "Trigger the app with double copy or the fallback shortcut to fetch the current clipboard, route target languages, and display a compact result window.";
  });

  const sourceEyebrow = computed(() =>
    store.requestSource === "fallbackShortcut"
      ? "Fallback shortcut"
      : "Double copy",
  );
  const sourceTitle = computed(() => "Original clipboard text");
  const sourceSubtitle = computed(
    () =>
      `${store.sourceCharacterCount} characters · safe plain-text rendering`,
  );
  const sourceLanguageLabel = computed(
    () => `Detected ${store.sourceLanguage}`,
  );
  const sourceOverflowHint = computed(
    () => store.sourceText.length > 320 && !store.sourceExpanded,
  );
  const sourceFooterText = computed(() => {
    if (!store.sourceText) {
      return "No clipboard text is currently loaded.";
    }

    return store.sourceExpanded
      ? "The full clipboard content is shown above."
      : "Expand to inspect the full clipboard content without losing the current translation context.";
  });

  const resultHeading = computed(() => {
    if (store.loading) {
      return "Active translation request";
    }

    if (store.error) {
      return "Safe fallback state";
    }

    return store.hasResult
      ? `${store.translations.length} translated section${store.translations.length === 1 ? "" : "s"}`
      : "No translated sections yet";
  });

  const resultSubtitle = computed(() => {
    if (store.loading) {
      return "System rules are locked first, then the current clipboard is sent to the configured provider.";
    }

    if (store.error) {
      return "The popup keeps the source text visible while surfacing a safe, non-HTML error message.";
    }

    return store.hasResult
      ? "Each result card is individually copyable and scrollable."
      : "The layout is already prepared for multi-target output.";
  });

  const providerLabel = computed(
    () => store.currentResult?.providerName || "Provider pending",
  );
  const modelLabel = computed(
    () => store.currentResult?.model || "Model pending",
  );
  const footerStatus = computed(() => {
    if (store.error) {
      return `Error: ${store.error.code}`;
    }

    if (store.loading) {
      return "Status: translating";
    }

    if (!store.hasResult) {
      return "Status: idle";
    }

    return store.currentResult?.fallbackUsed
      ? "Status: fallback routed"
      : "Status: ready";
  });
  const footerTiming = computed(() => {
    if (!store.currentResult) {
      return "Timing: pending";
    }

    return `Latency: ${store.currentResult.latencyMs} ms`;
  });
  const footerTargetCount = computed(
    () => `Targets: ${store.translations.length || 0}`,
  );
  function targetEyebrow(index: number) {
    return index === 0 ? "Primary target" : "Secondary target";
  }

  function targetTitle(language: string) {
    return `${language} translation`;
  }

  function targetSubtitle(index: number) {
    return index === 0
      ? "The first target stays prominent and copyable."
      : "Additional targets appear as separate sections.";
  }

  function targetIndexLabel(index: number) {
    return `Section ${index + 1}`;
  }

  function targetFooterText(index: number) {
    return index === 0
      ? "Use the copy action or the footer copy-all control."
      : "Each translated section keeps its own clipboard action.";
  }

  function targetTone(index: number): "neutral" | "sky" | "emerald" {
    if (index === 0) {
      return "sky";
    }

    return index % 2 === 0 ? "emerald" : "neutral";
  }

  return reactive({
    store,
    headerTitle,
    headerCopy,
    sourceEyebrow,
    sourceTitle,
    sourceSubtitle,
    sourceLanguageLabel,
    sourceOverflowHint,
    sourceFooterText,
    resultHeading,
    resultSubtitle,
    providerLabel,
    modelLabel,
    footerStatus,
    footerTiming,
    footerTargetCount,
    targetEyebrow,
    targetTitle,
    targetSubtitle,
    targetIndexLabel,
    targetFooterText,
    targetTone,
  });
}
