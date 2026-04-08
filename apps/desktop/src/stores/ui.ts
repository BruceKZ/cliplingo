import { computed, ref } from "vue";
import { defineStore } from "pinia";
import type { ThemeMode } from "@/types";

const DARK_QUERY = "(prefers-color-scheme: dark)";
const THEME_STORAGE_KEY = "cliplingo.theme";
const LOCALE_STORAGE_KEY = "cliplingo.locale";

export type AppLocale = "en" | "zh-CN";

function getInitialTheme(): ThemeMode {
  const stored = window.localStorage.getItem(THEME_STORAGE_KEY);
  if (stored === "light" || stored === "dark" || stored === "system") {
    return stored;
  }

  return "system";
}

function getInitialLocale(): AppLocale {
  const stored = window.localStorage.getItem(LOCALE_STORAGE_KEY);
  if (stored === "en" || stored === "zh-CN") {
    return stored;
  }

  return "en";
}

export const useUiStore = defineStore("ui", () => {
  const theme = ref<ThemeMode>(getInitialTheme());
  const locale = ref<AppLocale>(getInitialLocale());

  const resolvedTheme = computed<"light" | "dark">(() => {
    if (theme.value === "system") {
      return window.matchMedia(DARK_QUERY).matches ? "dark" : "light";
    }

    return theme.value;
  });

  function applyTheme(nextTheme: ThemeMode) {
    theme.value = nextTheme;
    window.localStorage.setItem(THEME_STORAGE_KEY, nextTheme);
    const root = document.documentElement;
    root.classList.toggle("dark", resolvedTheme.value === "dark");
  }

  function applyLocale(nextLocale: AppLocale) {
    locale.value = nextLocale;
    window.localStorage.setItem(LOCALE_STORAGE_KEY, nextLocale);
    document.documentElement.lang = nextLocale;
  }

  return {
    theme,
    locale,
    resolvedTheme,
    applyTheme,
    applyLocale,
  };
});
