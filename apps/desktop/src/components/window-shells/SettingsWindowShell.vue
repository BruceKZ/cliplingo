<template>
  <main class="min-h-screen bg-app-bg px-4 py-4 text-app-text">
    <section
      class="mx-auto flex min-h-[24rem] max-w-5xl flex-col rounded-3xl border border-app-line bg-app-panel p-6 shadow-sm"
    >
      <div class="space-y-2">
        <p
          class="text-xs font-semibold uppercase tracking-[0.24em] text-app-muted"
        >
          Settings Window
        </p>
        <h1 class="text-2xl font-semibold tracking-tight">
          Settings foundation
        </h1>
        <p class="text-sm leading-7 text-app-muted">
          This window is reserved for trigger, provider, language, history, and
          privacy configuration in later issues.
        </p>
      </div>

      <div class="mt-6 grid gap-3 md:grid-cols-2 xl:grid-cols-3">
        <div
          v-for="section in sections"
          :key="section"
          class="rounded-2xl border border-app-line px-4 py-3 text-sm text-app-muted"
        >
          {{ section }}
        </div>
      </div>
    </section>
  </main>
</template>

<script setup lang="ts">
const sections = [
  "General",
  "Trigger",
  "Language routing",
  "Provider",
  "Model parameters",
  "History and privacy",
];
</script>
