<template>
  <v-card variant="outlined" rounded="lg" class="settings-card">
    <v-card-text class="settings-content">
      <div class="category-tabs">
        <v-tabs
          v-model="activeTab"
          color="primary"
          align-tabs="start"
          density="comfortable"
          class="settings-tabs"
        >
          <v-tab value="general">{{ t("settings.general") }}</v-tab>
          <v-tab value="trigger" disabled>{{ t("settings.trigger") }}</v-tab>
          <v-tab value="routing">{{ t("settings.languageRouting") }}</v-tab>
          <v-tab value="privacy" disabled>{{ t("settings.privacy") }}</v-tab>
          <v-tab value="providers">{{ t("settings.providers") }}</v-tab>
        </v-tabs>
      </div>

      <v-window v-model="activeTab" class="settings-tab-panels" touchless>
        <v-window-item value="general">
          <div class="editor-section">
            <div class="section-copy">
              <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.general") }}</h2>
              <p class="text-body-2 text-medium-emphasis">{{ t("settings.providerSummaryHint") }}</p>
            </div>
            <v-row dense>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.language") }}</div>
                <v-select
                  v-model="localeModel"
                  class="settings-field"
                  :items="localeOptions"
                  item-title="title"
                  item-value="value"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.themeMode") }}</div>
                <v-select
                  v-model="themeModeModel"
                  class="settings-field"
                  :items="themeModeOptions"
                  item-title="title"
                  item-value="value"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                />
              </v-col>
              <v-col cols="12">
                <div class="toggle-row">
                  <div>
                    <div class="field-label mb-1">{{ t("settings.showTrayIcon") }}</div>
                  </div>
                  <v-switch
                    v-model="settingsStore.draft.ui.showTrayIcon"
                    color="primary"
                    hide-details
                    inset
                  />
                </div>
              </v-col>
            </v-row>
          </div>
        </v-window-item>

        <v-window-item value="trigger">
          <div class="editor-section">
            <div class="section-copy">
              <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.trigger") }}</h2>
              <p class="text-body-2 text-medium-emphasis">{{ t("settings.providersHelp") }}</p>
            </div>
            <v-row dense>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.doubleCopyWindowMs") }}</div>
                <v-text-field
                  v-model.number="settingsStore.draft.trigger.doubleCopyWindowMs"
                  class="settings-field"
                  type="text"
                  inputmode="numeric"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.fallbackShortcut") }}</div>
                <v-text-field
                  v-model.trim="settingsStore.draft.trigger.fallbackShortcut"
                  class="settings-field"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                />
              </v-col>
              <v-col cols="12">
                <div class="toggle-row">
                  <div>
                    <div class="field-label mb-1">{{ t("settings.replacePopupOnNewTrigger") }}</div>
                  </div>
                  <v-switch
                    v-model="settingsStore.draft.trigger.replacePopupOnNewTrigger"
                    color="primary"
                    hide-details
                    inset
                  />
                </div>
              </v-col>
            </v-row>
          </div>
        </v-window-item>

        <v-window-item value="routing">
          <div class="editor-section">
            <div class="section-copy">
              <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.languageRouting") }}</h2>
              <p class="text-body-2 text-medium-emphasis">{{ t("settings.sectionNavigationHint") }}</p>
            </div>

            <v-row dense>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.routingMode") }}</div>
                <v-select
                  v-model="settingsStore.routingKind"
                  class="settings-field"
                  :items="routingModeOptions"
                  item-title="title"
                  item-value="value"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                />
              </v-col>
            </v-row>

            <v-row v-if="settingsStore.routingKind === 'branching'" dense>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.englishSource") }}</div>
                <v-text-field v-model.trim="branchingEnglishSource" class="settings-field" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.englishTargets") }}</div>
                <v-textarea v-model="branchingEnglishTargets" class="settings-field settings-field--textarea" variant="outlined" density="comfortable" hide-details="auto" rows="3" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.chineseSource") }}</div>
                <v-text-field v-model.trim="branchingChineseSource" class="settings-field" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.chineseTargets") }}</div>
                <v-textarea v-model="branchingChineseTargets" class="settings-field settings-field--textarea" variant="outlined" density="comfortable" hide-details="auto" rows="3" />
              </v-col>
              <v-col cols="12">
                <div class="field-label">{{ t("settings.fallbackTargets") }}</div>
                <v-textarea v-model="branchingFallbackTargets" class="settings-field settings-field--textarea" variant="outlined" density="comfortable" hide-details="auto" rows="3" />
              </v-col>
            </v-row>

            <v-row v-else-if="settingsStore.routingKind === 'bidirectional'" dense>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.primarySource") }}</div>
                <v-text-field v-model.trim="bidirectionalPrimarySource" class="settings-field" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.primaryTargets") }}</div>
                <v-textarea v-model="bidirectionalPrimaryTargets" class="settings-field settings-field--textarea" variant="outlined" density="comfortable" hide-details="auto" rows="3" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.secondarySource") }}</div>
                <v-text-field v-model.trim="bidirectionalSecondarySource" class="settings-field" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.secondaryTargets") }}</div>
                <v-textarea v-model="bidirectionalSecondaryTargets" class="settings-field settings-field--textarea" variant="outlined" density="comfortable" hide-details="auto" rows="3" />
              </v-col>
            </v-row>

            <v-row v-else dense>
              <v-col cols="12">
                <div class="field-label">{{ t("settings.targetLanguages") }}</div>
                <v-textarea v-model="fixedTargetLanguages" class="settings-field settings-field--textarea" variant="outlined" density="comfortable" hide-details="auto" rows="4" />
              </v-col>
            </v-row>

            <v-divider class="my-6" />

            <v-row dense>
              <v-col cols="12">
                <div class="field-label">{{ t("settings.userTranslationRules") }}</div>
                <v-textarea
                  v-model.trim="userRulesModel"
                  class="settings-field settings-field--textarea"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                  rows="4"
                />
              </v-col>
              <v-col cols="12">
                <div class="toggle-row">
                  <div>
                    <div class="field-label mb-1">{{ t("settings.preserveParagraphs") }}</div>
                  </div>
                  <v-switch
                    v-model="settingsStore.draft.translation.preserveParagraphs"
                    color="primary"
                    hide-details
                    inset
                  />
                </div>
              </v-col>
            </v-row>
          </div>
        </v-window-item>

        <v-window-item value="privacy">
          <div class="editor-section">
            <div class="section-copy">
              <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.privacy") }}</h2>
              <p class="text-body-2 text-medium-emphasis">{{ t("settings.providerActionsHint") }}</p>
            </div>
            <v-row dense>
              <v-col cols="12">
                <div class="toggle-row">
                  <div>
                    <div class="field-label mb-1">{{ t("settings.enableLocalHistory") }}</div>
                  </div>
                  <v-switch
                    v-model="settingsStore.draft.history.enabled"
                    color="primary"
                    hide-details
                    inset
                  />
                </div>
              </v-col>
              <v-col cols="12" md="6">
                <div class="field-label">{{ t("settings.historyLimit") }}</div>
                <v-text-field
                  v-model.number="settingsStore.draft.history.maxItems"
                  class="settings-field"
                  type="text"
                  inputmode="numeric"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                />
              </v-col>
              <v-col cols="12">
                <div class="toggle-row">
                  <div>
                    <div class="field-label mb-1">{{ t("settings.storeFullTextInHistory") }}</div>
                  </div>
                  <v-switch
                    v-model="settingsStore.draft.history.storeFullText"
                    color="primary"
                    hide-details
                    inset
                  />
                </div>
              </v-col>
              <v-col cols="12">
                <div class="toggle-row">
                  <div>
                    <div class="field-label mb-1">{{ t("settings.logRawNetworkErrors") }}</div>
                  </div>
                  <v-switch
                    v-model="settingsStore.draft.debug.logRawNetworkErrors"
                    color="primary"
                    hide-details
                    inset
                  />
                </div>
              </v-col>
            </v-row>
          </div>
        </v-window-item>

        <v-window-item value="providers">
          <div class="editor-section">
            <div class="section-copy section-copy--row">
              <div>
                <h2 class="text-subtitle-1 font-weight-medium">{{ t("settings.providers") }}</h2>
                <p class="text-body-2 text-medium-emphasis">{{ t("settings.providersHelp") }}</p>
              </div>
              <v-btn
                class="app-btn"
                color="primary"
                variant="tonal"
                prepend-icon="mdi-server-outline"
                to="/providers"
              >
                {{ t("settings.manageProviders") }}
              </v-btn>
            </div>

            <ProviderOverviewPanel />
          </div>
        </v-window-item>
      </v-window>
    </v-card-text>

    <v-divider />

    <v-card-actions class="settings-actions">
      <div class="header-actions">
        <v-btn
          class="app-btn app-btn--compact"
          color="primary"
          variant="tonal"
          prepend-icon="mdi-content-save-outline"
          :loading="settingsStore.persistState === 'saving'"
          @click="saveSettings"
        >
          {{ settingsStore.persistState === "saving" ? t("settings.saving") : t("settings.save") }}
        </v-btn>
        <v-btn
          class="app-btn app-btn--compact"
          color="primary"
          variant="tonal"
          prepend-icon="mdi-restore"
          @click="settingsStore.resetDraft"
        >
          {{ t("settings.resetDefaults") }}
        </v-btn>
      </div>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import type { ThemeMode } from "@cliplingo/shared-types";
