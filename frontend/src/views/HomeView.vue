<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDataStore } from '@/stores/data'
import { useLangStore } from '@/stores/lang'
import { CATEGORIES, getSubTypeLabel } from '@/types'
import ItemCard from '@/components/ItemCard.vue'

const route = useRoute()
const router = useRouter()
const dataStore = useDataStore()
const langStore = useLangStore()

/** 实际用于搜索的关键词（按 Enter / 点按钮后才更新） */
const query = ref((route.query.q as string) ?? '')
/** 输入框的草稿值，不直接触发搜索 */
const inputDraft = ref((route.query.q as string) ?? '')
const activeCategory = ref<string | null>(null)
const activeSubType = ref<string | null>(null)

const currentCategoryInfo = computed(() =>
  CATEGORIES.find(c => c.key === activeCategory.value),
)

const defaultSubCategory = computed(() =>
  currentCategoryInfo.value?.defaultSubCategory,
)

const availableSubTypes = computed(() => {
  if (!activeCategory.value) return []
  const subTypes = dataStore.getSubTypes(activeCategory.value, defaultSubCategory.value ?? undefined)
  const cat = currentCategoryInfo.value
  return subTypes
    .filter(t => {
      if (!cat) return true
      const label = getSubTypeLabel(cat, t, langStore.t)
      return label && label !== ''
    })
    .map(t => ({
      value: t,
      label: cat ? getSubTypeLabel(cat, t, langStore.t) : t,
    }))
})

const results = computed(() => {
  if (query.value.trim()) {
    return dataStore.search(query.value, activeCategory.value ?? undefined, activeSubType.value ?? undefined)
  }
  if (activeCategory.value) {
    return dataStore.getByCategory(
      activeCategory.value,
      defaultSubCategory.value ?? undefined,
      activeSubType.value ?? undefined,
    )
  }
  return []
})

const isSearching = computed(() => query.value.trim().length > 0 || activeCategory.value !== null)

function submitSearch() {
  const q = inputDraft.value.trim()
  if (!q) return
  query.value = q
  activeCategory.value = null
  activeSubType.value = null
}

function selectCategory(key: string) {
  activeCategory.value = activeCategory.value === key ? null : key
  activeSubType.value = null
  query.value = ''
  inputDraft.value = ''
  router.replace({ query: {} })
}

function selectSubType(val: string) {
  activeSubType.value = activeSubType.value === val ? null : val
}

watch(() => route.query.q, (q) => {
  if (q) {
    query.value = q as string
    inputDraft.value = q as string
    activeCategory.value = null
    activeSubType.value = null
  }
})

