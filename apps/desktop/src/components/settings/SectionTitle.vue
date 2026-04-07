<template>
  <div class="space-y-2">
    <p class="text-xs font-semibold uppercase tracking-[0.24em] text-app-muted">
      {{ kicker }}
    </p>
    <h3 class="text-2xl font-semibold tracking-tight text-app-text">
      {{ title }}
    </h3>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  kicker: string;
  title: string;
}>();
</script>
