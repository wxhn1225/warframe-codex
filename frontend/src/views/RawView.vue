<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useDataStore } from '@/stores/data'
import { useLangStore } from '@/stores/lang'
import { getRawFileUrl, CATEGORIES } from '@/types'

const props = defineProps<{ path: string }>()
const router = useRouter()
const dataStore = useDataStore()
const langStore = useLangStore()

const rawData = ref<Record<string, unknown> | null>(null)
const loading = ref(true)
const error = ref('')
const showRelated = ref(true)
const copied = ref(false)

const rawUrl = computed(() => getRawFileUrl(props.path))

const fileName = computed(() => {
  const parts = props.path.split('/')
  return parts[parts.length - 1] + '.json'
})

/** 在搜索索引中查找 */
const indexEntry = computed(() =>
  dataStore.searchIndex.find(e => e.path === props.path),
)

const displayName = computed(() => {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') {
    return indexEntry.value?.name_zh ?? indexEntry.value?.name_en ?? fileName.value
  }
  return indexEntry.value?.name_en ?? indexEntry.value?.name_zh ?? fileName.value
})

// ──────────────────────────────────────────────
// 从原始数据中提取关联文件路径
// ──────────────────────────────────────────────
interface RelatedItem {
  field: string
  path: string
  nameZh?: string
  nameEn?: string
  inIndex: boolean
  indexCategory?: string
}

function extractGamePaths(
  val: unknown,
  fieldPrefix: string,
  results: RelatedItem[],
  seen: Set<string>,
): void {
  if (typeof val === 'string') {
    if (
      (val.startsWith('/Lotus/') || val.startsWith('/EE/')) &&
      !val.startsWith('/Lotus/Language/') &&
      !seen.has(val)
    ) {
      seen.add(val)
      const hit = dataStore.searchIndex.find(e => e.path === val)
      results.push({
        field: fieldPrefix,
        path: val,
        nameZh: hit?.name_zh,
        nameEn: hit?.name_en,
        inIndex: !!hit,
        indexCategory: hit?.category,
      })
    }
    return
  }
  if (Array.isArray(val)) {
    val.forEach((item, i) => extractGamePaths(item, `${fieldPrefix}[${i}]`, results, seen))
    return
  }
  if (val && typeof val === 'object') {
    for (const [k, v] of Object.entries(val as Record<string, unknown>)) {
      extractGamePaths(v, fieldPrefix ? `${fieldPrefix}.${k}` : k, results, seen)
    }
  }
}

const relatedItems = computed<RelatedItem[]>(() => {
  if (!rawData.value) return []
  const results: RelatedItem[] = []
  const seen = new Set<string>()
  seen.add(props.path)
  for (const [k, v] of Object.entries(rawData.value)) {
    extractGamePaths(v, k, results, seen)
  }
  return results
})

async function load() {
  loading.value = true
  error.value = ''
  rawData.value = null
  try {
    const res = await fetch(rawUrl.value)
    if (!res.ok) {
      error.value = `HTTP ${res.status}：文件不存在或无法访问`
      return
    }
    rawData.value = await res.json()
  } catch (e) {
    error.value = `加载失败：${e instanceof Error ? e.message : String(e)}`
  } finally {
    loading.value = false
  }
}

function getPathDisplayName(item: RelatedItem): string {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') return item.nameZh ?? item.nameEn ?? item.path.split('/').pop() ?? item.path
  return item.nameEn ?? item.nameZh ?? item.path.split('/').pop() ?? item.path
}

function navigateTo(item: RelatedItem) {
  if (item.inIndex && item.indexCategory) {
    router.push({ name: 'item', query: { path: item.path, category: item.indexCategory } })
  } else {
    router.push({ name: 'raw', query: { path: item.path } })
  }
}

async function copyJson() {
  try {
    await navigator.clipboard.writeText(JSON.stringify(rawData.value, null, 2))
    copied.value = true
    setTimeout(() => { copied.value = false }, 2000)
  } catch {
    // ignore
  }
}

const categoryInfo = computed(() =>
  indexEntry.value ? CATEGORIES.find(c => c.key === indexEntry.value!.category) : undefined,
)