watch(() => dataStore.homeResetCount, () => {
  query.value = ''
  inputDraft.value = ''
  activeCategory.value = null
  activeSubType.value = null
  router.replace({ query: {} })
})
onMounted(() => {
  if (route.query.q) {
    query.value = route.query.q as string
    inputDraft.value = route.query.q as string
  }
})
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-8">

    <!-- Hero 标题（未搜索时才展示） -->
    <div v-if="!isSearching" class="text-center mb-6">
      <h1 class="text-3xl font-bold mb-1.5" style="color: var(--color-text);">
        Warframe <span style="color: var(--color-primary);">Codex</span>
      </h1>
      <p class="text-sm" style="color: var(--color-text-muted);">
        全数据图鉴 · 15 种语言 · 44,000+ 条目
      </p>
    </div>

    <!-- 搜索框（始终显示，未搜索时居中突出，有结果时靠左紧凑） -->
    <div
      class="mb-6 transition-all"
      :class="isSearching ? '' : 'max-w-lg mx-auto'"
    >
      <div class="relative flex gap-2">
        <div class="relative flex-1">
          <svg
            class="absolute left-3.5 top-1/2 -translate-y-1/2 w-5 h-5 pointer-events-none"
            style="color: var(--color-text-subtle);"
            fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
          >
            <circle cx="11" cy="11" r="8"/><path stroke-linecap="round" d="M21 21l-4.35-4.35"/>
          </svg>
          <input
            v-model="inputDraft"
            class="input-search"
            :style="isSearching
              ? 'padding-left: 2.75rem; padding-right: 1rem; font-size: 0.9375rem;'
              : 'padding-left: 2.75rem; padding-right: 1rem; font-size: 1rem;'"
            placeholder="搜索武器、战甲、敌人、Mod..."
            :autofocus="!isSearching"
            @keydown.enter="submitSearch"
          />
        </div>
        <button
          class="shrink-0 px-5 py-2 rounded-lg font-medium text-sm transition-colors"
          style="background: var(--color-primary); color: #fff;"
          @click="submitSearch"
        >搜索</button>
        <!-- 有结果时显示清除按钮 -->
        <button
          v-if="isSearching"
          class="shrink-0 px-3 py-2 rounded-lg text-sm transition-colors"
          style="background: var(--color-surface); border: 1px solid var(--color-border); color: var(--color-text-muted);"
          @click="query = ''; inputDraft = ''; activeCategory = null; activeSubType = null; router.replace({ query: {} })"
        >清除</button>
      </div>
    </div>

    <!-- 分类卡片（未搜索时才展示） -->
    <div v-if="!isSearching" class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-2.5 mb-4">
      <button
        v-for="cat in CATEGORIES"
        :key="cat.key"
        class="card p-3.5 text-center cursor-pointer"
        @click="selectCategory(cat.key)"
      >
        <div
          class="w-8 h-8 rounded-lg mx-auto mb-2 flex items-center justify-center"
          :style="{ background: cat.color + '15' }"
        >
          <div class="w-3.5 h-3.5 rounded-sm" :style="{ background: cat.color }"></div>
        </div>
        <p class="text-sm font-semibold leading-tight" :style="{ color: cat.color }">
          {{ cat.label }}
        </p>
        <p class="text-xs mt-0.5 leading-tight" style="color: var(--color-text-subtle);">
          {{ cat.labelEn }}
        </p>
      </button>
    </div>

    <!-- 搜索/分类结果区 -->
    <template v-if="isSearching">
      <!-- 分类筛选条 -->
      <div class="flex items-center gap-1.5 mb-3 flex-wrap">
        <span class="text-xs shrink-0" style="color: var(--color-text-muted);">分类</span>
        <button
          v-for="cat in CATEGORIES"
          :key="cat.key"
          class="text-xs px-2.5 py-1 rounded-full font-medium transition-all"
          :style="{
            background: activeCategory === cat.key ? cat.color : 'var(--color-surface)',
            color: activeCategory === cat.key ? '#fff' : 'var(--color-text-muted)',
            border: activeCategory === cat.key ? `1px solid ${cat.color}` : '1px solid var(--color-border)',
          }"
          @click="selectCategory(cat.key)"
        >
          {{ cat.label }}
        </button>
      </div>

      <!-- 子分类筛选 -->
      <div v-if="availableSubTypes.length > 0" class="flex gap-1.5 flex-wrap mb-4 pl-0.5">
        <span class="text-xs self-center" style="color: var(--color-text-muted);">细分</span>
        <button
          v-for="sub in availableSubTypes"
          :key="sub.value"
          class="text-xs px-2.5 py-0.5 rounded-full transition-all"
          :style="{
            background: activeSubType === sub.value
              ? (currentCategoryInfo?.color ?? '#0ea5e9') + '15'
              : 'var(--color-surface)',
            color: activeSubType === sub.value
              ? (currentCategoryInfo?.color ?? '#0ea5e9')
              : 'var(--color-text-muted)',
            border: activeSubType === sub.value
              ? `1px solid ${currentCategoryInfo?.color ?? '#0ea5e9'}50`
              : '1px solid var(--color-border)',
            fontWeight: activeSubType === sub.value ? '600' : '400',
          }"
          @click="selectSubType(sub.value)"
        >
          {{ sub.label }}
        </button>
      </div>

      <!-- 加载中 -->
      <div v-if="dataStore.indexLoading" class="text-center py-20" style="color: var(--color-text-muted);">
        加载数据中...
      </div>

      <!-- 无结果 -->
      <div v-else-if="results.length === 0 && query.trim()" class="text-center py-20">
        <p class="text-lg mb-1" style="color: var(--color-text);">未找到 "{{ query }}"</p>
        <p class="text-sm" style="color: var(--color-text-muted);">尝试英文关键词或内部路径</p>
      </div>

      <!-- 结果数量 -->
      <p v-else-if="results.length > 0" class="text-xs mb-3" style="color: var(--color-text-muted);">
        共 {{ results.length }} 条结果
      </p>

      <!-- 结果列表 -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2.5">
        <ItemCard v-for="entry in results" :key="entry.path" :entry="entry" />
      </div>
    </template>
  </div>
</template>
