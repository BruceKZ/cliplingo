<template>
  <v-app :theme="uiStore.resolvedTheme">
    <v-navigation-drawer
      v-model="drawer"
      :rail="rail"
      permanent
      border="end"
      :width="248"
      :rail-width="76"
    >
      <div class="px-4 pt-6 pb-2 text-subtitle-1 text-medium-emphasis">Application</div>
      <v-list nav density="comfortable" class="pt-0">
        <v-list-item
          v-for="item in navItems"
          :key="item.to"
          :to="item.to"
          :active="route.path === item.to"
          rounded="lg"
          class="mx-2 mb-2"
          color="primary"
        >
          <template #prepend>
            <v-icon :icon="item.icon" />
          </template>
          <v-list-item-title>{{ item.label }}</v-list-item-title>
        </v-list-item>
      </v-list>

      <template #append>
        <div class="pa-3 d-flex flex-column ga-2">
          <v-btn
            block
            variant="tonal"
            color="primary"
            @click="toggleTheme"
          >
            Theme: {{ uiStore.resolvedTheme }}
          </v-btn>
          <v-btn block variant="text" @click="rail = !rail">
            {{ rail ? "Expand" : "Collapse" }}
          </v-btn>
        </div>
      </template>
    </v-navigation-drawer>

    <v-app-bar color="primary" density="comfortable" flat>
      <v-btn icon="mdi-menu" variant="text" @click="drawer = !drawer" />
      <v-avatar size="34" class="mr-3" color="white">
        <span class="text-primary font-weight-bold">C</span>
      </v-avatar>
      <v-toolbar-title class="font-weight-medium">ClipLingo</v-toolbar-title>
    </v-app-bar>

    <v-main class="bg-background">
      <v-container fluid class="pa-4 pa-md-6">
        <RouterView />
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { RouterView, useRoute } from "vue-router";
import { useUiStore } from "@/stores/ui";
import { useTranslationStore } from "@/stores/translation";
import { useI18n } from "@/i18n";
import {
  TRANSLATION_TRIGGER_EVENT,
  type TranslationTriggerPayload,
} from "@/components/translation/types";
import { router } from "@/router";

const uiStore = useUiStore();
const translationStore = useTranslationStore();
const route = useRoute();
const { t } = useI18n();
const drawer = ref(true);
const rail = ref(false);
let unlistenTrigger: (() => void) | null = null;

const navItems = computed(() => [
  { label: t("nav.translate"), to: "/", icon: "mdi-translate" },
  { label: t("nav.settings"), to: "/settings", icon: "mdi-cog-outline" },
  { label: t("nav.providers"), to: "/providers", icon: "mdi-database-cog-outline" },
]);

function toggleTheme() {
  const nextTheme = uiStore.resolvedTheme === "dark" ? "light" : "dark";
  uiStore.applyTheme(nextTheme);
}

onMounted(async () => {
  unlistenTrigger = await listen<TranslationTriggerPayload>(
    TRANSLATION_TRIGGER_EVENT,
    async (event) => {
      console.info(
        "[trigger] frontend received translation event",
        event.payload.source,
        event.payload.characterCount,
      );
      if (router.currentRoute.value.path !== "/") {
        await router.push("/");
      }
      await translationStore.handleTrigger(event.payload);
    },
  );
});

onBeforeUnmount(() => {
  unlistenTrigger?.();
});
</script>
