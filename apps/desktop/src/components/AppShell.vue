<template>
  <div class="min-h-screen bg-app-bg text-app-text">
    <div
      class="mx-auto flex min-h-screen max-w-7xl flex-col px-6 py-6 lg:px-10"
    >
      <header
        class="flex flex-col gap-4 rounded-3xl border border-app-line bg-app-panel px-5 py-4 md:flex-row md:items-center md:justify-between"
      >
        <div>
          <p
            class="text-xs font-semibold uppercase tracking-[0.24em] text-app-muted"
          >
            Desktop Translator
          </p>
          <h1 class="mt-1 text-xl font-semibold">ClipLingo</h1>
        </div>

        <div class="flex flex-wrap items-center gap-3">
          <nav class="flex flex-wrap gap-2">
            <RouterLink
              v-for="item in navItems"
              :key="item.to"
              :to="item.to"
              class="rounded-full border border-app-line px-3 py-2 text-sm text-app-muted transition-colors hover:border-app-accent hover:text-app-text"
              active-class="border-app-accent bg-app-accent-soft text-app-text"
            >
              {{ item.label }}
            </RouterLink>
          </nav>

          <button
            type="button"
            class="rounded-full border border-app-line px-3 py-2 text-sm text-app-muted transition-colors hover:border-app-accent hover:text-app-text"
            @click="toggleTheme"
          >
            Theme: {{ uiStore.resolvedTheme }}
          </button>
        </div>
      </header>

      <main class="flex-1 py-6">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { RouterLink, RouterView } from "vue-router";
import { useUiStore } from "@/stores/ui";

const uiStore = useUiStore();

const navItems = [
  { label: "Overview", to: "/" },
  { label: "Settings", to: "/settings" },
  { label: "History", to: "/history" },
];

function toggleTheme() {
  const nextTheme = uiStore.resolvedTheme === "dark" ? "light" : "dark";
  uiStore.applyTheme(nextTheme);
}

onMounted(() => {
  uiStore.applyTheme(uiStore.theme);
});
</script>
