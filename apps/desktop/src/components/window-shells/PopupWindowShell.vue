<template>
  <TranslationPopup />
</template>

<script setup lang="ts">
import TranslationPopup from "@/components/translation/TranslationPopup.vue";
import { useTranslationPopup } from "@/components/translation/useTranslationPopup";

useTranslationPopup();
</script>
