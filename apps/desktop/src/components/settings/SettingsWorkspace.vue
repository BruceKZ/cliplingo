<template>
  <v-sheet :class="compact ? 'pa-1' : ''">
    <v-row class="mb-4" dense>
      <v-col cols="12" xl="8">
        <v-card border rounded="lg">
          <v-card-title>Settings</v-card-title>
          <v-card-subtitle>Unified workspace</v-card-subtitle>
          <v-card-text class="d-flex ga-2 flex-wrap">
            <v-btn variant="tonal" color="primary" @click="settings.resetAll">Reset defaults</v-btn>
            <v-btn variant="tonal" @click="settings.addProvider">Add provider</v-btn>
            <v-btn variant="outlined" @click="settings.makeProviderActive(settings.selectedProviderId ?? settings.providers[0]?.id ?? '')">Focus active provider</v-btn>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" xl="4">
        <v-card border rounded="lg">
          <v-card-title>Live summary</v-card-title>
          <v-card-text>
            <v-list density="compact">
              <v-list-item title="Theme" :subtitle="settings.summary.themeMode" />
              <v-list-item title="Routing" :subtitle="settings.summary.routingKind" />
              <v-list-item title="Providers" :subtitle="String(settings.summary.providerCount)" />
              <v-list-item title="Active provider" :subtitle="settings.summary.activeProviderName" />
              <v-list-item title="Double copy window" :subtitle="`${settings.summary.doubleCopyWindowMs} ms`" />
            </v-list>
            <v-alert :type="settings.summary.hasErrors ? 'warning' : 'success'" variant="tonal" density="comfortable">
              {{ settings.summary.hasErrors ? "Validation needs attention" : "All visible fields validate cleanly" }}
            </v-alert>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-row dense>
      <v-col cols="12" xl="6">
        <v-card border rounded="lg" class="mb-4">
          <v-card-title>General</v-card-title>
          <v-card-text>
            <v-select v-model="themeMode" label="Theme mode" :items="['system', 'light', 'dark']" variant="outlined" density="comfortable" />
            <v-switch v-model="settings.draft.ui.showTrayIcon" label="Show tray icon" color="primary" hide-details class="mt-2" />
          </v-card-text>
        </v-card>

        <v-card border rounded="lg" class="mb-4">
          <v-card-title>Trigger</v-card-title>
          <v-card-text>
            <v-slider v-model="settings.draft.trigger.doubleCopyWindowMs" label="Double copy window" :min="250" :max="800" :step="10" thumb-label />
            <v-text-field
              v-model.number="settings.draft.trigger.doubleCopyWindowMs"
              type="number"
              label="Double copy window (ms)"
              variant="outlined"
              density="comfortable"
              :error-messages="settings.validation.trigger.doubleCopyWindowMs.valid ? [] : [settings.validation.trigger.doubleCopyWindowMs.message ?? 'Invalid value']"
            />
            <v-text-field
              v-model.trim="settings.draft.trigger.fallbackShortcut"
              label="Fallback shortcut"
              placeholder="CmdOrCtrl+Shift+Y"
              variant="outlined"
              density="comfortable"
              :error-messages="settings.validation.trigger.fallbackShortcut.valid ? [] : [settings.validation.trigger.fallbackShortcut.message ?? 'Invalid shortcut']"
            />
            <v-switch v-model="settings.draft.trigger.replacePopupOnNewTrigger" label="Replace popup on new trigger" color="primary" hide-details />
          </v-card-text>
        </v-card>

        <v-card border rounded="lg" class="mb-4">
          <v-card-title>Language routing</v-card-title>
          <v-card-text>
            <v-select
              v-model="routingKind"
              label="Routing mode"
              :items="['branching', 'bidirectional', 'fixed']"
              variant="outlined"
              density="comfortable"
            />

            <v-row v-if="routingKind === 'branching'" dense>
              <v-col cols="12" md="6">
                <v-text-field v-model.trim="branchingEnglishSourceText" label="English source" variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="6">
                <v-textarea v-model="branchingEnglishTargetsText" label="English targets" rows="2" variant="outlined" density="comfortable" auto-grow />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field v-model.trim="branchingChineseSourceText" label="Chinese source" variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="6">
                <v-textarea v-model="branchingChineseTargetsText" label="Chinese targets" rows="2" variant="outlined" density="comfortable" auto-grow />
              </v-col>
              <v-col cols="12">
                <v-textarea v-model="branchingFallbackTargetsText" label="Fallback targets" rows="2" variant="outlined" density="comfortable" auto-grow />
              </v-col>
            </v-row>

            <v-row v-else-if="routingKind === 'bidirectional'" dense>
              <v-col cols="12" md="6">
                <v-text-field v-model.trim="bidirectionalPrimarySourceText" label="Primary source" variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="6">
                <v-textarea v-model="bidirectionalPrimaryTargetsText" label="Primary targets" rows="2" variant="outlined" density="comfortable" auto-grow />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field v-model.trim="bidirectionalSecondarySourceText" label="Secondary source" variant="outlined" density="comfortable" />
              </v-col>
              <v-col cols="12" md="6">
                <v-textarea v-model="bidirectionalSecondaryTargetsText" label="Secondary targets" rows="2" variant="outlined" density="comfortable" auto-grow />
              </v-col>
            </v-row>

            <v-textarea
              v-else
              v-model="fixedTargetLanguagesText"
              label="Target languages"
              rows="3"
              variant="outlined"
              density="comfortable"
              auto-grow
            />

            <v-row dense>
              <v-col cols="12" md="7">
                <v-textarea
                  v-model.trim="userRulesText"
                  label="User translation rules"
                  rows="3"
                  placeholder="Tone, glossary, formatting preferences..."
                  variant="outlined"
                  density="comfortable"
                  auto-grow
                />
              </v-col>
              <v-col cols="12" md="5">
                <v-switch v-model="settings.draft.translation.preserveParagraphs" label="Preserve paragraphs" color="primary" hide-details class="mt-2" />
              </v-col>
            </v-row>

            <v-alert
              v-if="!settings.validation.translation.routingRule.valid"
              type="warning"
              variant="tonal"
              density="comfortable"
            >
              {{ settings.validation.translation.routingRule.message }}
            </v-alert>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" xl="6">
        <v-card border rounded="lg" class="mb-4">
          <v-card-title class="d-flex align-center justify-space-between">
            <span>Providers</span>
            <div class="d-flex ga-2">
              <v-btn size="small" variant="tonal" @click="settings.addProvider">Add</v-btn>
              <v-btn size="small" variant="outlined" @click="settings.makeProviderActive(settings.selectedProviderId ?? settings.providers[0]?.id ?? '')">Sync active</v-btn>
            </div>
          </v-card-title>

          <v-card-text>
            <v-expansion-panels multiple>
              <v-expansion-panel v-for="(provider, index) in settings.providers" :key="provider.id">
                <v-expansion-panel-title>
                  <div class="d-flex align-center ga-2">
                    <span>{{ provider.name || `Provider ${index + 1}` }}</span>
                    <v-chip size="x-small" label>{{ settings.activeProviderId === provider.id ? "Active" : "Draft" }}</v-chip>
                  </div>
                </v-expansion-panel-title>
                <v-expansion-panel-text>
                  <div class="d-flex flex-wrap ga-2 mb-3">
                    <v-btn size="small" variant="outlined" @click="settings.selectProvider(provider.id)">Select</v-btn>
                    <v-btn size="small" variant="tonal" @click="settings.makeProviderActive(provider.id)">Make active</v-btn>
                    <v-btn size="small" color="error" variant="outlined" :disabled="settings.providers.length === 1" @click="settings.removeProvider(provider.id)">Remove</v-btn>
                  </div>

                  <v-row dense>
                    <v-col cols="12" md="6">
                      <v-text-field v-model.trim="provider.id" label="Provider id" variant="outlined" density="comfortable" :error-messages="providerValidation(index).id.valid ? [] : [providerValidation(index).id.message ?? 'Invalid']" />
                    </v-col>
                    <v-col cols="12" md="6">
                      <v-text-field v-model.trim="provider.name" label="Display name" variant="outlined" density="comfortable" :error-messages="providerValidation(index).name.valid ? [] : [providerValidation(index).name.message ?? 'Invalid']" />
                    </v-col>
                    <v-col cols="12" md="6">
                      <v-select v-model="provider.kind" label="Provider kind" :items="['openai-compatible']" variant="outlined" density="comfortable" />
                    </v-col>
                    <v-col cols="12" md="6">
                      <v-select v-model="provider.authScheme" label="Auth scheme" :items="['bearer', 'none']" variant="outlined" density="comfortable" />
                    </v-col>
                    <v-col cols="12">
                      <v-text-field v-model.trim="provider.baseUrl" label="Base URL" placeholder="https://api.example.com" variant="outlined" density="comfortable" :error-messages="providerValidation(index).baseUrl.valid ? [] : [providerValidation(index).baseUrl.message ?? 'Invalid URL']" />
                    </v-col>
                    <v-col cols="12" md="6">
                      <v-text-field v-model.trim="provider.path" label="Request path" placeholder="/chat/completions" variant="outlined" density="comfortable" :error-messages="providerValidation(index).path.valid ? [] : [providerValidation(index).path.message ?? 'Invalid path']" />
                    </v-col>
                    <v-col cols="12" md="6">
                      <v-text-field v-model.trim="provider.organization" label="Organization" placeholder="Optional" variant="outlined" density="comfortable" />
                    </v-col>
                    <v-col cols="12">
                      <v-text-field
                        v-model.trim="provider.apiKeyDraft"
                        :type="provider.authScheme === 'none' ? 'text' : 'password'"
                        label="API key draft"
                        placeholder="sk-..."
                        variant="outlined"
                        density="comfortable"
                        :error-messages="providerValidation(index).apiKey.valid ? [] : [providerValidation(index).apiKey.message ?? 'Invalid key']"
                      />
                    </v-col>
                  </v-row>

                  <div class="d-flex align-center justify-space-between mb-2">
                    <span class="text-body-2">Custom headers</span>
                    <v-btn size="small" variant="outlined" @click="settings.addProviderHeader(provider.id)">Add header</v-btn>
                  </div>
                  <v-row v-for="(header, headerIndex) in provider.customHeaders" :key="`${provider.id}-${headerIndex}`" dense>
                    <v-col cols="12" md="5">
                      <v-text-field v-model.trim="header.name" label="Header name" variant="outlined" density="comfortable" />
                    </v-col>
                    <v-col cols="12" md="5">
                      <v-text-field v-model.trim="header.value" label="Header value" variant="outlined" density="comfortable" />
                    </v-col>
                    <v-col cols="12" md="2">
                      <v-btn block color="error" variant="outlined" @click="settings.removeProviderHeader(provider.id, headerIndex)">Remove</v-btn>
                    </v-col>
                  </v-row>
                  <v-alert v-if="!providerValidation(index).headers.valid" type="warning" variant="tonal" density="comfortable" class="mt-2">
                    {{ providerValidation(index).headers.message }}
                  </v-alert>
                </v-expansion-panel-text>
              </v-expansion-panel>
            </v-expansion-panels>
          </v-card-text>
        </v-card>

        <v-card border rounded="lg" class="mb-4">
          <v-card-title>Model params</v-card-title>
          <v-card-text>
            <v-select
              v-model="settings.selectedProviderId"
              label="Active provider"
              :items="settings.providers.map((provider) => ({ title: provider.name || provider.id, value: provider.id }))"
              variant="outlined"
              density="comfortable"
              @update:model-value="settings.makeProviderActive(settings.selectedProviderId ?? '')"
            />

            <v-row v-if="selectedProvider" dense>
              <v-col cols="12" md="6">
                <v-text-field v-model.trim="selectedProvider.model" label="Model" placeholder="gpt-4o-mini" variant="outlined" density="comfortable" :error-messages="providerValidation(selectedProviderIndex).model.valid ? [] : [providerValidation(selectedProviderIndex).model.message ?? 'Invalid model']" />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field v-model.number="selectedProvider.temperature" type="number" min="0" max="2" step="0.1" label="Temperature" variant="outlined" density="comfortable" :error-messages="providerValidation(selectedProviderIndex).temperature.valid ? [] : [providerValidation(selectedProviderIndex).temperature.message ?? 'Invalid temperature']" />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field v-model.number="selectedProvider.topP" type="number" min="0" max="1" step="0.05" label="Top P" variant="outlined" density="comfortable" :error-messages="providerValidation(selectedProviderIndex).topP.valid ? [] : [providerValidation(selectedProviderIndex).topP.message ?? 'Invalid topP']" />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field v-model.number="selectedProvider.maxTokens" type="number" min="1" max="32000" step="1" label="Max tokens" variant="outlined" density="comfortable" :error-messages="providerValidation(selectedProviderIndex).maxTokens.valid ? [] : [providerValidation(selectedProviderIndex).maxTokens.message ?? 'Invalid max tokens']" />
              </v-col>
              <v-col cols="12" md="6">
                <v-text-field v-model.number="selectedProvider.timeoutSecs" type="number" min="3" max="120" step="1" label="Request timeout" variant="outlined" density="comfortable" :error-messages="providerValidation(selectedProviderIndex).timeoutSecs.valid ? [] : [providerValidation(selectedProviderIndex).timeoutSecs.message ?? 'Invalid timeout']" />
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>

        <v-card border rounded="lg">
          <v-card-title>Privacy</v-card-title>
          <v-card-text>
            <v-switch v-model="settings.draft.history.enabled" label="Enable local history" color="primary" hide-details />
            <v-text-field
              v-model.number="settings.draft.history.maxItems"
              type="number"
              min="1"
              max="5000"
              step="1"
              label="History limit"
              variant="outlined"
              density="comfortable"
              class="mt-2"
              :error-messages="settings.validation.history.maxItems.valid ? [] : [settings.validation.history.maxItems.message ?? 'Invalid value']"
            />
            <v-switch v-model="settings.draft.history.storeFullText" label="Store full text in history" color="primary" hide-details />
            <v-switch v-model="settings.draft.debug.logRawNetworkErrors" label="Log raw network errors" color="primary" hide-details />
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-sheet>
</template>

