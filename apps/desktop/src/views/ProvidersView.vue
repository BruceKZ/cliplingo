<template>
  <section>
    <h1 class="text-h5 mb-4">{{ t("nav.providers") }}</h1>

    <v-row dense>
      <v-col cols="12" lg="4">
        <v-card border rounded="lg">
          <v-card-title class="d-flex justify-space-between align-center">
            <span>{{ t("settings.providers") }}</span>
            <v-btn size="small" variant="tonal" @click="providersStore.addProvider">
              {{ t("settings.addProvider") }}
            </v-btn>
          </v-card-title>
          <v-card-text>
            <v-list v-if="providersStore.providers.length > 0" density="comfortable" nav>
              <v-list-item
                v-for="provider in providersStore.providers"
                :key="provider.id"
                :active="provider.id === providersStore.selectedProviderId"
                @click="providersStore.selectProvider(provider.id)"
              >
                <v-list-item-title>{{ provider.name || provider.id }}</v-list-item-title>
                <template #append>
                  <v-chip v-if="provider.id === providersStore.activeProviderId" size="x-small" label>
                    Active
                  </v-chip>
                </template>
              </v-list-item>
            </v-list>
            <div v-else class="text-body-2 text-medium-emphasis py-2">
              No providers yet.
            </div>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" lg="8">
        <v-card v-if="selectedProvider" border rounded="lg">
          <v-card-title class="d-flex justify-space-between align-center flex-wrap ga-2">
            <span>{{ selectedProvider.name || selectedProvider.id }}</span>
            <div class="d-flex ga-2 flex-wrap">
              <v-btn size="small" variant="outlined" @click="providersStore.duplicateProvider(selectedProvider.id)">
                Duplicate
              </v-btn>
              <v-btn
                size="small"
                color="error"
                variant="outlined"
                @click="providersStore.removeProvider(selectedProvider.id)"
              >
                {{ t("settings.remove") }}
              </v-btn>
            </div>
          </v-card-title>

          <v-card-text>
            <v-row dense>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.providerId") }}</div>
                <v-text-field v-model.trim="selectedProvider.id" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.displayName") }}</div>
                <v-text-field v-model.trim="selectedProvider.name" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.providerKind") }}</div>
                <v-select v-model="selectedProvider.kind" :items="['openai-compatible']" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.authScheme") }}</div>
                <v-select v-model="selectedProvider.authScheme" :items="['bearer', 'none']" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12">
                <div class="text-body-2 mb-1">{{ t("settings.baseUrl") }}</div>
                <v-text-field v-model.trim="selectedProvider.baseUrl" variant="outlined" density="comfortable" hide-details="auto" placeholder="https://api.example.com" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.requestPath") }}</div>
                <v-text-field v-model.trim="selectedProvider.path" variant="outlined" density="comfortable" hide-details="auto" placeholder="/chat/completions" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.apiKeyDraft") }}</div>
                <v-text-field
                  v-model.trim="selectedProvider.apiKeyDraft"
                  :type="selectedProvider.authScheme === 'none' ? 'text' : 'password'"
                  variant="outlined"
                  density="comfortable"
                  hide-details="auto"
                  :placeholder="selectedProvider.hasSecret ? t('settings.replaceApiKey') : 'sk-...'"
                />
                <div class="d-flex align-center ga-2 flex-wrap mt-2">
                  <v-chip
                    v-if="selectedProvider.hasSecret"
                    size="small"
                    color="success"
                    variant="tonal"
                  >
                    {{ t("settings.apiKeySaved") }}
                  </v-chip>
                  <v-btn
                    v-if="selectedProvider.hasSecret || selectedProvider.apiKeyDraft"
                    size="small"
                    color="error"
                    variant="text"
                    @click="providersStore.clearProviderSecret(selectedProvider.id)"
                  >
                    {{ t("settings.clearApiKey") }}
                  </v-btn>
                </div>
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.model") }}</div>
                <v-text-field v-model.trim="selectedProvider.model" variant="outlined" density="comfortable" hide-details="auto" placeholder="gpt-4o-mini" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.temperature") }}</div>
                <v-text-field v-model.number="selectedProvider.temperature" type="text" inputmode="decimal" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.topP") }}</div>
                <v-text-field v-model.number="selectedProvider.topP" type="text" inputmode="decimal" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.maxTokens") }}</div>
                <v-text-field v-model.number="selectedProvider.maxTokens" type="text" inputmode="numeric" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="6">
                <div class="text-body-2 mb-1">{{ t("settings.requestTimeout") }}</div>
                <v-text-field v-model.number="selectedProvider.timeoutSecs" type="text" inputmode="numeric" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
            </v-row>

            <v-divider class="my-4" />

            <div class="d-flex align-center justify-space-between mb-2 flex-wrap ga-2">
              <span class="text-body-2">{{ t("settings.customHeaders") }}</span>
              <v-btn size="small" variant="outlined" @click="providersStore.addProviderHeader(selectedProvider.id)">
                {{ t("settings.addHeader") }}
              </v-btn>
            </div>

            <v-row
              v-for="(header, headerIndex) in selectedProvider.customHeaders"
              :key="`${selectedProvider.id}-${headerIndex}`"
              dense
            >
              <v-col cols="12" md="5">
                <div class="text-body-2 mb-1">Header name</div>
                <v-text-field v-model.trim="header.name" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="5">
                <div class="text-body-2 mb-1">Header value</div>
                <v-text-field v-model.trim="header.value" variant="outlined" density="comfortable" hide-details="auto" />
              </v-col>
              <v-col cols="12" md="2">
                <v-btn block color="error" variant="outlined" @click="providersStore.removeProviderHeader(selectedProvider.id, headerIndex)">
                  {{ t("settings.remove") }}
                </v-btn>
              </v-col>
            </v-row>

            <v-divider class="my-4" />

            <v-row dense>
              <v-col cols="12" md="4">
                <v-btn block variant="outlined" :loading="providersStore.persistState === 'saving'" @click="providersStore.persistCurrent()">
                  {{ providersStore.persistState === "saving" ? t("settings.saving") : t("settings.save") }}
                </v-btn>
              </v-col>
              <v-col cols="12" md="4">
                <v-btn block color="primary" :loading="providersStore.testState === 'running'" @click="providersStore.testConnection">
                  {{ providersStore.testState === "running" ? t("settings.testing") : t("settings.testTranslation") }}
                </v-btn>
              </v-col>
              <v-col cols="12" md="4">
                <v-btn block color="primary" variant="tonal" @click="providersStore.makeProviderActive(selectedProvider.id)">
                  {{ t("settings.makeActive") }}
                </v-btn>
              </v-col>
            </v-row>

            <div v-if="statusText" class="text-caption text-medium-emphasis mt-4">
              {{ statusText }}
            </div>
          </v-card-text>
        </v-card>
        <v-card v-else border rounded="lg">
          <v-card-text class="d-flex justify-center py-8">
            <v-btn color="primary" @click="providersStore.addProvider">
              {{ t("settings.addProvider") }}
            </v-btn>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </section>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useI18n } from "@/i18n";
import { useProvidersStore } from "@/stores/providers";

const providersStore = useProvidersStore();
const { t } = useI18n();

onMounted(() => {
  providersStore.refresh(true).catch(() => undefined);
});

const selectedProvider = computed(() => providersStore.selectedProvider);

const statusText = computed(() => {
  if (!providersStore.statusLine) {
    return "";
  }
  return `${t("settings.statusAt")}: ${new Date(providersStore.statusLine.at).toLocaleTimeString()} - ${providersStore.statusLine.message}`;
});
</script>
