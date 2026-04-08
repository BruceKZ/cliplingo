<template>
  <section class="providers-directory">
    <div class="directory-header">
      <div>
        <h1 class="text-h5 mb-1">{{ t("nav.providers") }}</h1>
        <p class="text-body-2 text-medium-emphasis">
          {{ t("settings.providersDirectoryHelp") }}
        </p>
      </div>
      <v-btn class="app-btn header-action-btn" color="primary" variant="tonal" prepend-icon="mdi-server-plus-outline" @click="handleAddProvider">
        {{ t("settings.addProvider") }}
      </v-btn>
    </div>

    <template v-if="providersStore.providers.length > 0">
      <div v-if="activeProvider" class="directory-group">
        <div class="group-header">
          <h2 class="group-title">{{ t("settings.activeProvider") }}</h2>
        </div>
        <div class="directory-list">
          <v-card
            :key="activeProvider.id"
            variant="outlined"
            rounded="lg"
            class="provider-directory-card provider-directory-card--active"
          >
            <v-card-text class="provider-card-body">
              <div class="provider-card-main">
                <div class="provider-badge">
                  {{ providerInitial(activeProvider) }}
                </div>
                <div class="provider-copy">
                  <div class="provider-topline">
                    <h3 class="provider-name">{{ activeProvider.name || t("settings.unnamedProvider") }}</h3>
                    <v-chip size="small" color="primary" variant="tonal">
                      {{ t("settings.activeProvider") }}
                    </v-chip>
                  </div>
                  <div class="provider-meta">
                    <div class="meta-pill">
                      <span class="meta-label">{{ t("settings.model") }}</span>
                      <span class="meta-value">{{ activeProvider.model || "-" }}</span>
                    </div>
                    <div class="meta-pill">
                      <span class="meta-label">{{ t("settings.providerValidation") }}</span>
                      <span class="meta-value">
                        {{ activeProvider.verifiedAt ? t("settings.providerVerified") : t("settings.providerNeedsVerification") }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              <div class="provider-actions">
                <v-btn class="app-btn app-btn--compact" color="primary" variant="tonal" prepend-icon="mdi-pencil-outline" @click="openProvider(activeProvider.id)">
                  {{ t("settings.edit") }}
                </v-btn>
                <v-btn class="app-btn app-btn--compact" color="primary" variant="tonal" prepend-icon="mdi-content-copy" @click="duplicateProvider(activeProvider.id)">
                  {{ t("settings.duplicate") }}
                </v-btn>
              </div>
            </v-card-text>
          </v-card>
        </div>
      </div>

      <div v-if="inactiveProviders.length > 0" class="directory-group">
        <div class="group-header">
          <h2 class="group-title">{{ t("settings.otherProviders") }}</h2>
        </div>
        <div class="directory-list">
          <v-card
            v-for="provider in inactiveProviders"
            :key="provider.id"
            variant="outlined"
            rounded="lg"
            class="provider-directory-card"
          >
            <v-card-text class="provider-card-body">
              <div class="provider-card-main">
                <div class="provider-badge">
                  {{ providerInitial(provider) }}
                </div>
                <div class="provider-copy">
                  <div class="provider-topline">
                    <h3 class="provider-name">{{ provider.name || t("settings.unnamedProvider") }}</h3>
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

              <div class="provider-actions">
                <v-btn class="app-btn app-btn--compact" color="primary" variant="tonal" prepend-icon="mdi-check-circle-outline" :disabled="!provider.verifiedAt" @click="makeProviderActive(provider.id)">
                  {{ t("settings.makeActive") }}
                </v-btn>
                <v-btn class="app-btn app-btn--compact" color="primary" variant="tonal" prepend-icon="mdi-pencil-outline" @click="openProvider(provider.id)">
                  {{ t("settings.edit") }}
                </v-btn>
                <v-btn class="app-btn app-btn--compact" color="primary" variant="tonal" prepend-icon="mdi-content-copy" @click="duplicateProvider(provider.id)">
                  {{ t("settings.duplicate") }}
                </v-btn>
              </div>
            </v-card-text>
          </v-card>
        </div>
      </div>
    </template>

    <v-card
      v-else
      variant="outlined"
      rounded="lg"
      class="provider-directory-card empty-card"
    >
      <v-card-text class="py-12 text-center">
        <div class="text-body-1 mb-2">{{ t("settings.noProvidersYet") }}</div>
        <div class="text-body-2 text-medium-emphasis mb-6">
          {{ t("settings.providersDirectoryEmpty") }}
        </div>
        <v-btn class="app-btn header-action-btn" color="primary" variant="tonal" prepend-icon="mdi-server-plus-outline" @click="handleAddProvider">
          {{ t("settings.addProvider") }}
        </v-btn>
      </v-card-text>
    </v-card>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "@/i18n";
import { useProvidersStore, type ProviderDraft } from "@/stores/providers";

const router = useRouter();
const providersStore = useProvidersStore();
const { t } = useI18n();

const activeProvider = computed(
  () =>
    providersStore.providers.find((provider) => provider.id === providersStore.activeProviderId) ??
    null,
);

const inactiveProviders = computed(() =>
  providersStore.providers.filter((provider) => provider.id !== providersStore.activeProviderId),
);

onMounted(() => {
  providersStore.refresh(true).catch(() => undefined);
});

function openProvider(providerId: string) {
  providersStore.selectProvider(providerId);
  router.push({ name: "provider-detail", params: { providerId } }).catch(() => undefined);
}

function handleAddProvider() {
  const providerId = providersStore.addProvider();
  router.push({ name: "provider-detail", params: { providerId } }).catch(() => undefined);
}

function duplicateProvider(providerId: string) {
  const nextId = providersStore.duplicateProvider(providerId);
  if (nextId) {
    router.push({ name: "provider-detail", params: { providerId: nextId } }).catch(() => undefined);
  }
}

function providerInitial(provider: ProviderDraft) {
  const source = provider.name.trim() || provider.baseUrl.trim() || "P";
  return source.slice(0, 1).toUpperCase();
}

function makeProviderActive(providerId: string) {
  providersStore.makeProviderActive(providerId).catch(() => undefined);
}
</script>

<style scoped>
.providers-directory {
  display: grid;
  gap: 24px;
}

.directory-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}