<script setup lang="ts">
import { computed, watch } from "vue";
import { useSettingsStore, type ProviderValidation } from "@/stores/settings";
import { useUiStore } from "@/stores/ui";
import type { RoutingKind, ThemeMode } from "@/types";

const { compact = false } = defineProps<{
  compact?: boolean;
}>();

const settings = useSettingsStore();
const uiStore = useUiStore();

watch(
  () => settings.draft.ui.themeMode,
  (nextTheme) => {
    uiStore.applyTheme(nextTheme);
  },
  { immediate: true },
);

const themeMode = computed<ThemeMode>({
  get: () => settings.draft.ui.themeMode,
  set: (nextTheme) => {
    settings.draft.ui.themeMode = nextTheme;
    uiStore.applyTheme(nextTheme);
  },
});

const routingKind = computed<RoutingKind>({
  get: () => settings.draft.translation.routingRule.kind,
  set: (nextKind) => {
    settings.setRoutingKind(nextKind);
  },
});

const selectedProvider = computed(() => {
  return (
    settings.providers.find(
      (provider) => provider.id === settings.selectedProviderId,
    ) ?? settings.providers[0] ?? null
  );
});

const selectedProviderIndex = computed(() => {
  if (!selectedProvider.value) {
    return 0;
  }

  return settings.providers.findIndex(
    (provider) => provider.id === selectedProvider.value?.id,
  );
});

