<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import type { SearchEntry } from '@/types'
import { getIconUrl, getSubTypeLabel, CATEGORIES } from '@/types'
import { useLangStore } from '@/stores/lang'

const props = defineProps<{ entry: SearchEntry }>()
const router = useRouter()
const langStore = useLangStore()

const displayName = computed(() => {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') return props.entry.name_zh ?? props.entry.name_en ?? ''
  return props.entry.name_en ?? props.entry.name_zh ?? ''
})

const cat = computed(() => CATEGORIES.find(c => c.key === props.entry.category))
const categoryColor = computed(() => cat.value?.color ?? '#64748b')
const categoryLabel = computed(() => cat.value?.label ?? props.entry.category.replace('Export', ''))

const subTypeLabel = computed(() => {
  if (!props.entry.sub_type || !cat.value) return ''
  return getSubTypeLabel(cat.value, props.entry.sub_type, langStore.t)
})

const iconUrl = computed(() => getIconUrl(props.entry.icon))

function navigate() {
  router.push({ name: 'item', query: { path: props.entry.path, category: props.entry.category } })
}
</script>

<template>
  <button
    class="card w-full text-left p-3.5 flex items-center gap-3 cursor-pointer group"
    @click="navigate"
  >
    <!-- 图标区 -->
    <div
      class="shrink-0 w-11 h-11 rounded-lg flex items-center justify-center overflow-hidden"
      :style="{ background: categoryColor + '12' }"
    >
      <img
        v-if="iconUrl"
        :src="iconUrl"
        :alt="displayName"
        class="w-9 h-9 object-contain"
        loading="lazy"
        @error="($event.target as HTMLImageElement).style.display='none'"
      />
      <!-- 无图标时显示首字 -->
      <span
        v-else
        class="text-base font-bold"
        :style="{ color: categoryColor }"
      >
        {{ (displayName || 'W').charAt(0).toUpperCase() }}
      </span>
    </div>

    <!-- 文字 -->
    <div class="flex-1 min-w-0">
      <p class="font-semibold text-sm truncate" style="color: var(--color-text);">
        {{ displayName || entry.path.split('/').pop() }}
      </p>
      <div class="flex items-center gap-1.5 mt-0.5 flex-wrap">
        <span
          class="text-xs font-medium"
          :style="{ color: categoryColor }"
        >
          {{ categoryLabel }}
        </span>
        <span v-if="subTypeLabel" class="text-xs" style="color: var(--color-text-subtle);">
          · {{ subTypeLabel }}
        </span>
      </div>
    </div>

    <!-- 箭头 -->
    <svg
      class="w-4 h-4 shrink-0 opacity-0 group-hover:opacity-100 transition-opacity"
      style="color: var(--color-text-subtle);"
      fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
    >
      <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
    </svg>
  </button>
</template>
