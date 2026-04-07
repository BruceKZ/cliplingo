<template>
  <section
    class="rounded-[1.5rem] border p-4 shadow-[0_24px_60px_rgba(15,23,42,0.12)] ring-1 backdrop-blur-sm"
    :class="accentClasses"
  >
    <div class="flex items-center justify-between gap-3">
      <div class="space-y-1">
        <p
          class="text-[0.72rem] font-semibold uppercase tracking-[0.26em]"
          :class="titleClasses"
        >
          {{ eyebrow }}
        </p>
        <h2 class="text-base font-semibold" :class="titleClasses">
          {{ title }}
        </h2>
        <p class="text-xs" :class="subtitleClasses">{{ subtitle }}</p>
      </div>

      <button
        type="button"
        class="rounded-full border px-3 py-1.5 text-xs font-medium transition"
        :class="buttonClasses"
        :disabled="!text"
        @click="emit('copy')"
      >
        {{ copied ? "Copied" : "Copy" }}
      </button>
    </div>

    <div class="mt-4 rounded-[1.2rem] border p-4" :class="panelClasses">
      <div
        class="flex items-center justify-between gap-2 text-[0.72rem] uppercase tracking-[0.24em]"
        :class="subtitleClasses"
      >
        <span>{{ language }}</span>
        <span>{{ indexLabel }}</span>
      </div>

      <pre
        class="mt-3 whitespace-pre-wrap break-words text-sm leading-7"
        :class="textClasses"
        >{{ renderedText }}</pre
      >

      <p class="mt-3 text-xs leading-6" :class="subtitleClasses">
        {{ footerText }}
      </p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  eyebrow: string;
  title: string;
  subtitle: string;
  language: string;
  text: string;
  copied: boolean;
  indexLabel: string;
  footerText: string;
  tone: "neutral" | "sky" | "emerald";
}>();

const emit = defineEmits<{
  (event: "copy"): void;
}>();

const renderedText = computed(
  () => props.text || "Waiting for translation output.",
);

const toneClasses = {
  neutral: {
    accentClasses: "border-white/8 bg-white/5 ring-white/5",
    titleClasses: "text-white",
    subtitleClasses: "text-slate-300",
    buttonClasses:
      "border-white/10 bg-white/5 text-slate-100 hover:border-sky-300/40 hover:bg-sky-400/10 disabled:text-slate-500",
    panelClasses: "border-white/8 bg-slate-950/55",
    textClasses: "text-slate-100",
  },
  sky: {
    accentClasses: "border-sky-300/20 bg-sky-400/8 ring-sky-300/15",
    titleClasses: "text-sky-50",
    subtitleClasses: "text-sky-100/70",
    buttonClasses:
      "border-sky-200/30 bg-sky-400/10 text-sky-50 hover:border-sky-200/60 hover:bg-sky-300/20 disabled:text-sky-100/35",
    panelClasses: "border-sky-200/15 bg-sky-950/35",
    textClasses: "text-sky-50",
  },
  emerald: {
    accentClasses: "border-emerald-300/20 bg-emerald-400/8 ring-emerald-300/15",
    titleClasses: "text-emerald-50",
    subtitleClasses: "text-emerald-100/70",
    buttonClasses:
      "border-emerald-200/30 bg-emerald-400/10 text-emerald-50 hover:border-emerald-200/60 hover:bg-emerald-300/20 disabled:text-emerald-100/35",
    panelClasses: "border-emerald-200/15 bg-emerald-950/35",
    textClasses: "text-emerald-50",
  },
} as const;

const theme = computed(() => toneClasses[props.tone]);
const accentClasses = computed(() => theme.value.accentClasses);
const titleClasses = computed(() => theme.value.titleClasses);
const subtitleClasses = computed(() => theme.value.subtitleClasses);
const buttonClasses = computed(() => theme.value.buttonClasses);
const panelClasses = computed(() => theme.value.panelClasses);
const textClasses = computed(() => theme.value.textClasses);
</script>
