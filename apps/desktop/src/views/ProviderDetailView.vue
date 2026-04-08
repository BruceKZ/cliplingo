<template>
  <section class="provider-editor-page">
    <div class="editor-shell">
      <div class="editor-header">
        <v-btn class="app-btn app-btn--ghost" color="primary" variant="text" prepend-icon="mdi-arrow-left" @click="goBack">
          {{ t("settings.backToProviders") }}
        </v-btn>
      </div>

      <v-card v-if="provider" variant="outlined" rounded="lg" class="editor-card">
        <v-card-item>
          <div class="w-100 d-flex justify-space-between align-start ga-3 flex-wrap">
            <div>
              <div class="d-flex align-center ga-2 flex-wrap">
                <h1 class="provider-title mb-0">{{ provider.name || t("settings.unnamedProvider") }}</h1>
                <v-chip
                  v-if="provider.id === providersStore.activeProviderId"
                  size="small"
                  color="primary"
                  variant="tonal"
                >
                  {{ t("settings.activeProvider") }}
                </v-chip>
              </div>
            </div>

            <div class="header-actions">
              <v-btn class="app-btn app-btn--compact" size="small" color="primary" variant="tonal" prepend-icon="mdi-content-save-outline" :loading="providersStore.persistState === 'saving'" @click="saveProvider">
                {{ providersStore.persistState === "saving" ? t("settings.saving") : t("settings.save") }}
              </v-btn>
              <v-btn class="app-btn app-btn--compact" size="small" color="primary" variant="tonal" prepend-icon="mdi-check-circle-outline" :disabled="!provider.verifiedAt" @click="makeProviderActive">
                {{ t("settings.makeActive") }}
              </v-btn>
              <v-btn class="app-btn app-btn--compact" size="small" color="error" variant="tonal" prepend-icon="mdi-delete-outline" @click="removeProvider">
                {{ t("settings.remove") }}
              </v-btn>
            </div>
          </div>
        </v-card-item>

        <v-divider />

        <v-card-text class="editor-content">
          <div class="category-tabs">
            <v-tabs
              v-model="activeTab"
              color="primary"
              align-tabs="start"
              density="comfortable"
              class="provider-tabs"
            >
              <v-tab value="basics">{{ t("settings.providerBasics") }}</v-tab>
              <v-tab value="connection">{{ t("settings.connectionSettings") }}</v-tab>
              <v-tab value="advanced">{{ t("settings.advancedSettings") }}</v-tab>
              <v-tab value="json">{{ t("settings.jsonEditor") }}</v-tab>
            </v-tabs>
          </div>

          <v-window v-model="activeTab" class="provider-tab-panels" touchless>
            <v-window-item value="basics">
              <div class="editor-section">
                <div class="section-copy">
                  <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.providerBasics") }}</h2>
                  <p class="text-body-2 text-medium-emphasis">{{ t("settings.providerBasicsHint") }}</p>
                </div>
                <v-row dense>
                  <v-col cols="12" md="6">
                    <div class="field-label">{{ t("settings.displayName") }}</div>
                    <v-text-field
                      v-model.trim="provider.name"
                      class="provider-field"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <div class="field-label">{{ t("settings.providerKind") }}</div>
                    <v-select
                      v-model="provider.kind"
                      class="provider-field"
                      :items="providerKindOptions"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      disabled
                    />
                  </v-col>
                </v-row>
              </div>

              <v-divider class="my-6" />

              <div class="editor-section">
                <div class="section-copy section-copy--row">
                  <div>
                    <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.testTranslation") }}</h2>
                    <p class="text-body-2 text-medium-emphasis">{{ t("settings.requestSettingsHint") }}</p>
                  </div>
                  <v-btn class="app-btn" color="primary" variant="tonal" prepend-icon="mdi-flask-outline" :loading="providersStore.testState === 'running'" @click="testProvider">
                    {{ providersStore.testState === "running" ? t("settings.testing") : t("settings.testTranslation") }}
                  </v-btn>
                </div>
              </div>
            </v-window-item>

            <v-window-item value="connection">
              <div class="editor-section">
                <div class="section-copy">
                  <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.connectionSettings") }}</h2>
                  <p class="text-body-2 text-medium-emphasis">{{ t("settings.connectionSettingsHint") }}</p>
                </div>
                <v-row dense>
                  <v-col cols="12">
                    <div class="field-label">{{ t("settings.baseUrl") }}</div>
                    <v-text-field
                      v-model.trim="provider.baseUrl"
                      class="provider-field"
                      :hint="t('settings.baseUrlHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      placeholder="https://api.example.com"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <div class="field-label">{{ t("settings.requestPath") }}</div>
                    <v-text-field
                      v-model.trim="provider.path"
                      class="provider-field"
                      :hint="t('settings.requestPathHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      placeholder="/chat/completions"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <div class="field-label">{{ t("settings.authScheme") }}</div>
                    <v-select
                      v-model="provider.authScheme"
                      class="provider-field"
                      :items="authSchemeOptions"
                      item-title="title"
                      item-value="value"
                      :hint="t('settings.authSchemeHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12">
                    <div class="field-label field-label--row">
                      <div class="d-flex align-center ga-2 flex-wrap">
                        <span>{{ t("settings.apiKeyDraft") }}</span>
                        <v-chip
                          v-if="provider.hasSecret"
                          size="small"
                          color="success"
                          variant="tonal"
                        >
                          {{ t("settings.apiKeySaved") }}
                        </v-chip>
                      </div>
                    </div>
                    <div class="api-key-row">
                      <v-text-field
                        v-model.trim="provider.apiKeyDraft"
                        class="provider-field api-key-field"
                        :hint="t('settings.apiKeyHint')"
                        :type="provider.authScheme === 'none' ? 'text' : 'password'"
                        variant="outlined"
                        density="comfortable"
                        hide-details="auto"
                        :placeholder="provider.hasSecret ? 'sk-••••••••••••' : 'sk-...'"
                      />
                      <v-btn
                        v-if="provider.hasSecret || provider.apiKeyDraft"
                        class="app-btn api-key-clear-btn"
                        color="error"
                        variant="tonal"
                        prepend-icon="mdi-key-remove"
                        @click="providersStore.clearProviderSecret(provider.id)"
                      >
                        {{ t("settings.clear") }}
                      </v-btn>
                    </div>
                  </v-col>
                  <v-col cols="12" md="6">
                    <div class="field-label">{{ t("settings.model") }}</div>
                    <v-text-field
                      v-model.trim="provider.model"
                      class="provider-field"
                      :hint="t('settings.modelHint')"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                      placeholder="gpt-4o-mini"
                    />
                  </v-col>
                  <v-col cols="12" md="6">
                    <div class="field-label">{{ t("settings.requestTimeout") }}</div>
                    <v-text-field
                      v-model.number="provider.timeoutSecs"
                      class="provider-field"
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
            </v-window-item>

            <v-window-item value="advanced">
              <div class="editor-section">
                <div class="section-copy">
                  <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.advancedSettings") }}</h2>
                  <p class="text-body-2 text-medium-emphasis">{{ t("settings.advancedSettingsHint") }}</p>
                </div>
                <v-row dense>
                  <v-col cols="12" md="4">
                    <div class="field-label">{{ t("settings.temperature") }}</div>
                    <v-text-field
                      v-model.number="provider.temperature"
                      class="provider-field"
                      :hint="t('settings.temperatureHint')"
                      type="text"
                      inputmode="decimal"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12" md="4">
                    <div class="field-label">{{ t("settings.topP") }}</div>
                    <v-text-field
                      v-model.number="provider.topP"
                      class="provider-field"
                      :hint="t('settings.topPHint')"
                      type="text"
                      inputmode="decimal"
                      variant="outlined"
                      density="comfortable"
                      hide-details="auto"
                    />
                  </v-col>
                  <v-col cols="12" md="4">
                    <div class="field-label">{{ t("settings.maxTokens") }}</div>
                    <v-text-field
                      v-model.number="provider.maxTokens"
                      class="provider-field"
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

              <div class="editor-section">
                <div class="section-copy section-copy--row">
                  <div>
                    <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.customHeaders") }}</h2>
                    <p class="text-body-2 text-medium-emphasis">{{ t("settings.headersHint") }}</p>
                  </div>
                  <v-btn class="app-btn" size="small" color="primary" variant="tonal" prepend-icon="mdi-plus" @click="providersStore.addProviderHeader(provider.id)">
                    {{ t("settings.addHeader") }}
                  </v-btn>
                </div>

                <div v-if="provider.customHeaders.length === 0" class="text-body-2 text-medium-emphasis py-2">
                  {{ t("settings.noHeaders") }}
                </div>

                <div v-else class="header-list">
                  <div
                    v-for="(header, headerIndex) in provider.customHeaders"
                    :key="`${provider.id}-${headerIndex}`"
                    class="header-row"
                  >
                    <v-row dense>
                      <v-col cols="12" md="5">
                        <div class="field-label">{{ t("settings.headerName") }}</div>
                        <v-text-field
                          v-model.trim="header.name"
                          class="provider-field"
                          variant="outlined"
                          density="comfortable"
                          hide-details="auto"
                        />
                      </v-col>
                      <v-col cols="12" md="5">
                        <div class="field-label">{{ t("settings.headerValue") }}</div>
                        <v-text-field
                          v-model.trim="header.value"
                          class="provider-field"
                          variant="outlined"
                          density="comfortable"
                          hide-details="auto"
                        />
                      </v-col>
                      <v-col cols="12" md="3">
                        <v-btn block color="error" variant="tonal" class="header-remove-btn" prepend-icon="mdi-delete-outline" @click="providersStore.removeProviderHeader(provider.id, headerIndex)">
                          {{ t("settings.remove") }}
                        </v-btn>
                      </v-col>
                    </v-row>
                  </div>
                </div>
              </div>
            </v-window-item>

            <v-window-item value="json">
              <div class="editor-section">
                <div class="section-copy">
                  <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.providerJsonLabel") }}</h2>
                  <p class="text-body-2 text-medium-emphasis">
                    {{ t("settings.jsonEditorHelp") }}
                  </p>
                </div>

                <JsonCodeEditor
                  v-model="jsonDraft"
                />

                <div v-if="jsonError" class="mt-3">
                  <v-alert type="error" variant="tonal" density="comfortable">
                    {{ jsonError }}
                  </v-alert>
                </div>
              </div>
            </v-window-item>
          </v-window>
        </v-card-text>

        <v-divider />

        <v-card-text v-if="statusLine" class="pt-4">
          <v-alert :type="statusToneType" variant="tonal" density="comfortable" :icon="statusIcon">
            <div class="text-body-2 font-weight-medium">{{ statusToneLabel }}</div>
            <div class="text-body-2">{{ statusText }}</div>
          </v-alert>
        </v-card-text>
      </v-card>

      <v-card v-else variant="outlined" rounded="lg" class="editor-card">
        <v-card-text class="py-12 text-center">
          <div class="text-body-1 mb-2">{{ t("settings.providerNotFound") }}</div>
          <div class="text-body-2 text-medium-emphasis mb-6">
            {{ t("settings.providerNotFoundHint") }}
          </div>
          <v-btn class="app-btn app-btn--ghost" color="primary" variant="text" prepend-icon="mdi-arrow-left" @click="goBack">
            {{ t("settings.backToProviders") }}
          </v-btn>
        </v-card-text>
      </v-card>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
