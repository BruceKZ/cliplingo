<template>
  <section
    class="border-white/8 rounded-[1.5rem] border bg-slate-950/45 p-4 shadow-[0_24px_60px_rgba(15,23,42,0.16)] ring-1 ring-white/5 backdrop-blur-sm"
  >
    <div class="flex items-center justify-between gap-3">
      <div class="space-y-1">
        <p
          class="text-[0.72rem] font-semibold uppercase tracking-[0.26em] text-sky-200/80"
        >
          {{ eyebrow }}
        </p>
        <h2 class="text-base font-semibold text-white">{{ title }}</h2>
        <p class="text-xs text-slate-300">{{ subtitle }}</p>
      </div>

      <div class="flex items-center gap-2">
        <button
          type="button"
          class="rounded-full border border-white/10 bg-white/5 px-3 py-1.5 text-xs font-medium text-slate-100 transition hover:border-sky-300/40 hover:bg-sky-400/10"
          :disabled="!text"
          @click="emit('copy')"
        >
          {{ copied ? "Copied" : "Copy" }}
        </button>
        <button
          type="button"
          class="rounded-full border border-white/10 bg-white/5 px-3 py-1.5 text-xs font-medium text-slate-100 transition hover:border-sky-300/40 hover:bg-sky-400/10"
          :disabled="!text"
          @click="emit('toggle-expanded')"
        >
          {{ expanded ? "Collapse" : "Expand" }}
        </button>
      </div>
    </div>

    <div
      class="border-white/8 mt-4 rounded-[1.2rem] border bg-slate-900/55 p-4"
    >
      <div
        class="flex items-center justify-between gap-2 text-[0.72rem] uppercase tracking-[0.24em] text-slate-400"
      >
        <span>{{ language }}</span>
        <span>{{ characterCount }} chars</span>
      </div>

      <div
        class="relative mt-3 overflow-hidden rounded-[1rem] bg-slate-950/55"
        :class="expanded ? 'max-h-none' : 'max-h-32'"
      >
        <pre
          class="whitespace-pre-wrap break-words px-4 py-3 text-sm leading-7 text-slate-100"
          >{{ renderedText }}</pre
        >

        <div
          v-if="!expanded && overflowHint"
          class="pointer-events-none absolute inset-x-0 bottom-0 h-10 bg-gradient-to-t from-slate-950 to-transparent"
        />
      </div>

      <p class="mt-3 text-xs leading-6 text-slate-400">
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
  characterCount: number;
  expanded: boolean;
  copied: boolean;
  overflowHint: boolean;
  footerText: string;
}>();

const emit = defineEmits<{
  (event: "copy"): void;
  (event: "toggle-expanded"): void;
}>();

const renderedText = computed(() => props.text || "Waiting for content.");
</script>
