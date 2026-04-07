import { onBeforeUnmount, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  TRANSLATION_TRIGGER_EVENT,
  type TranslationTriggerPayload,
} from "./types";
import { useTranslationStore } from "@/stores/translation";

export function useTranslationPopup() {
  const store = useTranslationStore();
  let unlistenTrigger: (() => void) | null = null;

  async function bindTriggerListener() {
    unlistenTrigger = await listen<TranslationTriggerPayload>(
      TRANSLATION_TRIGGER_EVENT,
      async (event) => {
        await store.handleTrigger(event.payload);
      },
    );
  }

  async function handleEscape(event: KeyboardEvent) {
    if (event.key !== "Escape") {
      return;
    }

    event.preventDefault();
    await store.closePopup();
  }

  onMounted(async () => {
    await bindTriggerListener();
    window.addEventListener("keydown", handleEscape);

    try {
      await getCurrentWindow().setAlwaysOnTop(store.pinned);
    } catch {
      // The window can still render without the platform chrome adjustment.
    }
  });

  onBeforeUnmount(() => {
    unlistenTrigger?.();
    window.removeEventListener("keydown", handleEscape);
    store.closePopup().catch(() => undefined);
  });

  return {
    store,
  };
}
