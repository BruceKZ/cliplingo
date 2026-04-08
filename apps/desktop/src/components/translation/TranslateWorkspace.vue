<template>
  <v-row dense>
    <v-col cols="12">
      <v-card border rounded="lg">
        <v-card-title class="d-flex align-center justify-space-between">
          <span class="text-h5">Translate</span>
          <div class="d-flex ga-2">
            <v-btn color="primary" :loading="store.loading" @click="translateNow">
              Translate
            </v-btn>
            <v-btn
              variant="outlined"
              :disabled="!hasOutput"
              @click="copyOutput"
            >
              Copy
            </v-btn>
          </div>
        </v-card-title>
        <v-card-subtitle class="d-flex ga-2 flex-wrap">
          <v-chip size="small" label>
            {{ sourceChip }}
          </v-chip>
          <v-chip size="small" label>
            {{ outputChip }}
          </v-chip>
          <v-chip v-if="store.error" size="small" color="error" label>
            {{ store.error.code }}
          </v-chip>
        </v-card-subtitle>
      </v-card>
    </v-col>

    <v-col cols="12" md="6">
      <v-card border rounded="lg" class="fill-height">
        <v-card-title>Source</v-card-title>
        <v-card-text>
          <v-textarea
            v-model="sourceText"
            rows="18"
            auto-grow
            variant="outlined"
            density="comfortable"
            placeholder="Paste text here, or trigger with Cmd/Ctrl+C+C"
          />
        </v-card-text>
      </v-card>
    </v-col>

    <v-col cols="12" md="6">
      <v-card border rounded="lg" class="fill-height">
        <v-card-title>Translation</v-card-title>
        <v-card-text>
          <v-textarea
            :model-value="outputText"
            rows="18"
            auto-grow
            variant="outlined"
            density="comfortable"
            readonly
            placeholder=""
          />
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useTranslationStore } from "@/stores/translation";

const store = useTranslationStore();

const sourceText = computed({
  get: () => store.sourceText,
  set: (nextValue: string) => {
    store.sourceText = nextValue;
    store.sourceCharacterCount = nextValue.length;
  },
});

const outputText = computed(() => {
  if (!store.translations.length) {
    return "";
  }

  return store.translations
    .map((item) => `[${item.targetLanguage}]\n${item.text}`)
    .join("\n\n");
});

const hasOutput = computed(() => outputText.value.trim().length > 0);
const sourceChip = computed(() => `${store.sourceCharacterCount} chars`);
const outputChip = computed(
  () => `${store.translations.length} result${store.translations.length === 1 ? "" : "s"}`,
);

async function translateNow() {
  await store.retryTranslation();
}

async function copyOutput() {
  await store.copyAllTranslations();
}
</script>
