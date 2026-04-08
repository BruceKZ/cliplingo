<template>
  <v-card rounded="lg" border>
    <v-card-item>
      <template #title>{{ title }}</template>
      <template #subtitle>
        {{ eyebrow }} · {{ subtitle }}
      </template>
      <template #append>
        <div class="d-flex ga-2">
          <v-btn size="small" variant="tonal" :disabled="!text" @click="emit('copy')">
            {{ copied ? "Copied" : "Copy" }}
          </v-btn>
          <v-btn size="small" variant="text" :disabled="!text" @click="emit('toggle-expanded')">
            {{ expanded ? "Collapse" : "Expand" }}
          </v-btn>
        </div>
      </template>
    </v-card-item>

    <v-card-text>
      <div class="text-caption text-medium-emphasis mb-2">
        {{ language }} · {{ characterCount }} chars
      </div>
      <pre
        class="ma-0 rounded border pa-3 whitespace-pre-wrap break-words leading-7"
        :class="expanded ? '' : 'max-h-44 overflow-hidden'"
      >{{ renderedText }}</pre>
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
  characterCount: number;
  expanded: boolean;
  copied: boolean;
  overflowHint: boolean;
  footerText: string;
}>();

const emit = defineEmits<{
  (event: "copy"): void;
  (event: "toggle-expanded"): void;
}>();

const renderedText = computed(() => props.text || "No clipboard content yet.");
</script>
