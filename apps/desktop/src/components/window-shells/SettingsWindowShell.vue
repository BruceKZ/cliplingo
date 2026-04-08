<template>
  <v-app :theme="uiStore.resolvedTheme">
    <v-main class="bg-background">
      <v-container fluid class="pa-4 pa-md-6">
        <h1 class="text-h5 mb-4">{{ t("settings.title") }}</h1>
        <SettingsWorkspace compact />
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import SettingsWorkspace from "@/components/settings/SettingsWorkspace.vue";
import { useI18n } from "@/i18n";
import { useUiStore } from "@/stores/ui";

const uiStore = useUiStore();
const { t } = useI18n();
</script>