.directory-group {
  display: grid;
  gap: 12px;
}

.group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding-top: 4px;
}

.group-title {
  font-size: 0.92rem;
  line-height: 1.3;
  font-weight: 700;
  color: var(--color-muted);
}

.directory-list {
  display: grid;
  gap: 12px;
}

.provider-directory-card {
  border-color: var(--color-line);
  transition: border-color 0.16s ease, background-color 0.16s ease;
}

.provider-directory-card:hover {
  border-color: var(--color-accent);
}

.provider-directory-card--active {
  background: color-mix(in srgb, var(--color-accent-soft) 20%, var(--color-panel) 80%);
}

.provider-card-body {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 18px 20px;
}

.provider-card-main {
  min-width: 0;
  display: flex;
  align-items: flex-start;
  gap: 14px;
}

.provider-badge {
  width: 32px;
  height: 32px;
  flex: 0 0 auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 9px;
  background: color-mix(in srgb, var(--color-accent-soft) 82%, var(--color-panel) 18%);
  color: var(--color-accent);
  font-weight: 700;
}

.provider-copy {
  min-width: 0;
  display: grid;
  gap: 8px;
}

.provider-topline {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.provider-name {
  font-size: 1.06rem;
  line-height: 1.25;
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

.providers-directory :deep(.app-btn) {
  min-height: 40px;
  min-width: 96px;
  text-transform: none;
  font-weight: 600;
  font-size: 0.92rem;
  line-height: 1;
  letter-spacing: 0;
  white-space: nowrap;
}

.providers-directory :deep(.app-btn .v-icon) {
  font-size: 1rem;
}

.providers-directory :deep(.header-action-btn) {
  min-width: 156px;
}

.providers-directory :deep(.app-btn--compact) {
  min-width: auto;
  padding-inline: 14px;
}

.meta-value {
  min-width: 0;
  font-size: 0.82rem;
  line-height: 1.35;
  color: var(--color-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.provider-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  justify-content: flex-end;
  flex: 0 0 auto;
}

.empty-card {
  cursor: default;
}

.empty-card:hover {
  border-color: var(--color-line);
}

@media (max-width: 959px) {
  .provider-card-body {
    flex-direction: column;
    align-items: stretch;
  }

  .provider-actions {
    justify-content: flex-start;
  }
}
</style>
