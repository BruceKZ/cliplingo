<template>
  <section class="space-y-6">
    <header class="space-y-2">
      <p class="text-sm font-medium uppercase tracking-[0.24em] text-app-muted">
        Settings Shell
      </p>
      <h1 class="text-3xl font-semibold tracking-tight text-app-text">
        Settings groups are scaffolded for later issues.
      </h1>
      <p class="max-w-3xl text-sm leading-7 text-app-muted">
        This shell reserves the final information architecture without
        implementing provider forms or persistence ahead of their dedicated
        issues.
      </p>
    </header>

    <div class="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
      <article
        v-for="group in groups"
        :key="group"
        class="rounded-2xl border border-app-line bg-app-panel p-5 text-sm text-app-muted"
      >
        {{ group }}
      </article>
    </div>
  </section>
</template>

<script setup lang="ts">
const groups = [
  "General settings",
  "Trigger settings",
  "Language routing",
  "Provider configuration",
  "Model parameters",
  "History and privacy",
  "Debug options",
];
</script>