import ProviderOverviewPanel from "@/components/settings/ProviderOverviewPanel.vue";
import { useI18n } from "@/i18n";
import { useSettingsStore } from "@/stores/settings";
import { useUiStore, type AppLocale } from "@/stores/ui";

type SettingsTab = "general" | "trigger" | "routing" | "privacy" | "providers";

const settingsStore = useSettingsStore();
const uiStore = useUiStore();
const { t } = useI18n();

const activeTab = ref<SettingsTab>("general");

const localeOptions = computed(() => [
  { title: "English", value: "en" },
  { title: "简体中文", value: "zh-CN" },
]);

const themeModeOptions = computed(() => [
  { title: "System", value: "system" },
  { title: "Light", value: "light" },
  { title: "Dark", value: "dark" },
]);

const routingModeOptions = computed(() => [
  { title: "Branching", value: "branching" },
  { title: "Bidirectional", value: "bidirectional" },
  { title: "Fixed", value: "fixed" },
]);

const localeModel = computed<AppLocale>({
  get: () => uiStore.locale,
  set: (value) => settingsStore.setLocale(value),
});

const themeModeModel = computed<ThemeMode>({
  get: () => settingsStore.draft.ui.themeMode,
  set: (value) => settingsStore.setThemeMode(value),
});

const userRulesModel = computed({
  get: () => settingsStore.draft.translation.userRules ?? "",
  set: (value: string) => {
    settingsStore.draft.translation.userRules = value.trim() ? value : null;
  },
});

