<template>
  <v-card border rounded="lg">
    <v-card-title class="d-flex align-center justify-space-between flex-wrap ga-2">
      <span>{{ t("settings.providers") }}</span>
      <div class="d-flex align-center ga-2 flex-wrap">
        <v-select
          v-model="autoRefreshSecs"
          :items="refreshOptions"
          item-title="label"
          item-value="value"
          density="compact"
          hide-details
          variant="outlined"
          style="min-width: 180px"
        />
        <v-btn
          size="small"
          variant="outlined"
          :loading="loading"
          @click="refreshProviders"
        >
          {{ t("settings.refreshProviders") }}
        </v-btn>
      </div>
    </v-card-title>

    <v-card-text>
      <div v-if="statusText" class="text-caption text-medium-emphasis mb-4">
        {{ statusText }}
      </div>

      <div v-if="error" class="text-body-2 text-error mb-4">
        {{ error }}
      </div>

      <v-list v-if="providers.length > 0" density="comfortable">
        <v-list-item v-for="provider in providers" :key="provider.id">
          <v-list-item-title class="d-flex align-center ga-2">
            <span>{{ provider.name || provider.id }}</span>
            <v-chip
              v-if="provider.id === activeProviderId"
              size="x-small"
              label
            >
              {{ t("settings.activeProvider") }}
            </v-chip>
          </v-list-item-title>
          <v-list-item-subtitle>
            {{ provider.baseUrl || "-" }}
          </v-list-item-subtitle>
          <v-list-item-subtitle>
            {{ provider.model || "-" }}
          </v-list-item-subtitle>
        </v-list-item>
      </v-list>

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
