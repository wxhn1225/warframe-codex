<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import type { SearchEntry } from '@/types'
import { getIconUrl, CATEGORIES } from '@/types'
import { useLangStore } from '@/stores/lang'

const props = defineProps<{ entry: SearchEntry }>()
const router = useRouter()
const langStore = useLangStore()

const displayName = computed(() => {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') return props.entry.name_zh ?? props.entry.name_en ?? ''
  return props.entry.name_en ?? props.entry.name_zh ?? ''
})

const categoryColor = computed(() => {
  return CATEGORIES.find(c => c.key === props.entry.category)?.color ?? '#64748b'
})

const categoryLabel = computed(() => {
  const cat = CATEGORIES.find(c => c.key === props.entry.category)
  return cat?.label ?? props.entry.category.replace('Export', '')
})

const iconUrl = computed(() => getIconUrl(props.entry.icon))

function navigate() {
  router.push({
    name: 'item',
    query: { path: props.entry.path, category: props.entry.category },
  })
}
</script>

<template>
  <button
    class="card w-full text-left p-4 flex items-start gap-3 cursor-pointer"
    @click="navigate"
  >
    <!-- 图标 -->
    <div
      class="shrink-0 w-12 h-12 rounded-lg flex items-center justify-center overflow-hidden"
      style="background: var(--color-surface-3);"
    >
      <img
        v-if="iconUrl"
        :src="iconUrl"
        :alt="displayName"
        class="w-10 h-10 object-contain"
        loading="lazy"
        @error="($event.target as HTMLImageElement).style.display='none'"
      />
    </div>

    <!-- 文字 -->
    <div class="flex-1 min-w-0">
      <p class="font-semibold text-sm truncate" style="color: var(--color-text);">
        {{ displayName || entry.path.split('/').pop() }}
      </p>
      <p class="text-xs mt-0.5 truncate" style="color: var(--color-text-muted);">
        {{ entry.path.split('/').slice(-3, -1).join(' / ') }}
      </p>
      <span
        class="tag mt-1.5 text-xs"
        :style="{ background: categoryColor + '20', color: categoryColor }"
      >
        {{ categoryLabel }}
      </span>
    </div>
  </button>
</template>
