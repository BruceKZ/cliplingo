<template>
  <section :class="compact ? 'space-y-5' : 'space-y-6'">
    <div class="grid gap-4 xl:grid-cols-[1.35fr_0.85fr]">
      <article
        class="rounded-3xl border border-app-line bg-gradient-to-br from-app-panel via-app-panel to-app-accent-soft/40 p-6"
      >
        <p class="text-xs font-semibold uppercase tracking-[0.24em] text-app-accent">
          Settings Draft
        </p>
        <h2 class="mt-3 text-2xl font-semibold tracking-tight text-app-text">
          Tune trigger, provider, routing, and privacy without leaving the app.
        </h2>
        <p class="mt-3 max-w-3xl text-sm leading-7 text-app-muted">
          These controls are live drafts. Validation is shown inline and the
          active theme updates immediately.
        </p>

        <div class="mt-5 flex flex-wrap gap-2">
          <button
            type="button"
            class="rounded-full border border-app-line bg-app-bg px-4 py-2 text-sm text-app-text transition-colors hover:border-app-accent"
            @click="settings.resetAll"
          >
            Reset defaults
          </button>
          <button
            type="button"
            class="rounded-full border border-app-line bg-app-bg px-4 py-2 text-sm text-app-text transition-colors hover:border-app-accent"
            @click="settings.addProvider"
          >
            Add provider
          </button>
          <button
            type="button"
            class="rounded-full border border-app-line bg-app-bg px-4 py-2 text-sm text-app-text transition-colors hover:border-app-accent"
            @click="settings.makeProviderActive(settings.selectedProviderId ?? settings.providers[0]?.id ?? '')"
          >
            Focus active provider
          </button>
        </div>
      </article>

      <aside class="grid gap-4">
        <div class="rounded-3xl border border-app-line bg-app-panel p-5">
          <p class="text-xs font-semibold uppercase tracking-[0.24em] text-app-muted">
            Live Summary
          </p>
          <dl class="mt-4 grid gap-3 text-sm">
            <div class="flex items-center justify-between gap-4">
              <dt class="text-app-muted">Theme</dt>
              <dd class="font-medium text-app-text">{{ settings.summary.themeMode }}</dd>
            </div>
            <div class="flex items-center justify-between gap-4">
              <dt class="text-app-muted">Routing</dt>
              <dd class="font-medium text-app-text">{{ settings.summary.routingKind }}</dd>
            </div>
            <div class="flex items-center justify-between gap-4">
              <dt class="text-app-muted">Providers</dt>
              <dd class="font-medium text-app-text">{{ settings.summary.providerCount }}</dd>
            </div>
            <div class="flex items-center justify-between gap-4">
              <dt class="text-app-muted">Active provider</dt>
              <dd class="max-w-[12rem] truncate font-medium text-app-text">
                {{ settings.summary.activeProviderName }}
              </dd>
            </div>
            <div class="flex items-center justify-between gap-4">
              <dt class="text-app-muted">Double copy window</dt>
              <dd class="font-medium text-app-text">
                {{ settings.summary.doubleCopyWindowMs }} ms
              </dd>
            </div>
          </dl>
        </div>

        <div
          class="rounded-3xl border px-5 py-4 text-sm"
          :class="
            settings.summary.hasErrors
              ? 'border-amber-500/40 bg-amber-500/10 text-amber-100 dark:text-amber-100'
              : 'border-emerald-500/40 bg-emerald-500/10 text-emerald-100 dark:text-emerald-100'
          "
        >
          <p class="font-medium">
            {{ settings.summary.hasErrors ? "Validation needs attention" : "All visible fields validate cleanly" }}
          </p>
          <p class="mt-1 leading-6 opacity-90">
            Every field updates immediately. Future persistence work can reuse
            this same draft surface.
          </p>
        </div>
      </aside>
    </div>

    <div class="grid gap-6 xl:grid-cols-2">
      <section class="rounded-3xl border border-app-line bg-app-panel p-6">
        <SectionTitle kicker="General" title="Application shell" />

        <div class="mt-6 grid gap-4 sm:grid-cols-2">
          <label class="space-y-2">
            <span class="text-sm font-medium text-app-text">Theme mode</span>
            <select v-model="themeMode" class="field-input">
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </label>

          <label class="space-y-2">
            <span class="text-sm font-medium text-app-text">Show tray icon</span>
            <div class="flex items-center gap-3 rounded-2xl border border-app-line bg-app-bg px-4 py-3">
              <input v-model="settings.draft.ui.showTrayIcon" type="checkbox" class="h-4 w-4" />
              <span class="text-sm text-app-muted">Keep the app in the system tray</span>
            </div>
          </label>
        </div>
      </section>

      <section class="rounded-3xl border border-app-line bg-app-panel p-6">
        <SectionTitle kicker="Trigger" title="Copy shortcut and timing" />

        <div class="mt-6 space-y-5">
          <label class="space-y-2">
            <div class="flex items-center justify-between gap-4">
              <span class="text-sm font-medium text-app-text">Double copy window</span>
              <span class="text-xs text-app-muted">{{ settings.draft.trigger.doubleCopyWindowMs }} ms</span>
            </div>
            <input
              v-model.number="settings.draft.trigger.doubleCopyWindowMs"
              type="range"
              min="250"
              max="800"
              step="10"
              class="w-full accent-[color:var(--color-accent)]"
            />
            <input
              v-model.number="settings.draft.trigger.doubleCopyWindowMs"
              type="number"
              min="250"
              max="800"
              step="10"
              class="field-input"
            />
            <p v-if="!settings.validation.trigger.doubleCopyWindowMs.valid" class="text-xs text-app-danger">
              {{ settings.validation.trigger.doubleCopyWindowMs.message }}
            </p>
          </label>

          <label class="space-y-2">
            <span class="text-sm font-medium text-app-text">Fallback shortcut</span>
            <input
              v-model.trim="settings.draft.trigger.fallbackShortcut"
              type="text"
              placeholder="CmdOrCtrl+Shift+Y"
              class="field-input"
            />
            <p v-if="!settings.validation.trigger.fallbackShortcut.valid" class="text-xs text-app-danger">
              {{ settings.validation.trigger.fallbackShortcut.message }}
            </p>
          </label>

          <label class="flex items-center gap-3 rounded-2xl border border-app-line bg-app-bg px-4 py-3">
            <input
              v-model="settings.draft.trigger.replacePopupOnNewTrigger"
              type="checkbox"
              class="h-4 w-4"
            />
            <span class="text-sm text-app-muted">
              Replace the current popup when a new translation fires
            </span>
          </label>
        </div>
      </section>
    </div>

    <section class="rounded-3xl border border-app-line bg-app-panel p-6">
      <SectionTitle kicker="Language routing" title="Automatic target selection" />

      <div class="mt-6 grid gap-4 lg:grid-cols-[220px_1fr]">
        <label class="space-y-2">
          <span class="text-sm font-medium text-app-text">Routing mode</span>
          <select
            v-model="routingKind"
            class="field-input"
          >
            <option value="branching">Branching</option>
            <option value="bidirectional">Bidirectional</option>
            <option value="fixed">Fixed</option>
          </select>
        </label>

        <div class="space-y-4">
          <div v-if="routingKind === 'branching'" class="grid gap-4 xl:grid-cols-2">
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">English source</span>
              <input v-model.trim="branchingEnglishSourceText" type="text" class="field-input" />
            </label>
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">English targets</span>
              <textarea v-model="branchingEnglishTargetsText" rows="2" class="field-input" />
            </label>
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Chinese source</span>
              <input v-model.trim="branchingChineseSourceText" type="text" class="field-input" />
            </label>
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Chinese targets</span>
              <textarea v-model="branchingChineseTargetsText" rows="2" class="field-input" />
            </label>
            <label class="space-y-2 xl:col-span-2">
              <span class="text-sm font-medium text-app-text">Fallback targets</span>
              <textarea v-model="branchingFallbackTargetsText" rows="2" class="field-input" />
            </label>
          </div>

          <div v-else-if="routingKind === 'bidirectional'" class="grid gap-4 xl:grid-cols-2">
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Primary source</span>
              <input v-model.trim="bidirectionalPrimarySourceText" type="text" class="field-input" />
            </label>
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Primary targets</span>
              <textarea v-model="bidirectionalPrimaryTargetsText" rows="2" class="field-input" />
            </label>
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Secondary source</span>
              <input v-model.trim="bidirectionalSecondarySourceText" type="text" class="field-input" />
            </label>
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Secondary targets</span>
              <textarea v-model="bidirectionalSecondaryTargetsText" rows="2" class="field-input" />
            </label>
          </div>

          <div v-else class="space-y-4">
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Target languages</span>
              <textarea v-model="fixedTargetLanguagesText" rows="3" class="field-input" />
            </label>
          </div>

          <div class="grid gap-4 xl:grid-cols-2">
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">User translation rules</span>
              <textarea
                v-model.trim="userRulesText"
                rows="4"
                placeholder="Tone, glossary, formatting preferences..."
                class="field-input"
              />
            </label>
            <label class="flex items-center gap-3 rounded-2xl border border-app-line bg-app-bg px-4 py-3">
              <input
                v-model="settings.draft.translation.preserveParagraphs"
                type="checkbox"
                class="h-4 w-4"
              />
              <span class="text-sm text-app-muted">
                Preserve original paragraph structure in translated output
              </span>
            </label>
          </div>

          <p v-if="!settings.validation.translation.routingRule.valid" class="text-xs text-app-danger">
            {{ settings.validation.translation.routingRule.message }}
          </p>
        </div>
      </div>
    </section>

    <section class="rounded-3xl border border-app-line bg-app-panel p-6">
      <div class="flex flex-col gap-3 lg:flex-row lg:items-end lg:justify-between">
        <SectionTitle kicker="Provider" title="OpenAI-compatible configurations" />
        <div class="flex flex-wrap gap-2">
          <button
            type="button"
            class="rounded-full border border-app-line bg-app-bg px-4 py-2 text-sm text-app-text transition-colors hover:border-app-accent"
            @click="settings.addProvider"
          >
            Add provider
          </button>
          <button
            type="button"
            class="rounded-full border border-app-line bg-app-bg px-4 py-2 text-sm text-app-text transition-colors hover:border-app-accent"
            @click="settings.makeProviderActive(settings.selectedProviderId ?? settings.providers[0]?.id ?? '')"
          >
            Sync active selection
          </button>
        </div>
      </div>

      <div class="mt-6 space-y-4">
        <article
          v-for="(provider, index) in settings.providers"
          :key="provider.id"
          class="rounded-3xl border border-app-line bg-app-bg p-5"
        >
          <div class="flex flex-col gap-3 border-b border-app-line pb-4 lg:flex-row lg:items-center lg:justify-between">
            <div class="space-y-1">
              <div class="flex flex-wrap items-center gap-2">
                <h3 class="text-lg font-semibold text-app-text">
                  {{ provider.name || `Provider ${index + 1}` }}
                </h3>
                <span
                  class="rounded-full px-3 py-1 text-xs font-medium"
                  :class="
                    settings.activeProviderId === provider.id
                      ? 'bg-app-accent-soft text-app-text'
                      : 'bg-app-bg text-app-muted'
                  "
                >
                  {{ settings.activeProviderId === provider.id ? "Active" : "Draft" }}
                </span>
              </div>
              <p class="text-xs text-app-muted">
                Provider id: <span class="font-medium">{{ provider.id }}</span>
              </p>
            </div>

            <div class="flex flex-wrap gap-2">
              <button
                type="button"
                class="rounded-full border border-app-line px-4 py-2 text-sm text-app-text transition-colors hover:border-app-accent"
                @click="settings.selectProvider(provider.id)"
              >
                Select
              </button>
              <button
                type="button"
                class="rounded-full border border-app-line px-4 py-2 text-sm text-app-text transition-colors hover:border-app-accent"
                @click="settings.makeProviderActive(provider.id)"
              >
                Make active
              </button>
              <button
                type="button"
                class="rounded-full border border-app-line px-4 py-2 text-sm text-app-text transition-colors hover:border-app-danger disabled:cursor-not-allowed disabled:opacity-50"
                :disabled="settings.providers.length === 1"
                @click="settings.removeProvider(provider.id)"
              >
                Remove
              </button>
            </div>
          </div>

          <div class="mt-5 grid gap-4 lg:grid-cols-2">
            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Provider id</span>
              <input v-model.trim="provider.id" type="text" class="field-input" />
              <p v-if="!providerValidation(index).id.valid" class="text-xs text-app-danger">
                {{ providerValidation(index).id.message }}
              </p>
            </label>

            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Display name</span>
              <input v-model.trim="provider.name" type="text" class="field-input" />
              <p v-if="!providerValidation(index).name.valid" class="text-xs text-app-danger">
                {{ providerValidation(index).name.message }}
              </p>
            </label>

            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Provider kind</span>
              <select v-model="provider.kind" class="field-input">
                <option value="openai-compatible">OpenAI-compatible</option>
              </select>
            </label>

            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Auth scheme</span>
              <select v-model="provider.authScheme" class="field-input">
                <option value="bearer">Bearer</option>
                <option value="none">None</option>
              </select>
            </label>

            <label class="space-y-2 lg:col-span-2">
              <span class="text-sm font-medium text-app-text">Base URL</span>
              <input
                v-model.trim="provider.baseUrl"
                type="url"
                placeholder="https://api.example.com"
                class="field-input"
              />
              <p v-if="!providerValidation(index).baseUrl.valid" class="text-xs text-app-danger">
                {{ providerValidation(index).baseUrl.message }}
              </p>
            </label>

            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Request path</span>
              <input
                v-model.trim="provider.path"
                type="text"
                placeholder="/chat/completions"
                class="field-input"
              />
              <p v-if="!providerValidation(index).path.valid" class="text-xs text-app-danger">
                {{ providerValidation(index).path.message }}
              </p>
            </label>

            <label class="space-y-2">
              <span class="text-sm font-medium text-app-text">Organization</span>
              <input
                v-model.trim="provider.organization"
                type="text"
                placeholder="Optional"
                class="field-input"
              />
            </label>

            <label class="space-y-2 lg:col-span-2">
              <span class="text-sm font-medium text-app-text">API key draft</span>
              <input
                v-model.trim="provider.apiKeyDraft"
                :type="provider.authScheme === 'none' ? 'text' : 'password'"
                placeholder="sk-..."
                class="field-input"
              />
              <p v-if="!providerValidation(index).apiKey.valid" class="text-xs text-app-danger">
                {{ providerValidation(index).apiKey.message }}
              </p>
            </label>

            <div class="lg:col-span-2">
              <div class="flex items-center justify-between gap-3">
                <span class="text-sm font-medium text-app-text">Custom headers</span>
                <button
                  type="button"
                  class="rounded-full border border-app-line px-3 py-1.5 text-xs text-app-text transition-colors hover:border-app-accent"
                  @click="settings.addProviderHeader(provider.id)"
                >
                  Add header
                </button>
              </div>

              <div class="mt-3 space-y-3">
                <div
                  v-for="(header, headerIndex) in provider.customHeaders"
                  :key="`${provider.id}-${headerIndex}`"
                  class="grid gap-3 rounded-2xl border border-app-line bg-app-panel p-3 md:grid-cols-[1fr_1fr_auto]"
                >
                  <input v-model.trim="header.name" type="text" placeholder="Header name" class="field-input" />
                  <input v-model.trim="header.value" type="text" placeholder="Header value" class="field-input" />
                  <button
                    type="button"
                    class="rounded-xl border border-app-line px-3 py-2 text-sm text-app-text transition-colors hover:border-app-danger"
                    @click="settings.removeProviderHeader(provider.id, headerIndex)"
                  >
                    Remove
                  </button>
                </div>
              </div>

              <p v-if="!providerValidation(index).headers.valid" class="mt-3 text-xs text-app-danger">
                {{ providerValidation(index).headers.message }}
              </p>
            </div>
          </div>
        </article>
      </div>
    </section>

    <section class="rounded-3xl border border-app-line bg-app-panel p-6">
      <div class="flex flex-col gap-3 lg:flex-row lg:items-end lg:justify-between">
        <SectionTitle kicker="Model params" title="Selected provider generation controls" />
        <label class="w-full max-w-sm space-y-2">
          <span class="text-sm font-medium text-app-text">Active provider</span>
          <select
            v-model="settings.selectedProviderId"
            class="field-input"
            @change="settings.makeProviderActive(settings.selectedProviderId ?? '')"
          >
            <option
              v-for="provider in settings.providers"
              :key="provider.id"
              :value="provider.id"
            >
              {{ provider.name || provider.id }}
            </option>
          </select>
        </label>
      </div>

      <div v-if="selectedProvider" class="mt-6 grid gap-4 lg:grid-cols-2 xl:grid-cols-4">
        <label class="space-y-2">
          <span class="text-sm font-medium text-app-text">Model</span>
          <input v-model.trim="selectedProvider.model" type="text" placeholder="gpt-4o-mini" class="field-input" />
          <p v-if="!providerValidation(selectedProviderIndex).model.valid" class="text-xs text-app-danger">
            {{ providerValidation(selectedProviderIndex).model.message }}
          </p>
        </label>

        <label class="space-y-2">
          <span class="text-sm font-medium text-app-text">Temperature</span>
          <input
            v-model.number="selectedProvider.temperature"
            type="number"
            min="0"
            max="2"
            step="0.1"
            class="field-input"
          />
          <p v-if="!providerValidation(selectedProviderIndex).temperature.valid" class="text-xs text-app-danger">
            {{ providerValidation(selectedProviderIndex).temperature.message }}
          </p>
        </label>

        <label class="space-y-2">
          <span class="text-sm font-medium text-app-text">Top P</span>
          <input
            v-model.number="selectedProvider.topP"
            type="number"
            min="0"
            max="1"
            step="0.05"
            class="field-input"
          />
          <p v-if="!providerValidation(selectedProviderIndex).topP.valid" class="text-xs text-app-danger">
            {{ providerValidation(selectedProviderIndex).topP.message }}
          </p>
        </label>

        <label class="space-y-2">
          <span class="text-sm font-medium text-app-text">Max tokens</span>
          <input
            v-model.number="selectedProvider.maxTokens"
            type="number"
            min="1"
            max="32000"
            step="1"
            class="field-input"
          />
          <p v-if="!providerValidation(selectedProviderIndex).maxTokens.valid" class="text-xs text-app-danger">
            {{ providerValidation(selectedProviderIndex).maxTokens.message }}
          </p>
        </label>

        <label class="space-y-2">
          <span class="text-sm font-medium text-app-text">Request timeout</span>
          <input
            v-model.number="selectedProvider.timeoutSecs"
            type="number"
            min="3"
            max="120"
            step="1"
            class="field-input"
          />
          <p v-if="!providerValidation(selectedProviderIndex).timeoutSecs.valid" class="text-xs text-app-danger">
            {{ providerValidation(selectedProviderIndex).timeoutSecs.message }}
          </p>
        </label>
      </div>
    </section>

    <section class="rounded-3xl border border-app-line bg-app-panel p-6">
      <SectionTitle kicker="Privacy" title="History, logs, and safety" />

      <div class="mt-6 grid gap-4 lg:grid-cols-2">
        <label class="flex items-center gap-3 rounded-2xl border border-app-line bg-app-bg px-4 py-3">
          <input v-model="settings.draft.history.enabled" type="checkbox" class="h-4 w-4" />
          <span class="text-sm text-app-muted">Enable local history</span>
        </label>

        <label class="space-y-2">
          <span class="text-sm font-medium text-app-text">History limit</span>
          <input
            v-model.number="settings.draft.history.maxItems"
            type="number"
            min="1"
            max="5000"
            step="1"
            class="field-input"
          />
          <p v-if="!settings.validation.history.maxItems.valid" class="text-xs text-app-danger">
            {{ settings.validation.history.maxItems.message }}
          </p>
        </label>

        <label class="flex items-center gap-3 rounded-2xl border border-app-line bg-app-bg px-4 py-3">
          <input v-model="settings.draft.history.storeFullText" type="checkbox" class="h-4 w-4" />
          <span class="text-sm text-app-muted">
            Store full text in history instead of a clipped preview
          </span>
        </label>

        <label class="flex items-center gap-3 rounded-2xl border border-app-line bg-app-bg px-4 py-3">
          <input v-model="settings.draft.debug.logRawNetworkErrors" type="checkbox" class="h-4 w-4" />
          <span class="text-sm text-app-muted">
            Log raw network errors for troubleshooting
          </span>
        </label>
      </div>
    </section>
  </section>
</template>

<script setup lang="ts">
import { computed, watch } from "vue";
import SectionTitle from "./SectionTitle.vue";
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

<style scoped>
.field-input {
  @apply w-full rounded-2xl border border-app-line bg-app-bg px-4 py-3 text-sm text-app-text outline-none transition-colors focus:border-app-accent;
}
</style>
