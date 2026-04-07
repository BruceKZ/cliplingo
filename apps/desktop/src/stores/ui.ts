import { computed, ref } from "vue";
import { defineStore } from "pinia";
import type { ThemeMode } from "@/types";

const DARK_QUERY = "(prefers-color-scheme: dark)";

export const useUiStore = defineStore("ui", () => {
  const theme = ref<ThemeMode>("system");

  const resolvedTheme = computed<"light" | "dark">(() => {
    if (theme.value === "system") {
      return window.matchMedia(DARK_QUERY).matches ? "dark" : "light";
    }

    return theme.value;
  });

  function applyTheme(nextTheme: ThemeMode) {
    theme.value = nextTheme;
    const root = document.documentElement;
    root.classList.toggle("dark", resolvedTheme.value === "dark");
  }

  return {
    theme,
    resolvedTheme,
    applyTheme,
  };
});
