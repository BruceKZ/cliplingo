<template>
  <section class="space-y-6">
    <header class="space-y-2">
      <p class="text-sm font-medium uppercase tracking-[0.24em] text-app-muted">
        Settings
      </p>
      <h1 class="text-3xl font-semibold tracking-tight text-app-text">
        Configure trigger behavior, providers, routing, and privacy.
      </h1>
      <p class="max-w-3xl text-sm leading-7 text-app-muted">
        This draft surface keeps editing local and immediate. The same workspace
        is reused by the dedicated settings window so behavior stays consistent.
      </p>
    </header>

    <SettingsWorkspace />
  </section>
</template>

<script setup lang="ts">
import SettingsWorkspace from "@/components/settings/SettingsWorkspace.vue";
</script>
