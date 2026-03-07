<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useDataStore } from '@/stores/data'
import { CATEGORIES } from '@/types'
import ItemCard from '@/components/ItemCard.vue'

const props = defineProps<{ key: string }>()
const router = useRouter()
const dataStore = useDataStore()

const category = computed(() => CATEGORIES.find(c => c.key === props.key))
const entries = computed(() => {
  const defaultSub = category.value?.defaultSubCategory
  return dataStore.getByCategory(props.key, defaultSub ?? undefined)
})
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-8">
    <button
      class="flex items-center gap-1.5 text-sm mb-6"
      style="color: var(--color-text-muted);"
      @click="router.back()"
    >
      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
      </svg>
      返回
    </button>

    <h1 class="text-2xl font-bold mb-6" :style="{ color: category?.color ?? 'var(--color-text)' }">
      {{ category?.label ?? key }}
    </h1>

    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
      <ItemCard v-for="entry in entries" :key="entry.path" :entry="entry" />
    </div>
  </div>
</template>
