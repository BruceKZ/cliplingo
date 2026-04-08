<template>
  <v-card rounded="lg" border>
    <v-card-item>
      <template #title>{{ title }}</template>
      <template #subtitle>{{ eyebrow }} · {{ subtitle }}</template>
      <template #append>
        <v-btn size="small" variant="tonal" :disabled="!text" @click="emit('copy')">
          {{ copied ? "Copied" : "Copy" }}
        </v-btn>
      </template>
    </v-card-item>

    <v-card-text>
      <div class="text-caption text-medium-emphasis mb-2">
        {{ language }} · {{ indexLabel }}
      </div>
      <pre class="ma-0 rounded border pa-3 whitespace-pre-wrap break-words leading-7">{{ renderedText }}</pre>
      <div class="text-caption text-medium-emphasis mt-2">{{ footerText }}</div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  eyebrow: string;
  title: string;
  subtitle: string;
  language: string;
  text: string;
  copied: boolean;
  indexLabel: string;
  footerText: string;
  tone: "neutral" | "sky" | "emerald";
}>();

const emit = defineEmits<{
  (event: "copy"): void;
}>();

const renderedText = computed(() => props.text || "No translated output yet.");
</script>
