<template>
  <v-card rounded="lg" variant="flat" border>
    <v-card-title class="text-h5">Settings</v-card-title>
    <v-card-subtitle>Trigger, provider, routing, and privacy</v-card-subtitle>
    <v-card-text>
      <SettingsWorkspace />
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import SettingsWorkspace from "@/components/settings/SettingsWorkspace.vue";
</script>