import JsonCodeEditor from "@/components/settings/JsonCodeEditor.vue";
import { useI18n } from "@/i18n";
import { useProvidersStore, type ProviderDraft } from "@/stores/providers";

type ProviderTab = "basics" | "connection" | "advanced" | "json";

interface ProviderJsonDocument {
  id?: string;
  name?: string;
  kind?: string;
  baseUrl?: string;
  path?: string;
  authScheme?: string;
  apiKey?: string;
  organization?: string;
  model?: string;
  temperature?: number;
  topP?: number;
  maxTokens?: number | null;
  timeoutSecs?: number;
  customHeaders?: Array<{ name: string; value: string }>;
  enabled?: boolean;
}

const route = useRoute();
const router = useRouter();
const providersStore = useProvidersStore();
const { t } = useI18n();

const providerKindOptions = ["openai-compatible"];
const authSchemeOptions = computed(() => [
  { title: "Bearer", value: "bearer" },
  { title: "None", value: "none" },
]);

const activeTab = ref<ProviderTab>("basics");
const jsonDraft = ref("");
const jsonError = ref("");
const editableProvider = ref<ProviderDraft | null>(null);

const providerId = computed(() => String(route.params.providerId ?? ""));
const provider = computed(() => editableProvider.value);
const statusLine = computed(() => providersStore.statusLine);

