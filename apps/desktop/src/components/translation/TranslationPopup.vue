<template>
  <main
    class="relative flex min-h-screen items-stretch overflow-hidden bg-[radial-gradient(circle_at_top_left,rgba(96,165,250,0.18),transparent_34%),radial-gradient(circle_at_top_right,rgba(16,185,129,0.16),transparent_28%),linear-gradient(180deg,rgba(2,6,23,0.96),rgba(15,23,42,0.96))] px-4 py-4 text-white"
  >
    <div class="pointer-events-none absolute inset-0 overflow-hidden">
      <div
        class="absolute -left-14 top-10 h-48 w-48 rounded-full bg-sky-400/10 blur-3xl"
      />
      <div
        class="absolute bottom-0 right-0 h-56 w-56 translate-x-1/3 rounded-full bg-emerald-400/10 blur-3xl"
      />
    </div>

    <section
      class="bg-slate-950/78 relative mx-auto flex max-h-[calc(100vh-2rem)] w-full max-w-6xl flex-col overflow-hidden rounded-[2rem] border border-white/10 shadow-[0_32px_90px_rgba(2,6,23,0.55)] ring-1 ring-white/5 backdrop-blur-xl"
    >
      <header
        class="flex flex-col gap-4 border-b border-white/10 px-5 py-4 md:flex-row md:items-start md:justify-between"
      >
        <div class="space-y-2">
          <div class="flex flex-wrap items-center gap-2">
            <span
              class="rounded-full border border-sky-300/20 bg-sky-400/10 px-3 py-1 text-[0.72rem] font-semibold uppercase tracking-[0.24em] text-sky-100"
            >
              Translation Popup
            </span>
            <span
              class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-[0.72rem] font-medium uppercase tracking-[0.22em] text-slate-300"
            >
              {{ popup.footerStatus }}
            </span>
          </div>

          <h1 class="text-2xl font-semibold tracking-tight md:text-[2rem]">
            {{ popup.headerTitle }}
          </h1>
          <p class="max-w-3xl text-sm leading-7 text-slate-300">
            {{ popup.headerCopy }}
          </p>
        </div>

        <div class="flex flex-wrap items-center gap-2 md:justify-end">
          <button
            type="button"
            class="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-slate-100 transition hover:border-sky-300/40 hover:bg-sky-400/10 disabled:cursor-not-allowed disabled:opacity-45"
            :disabled="!popup.store.hasResult"
            @click="popup.store.copyAllTranslations"
          >
            Copy all
          </button>
          <button
            type="button"
            class="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-slate-100 transition hover:border-amber-300/40 hover:bg-amber-400/10 disabled:cursor-not-allowed disabled:opacity-45"
            :disabled="!popup.store.sourceText"
            @click="popup.store.retryTranslation"
          >
            Retry
          </button>
          <button
            type="button"
            class="rounded-full border px-4 py-2 text-sm font-medium transition"
            :class="
              popup.store.pinned ? popup.pinnedClasses : popup.unpinnedClasses
            "
            @click="popup.store.togglePinned"
          >
            {{ popup.store.pinned ? "Pinned" : "Pin" }}
          </button>
          <button
            type="button"
            class="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm font-medium text-slate-100 transition hover:border-white/30 hover:bg-white/10"
            @click="popup.store.closePopup"
          >
            Close
          </button>
        </div>
      </header>

      <div
        class="grid flex-1 gap-4 overflow-y-auto px-5 py-5 lg:grid-cols-[1fr_1.25fr]"
      >
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

        <section class="space-y-4">
          <div
            class="border-white/8 rounded-[1.5rem] border bg-white/5 p-4 shadow-[0_24px_60px_rgba(15,23,42,0.12)] ring-1 ring-white/5 backdrop-blur-sm"
          >
            <div class="flex flex-wrap items-center justify-between gap-3">
              <div class="space-y-1">
                <p
                  class="text-[0.72rem] font-semibold uppercase tracking-[0.26em] text-emerald-200/80"
                >
                  Result overview
                </p>
                <h2 class="text-base font-semibold text-white">
                  {{ popup.resultHeading }}
                </h2>
                <p class="text-xs text-slate-300">
                  {{ popup.resultSubtitle }}
                </p>
              </div>

              <div class="flex flex-wrap items-center gap-2">
                <span
                  class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-[0.72rem] font-medium uppercase tracking-[0.22em] text-slate-300"
                >
                  {{ popup.providerLabel }}
                </span>
                <span
                  class="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-[0.72rem] font-medium uppercase tracking-[0.22em] text-slate-300"
                >
                  {{ popup.modelLabel }}
                </span>
              </div>
            </div>

            <div
              v-if="popup.store.loading"
              class="mt-4 rounded-[1.2rem] border border-dashed border-slate-500/40 bg-slate-950/55 p-5 text-sm leading-7 text-slate-300"
            >
              Translating source text. This panel keeps the window responsive
              while the provider completes the request.
            </div>

            <div
              v-else-if="popup.store.error"
              class="mt-4 rounded-[1.2rem] border border-rose-400/20 bg-rose-950/35 p-5 text-sm leading-7 text-rose-100"
            >
              <p class="font-medium">{{ popup.store.error.code }}</p>
              <p class="mt-2 text-rose-100/80">
                {{ popup.store.error.message }}
              </p>
              <button
                type="button"
                class="mt-4 rounded-full border border-rose-300/30 bg-rose-400/10 px-4 py-2 text-xs font-medium uppercase tracking-[0.22em] text-rose-50 transition hover:bg-rose-300/20"
                :disabled="!popup.store.error.retryable"
                @click="popup.store.retryTranslation"
              >
                Retry translation
              </button>
            </div>

            <div
              v-else-if="!popup.store.hasResult"
              class="mt-4 rounded-[1.2rem] border border-dashed border-white/10 bg-slate-950/55 p-5 text-sm leading-7 text-slate-300"
            >
              Waiting for a clipboard trigger. The popup will fill in the
              translated sections once a request arrives.
            </div>
          </div>

          <div class="space-y-4">
            <TranslationTargetCard
              v-for="(translation, index) in popup.store.translations"
              :key="translation.targetLanguage"
              :eyebrow="popup.targetEyebrow(index)"
              :title="popup.targetTitle(translation.targetLanguage)"
              :subtitle="popup.targetSubtitle(index)"
              :language="translation.targetLanguage"
              :text="translation.text"
              :copied="
                popup.store.copiedTargetLanguage === translation.targetLanguage
              "
              :index-label="popup.targetIndexLabel(index)"
              :footer-text="popup.targetFooterText(index)"
              :tone="popup.targetTone(index)"
              @copy="popup.store.copyTarget(translation.targetLanguage)"
            />
          </div>
        </section>
      </div>

      <footer
        class="flex flex-col gap-3 border-t border-white/10 px-5 py-4 text-xs text-slate-300 md:flex-row md:items-center md:justify-between"
      >
        <div class="flex flex-wrap items-center gap-2">
          <span
            class="rounded-full border border-white/10 bg-white/5 px-3 py-1"
          >
            {{ popup.footerStatus }}
          </span>
          <span
            class="rounded-full border border-white/10 bg-white/5 px-3 py-1"
          >
            {{ popup.footerTiming }}
          </span>
          <span
            class="rounded-full border border-white/10 bg-white/5 px-3 py-1"
          >
            {{ popup.footerTargetCount }}
          </span>
        </div>

        <p class="text-right leading-6 text-slate-400">
          Press `Esc` to hide the popup.
        </p>
      </footer>
    </section>
  </main>
</template>

<script setup lang="ts">
import TranslationSourceCard from "@/components/translation/TranslationSourceCard.vue";
import TranslationTargetCard from "@/components/translation/TranslationTargetCard.vue";
import { useTranslationPopupViewModel } from "@/components/translation/useTranslationPopupViewModel";

const popup = useTranslationPopupViewModel();
</script>
