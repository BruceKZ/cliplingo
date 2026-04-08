<template>
  <section class="provider-settings">
    <v-row dense>
      <v-col cols="12" lg="4" xl="3">
        <v-card variant="outlined" rounded="lg" class="provider-panel">
          <v-card-item class="pb-2">
            <v-card-title>{{ t("settings.providers") }}</v-card-title>
            <v-card-subtitle>{{ t("settings.providerListHelp") }}</v-card-subtitle>
            <template #append>
              <v-btn size="small" variant="tonal" @click="providersStore.addProvider">
                {{ t("settings.addProvider") }}
              </v-btn>
            </template>
          </v-card-item>
          <v-divider />
          <v-list v-if="providersStore.providers.length > 0" nav density="comfortable" class="py-2">
            <v-list-item
              v-for="provider in providersStore.providers"
              :key="provider.id"
              :active="provider.id === providersStore.selectedProviderId"
              rounded="lg"
              @click="providersStore.selectProvider(provider.id)"
            >
              <v-list-item-title>{{ provider.name || provider.id }}</v-list-item-title>
              <v-list-item-subtitle>{{ provider.baseUrl || provider.id }}</v-list-item-subtitle>
              <template #append>
                <v-chip
                  v-if="provider.id === providersStore.activeProviderId"
                  size="x-small"
                  color="primary"
                  variant="tonal"
                >
                  {{ t("settings.activeProvider") }}
                </v-chip>
              </template>
            </v-list-item>
          </v-list>
          <div v-else class="px-6 py-8 text-body-2 text-medium-emphasis">
            {{ t("settings.noProvidersYet") }}
          </div>
        </v-card>
      </v-col>

      <v-col cols="12" lg="8" xl="9">
        <v-card v-if="selectedProvider" variant="outlined" rounded="lg" class="provider-panel">
          <v-card-item class="pb-0">
            <div class="d-flex flex-wrap align-start justify-space-between ga-3 w-100">
              <div>
                <div class="d-flex align-center ga-2 flex-wrap">
                  <h2 class="text-h6 font-weight-medium mb-0">
                    {{ selectedProvider.name || selectedProvider.id }}
                  </h2>
                  <v-chip
                    v-if="selectedProvider.id === providersStore.activeProviderId"
                    size="small"
                    color="primary"
                    variant="tonal"
                  >
                    {{ t("settings.activeProvider") }}
                  </v-chip>
                </div>
                <p class="text-body-2 text-medium-emphasis mt-1">
                  {{ t("settings.providersHelp") }}
                </p>
              </div>

              <div class="d-flex ga-2 flex-wrap">
                <v-btn size="small" variant="outlined" @click="providersStore.duplicateProvider(selectedProvider.id)">
                  Duplicate
                </v-btn>
                <v-btn
                  size="small"
                  color="error"
                  variant="outlined"
                  @click="providersStore.removeProvider(selectedProvider.id)"
                >
                  {{ t("settings.remove") }}
                </v-btn>
              </div>
            </div>
          </v-card-item>

          <v-card-text class="pt-4">
            <v-alert
              class="mb-6"
              variant="tonal"
              type="info"
              density="comfortable"
              icon="mdi-information-outline"
            >
              {{ t("settings.providerRequiredHint") }}
            </v-alert>

            <v-form>
              <div class="settings-section">
                <div class="settings-section__header">
                  <h3 class="text-subtitle-1 font-weight-medium">{{ t("settings.providerBasics") }}</h3>
                  <p class="text-body-2 text-medium-emphasis">
                    {{ t("settings.providerBasicsHint") }}
                  </p>
                </div>
                <v-row dense>
                  <v-col cols="12" md="6">
                    <v-text-field
                      v-model.trim="selectedProvider.name"
                      class="provider-field"
                      :label="t('settings.displayName')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-text-field
                      v-model.trim="selectedProvider.id"
                      class="provider-field"
                      :label="t('settings.providerId')"
                      :hint="t('settings.providerIdHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-select
                      v-model="selectedProvider.kind"
                      class="provider-field"
                      :items="providerKindOptions"
                      :label="t('settings.providerKind')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      disabled
                    />
                  </v-col>
                </v-row>
              </div>

              <v-divider class="my-6" />

              <div class="settings-section">
                <div class="settings-section__header">
                  <h3 class="text-subtitle-1 font-weight-medium">{{ t("settings.connectionSettings") }}</h3>
                  <p class="text-body-2 text-medium-emphasis">
                    {{ t("settings.connectionSettingsHint") }}
                  </p>
                </div>
                <v-row dense>
                  <v-col cols="12">
                    <v-text-field
                      v-model.trim="selectedProvider.baseUrl"
                      class="provider-field"
                      :label="t('settings.baseUrl')"
                      :hint="t('settings.baseUrlHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      placeholder="https://api.example.com"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-text-field
                      v-model.trim="selectedProvider.path"
                      class="provider-field"
                      :label="t('settings.requestPath')"
                      :hint="t('settings.requestPathHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      placeholder="/chat/completions"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-select
                      v-model="selectedProvider.authScheme"
                      class="provider-field"
                      :items="authSchemeOptions"
                      item-title="title"
                      item-value="value"
                      :label="t('settings.authScheme')"
                      :hint="t('settings.authSchemeHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12">
                    <v-text-field
                      v-model.trim="selectedProvider.apiKeyDraft"
                      class="provider-field"
                      :label="t('settings.apiKeyDraft')"
                      :hint="t('settings.apiKeyHint')"
                      :type="selectedProvider.authScheme === 'none' ? 'text' : 'password'"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      :placeholder="selectedProvider.hasSecret ? undefined : 'sk-...'"
                    />
                    <div class="d-flex align-center ga-2 flex-wrap mt-2">
                      <v-chip
                        v-if="selectedProvider.hasSecret"
                        size="small"
                        color="success"
                        variant="tonal"
                      >
                        {{ t("settings.apiKeySaved") }}
                      </v-chip>
                      <v-btn
                        v-if="selectedProvider.hasSecret || selectedProvider.apiKeyDraft"
                        size="small"
                        color="error"
                        variant="text"
                        @click="providersStore.clearProviderSecret(selectedProvider.id)"
                      >
                        {{ t("settings.clearApiKey") }}
                      </v-btn>
                    </div>
                  </v-col>
                </v-row>
              </div>

              <v-divider class="my-6" />

              <div class="settings-section">
                <div class="settings-section__header">
                  <h3 class="text-subtitle-1 font-weight-medium">{{ t("settings.requestSettings") }}</h3>
                  <p class="text-body-2 text-medium-emphasis">
                    {{ t("settings.requestSettingsHint") }}
                  </p>
                </div>
                <v-row dense>
                  <v-col cols="12" md="6">
                    <v-text-field
                      v-model.trim="selectedProvider.model"
                      class="provider-field"
                      :label="t('settings.model')"
                      :hint="t('settings.modelHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      placeholder="gpt-4o-mini"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <v-text-field
                      v-model.number="selectedProvider.timeoutSecs"
                      class="provider-field"
                      :label="t('settings.requestTimeout')"
                      :hint="t('settings.requestTimeoutHint')"
                      type="text"
                      inputmode="numeric"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                </v-row>
              </div>

              <v-divider class="my-6" />

              <div class="settings-section">
                <div class="settings-section__header">
                  <h3 class="text-subtitle-1 font-weight-medium">{{ t("settings.advancedSettings") }}</h3>
                  <p class="text-body-2 text-medium-emphasis">
                    {{ t("settings.advancedSettingsHint") }}
                  </p>
                </div>
                <v-row dense>
                  <v-col cols="12" md="4">
                    <v-text-field
                      v-model.number="selectedProvider.temperature"
                      class="provider-field"
                      :label="t('settings.temperature')"
                      :hint="t('settings.temperatureHint')"
                      type="text"
                      inputmode="decimal"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-text-field
                      v-model.number="selectedProvider.topP"
                      class="provider-field"
                      :label="t('settings.topP')"
                      :hint="t('settings.topPHint')"
                      type="text"
                      inputmode="decimal"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-text-field
                      v-model.number="selectedProvider.maxTokens"
                      class="provider-field"
                      :label="t('settings.maxTokens')"
                      :hint="t('settings.maxTokensHint')"
                      type="text"
                      inputmode="numeric"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                </v-row>
              </div>

              <v-divider class="my-6" />

              <div class="settings-section">
                <div class="d-flex align-start justify-space-between flex-wrap ga-3 mb-4">
                  <div class="settings-section__header mb-0">
                    <h3 class="text-subtitle-1 font-weight-medium">{{ t("settings.customHeaders") }}</h3>
                    <p class="text-body-2 text-medium-emphasis">
                      {{ t("settings.headersHint") }}
                    </p>
                  </div>
                  <v-btn size="small" variant="outlined" @click="providersStore.addProviderHeader(selectedProvider.id)">
                    {{ t("settings.addHeader") }}
                  </v-btn>
                </div>

                <div v-if="selectedProvider.customHeaders.length === 0" class="text-body-2 text-medium-emphasis py-2">
                  {{ t("settings.noHeaders") }}
                </div>

                <div v-else class="header-list">
                  <div
                    v-for="(header, headerIndex) in selectedProvider.customHeaders"
                    :key="`${selectedProvider.id}-${headerIndex}`"
                    class="header-row"
                  >
                    <v-row dense>
                      <v-col cols="12" md="5">
                        <v-text-field
                          v-model.trim="header.name"
                          class="provider-field"
                          :label="t('settings.headerName')"
                          variant="outlined"
                          density="comfortable"
                          hide-details="auto"
                        />
                      </v-col>
                      <v-col cols="12" md="5">
                        <v-text-field
                          v-model.trim="header.value"
                          class="provider-field"
                          :label="t('settings.headerValue')"
                          variant="outlined"
                          density="comfortable"
                          hide-details="auto"
                        />
                      </v-col>
                      <v-col cols="12" md="2">
                        <v-btn
                          block
                          color="error"
                          variant="outlined"
                          class="header-remove-btn"
                          @click="providersStore.removeProviderHeader(selectedProvider.id, headerIndex)"
                        >
                          {{ t("settings.remove") }}
                        </v-btn>
                      </v-col>
                    </v-row>
                  </div>
                </div>
              </div>
            </v-form>
          </v-card-text>

          <v-divider />

          <v-card-actions class="px-6 py-4">
            <v-row dense class="w-100">
              <v-col cols="12" md="4">
                <v-btn block variant="outlined" :loading="providersStore.persistState === 'saving'" @click="providersStore.persistCurrent()">
                  {{ providersStore.persistState === "saving" ? t("settings.saving") : t("settings.save") }}
                </v-btn>
              </v-col>
              <v-col cols="12" md="4">
                <v-btn block color="primary" :loading="providersStore.testState === 'running'" @click="providersStore.testConnection()">
                  {{ providersStore.testState === "running" ? t("settings.testing") : t("settings.testTranslation") }}
                </v-btn>
              </v-col>
              <v-col cols="12" md="4">
                <v-btn block color="primary" variant="tonal" @click="providersStore.makeProviderActive(selectedProvider.id)">
                  {{ t("settings.makeActive") }}
                </v-btn>
              </v-col>
            </v-row>
          </v-card-actions>

          <v-divider />

          <v-card-text v-if="statusLine" class="pt-4">
            <v-alert
              :type="statusToneType"
              variant="tonal"
              density="comfortable"
              :icon="statusIcon"
            >
              <div class="text-body-2 font-weight-medium">
                {{ statusToneLabel }}
              </div>
              <div class="text-body-2">
                {{ statusText }}
              </div>
            </v-alert>
          </v-card-text>
        </v-card>

        <v-card v-else variant="outlined" rounded="lg" class="provider-panel">
          <v-card-text class="d-flex flex-column align-center justify-center py-12 ga-4">
            <div class="text-body-1 text-medium-emphasis">
              {{ t("settings.noProvider") }}
            </div>
            <v-btn color="primary" @click="providersStore.addProvider">
              {{ t("settings.addProvider") }}
            </v-btn>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </section>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "@/i18n";