const userRulesText = computed<string>({
  get: () => settings.draft.translation.userRules ?? "",
  set: (nextRules) => {
    settings.draft.translation.userRules = nextRules;
  },
});

function providerValidation(index: number): ProviderValidation {
  return (
    settings.validation.providers[index] ?? {
      id: { valid: false, message: "Provider is not available." },
      name: { valid: false, message: "Provider is not available." },
      baseUrl: { valid: false, message: "Provider is not available." },
      path: { valid: false, message: "Provider is not available." },
      apiKey: { valid: false, message: "Provider is not available." },
      model: { valid: false, message: "Provider is not available." },
      temperature: { valid: false, message: "Provider is not available." },
      topP: { valid: false, message: "Provider is not available." },
      maxTokens: { valid: false, message: "Provider is not available." },
      timeoutSecs: { valid: false, message: "Provider is not available." },
      headers: { valid: false, message: "Provider is not available." },
      hasErrors: true,
    }
  );
}

function listField(
  getter: () => readonly string[],
  setter: (next: string[]) => void,
) {
  return computed({
    get: () => settings.joinList(getter()),
    set: (nextValue: string) => {
      setter(settings.splitList(nextValue));
    },
  });
}

const branchingEnglishSourceText = computed<string>({
  get: () =>
    settings.draft.translation.routingRule.kind === "branching"
      ? settings.draft.translation.routingRule.englishSourceLanguage
      : "",
  set: (nextValue) => {
    if (settings.draft.translation.routingRule.kind === "branching") {
      settings.draft.translation.routingRule.englishSourceLanguage = nextValue;
    }
  },
});

