<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useDataStore } from '@/stores/data'
import { CATEGORIES } from '@/types'
import ItemCard from '@/components/ItemCard.vue'

const route = useRoute()
const router = useRouter()
const dataStore = useDataStore()

const query = ref((route.query.q as string) ?? '')
const activeCategory = ref<string | null>(null)

const results = computed(() => {
  if (query.value.trim()) {
    return dataStore.search(query.value, activeCategory.value ?? undefined)
  }
  if (activeCategory.value) {
    const cat = CATEGORIES.find(c => c.key === activeCategory.value)
    const sub = cat?.subCategories?.[0]
    return dataStore.getByCategory(activeCategory.value, sub).slice(0, 80)
  }
  return []
})

const isSearching = computed(() => query.value.trim().length > 0 || activeCategory.value !== null)

function selectCategory(key: string) {
  activeCategory.value = activeCategory.value === key ? null : key
  query.value = ''
  router.replace({ query: {} })
}

watch(() => route.query.q, (q) => {
  if (q) {
    query.value = q as string
    activeCategory.value = null
  }
})

onMounted(() => {
  if (route.query.q) query.value = route.query.q as string
})
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 py-10">

    <!-- Hero 搜索区 -->
    <div v-if="!isSearching" class="text-center mb-12">
      <h1 class="text-3xl font-bold mb-2" style="color: var(--color-text);">
        <span style="color: var(--color-primary);">Warframe</span>
        <span> Codex</span>
      </h1>
      <p class="text-sm mb-8" style="color: var(--color-text-muted);">
        全数据图鉴 · 15 种语言 · 44,000+ 条目
      </p>

      <div class="max-w-lg mx-auto mb-10">
        <input
          v-model="query"
          class="input-search text-base"
          placeholder="搜索武器、战甲、敌人、Mod..."
          autofocus
        />
      </div>

      <!-- 分类卡片 -->
      <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-3">
        <button
          v-for="cat in CATEGORIES"
          :key="cat.key"
          class="card p-3 text-center cursor-pointer"
          @click="selectCategory(cat.key)"
        >
          <p class="text-sm font-semibold" :style="{ color: cat.color }">
            {{ cat.label }}
          </p>
          <p class="text-xs mt-0.5" style="color: var(--color-text-muted);">
            {{ cat.labelEn }}
          </p>
        </button>
      </div>
    </div>

    <!-- 搜索/分类结果 -->
    <template v-else>
      <!-- 顶部导航条 -->
      <div class="flex items-center gap-3 mb-6 flex-wrap">
        <button
          class="text-sm px-3 py-1.5 rounded-lg transition-colors"
          style="background: var(--color-surface-3); color: var(--color-text-muted);"
          @click="query = ''; activeCategory = null; router.replace({ query: {} })"
        >
          &larr; 返回
        </button>

        <!-- 分类筛选 -->
        <div class="flex gap-2 flex-wrap">
          <button
            v-for="cat in CATEGORIES"
            :key="cat.key"
            class="text-xs px-3 py-1 rounded-full transition-all font-medium"
            :style="{
              background: activeCategory === cat.key ? cat.color + '30' : 'var(--color-surface-3)',
              color: activeCategory === cat.key ? cat.color : 'var(--color-text-muted)',
              border: activeCategory === cat.key ? `1px solid ${cat.color}60` : '1px solid transparent',
            }"
            @click="selectCategory(cat.key)"
          >
            {{ cat.label }}
          </button>
        </div>
      </div>

      <!-- 搜索框（始终可见） -->
      <div class="mb-6">
        <input
          v-model="query"
          class="input-search max-w-lg"
          placeholder="继续搜索..."
        />
      </div>

      <!-- 加载提示 -->
      <div v-if="dataStore.indexLoading" class="text-center py-20" style="color: var(--color-text-muted);">
        <p>加载数据中...</p>
      </div>

      <!-- 无结果 -->
      <div
        v-else-if="results.length === 0 && query.trim()"
        class="text-center py-20"
        style="color: var(--color-text-muted);"
      >
        <p class="text-lg mb-2">未找到 "{{ query }}"</p>
        <p class="text-sm">尝试英文关键词或内部路径</p>
      </div>

      <!-- 结果数量 -->
      <p v-else-if="results.length > 0" class="text-xs mb-4" style="color: var(--color-text-muted);">
        共 {{ results.length }} 条结果
        <span v-if="results.length === 100">（显示前 100 条）</span>
      </p>

      <!-- 结果列表 -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        <ItemCard
          v-for="entry in results"
          :key="entry.path"
          :entry="entry"
        />
      </div>
    </template>
  </div>
</template>