const branchingEnglishSource = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "branching"
      ? settingsStore.draft.translation.routingRule.englishSourceLanguage
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "branching") {
      settingsStore.draft.translation.routingRule.englishSourceLanguage = value.trim();
    }
  },
});

const branchingEnglishTargets = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "branching"
      ? settingsStore.joinList(settingsStore.draft.translation.routingRule.englishTargetLanguages)
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "branching") {
      settingsStore.updateList(value, (next) => {
        if (settingsStore.draft.translation.routingRule.kind === "branching") {
          settingsStore.draft.translation.routingRule.englishTargetLanguages = next;
        }
      });
    }
  },
});

const branchingChineseSource = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "branching"
      ? settingsStore.draft.translation.routingRule.chineseSourceLanguage
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "branching") {
      settingsStore.draft.translation.routingRule.chineseSourceLanguage = value.trim();
    }
  },
});

const branchingChineseTargets = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "branching"
      ? settingsStore.joinList(settingsStore.draft.translation.routingRule.chineseTargetLanguages)
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "branching") {
      settingsStore.updateList(value, (next) => {
        if (settingsStore.draft.translation.routingRule.kind === "branching") {
          settingsStore.draft.translation.routingRule.chineseTargetLanguages = next;
        }
      });
    }
  },
});

const branchingFallbackTargets = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "branching"
      ? settingsStore.joinList(settingsStore.draft.translation.routingRule.fallbackTargetLanguages)
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "branching") {
      settingsStore.updateList(value, (next) => {
        if (settingsStore.draft.translation.routingRule.kind === "branching") {
          settingsStore.draft.translation.routingRule.fallbackTargetLanguages = next;
        }
      });
    }
  },
});

const bidirectionalPrimarySource = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "bidirectional"
      ? settingsStore.draft.translation.routingRule.primarySourceLanguage
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "bidirectional") {
      settingsStore.draft.translation.routingRule.primarySourceLanguage = value.trim();
    }
  },
});

