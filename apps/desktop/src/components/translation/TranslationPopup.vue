<template>
  <v-card rounded="lg" border class="h-100 d-flex flex-column">
    <v-card-item>
      <template #title>{{ popup.headerTitle }}</template>
      <template #subtitle>{{ popup.headerCopy }}</template>
      <template #append>
        <div class="d-flex ga-2 flex-wrap justify-end">
          <v-btn size="small" variant="tonal" :disabled="!popup.store.hasResult" @click="popup.store.copyAllTranslations">
            Copy all
          </v-btn>
          <v-btn size="small" variant="tonal" :disabled="!popup.store.sourceText" @click="popup.store.retryTranslation">
            Retry
          </v-btn>
          <v-btn
            v-if="windowControls"
            size="small"
            :variant="popup.store.pinned ? 'flat' : 'outlined'"
            color="primary"
            @click="popup.store.togglePinned"
          >
            {{ popup.store.pinned ? "Pinned" : "Pin" }}
          </v-btn>
          <v-btn
            v-if="windowControls"
            size="small"
            variant="outlined"
            @click="popup.store.closePopup"
          >
            Close
          </v-btn>
        </div>
      </template>
    </v-card-item>

    <v-card-text class="pt-0">
      <div class="d-flex ga-2 mb-4 flex-wrap">
        <v-chip size="small" label>{{ popup.footerStatus }}</v-chip>
        <v-chip size="small" label>{{ popup.footerTiming }}</v-chip>
        <v-chip size="small" label>{{ popup.footerTargetCount }}</v-chip>
        <v-chip size="small" label>{{ popup.providerLabel }}</v-chip>
        <v-chip size="small" label>{{ popup.modelLabel }}</v-chip>
      </div>

      <v-row>
        <v-col cols="12" md="5">
          <TranslationSourceCard
            :eyebrow="popup.sourceEyebrow"
            :title="popup.sourceTitle"
            :subtitle="popup.sourceSubtitle"
            :language="popup.sourceLanguageLabel"
            :text="popup.store.sourceText"
            :character-count="popup.store.sourceCharacterCount"
            :expanded="popup.store.sourceExpanded"
            :copied="popup.store.copiedSource"
            :overflow-hint="popup.sourceOverflowHint"
            :footer-text="popup.sourceFooterText"
            @copy="popup.store.copySource"
            @toggle-expanded="popup.store.toggleSourceExpanded"
          />
        </v-col>

        <v-col cols="12" md="7">
          <v-alert
            v-if="popup.store.error"
            type="error"
            variant="tonal"
            class="mb-4"
          >
            {{ popup.store.error.code }}: {{ popup.store.error.message }}
          </v-alert>

          <v-row>
            <v-col
              v-for="(translation, index) in popup.store.translations"
              :key="translation.targetLanguage"
              cols="12"
            >
              <TranslationTargetCard
                :eyebrow="popup.targetEyebrow(index)"
                :title="popup.targetTitle(translation.targetLanguage)"
                :subtitle="popup.targetSubtitle(index)"
                :language="translation.targetLanguage"
                :text="translation.text"
                :copied="popup.store.copiedTargetLanguage === translation.targetLanguage"
                :index-label="popup.targetIndexLabel(index)"
                :footer-text="popup.targetFooterText(index)"
                :tone="popup.targetTone(index)"
                @copy="popup.store.copyTarget(translation.targetLanguage)"
              />
            </v-col>
          </v-row>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import TranslationSourceCard from "@/components/translation/TranslationSourceCard.vue";
import TranslationTargetCard from "@/components/translation/TranslationTargetCard.vue";
import { useTranslationPopupViewModel } from "@/components/translation/useTranslationPopupViewModel";

withDefaults(
  defineProps<{
    windowControls?: boolean;
  }>(),
  {
    windowControls: true,
  },
);

const popup = useTranslationPopupViewModel();
</script>
