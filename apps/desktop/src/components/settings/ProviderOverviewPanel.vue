<template>
  <v-card variant="outlined" rounded="lg" class="overview-card">
    <v-card-item>
      <div class="panel-header">
        <div class="section-copy">
          <h2 class="panel-title">{{ t("settings.providers") }}</h2>
          <p class="panel-subtitle">{{ t("settings.providersHelp") }}</p>
        </div>
        <div class="panel-actions">
          <div class="refresh-select">
            <span class="field-label">{{ t("settings.refreshProviders") }}</span>
            <v-select
              v-model="autoRefreshSecs"
              :items="refreshOptions"
              item-title="label"
              item-value="value"
              density="comfortable"
              hide-details
              variant="outlined"
              class="refresh-field"
            />
          </div>
          <v-btn
            class="app-btn app-btn--compact"
            color="primary"
            variant="tonal"
            prepend-icon="mdi-refresh"
            :loading="loading"
            @click="refreshProviders"
          >
            {{ t("settings.refreshProviders") }}
          </v-btn>
        </div>
      </div>
    </v-card-item>

    <v-divider />

    <v-card-text class="overview-body">
      <div v-if="statusText" class="status-line">
        {{ statusText }}
      </div>

      <div v-if="error" class="text-body-2 text-error mb-4">
        {{ error }}
      </div>

      <div v-if="providers.length > 0" class="provider-list">
        <div v-for="provider in providers" :key="provider.id" class="provider-row">
          <div class="provider-copy">
            <div class="provider-name-row">
              <span class="provider-name">{{ provider.name || t("settings.unnamedProvider") }}</span>
              <v-chip
                v-if="provider.id === activeProviderId"
                size="small"
                color="primary"
                variant="tonal"
              >
                {{ t("settings.activeProvider") }}
              </v-chip>
            </div>
            <div class="provider-meta">
              <div class="meta-pill">
                <span class="meta-label">{{ t("settings.model") }}</span>
                <span class="meta-value">{{ provider.model || "-" }}</span>
              </div>
              <div class="meta-pill">
                <span class="meta-label">{{ t("settings.providerValidation") }}</span>
                <span class="meta-value">
                  {{ provider.verifiedAt ? t("settings.providerVerified") : t("settings.providerNeedsVerification") }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="text-body-2 text-medium-emphasis py-4">
        {{ t("settings.noProvidersYet") }}
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ProviderConfig, ProviderDirectory } from "@cliplingo/shared-types";
import { useI18n } from "@/i18n";

const { t } = useI18n();

const providers = ref<ProviderConfig[]>([]);
const activeProviderId = ref<string | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);
const lastUpdatedAt = ref<number | null>(null);
const autoRefreshSecs = ref(0);

const refreshOptions = computed(() => [
  { label: t("settings.autoRefreshOff"), value: 0 },
  { label: "15s", value: 15 },
  { label: "30s", value: 30 },
  { label: "60s", value: 60 },
]);

const statusText = computed(() => {
  if (!lastUpdatedAt.value) {
    return "";
  }

  return `${t("settings.statusAt")}: ${new Date(lastUpdatedAt.value).toLocaleTimeString()}`;
});

let timerId: number | null = null;

async function refreshProviders() {
  loading.value = true;
  error.value = null;

  try {
    const directory = await invoke<ProviderDirectory>("list_providers");
    providers.value = directory.providers;
    activeProviderId.value = directory.activeProviderId;
    lastUpdatedAt.value = Date.now();
  } catch (cause) {
    error.value = cause instanceof Error ? cause.message : String(cause);
  } finally {
    loading.value = false;
  }
}

function clearTimer() {
  if (timerId !== null) {
    window.clearInterval(timerId);
    timerId = null;
  }
}

watch(autoRefreshSecs, (value) => {
  clearTimer();
  if (value > 0) {
    timerId = window.setInterval(() => {
      void refreshProviders();
    }, value * 1_000);
  }
});

onMounted(() => {
  void refreshProviders();
});

onBeforeUnmount(() => {
  clearTimer();
});
</script>

<style scoped>
.overview-card {
  border-color: var(--color-line);
}

.panel-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}

.section-copy {
  display: grid;
  gap: 6px;
}

.panel-title {
  margin: 0;
  font-size: 1rem;
  line-height: 1.35;
  font-weight: 600;
  color: var(--color-text);
}

.panel-subtitle {
  margin: 0;
  font-size: 0.92rem;
  line-height: 1.45;
  color: var(--color-muted);
}

.panel-actions {
  display: flex;
  align-items: flex-end;
  gap: 10px;
  flex-wrap: wrap;
}

.refresh-select {
  display: grid;
  gap: 8px;
}

.field-label {
  font-size: 0.875rem;
  font-weight: 600;
  line-height: 1.35;
  color: var(--color-text);
}

.refresh-field {
  min-width: 180px;
}

.overview-body {
  padding-top: 20px;
}

.status-line {
  margin-bottom: 16px;
  font-size: 0.8rem;
  line-height: 1.4;
  color: var(--color-muted);
}

.provider-list {
  display: grid;
  gap: 12px;
}

.provider-row {
  padding: 16px;
  border: 1px solid var(--color-line);
  border-radius: 12px;
  background: color-mix(in srgb, var(--color-panel) 96%, var(--color-bg) 4%);
}

.provider-copy {
  display: grid;
  gap: 10px;
}

.provider-name-row {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.provider-name {
  font-size: 1rem;
  line-height: 1.3;
  font-weight: 600;
  color: var(--color-text);
}

.provider-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.meta-pill {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
  padding: 6px 10px;
  border: 1px solid var(--color-line);
  border-radius: 999px;
  background: var(--color-panel);
}

.meta-label {
  font-size: 0.76rem;
  font-weight: 600;
  line-height: 1.35;
  color: var(--color-muted);
}

.meta-value {
  min-width: 0;
  font-size: 0.82rem;
  line-height: 1.35;
  color: var(--color-text);
}

:deep(.refresh-field .v-field--variant-outlined .v-field__outline) {
  --v-field-border-opacity: 1;
}

:deep(.refresh-field .v-field) {
  background: var(--color-panel);
  min-height: 40px;
}

:deep(.refresh-field .v-field__input) {
  min-height: 40px;
  padding-top: 6px;
  padding-bottom: 6px;
  padding-inline: 12px;
  font-size: 0.94rem;
  line-height: 1.4;
  color: var(--color-text);
}

:deep(.app-btn) {
  min-height: 40px;
  min-width: 112px;
  text-transform: none;
  font-weight: 600;
  font-size: 0.92rem;
  line-height: 1;
  letter-spacing: 0;
  white-space: nowrap;
}

:deep(.app-btn .v-icon) {
  font-size: 1rem;
}

:deep(.app-btn--compact) {
  min-width: 96px;
  padding-inline: 14px;
}

@media (max-width: 959px) {
  .panel-actions {
    align-items: stretch;
  }

  .refresh-field {
    min-width: 0;
    width: 100%;
  }
}
</style>
