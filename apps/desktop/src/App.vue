<script setup lang="ts">
import { computed } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import AppShell from "@/components/AppShell.vue";
import PopupWindowShell from "@/components/window-shells/PopupWindowShell.vue";
import SettingsWindowShell from "@/components/window-shells/SettingsWindowShell.vue";
import { useUiStore } from "@/stores/ui";

const uiStore = useUiStore();
const currentWindow = getCurrentWindow();

const shellComponent = computed(() => {
  if (currentWindow.label === "translation-popup") {
    return PopupWindowShell;
  }

  if (currentWindow.label === "settings") {
    return SettingsWindowShell;
  }

  return AppShell;
});

uiStore.applyTheme(uiStore.theme);
uiStore.applyLocale(uiStore.locale);
</script>

<template>
  <component :is="shellComponent" />
</template>
