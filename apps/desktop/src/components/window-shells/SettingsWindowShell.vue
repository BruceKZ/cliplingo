<template>
  <v-app :theme="uiStore.resolvedTheme">
    <v-main>
      <v-container fluid class="pa-4 pa-md-6">
        <v-card rounded="lg" variant="flat" border>
          <v-card-title class="text-h5">ClipLingo Settings</v-card-title>
          <v-card-subtitle>Unified settings workspace</v-card-subtitle>
          <v-card-text>
            <SettingsWorkspace compact />
          </v-card-text>
        </v-card>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import SettingsWorkspace from "@/components/settings/SettingsWorkspace.vue";
import { useUiStore } from "@/stores/ui";

const uiStore = useUiStore();
</script>
