<template>
  <section class="settings-view">
    <div class="d-flex justify-space-between align-center flex-wrap ga-2 mb-4">
      <h1 class="text-h5">{{ t("settings.title") }}</h1>
      <v-btn color="primary" variant="tonal" to="/providers">
        {{ t("settings.manageProviders") }}
      </v-btn>
    </div>
    <ProviderOverviewPanel />
  </section>
</template>

<script setup lang="ts">
import ProviderOverviewPanel from "@/components/settings/ProviderOverviewPanel.vue";
import { useI18n } from "@/i18n";

const { t } = useI18n();
</script>
