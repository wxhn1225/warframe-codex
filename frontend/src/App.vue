<script setup lang="ts">
import { onMounted } from 'vue'
import { RouterView } from 'vue-router'
import AppHeader from '@/components/AppHeader.vue'
import { useLangStore } from '@/stores/lang'
import { useDataStore } from '@/stores/data'

const langStore = useLangStore()
const dataStore = useDataStore()

onMounted(async () => {
  await langStore.init()
  await dataStore.loadSearchIndex()
})
</script>

<template>
  <div class="min-h-screen flex flex-col" style="background: var(--color-surface);">
    <AppHeader />
    <main class="flex-1">
      <RouterView v-slot="{ Component }">
        <Transition name="fade" mode="out-in">
          <component :is="Component" />
        </Transition>
      </RouterView>
    </main>
  </div>
</template>