const bidirectionalPrimaryTargets = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "bidirectional"
      ? settingsStore.joinList(settingsStore.draft.translation.routingRule.primaryTargetLanguages)
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "bidirectional") {
      settingsStore.updateList(value, (next) => {
        if (settingsStore.draft.translation.routingRule.kind === "bidirectional") {
          settingsStore.draft.translation.routingRule.primaryTargetLanguages = next;
        }
      });
    }
  },
});

const bidirectionalSecondarySource = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "bidirectional"
      ? settingsStore.draft.translation.routingRule.secondarySourceLanguage
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "bidirectional") {
      settingsStore.draft.translation.routingRule.secondarySourceLanguage = value.trim();
    }
  },
});

const bidirectionalSecondaryTargets = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "bidirectional"
      ? settingsStore.joinList(settingsStore.draft.translation.routingRule.secondaryTargetLanguages)
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "bidirectional") {
      settingsStore.updateList(value, (next) => {
        if (settingsStore.draft.translation.routingRule.kind === "bidirectional") {
          settingsStore.draft.translation.routingRule.secondaryTargetLanguages = next;
        }
      });
    }
  },
});

const fixedTargetLanguages = computed({
  get: () =>
    settingsStore.draft.translation.routingRule.kind === "fixed"
      ? settingsStore.joinList(settingsStore.draft.translation.routingRule.targetLanguages)
      : "",
  set: (value: string) => {
    if (settingsStore.draft.translation.routingRule.kind === "fixed") {
      settingsStore.updateList(value, (next) => {
        if (settingsStore.draft.translation.routingRule.kind === "fixed") {
          settingsStore.draft.translation.routingRule.targetLanguages = next;
        }
      });
    }
  },
});

onMounted(() => {
  settingsStore.load().catch(() => undefined);
});

async function saveSettings() {
  await settingsStore.save().catch(() => undefined);
}

</script>

<style scoped>
.settings-card {
  border-color: var(--color-line);
}

.settings-content {
  padding-top: 24px;
}

.category-tabs {
  margin-bottom: 24px;
}

.settings-tab-panels {
  overflow: visible;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
  justify-content: flex-end;
  width: 100%;
}

.settings-actions {
  padding: 16px 24px;
}

.editor-section {
  display: grid;
  gap: 18px;
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

.field-label {
  margin-bottom: 8px;
  font-size: 0.875rem;
  font-weight: 600;
  line-height: 1.35;
  color: var(--color-text);
}

.toggle-row {
  min-height: 56px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 0 14px;
  border: 1px solid var(--color-line);
  border-radius: 12px;
  background: var(--color-panel);
}

.settings-card :deep(.settings-field .v-field--variant-outlined .v-field__outline) {
  --v-field-border-opacity: 1;
}

.settings-card :deep(.settings-field .v-field) {
  background: var(--color-panel);
  min-height: 40px;
}

.settings-card :deep(.settings-field .v-field__input) {
  min-height: 40px;
  padding-top: 6px;
  padding-bottom: 6px;
  padding-inline: 12px;
  font-size: 0.94rem;
  line-height: 1.4;
  color: var(--color-text);
}

.settings-card :deep(.settings-field--textarea .v-field__input) {
  min-height: 112px;
  padding-top: 12px;
  padding-bottom: 12px;
}

.settings-card :deep(.settings-field .v-messages) {
  padding-top: 6px;
  font-size: 0.78rem;
  line-height: 1.4;
}

.settings-card :deep(.app-btn) {
  min-height: 40px;
  min-width: 112px;
  text-transform: none;
  font-weight: 600;
  font-size: 0.92rem;
  line-height: 1;
  letter-spacing: 0;
  white-space: nowrap;
}

.settings-card :deep(.app-btn .v-icon) {
  font-size: 1rem;
}

.settings-card :deep(.app-btn--compact) {
  min-width: 96px;
  padding-inline: 14px;
}

.settings-card :deep(.settings-tabs .v-tab) {
  min-height: 40px;
  text-transform: none;
  font-size: 0.92rem;
  font-weight: 600;
  letter-spacing: 0;
}

@media (max-width: 959px) {
  .toggle-row {
    align-items: flex-start;
    padding-block: 10px;
  }
}
</style>
