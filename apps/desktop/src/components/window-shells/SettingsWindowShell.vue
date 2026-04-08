<template>
  <v-app :theme="uiStore.resolvedTheme">
    <v-main class="bg-background">
      <v-container fluid class="settings-container">
        <section class="settings-page">
          <div class="settings-shell">
            <div class="settings-header">
              <div>
                <h1 class="text-h5 mb-1">{{ t("settings.title") }}</h1>
              </div>
            </div>

            <AppSettingsWorkspace />
          </div>
        </section>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import AppSettingsWorkspace from "@/components/settings/AppSettingsWorkspace.vue";
import { useI18n } from "@/i18n";
import { useUiStore } from "@/stores/ui";

const uiStore = useUiStore();
const { t } = useI18n();
</script>

<style scoped>
.settings-container {
  padding: 24px;
}

.settings-page {
  display: grid;
  gap: 24px;
}

.settings-shell {
  display: grid;
  gap: 24px;
}

.settings-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}

@media (max-width: 959px) {
  .settings-container {
    padding: 16px;
  }
}
</style>