watch(
  () => route.params.providerId,
  (nextProviderId) => {
    const nextId = String(nextProviderId ?? "");
    const nextProvider = providersStore.getProvider(nextId);
    editableProvider.value = nextProvider;
    if (nextProvider) {
      providersStore.selectProvider(nextProvider.id);
      jsonDraft.value = serializeProvider(nextProvider);
      jsonError.value = "";
    }
  },
  { immediate: true },
);

watch(activeTab, (tab) => {
  if (tab === "json" && provider.value) {
    jsonDraft.value = serializeProvider(provider.value);
    jsonError.value = "";
  }
});

watch(
  () => editableProvider.value?.id,
  (nextId) => {
    if (nextId) {
      providersStore.selectProvider(nextId);
    }
  },
);

onMounted(async () => {
  await providersStore.refresh().catch(() => undefined);
  const nextProvider = providersStore.getProvider(providerId.value);
  editableProvider.value = nextProvider;
  if (nextProvider) {
    providersStore.selectProvider(nextProvider.id);
    jsonDraft.value = serializeProvider(nextProvider);
  }
});

function serializeProvider(current: ProviderDraft) {
  return JSON.stringify(
    {
      name: current.name,
      kind: current.kind,
      baseUrl: current.baseUrl,
      path: current.path,
      authScheme: current.authScheme,
      apiKey: current.apiKeyDraft || "",
      organization: current.organization || "",
      model: current.model,
      temperature: current.temperature,
      topP: current.topP,
      maxTokens: current.maxTokens,
      timeoutSecs: current.timeoutSecs,
      customHeaders: current.customHeaders.map((header) => ({
        name: header.name,
        value: header.value,
      })),
      enabled: current.enabled,
      hasSavedApiKey: current.hasSecret,
    },
    null,
    2,
  );
}

