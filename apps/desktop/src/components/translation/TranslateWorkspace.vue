<template>
  <section class="flex h-100 min-h-0 flex-col">
    <div class="shrink-0">
      <div class="grid grid-cols-1 gap-2 md:grid-cols-2">
        <div class="min-w-0">
          <v-select
            v-model="selectedSourceLanguage"
            :items="languageOptions"
            item-title="title"
            item-value="value"
            label="输入语言"
            variant="outlined"
            density="comfortable"
            hide-details
          />
        </div>

        <div class="min-w-0">
          <div class="flex items-center gap-2">
            <v-select
              v-model="selectedTargetLanguage"
              :items="targetLanguageOptions"
              item-title="title"
              item-value="value"
              label="目标语言"
              variant="outlined"
              density="comfortable"
              hide-details
              class="flex-1"
            />
            <v-btn
              color="primary"
              :loading="store.loading"
              icon="mdi-translate"
              variant="tonal"
              class="shrink-0"
              @click="translateNow"
            />
          </div>
        </div>
      </div>

      <div v-if="store.error" class="mt-3 text-caption text-error">
        {{ store.error.message }}
      </div>
    </div>

    <div class="mt-4 grid min-h-0 flex-1 grid-cols-1 gap-2 md:grid-cols-2">
      <div class="flex min-h-0">
        <v-card
          rounded="md"
          elevation="0"
          class="translation-panel relative flex min-h-0 flex-1 flex-col"
        >
        <v-btn
          icon="mdi-content-copy"
          variant="text"
          density="comfortable"
          size="x-small"
          :disabled="!sourceText.trim()"
          class="absolute right-4 top-5 z-10"
          @click="copySource"
        />
        <v-card-text class="flex min-h-0 flex-1 pa-4">
          <textarea
            v-model="sourceText"
            class="translation-textarea h-full min-h-0 w-full flex-1 resize-none overflow-y-auto rounded-md border-0 bg-transparent px-4 py-3 pr-12 text-base leading-7 outline-none"
            placeholder="Paste text here, or trigger with Cmd/Ctrl+C+C"
          />
        </v-card-text>
        </v-card>
      </div>

      <div class="flex min-h-0">
        <v-card
          rounded="md"
          elevation="0"
          class="translation-panel relative flex min-h-0 flex-1 flex-col"
        >
        <v-btn
          icon="mdi-content-copy"
          variant="text"
          density="comfortable"
          size="x-small"
          :disabled="!hasOutput"
          class="absolute right-4 top-5 z-10"
          @click="copyOutput"
        />
        <v-card-text class="flex min-h-0 flex-1 pa-4">
          <textarea
            :value="outputText"
            readonly
            class="translation-textarea h-full min-h-0 w-full flex-1 resize-none overflow-y-auto rounded-md border-0 bg-transparent px-4 py-3 pr-12 text-base leading-7 outline-none"
          />
        </v-card-text>
        </v-card>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useTranslationStore } from "@/stores/translation";

const store = useTranslationStore();

interface LanguageOption {
  title: string;
  value: string;
}

const languageOptions: LanguageOption[] = [
  { title: "自动检测", value: "auto" },
  { title: "English", value: "en" },
  { title: "简体中文", value: "zh-CN" },
  { title: "繁體中文", value: "zh-TW" },
  { title: "日本語", value: "ja" },
  { title: "한국어", value: "ko" },
  { title: "Français", value: "fr" },
  { title: "Deutsch", value: "de" },
  { title: "Español", value: "es" },
  { title: "Русский", value: "ru" },
];

const targetLanguageOptions = computed(() =>
  languageOptions.filter((item) => item.value !== "auto"),
);

const selectedSourceLanguage = ref("auto");
const selectedTargetLanguage = ref("zh-CN");

const sourceText = computed({
  get: () => store.sourceText,
  set: (nextValue: string) => {
    store.sourceText = nextValue;
    store.sourceCharacterCount = nextValue.length;
  },
});

const outputText = computed(() => store.translations[0]?.text ?? "");
const hasOutput = computed(() => outputText.value.trim().length > 0);
async function translateNow() {
  await store.retryTranslation({
    sourceLanguage:
      selectedSourceLanguage.value === "auto"
        ? undefined
        : selectedSourceLanguage.value,
    targetLanguages: [selectedTargetLanguage.value],
  });
}

async function copyOutput() {
  const targetLanguage =
    store.translations[0]?.targetLanguage ?? selectedTargetLanguage.value;
  await store.copyTarget(targetLanguage);
}

async function copySource() {
  await store.copySource();
}
</script>

<style scoped>
.translation-panel {
  border: 1px solid rgba(var(--v-theme-on-surface), 0.12);
  background: rgb(var(--v-theme-surface));
}

.translation-textarea {
  color: rgb(var(--v-theme-on-surface));
  caret-color: rgb(var(--v-theme-primary));
}

.translation-textarea::placeholder {
  color: rgba(var(--v-theme-on-surface), 0.5);
}
</style>