const branchingEnglishTargetsText = listField(
  () =>
    settings.draft.translation.routingRule.kind === "branching"
      ? settings.draft.translation.routingRule.englishTargetLanguages
      : [],
  (next) => {
    if (settings.draft.translation.routingRule.kind === "branching") {
      settings.draft.translation.routingRule.englishTargetLanguages = next;
    }
  },
);

const branchingChineseSourceText = computed<string>({
  get: () =>
    settings.draft.translation.routingRule.kind === "branching"
      ? settings.draft.translation.routingRule.chineseSourceLanguage
      : "",
  set: (nextValue) => {
    if (settings.draft.translation.routingRule.kind === "branching") {
      settings.draft.translation.routingRule.chineseSourceLanguage = nextValue;
    }
  },
});

const branchingChineseTargetsText = listField(
  () =>
    settings.draft.translation.routingRule.kind === "branching"
      ? settings.draft.translation.routingRule.chineseTargetLanguages
      : [],
  (next) => {
    if (settings.draft.translation.routingRule.kind === "branching") {
      settings.draft.translation.routingRule.chineseTargetLanguages = next;
    }
  },
);

const branchingFallbackTargetsText = listField(
  () =>
    settings.draft.translation.routingRule.kind === "branching"
      ? settings.draft.translation.routingRule.fallbackTargetLanguages
      : [],
  (next) => {
    if (settings.draft.translation.routingRule.kind === "branching") {
      settings.draft.translation.routingRule.fallbackTargetLanguages = next;
    }
  },
);