function applyJsonToProvider(current: ProviderDraft, parsed: ProviderJsonDocument) {
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new Error(t("settings.invalidJsonObject"));
  }

  if (parsed.name !== undefined && typeof parsed.name !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.kind !== undefined && typeof parsed.kind !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.baseUrl !== undefined && typeof parsed.baseUrl !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.path !== undefined && typeof parsed.path !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.authScheme !== undefined && typeof parsed.authScheme !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.apiKey !== undefined && typeof parsed.apiKey !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.organization !== undefined && typeof parsed.organization !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.model !== undefined && typeof parsed.model !== "string") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.temperature !== undefined && typeof parsed.temperature !== "number") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.topP !== undefined && typeof parsed.topP !== "number") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (
    parsed.maxTokens !== undefined &&
    parsed.maxTokens !== null &&
    typeof parsed.maxTokens !== "number"
  ) {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.timeoutSecs !== undefined && typeof parsed.timeoutSecs !== "number") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.enabled !== undefined && typeof parsed.enabled !== "boolean") {
    throw new Error(t("settings.invalidJsonField"));
  }
  if (parsed.customHeaders !== undefined && !Array.isArray(parsed.customHeaders)) {
    throw new Error(t("settings.invalidJsonField"));
  }

  current.name = parsed.name ?? current.name;
  current.kind = (parsed.kind as ProviderDraft["kind"] | undefined) ?? current.kind;
  current.baseUrl = parsed.baseUrl ?? current.baseUrl;
  current.path = parsed.path ?? current.path;
  current.authScheme =
    (parsed.authScheme as ProviderDraft["authScheme"] | undefined) ?? current.authScheme;
  current.organization = parsed.organization ?? current.organization;
  current.model = parsed.model ?? current.model;
  current.temperature = parsed.temperature ?? current.temperature;
  current.topP = parsed.topP ?? current.topP;
  current.maxTokens = parsed.maxTokens ?? current.maxTokens;
  current.timeoutSecs = parsed.timeoutSecs ?? current.timeoutSecs;
  current.enabled = parsed.enabled ?? current.enabled;

  if (parsed.apiKey !== undefined) {
    current.apiKeyDraft = parsed.apiKey;
  }

  if (parsed.customHeaders) {
    current.customHeaders.splice(
      0,
      current.customHeaders.length,
      ...parsed.customHeaders.map((header) => {
        if (
          typeof header !== "object" ||
          header === null ||
          typeof header.name !== "string" ||
          typeof header.value !== "string"
        ) {
          throw new Error(t("settings.invalidJsonField"));
        }

        return {
          name: header.name,
          value: header.value,
        };
      }),
    );
  }
}

