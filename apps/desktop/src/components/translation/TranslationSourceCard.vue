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
      <pre class="translation-pre" :class="expanded ? 'translation-pre--expanded' : 'translation-pre--collapsed'">{{ renderedText }}</pre>
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

<style scoped>
.translation-pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  border: 1px solid rgba(128, 128, 128, 0.25);
  border-radius: 10px;
  padding: 12px;
  line-height: 1.55;
}

.translation-pre--collapsed {
  max-height: 180px;
  overflow: hidden;
}

.translation-pre--expanded {
  max-height: none;
}
</style>
