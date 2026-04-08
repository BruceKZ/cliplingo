<template>
  <v-app :theme="uiStore.resolvedTheme">
    <v-main>
      <v-container fluid class="pa-3 fill-height">
        <TranslationPopup />
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import TranslationPopup from "@/components/translation/TranslationPopup.vue";
import { useTranslationPopup } from "@/components/translation/useTranslationPopup";
import { useUiStore } from "@/stores/ui";

useTranslationPopup();
const uiStore = useUiStore();
</script>
