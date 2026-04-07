<template>
  <main class="min-h-screen bg-app-bg px-4 py-4 text-app-text">
    <section
      class="mx-auto flex min-h-[24rem] max-w-6xl flex-col rounded-3xl border border-app-line bg-app-panel p-6 shadow-sm"
    >
      <div class="space-y-2">
        <p
          class="text-xs font-semibold uppercase tracking-[0.24em] text-app-muted"
        >
          Settings Window
        </p>
        <h1 class="text-2xl font-semibold tracking-tight">
          ClipLingo settings
        </h1>
        <p class="text-sm leading-7 text-app-muted">
          Trigger, provider, language, history, and privacy controls stay in a
          single workspace so the route-based page and the dedicated window stay
          aligned.
        </p>
      </div>

      <div class="mt-6">
        <SettingsWorkspace compact />
      </div>
    </section>
  </main>
</template>

<script setup lang="ts">
import SettingsWorkspace from "@/components/settings/SettingsWorkspace.vue";
</script>