import { useProvidersStore } from "@/stores/providers";

const providersStore = useProvidersStore();
const { t } = useI18n();

const providerKindOptions = ["openai-compatible"];
const authSchemeOptions = computed(() => [
  { title: "Bearer", value: "bearer" },
  { title: "None", value: "none" },
]);

const selectedProvider = computed(() => providersStore.selectedProvider);
const statusLine = computed(() => providersStore.statusLine);

const statusToneType = computed(() => {
  switch (providersStore.statusLine?.tone) {
    case "success":
      return "success";
    case "warning":
      return "warning";
    case "error":
      return "error";
    default:
      return "info";
  }
});

const statusIcon = computed(() => {
  switch (providersStore.statusLine?.tone) {
    case "success":
      return "mdi-check-circle-outline";
    case "warning":
      return "mdi-alert-outline";
    case "error":
      return "mdi-close-circle-outline";
    default:
      return "mdi-information-outline";
  }
});

const statusToneLabel = computed(() => {
  switch (providersStore.statusLine?.tone) {
    case "success":
      return t("settings.statusSuccess");
    case "warning":
      return t("settings.statusWarning");
    case "error":
      return t("settings.statusError");
    default:
      return t("settings.statusInfo");
  }
});

const statusText = computed(() => {
  if (!providersStore.statusLine) {
    return "";
  }

  return `${t("settings.statusAt")}: ${new Date(providersStore.statusLine.at).toLocaleTimeString()} - ${providersStore.statusLine.message}`;
});
</script>

<style scoped>
.provider-panel {
  border-color: var(--color-line);
}

.settings-section__header {
  margin-bottom: 16px;
}

.header-list {
  display: grid;
  gap: 12px;
}

.header-row {
  padding: 16px;
  border: 1px solid var(--color-line);
  border-radius: 12px;
  background: color-mix(in srgb, var(--color-panel) 96%, var(--color-bg) 4%);
}

.provider-settings :deep(.v-list-item--active) {
  background: color-mix(in srgb, var(--color-accent-soft) 78%, var(--color-panel) 22%);
}

.provider-settings :deep(.provider-field .v-field--variant-outlined .v-field__outline) {
  --v-field-border-opacity: 1;
}

.provider-settings :deep(.provider-field .v-field) {
  background: var(--color-panel);
}

.header-remove-btn {
  min-height: 48px;
}
</style>
