<template>
  <section class="settings-view">
    <h1 class="text-h5 mb-4">{{ t("settings.title") }}</h1>
    <SettingsWorkspace />
  </section>
</template>

<script setup lang="ts">
import SettingsWorkspace from "@/components/settings/SettingsWorkspace.vue";
import { useI18n } from "@/i18n";

const { t } = useI18n();
</script>