const bidirectionalPrimarySourceText = computed<string>({
  get: () =>
    settings.draft.translation.routingRule.kind === "bidirectional"
      ? settings.draft.translation.routingRule.primarySourceLanguage
      : "",
  set: (nextValue) => {
    if (settings.draft.translation.routingRule.kind === "bidirectional") {
      settings.draft.translation.routingRule.primarySourceLanguage = nextValue;
    }
  },
});

const bidirectionalPrimaryTargetsText = listField(
  () =>
    settings.draft.translation.routingRule.kind === "bidirectional"
      ? settings.draft.translation.routingRule.primaryTargetLanguages
      : [],
  (next) => {
    if (settings.draft.translation.routingRule.kind === "bidirectional") {
      settings.draft.translation.routingRule.primaryTargetLanguages = next;
    }
  },
);

const bidirectionalSecondarySourceText = computed<string>({
  get: () =>
    settings.draft.translation.routingRule.kind === "bidirectional"
      ? settings.draft.translation.routingRule.secondarySourceLanguage
      : "",
  set: (nextValue) => {
    if (settings.draft.translation.routingRule.kind === "bidirectional") {
      settings.draft.translation.routingRule.secondarySourceLanguage = nextValue;
    }
  },
});

const bidirectionalSecondaryTargetsText = listField(
  () =>
    settings.draft.translation.routingRule.kind === "bidirectional"
      ? settings.draft.translation.routingRule.secondaryTargetLanguages
      : [],
  (next) => {
    if (settings.draft.translation.routingRule.kind === "bidirectional") {
      settings.draft.translation.routingRule.secondaryTargetLanguages = next;
    }
  },
);

const fixedTargetLanguagesText = listField(
  () =>
    settings.draft.translation.routingRule.kind === "fixed"
      ? settings.draft.translation.routingRule.targetLanguages
      : [],
  (next) => {
    if (settings.draft.translation.routingRule.kind === "fixed") {
      settings.draft.translation.routingRule.targetLanguages = next;
    }
  },
);
</script>