onMounted(load)
watch(() => props.path, load)
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 py-8">
    <!-- 返回按钮 -->
    <button
      class="flex items-center gap-1.5 text-sm mb-6 transition-colors"
      style="color: var(--color-text-muted);"
      @click="router.back()"
    >
      <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" />
      </svg>
      返回
    </button>

    <!-- 头部 -->
    <div class="card p-5 mb-4">
      <div class="flex items-start justify-between gap-4">
        <div class="flex-1 min-w-0">
          <!-- 如果在搜索索引中，显示中文名和分类 -->
          <div v-if="indexEntry" class="flex items-center gap-2 mb-2 flex-wrap">
            <span
              v-if="categoryInfo"
              class="tag text-xs"
              :style="{ background: (categoryInfo.color) + '20', color: categoryInfo.color }"
            >
              {{ categoryInfo.label }}
            </span>
            <span class="tag text-xs" style="color: var(--color-gold);">原始引擎文件</span>
          </div>
          <div v-else class="flex items-center gap-2 mb-2">
            <span class="tag text-xs" style="color: var(--color-text-muted);">原始引擎文件</span>
          </div>

          <h1 class="text-xl font-bold mb-1" style="color: var(--color-text);">
            {{ displayName }}
          </h1>

          <!-- 如果在图鉴中，显示跳转按钮 -->
          <button
            v-if="indexEntry"
            class="text-xs mt-1 transition-colors"
            style="color: var(--color-primary);"
            @click="router.push({ name: 'item', query: { path, category: indexEntry!.category } })"
          >
            查看图鉴页面 &rarr;
          </button>
        </div>

        <!-- GitHub 源文件链接 -->
        <a
          :href="rawUrl"
          target="_blank"
          rel="noopener"
          class="shrink-0 text-xs px-3 py-1.5 rounded-lg transition-colors"
          style="background: var(--color-surface-3); color: var(--color-text-muted);"
        >
          GitHub 源
        </a>
      </div>

      <!-- 内部路径 -->
      <div
        class="mt-3 px-3 py-2 rounded-lg font-mono text-xs truncate"
        style="background: var(--color-surface-3); color: var(--color-text-muted);"
      >
        {{ path }}
      </div>
    </div>

    <!-- 加载中 -->
    <div v-if="loading" class="text-center py-20" style="color: var(--color-text-muted);">
      <p>加载原始文件中...</p>
      <p class="text-xs mt-2 font-mono" style="color: var(--color-text-muted); opacity: 0.6;">{{ rawUrl }}</p>
    </div>

    <!-- 错误 -->
    <div v-else-if="error" class="card p-6 text-center">
      <p class="mb-2" style="color: #e63946;">{{ error }}</p>
      <p class="text-xs mb-4" style="color: var(--color-text-muted);">来源：{{ rawUrl }}</p>
      <a
        :href="rawUrl"
        target="_blank"
        rel="noopener"
        class="text-sm"
        style="color: var(--color-primary);"
      >
        在 GitHub 中打开 &rarr;
      </a>
    </div>

    <template v-else-if="rawData">
      <!-- 关联文件 -->
      <div class="card p-5 mb-4">
        <button
          class="w-full flex items-center justify-between text-left"
          @click="showRelated = !showRelated"
        >
          <span class="text-sm font-semibold" style="color: var(--color-primary);">
            关联文件
            <span class="ml-1 font-normal" style="color: var(--color-text-muted);">
              ({{ relatedItems.length }})
            </span>
          </span>
          <svg
            class="w-4 h-4 transition-transform"
            :class="{ 'rotate-180': showRelated }"
            fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
            style="color: var(--color-text-muted);"
          >
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
          </svg>
        </button>

        <div v-if="showRelated && relatedItems.length > 0" class="mt-4 space-y-1">
          <div
            v-for="item in relatedItems"
            :key="item.path"
            class="flex items-center gap-3 py-2 px-3 rounded-lg"
            style="background: var(--color-surface-3);"
          >
            <span
              class="shrink-0 text-xs font-mono w-36 truncate"
              style="color: var(--color-text-muted);"
              :title="item.field"
            >
              {{ item.field }}
            </span>
            <div class="flex-1 min-w-0">
              <p v-if="item.nameZh || item.nameEn" class="text-sm font-medium truncate" style="color: var(--color-text);">
                {{ getPathDisplayName(item) }}
              </p>
              <p class="text-xs font-mono truncate" style="color: var(--color-text-muted);">
                {{ item.path }}
              </p>
            </div>
            <div class="flex gap-1.5 shrink-0">
              <button
                v-if="item.inIndex"
                class="text-xs px-2.5 py-1 rounded-lg transition-colors font-medium"
                style="background: rgba(0,180,216,0.15); color: var(--color-primary);"
                @click="navigateTo(item)"
              >
                图鉴
              </button>
              <button
                class="text-xs px-2.5 py-1 rounded-lg transition-colors"
                style="background: var(--color-surface); color: var(--color-text-muted);"
                @click="navigateTo(item)"
              >
                原始文件
              </button>
            </div>
          </div>
        </div>

        <p v-else-if="showRelated && relatedItems.length === 0" class="mt-3 text-sm" style="color: var(--color-text-muted);">
          无关联游戏路径
        </p>
      </div>

      <!-- 原始 JSON 内容 -->
      <div class="card">
        <div class="flex items-center justify-between px-5 pt-5 pb-3">
          <h2 class="text-sm font-semibold" style="color: var(--color-primary);">
            原始内容
            <span class="ml-1 font-mono font-normal text-xs" style="color: var(--color-text-muted);">
              {{ fileName }}
            </span>
          </h2>
          <button
            class="text-xs px-3 py-1.5 rounded-lg transition-colors"
            :style="{
              background: copied ? 'rgba(45,156,85,0.2)' : 'var(--color-surface-3)',
              color: copied ? '#2d9c55' : 'var(--color-text-muted)',
            }"
            @click="copyJson"
          >
            {{ copied ? '已复制' : '复制 JSON' }}
          </button>
        </div>
        <div class="px-5 pb-5">
          <pre
            class="text-xs overflow-auto p-4 rounded-xl"
            style="background: var(--color-surface); color: var(--color-text-muted); max-height: 700px; white-space: pre-wrap; word-break: break-all;"
          >{{ JSON.stringify(rawData, null, 2) }}</pre>
        </div>
      </div>
    </template>
  </div>
</template>