async function saveProvider() {
  const current = provider.value;
  if (!current) {
    return;
  }

  const previousId = current.id;

  if (activeTab.value === "json") {
    try {
      const parsed = JSON.parse(jsonDraft.value) as ProviderJsonDocument;
      applyJsonToProvider(current, parsed);
      jsonError.value = "";
    } catch (error) {
      jsonError.value = error instanceof Error ? error.message : String(error);
      return;
    }
  }

  await providersStore.persistCurrent();

  editableProvider.value = providersStore.selectedProvider;
  const nextId = providersStore.selectedProvider?.id ?? current.id;
  if (nextId !== previousId || providerId.value !== nextId) {
    await router.replace({ name: "provider-detail", params: { providerId: nextId } });
  }

  if (editableProvider.value) {
    providersStore.selectProvider(editableProvider.value.id);
    jsonDraft.value = serializeProvider(editableProvider.value);
  }
}

async function testProvider() {
  await saveProvider();
  if (provider.value) {
    await providersStore.testConnection();
  }
}

async function removeProvider() {
  const current = provider.value;
  if (!current) {
    return;
  }

  await providersStore.removeProvider(current.id);
  editableProvider.value = null;
  await router.push({ name: "providers" });
}

function makeProviderActive() {
  const current = provider.value;
  if (!current) {
    return;
  }

  providersStore.makeProviderActive(current.id).catch(() => undefined);
}

function goBack() {
  router.push({ name: "providers" }).catch(() => undefined);
}

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
.provider-editor-page {
  display: flex;
  justify-content: center;
}

.editor-shell {
  width: 100%;
  max-width: 980px;
  display: grid;
  gap: 20px;
}

.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}

.editor-card {
  border-color: var(--color-line);
}

.editor-content {
  padding-top: 24px;
}

.editor-section {
  display: grid;
  gap: 18px;
}

.category-tabs {
  margin-bottom: 24px;
}

.provider-tab-panels {
  overflow: visible;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.provider-title {
  font-size: 2rem;
  line-height: 1.2;
  font-weight: 500;
}

.section-copy {
  display: grid;
  gap: 6px;
}

.section-copy--row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  flex-wrap: wrap;
}

.api-key-row {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 12px;
  align-items: start;
}

.field-label {
  margin-bottom: 8px;
  font-size: 0.875rem;
  font-weight: 600;
  line-height: 1.35;
  color: var(--color-text);
}

.field-label--row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
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

.provider-editor-page :deep(.provider-field .v-field--variant-outlined .v-field__outline) {
  --v-field-border-opacity: 1;
}

.provider-editor-page :deep(.provider-field .v-field) {
  background: var(--color-panel);
  min-height: 40px;
}

.provider-editor-page :deep(.provider-field .v-field__input) {
  min-height: 40px;
  padding-top: 6px;
  padding-bottom: 6px;
  padding-inline: 12px;
  font-size: 0.94rem;
  line-height: 1.4;
  color: var(--color-text);
}

.provider-editor-page :deep(.provider-field .v-messages) {
  padding-top: 6px;
  font-size: 0.78rem;
  line-height: 1.4;
}

.provider-editor-page :deep(.app-btn) {
  min-height: 40px;
  min-width: 112px;
  text-transform: none;
  font-weight: 600;
  font-size: 0.92rem;
  line-height: 1;
  letter-spacing: 0;
  white-space: nowrap;
}

.provider-editor-page :deep(.provider-tabs .v-tab) {
  min-height: 40px;
  text-transform: none;
  font-size: 0.92rem;
  font-weight: 600;
  letter-spacing: 0;
}

.provider-editor-page :deep(.app-btn .v-icon) {
  font-size: 1rem;
}

.provider-editor-page :deep(.app-btn--ghost) {
  min-width: auto;
  padding-inline: 6px;
}

.provider-editor-page :deep(.app-btn--compact) {
  min-width: 96px;
  padding-inline: 14px;
}

.api-key-clear-btn {
  min-width: 88px;
  min-height: 40px;
}

.header-remove-btn {
  min-height: 40px;
  min-width: 96px;
}

@media (max-width: 959px) {
  .provider-title {
    font-size: 1.75rem;
  }

  .api-key-row {
    grid-template-columns: 1fr;
  }

  .api-key-clear-btn {
    width: 100%;
  }
}
</style>
