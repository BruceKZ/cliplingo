<template>
  <section class="providers-view">
    <div class="mb-4">
      <h1 class="text-h5 mb-1">{{ t("nav.providers") }}</h1>
      <p class="text-body-2 text-medium-emphasis">
        {{ t("settings.providersHelp") }}
      </p>
    </div>

    <v-row dense>
      <v-col cols="12" lg="4">
        <v-card border rounded="lg">
          <v-card-title class="d-flex justify-space-between align-center">
            <span>{{ t("settings.providers") }}</span>
            <v-btn size="small" variant="tonal" @click="providersStore.addProvider">
              {{ t("settings.addProvider") }}
            </v-btn>
          </v-card-title>
          <v-card-text>
            <p class="text-body-2 text-medium-emphasis mb-4">
              {{ t("settings.providerListHelp") }}
            </p>
            <v-list v-if="providersStore.providers.length > 0" density="comfortable" nav>
              <v-list-item
                v-for="provider in providersStore.providers"
                :key="provider.id"
                :active="provider.id === providersStore.selectedProviderId"
                @click="providersStore.selectProvider(provider.id)"
              >
                <v-list-item-title>{{ provider.name || provider.id }}</v-list-item-title>
                <template #append>
                  <v-chip v-if="provider.id === providersStore.activeProviderId" size="x-small" label>
                    Active
                  </v-chip>
                </template>
              </v-list-item>
            </v-list>
            <div v-else class="text-body-2 text-medium-emphasis py-2">
              {{ t("settings.noProvidersYet") }}
            </div>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" lg="8">
        <v-card v-if="selectedProvider" border rounded="lg">
          <v-toolbar color="transparent">
            <v-toolbar-title class="d-flex align-center flex-wrap ga-2">
              <span>{{ selectedProvider.name || selectedProvider.id }}</span>
              <v-chip v-if="selectedProvider.id === providersStore.activeProviderId" size="small" label>
                {{ t("settings.activeProvider") }}
              </v-chip>
            </v-toolbar-title>
            <div class="d-flex ga-2 flex-wrap pr-4">
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
          </v-toolbar>

          <v-card-text>
            <v-alert
              class="mb-4"
              variant="tonal"
              type="info"
              density="comfortable"
              icon="mdi-information-outline"
            >
              {{ t("settings.providerRequiredHint") }}
            </v-alert>

            <v-card variant="outlined" rounded="lg" class="mb-4 provider-summary-card">
              <v-card-text>
                <v-row dense>
                  <v-col cols="12" md="4">
                    <div class="text-caption text-medium-emphasis">{{ t("settings.displayName") }}</div>
                    <div class="text-body-1 font-weight-medium">{{ selectedProvider.name || "-" }}</div>
                  </v-col>
                  <v-col cols="12" md="4">
                    <div class="text-caption text-medium-emphasis">{{ t("settings.providerId") }}</div>
                    <div class="text-body-1 font-weight-medium">{{ selectedProvider.id || "-" }}</div>
                  </v-col>
                  <v-col cols="12" md="4">
                    <div class="text-caption text-medium-emphasis">{{ t("settings.model") }}</div>
                    <div class="text-body-1 font-weight-medium">{{ selectedProvider.model || "-" }}</div>
                  </v-col>
                </v-row>
              </v-card-text>
            </v-card>

            <v-tabs
              v-model="activeSection"
              color="primary"
              align-tabs="start"
              class="mb-4"
            >
              <v-tab value="connection" prepend-icon="mdi-connection">
                {{ t("settings.connectionSettings") }}
              </v-tab>
              <v-tab value="request" prepend-icon="mdi-tune-variant">
                {{ t("settings.requestSettings") }}
              </v-tab>
              <v-tab value="advanced" prepend-icon="mdi-tune">
                {{ t("settings.advancedSettings") }}
              </v-tab>
              <v-tab value="headers" prepend-icon="mdi-format-list-bulleted-square">
                {{ t("settings.customHeaders") }}
              </v-tab>
            </v-tabs>

            <v-form>
              <v-window v-model="activeSection">
                <v-window-item value="connection">
                  <v-card variant="outlined" rounded="lg">
                    <v-card-item>
                      <v-card-title>{{ t("settings.connectionSettings") }}</v-card-title>
                      <v-card-subtitle>{{ t("settings.connectionSettingsHint") }}</v-card-subtitle>
                    </v-card-item>
                    <v-divider />
                    <v-card-text>
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
                              prepend-icon="mdi-key-remove-outline"
                              @click="providersStore.clearProviderSecret(selectedProvider.id)"
                            >
                              {{ t("settings.clearApiKey") }}
                            </v-btn>
                          </div>
                        </v-col>
                      </v-row>
                    </v-card-text>
                  </v-card>
                </v-window-item>

                <v-window-item value="request">
                  <div class="provider-panel-stack">
                    <v-card variant="outlined" rounded="lg">
                      <v-card-item>
                        <v-card-title>{{ t("settings.requestSettings") }}</v-card-title>
                        <v-card-subtitle>{{ t("settings.requestSettingsHint") }}</v-card-subtitle>
                      </v-card-item>
                      <v-divider />
                      <v-card-text>
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
                      </v-card-text>
                    </v-card>

                    <v-card variant="outlined" rounded="lg">
                      <v-card-item>
                        <v-card-title>{{ t("settings.providerBasics") }}</v-card-title>
                        <v-card-subtitle>{{ t("settings.providerBasicsHint") }}</v-card-subtitle>
                      </v-card-item>
                      <v-divider />
                      <v-card-text>
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
                      </v-card-text>
                    </v-card>
                  </div>
                </v-window-item>

                <v-window-item value="advanced">
                  <v-card variant="outlined" rounded="lg">
                    <v-card-item>
                      <v-card-title>{{ t("settings.advancedSettings") }}</v-card-title>
                      <v-card-subtitle>{{ t("settings.advancedSettingsHint") }}</v-card-subtitle>
                    </v-card-item>
                    <v-divider />
                    <v-card-text>
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
                    </v-card-text>
                  </v-card>
                </v-window-item>

                <v-window-item value="headers">
                  <v-card variant="outlined" rounded="lg">
                    <v-card-item>
                      <template #append>
                        <v-btn size="small" variant="outlined" @click="providersStore.addProviderHeader(selectedProvider.id)">
                          <template #prepend>
                            <v-icon icon="mdi-plus" />
                          </template>
                          {{ t("settings.addHeader") }}
                        </v-btn>
                      </template>
                      <v-card-title>{{ t("settings.customHeaders") }}</v-card-title>
                      <v-card-subtitle>{{ t("settings.headersHint") }}</v-card-subtitle>
                    </v-card-item>
                    <v-divider />
                    <v-card-text>
                      <v-list
                        v-if="selectedProvider.customHeaders.length > 0"
                        lines="two"
                        border
                        rounded="lg"
                      >
                        <v-list-item
                          v-for="(header, headerIndex) in selectedProvider.customHeaders"
                          :key="`${selectedProvider.id}-${headerIndex}`"
                        >
                          <v-row dense class="mt-1">
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
                              <v-btn block color="error" variant="outlined" class="header-remove-btn" @click="providersStore.removeProviderHeader(selectedProvider.id, headerIndex)">
                                <template #prepend>
                                  <v-icon icon="mdi-delete-outline" />
                                </template>
                                {{ t("settings.remove") }}
                              </v-btn>
                            </v-col>
                          </v-row>
                        </v-list-item>
                      </v-list>

                      <v-alert
                        v-else
                        variant="tonal"
                        type="info"
                        density="comfortable"
                        icon="mdi-information-outline"
                      >
                        {{ t("settings.noHeaders") }}
                      </v-alert>
                    </v-card-text>
                  </v-card>
                </v-window-item>
              </v-window>
            </v-form>

            <v-card variant="outlined" rounded="lg" class="mt-4">
              <v-card-item>
                <v-card-title>{{ t("settings.providerActions") }}</v-card-title>
                <v-card-subtitle>{{ t("settings.providerActionsHint") }}</v-card-subtitle>
              </v-card-item>
              <v-divider />
              <v-card-text>
                <v-row dense>
                  <v-col cols="12" md="4">
                    <v-btn block variant="outlined" prepend-icon="mdi-content-save-outline" :loading="providersStore.persistState === 'saving'" @click="providersStore.persistCurrent()">
                      {{ providersStore.persistState === "saving" ? t("settings.saving") : t("settings.save") }}
                    </v-btn>
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-btn block color="primary" prepend-icon="mdi-flask-outline" :loading="providersStore.testState === 'running'" @click="providersStore.testConnection">
                      {{ providersStore.testState === "running" ? t("settings.testing") : t("settings.testTranslation") }}
                    </v-btn>
                  </v-col>
                  <v-col cols="12" md="4">
                    <v-btn block color="primary" variant="tonal" prepend-icon="mdi-check-circle-outline" @click="providersStore.makeProviderActive(selectedProvider.id)">
                      {{ t("settings.makeActive") }}
                    </v-btn>
                  </v-col>
                </v-row>

                <v-alert
                  v-if="statusLine"
                  class="mt-4"
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
          </v-card-text>
        </v-card>
        <v-card v-else border rounded="lg">
          <v-card-text class="d-flex justify-center py-8">
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
import { computed, onMounted, ref } from "vue";
import { useI18n } from "@/i18n";
import { useProvidersStore } from "@/stores/providers";

const providersStore = useProvidersStore();
const { t } = useI18n();

const providerKindOptions = ["openai-compatible"];
const activeSection = ref("connection");
const authSchemeOptions = computed(() => [
  { title: "Bearer", value: "bearer" },
  { title: "None", value: "none" },
]);

onMounted(() => {
  providersStore.refresh(true).catch(() => undefined);
});

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
.provider-summary-card {
  background: color-mix(in srgb, var(--color-panel) 92%, var(--color-accent-soft) 8%);
}

.provider-panel-stack {
  display: grid;
  gap: 16px;
}

.providers-view :deep(.v-card--variant-outlined) {
  border-color: var(--color-line);
}

.providers-view :deep(.v-tabs) {
  border: 1px solid var(--color-line);
  border-radius: 12px;
  padding-inline: 8px;
  background: var(--color-panel);
}

.providers-view :deep(.v-tab) {
  min-height: 52px;
  text-transform: none;
  font-weight: 600;
}

.providers-view :deep(.provider-field .v-field) {
  border: 1px solid var(--color-line);
  border-radius: 12px;
  background: color-mix(in srgb, var(--color-panel) 96%, var(--color-bg) 4%);
  box-shadow: none;
}

.providers-view :deep(.provider-field .v-field--focused) {
  border-color: var(--color-accent);
}

.providers-view :deep(.provider-field .v-field__outline) {
  --v-field-border-opacity: 0;
}

.providers-view :deep(.provider-field .v-field__input) {
  padding-top: 16px;
}

.providers-view :deep(.provider-field .v-label.v-field-label) {
  opacity: 0.84;
}

.header-remove-btn {
  min-height: 48px;
}
</style>
