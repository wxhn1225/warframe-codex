<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useDataStore } from '@/stores/data'
import { useLangStore } from '@/stores/lang'
import { getIconUrl, CATEGORIES } from '@/types'

const props = defineProps<{ path: string; category: string }>()
const router = useRouter()
const dataStore = useDataStore()
const langStore = useLangStore()

const entryData = ref<Record<string, unknown> | null>(null)
const loading = ref(true)
const showRawJson = ref(false)

/** 搜索索引中的基本信息 */
const indexEntry = computed(() =>
  dataStore.searchIndex.find(e => e.path === props.path),
)

const displayName = computed(() => {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') {
    return indexEntry.value?.name_zh ?? indexEntry.value?.name_en ?? props.path.split('/').pop()
  }
  return indexEntry.value?.name_en ?? indexEntry.value?.name_zh ?? props.path.split('/').pop()
})

const categoryInfo = computed(() =>
  CATEGORIES.find(c => c.key === props.category),
)

const iconUrl = computed(() => getIconUrl(indexEntry.value?.icon))

/** 描述文本（从 lang key 翻译） */
const description = computed(() => {
  if (!entryData.value?.description) return ''
  const val = entryData.value.description as string
  if (val.startsWith('/Lotus/Language/')) return langStore.t(val) || val
  return val
})

// ──────────────────────────────────────────────
// 关联文件：从 Export 数据递归提取所有游戏路径
// ──────────────────────────────────────────────
interface RelatedItem {
  field: string
  path: string
  nameZh?: string
  nameEn?: string
  inIndex: boolean   // 是否在搜索索引中（有 Export 数据）
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

const forwardRefs = computed<RelatedItem[]>(() => {
  if (!entryData.value) return []
  const results: RelatedItem[] = []
  const seen = new Set<string>()
  seen.add(props.path) // 排除自身
  for (const [k, v] of Object.entries(entryData.value)) {
    if (k === 'icon') continue
    extractGamePaths(v, k, results, seen)
  }
  return results
})

/** 反向引用：在当前 Export 文件中找所有引用了 props.path 的条目 */
const reverseRefs = ref<RelatedItem[]>([])
const reverseLoading = ref(false)

async function loadReverseRefs() {
  if (!props.path || reverseLoading.value) return
  reverseLoading.value = true
  try {
    const data = await dataStore.loadExport(props.category)
    const found: RelatedItem[] = []
    const seen = new Set<string>()

    function containsPath(val: unknown, target: string): boolean {
      if (typeof val === 'string') return val === target
      if (Array.isArray(val)) return val.some(v => containsPath(v, target))
      if (val && typeof val === 'object') {
        return Object.values(val as Record<string, unknown>).some(v => containsPath(v, target))
      }
      return false
    }

    function scanEntries(obj: Record<string, unknown>) {
      for (const [k, v] of Object.entries(obj)) {
        if (k === props.path) continue
        if (k.startsWith('/') && v && typeof v === 'object') {
          if (containsPath(v, props.path) && !seen.has(k)) {
            seen.add(k)
            const hit = dataStore.searchIndex.find(e => e.path === k)
            found.push({
              field: '被引用',
              path: k,
              nameZh: hit?.name_zh,
              nameEn: hit?.name_en,
              inIndex: !!hit,
              indexCategory: hit?.category,
            })
          }
        } else if (v && typeof v === 'object' && !Array.isArray(v)) {
          scanEntries(v as Record<string, unknown>)
        }
      }
    }

    scanEntries(data)
    reverseRefs.value = found
  } finally {
    reverseLoading.value = false
  }
}

const allRelatedItems = computed<RelatedItem[]>(() => {
  const all = [...forwardRefs.value]
  const existingPaths = new Set(all.map(r => r.path))
  reverseRefs.value.forEach(r => {
    if (!existingPaths.has(r.path)) {
      all.push(r)
      existingPaths.add(r.path)
    }
  })
  return all
})

// ──────────────────────────────────────────────

async function load() {
  loading.value = true
  reverseRefs.value = []
  try {
    const data = await dataStore.loadExport(props.category)
    let found: unknown = data[props.path]
    if (!found) {
      for (const val of Object.values(data)) {
        if (val && typeof val === 'object' && !Array.isArray(val)) {
          const inner = (val as Record<string, unknown>)[props.path]
          if (inner) { found = inner; break }
        }
      }
    }
    entryData.value = (found as Record<string, unknown>) ?? null
  } finally {
    loading.value = false
  }
  loadReverseRefs()
}

function getPathDisplayName(item: RelatedItem): string {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') return item.nameZh ?? item.nameEn ?? item.path.split('/').pop() ?? item.path
  return item.nameEn ?? item.nameZh ?? item.path.split('/').pop() ?? item.path
}

function navigateToItem(item: RelatedItem) {
  if (item.inIndex && item.indexCategory) {
    router.push({ name: 'item', query: { path: item.path, category: item.indexCategory } })
  } else {
    router.push({ name: 'raw', query: { path: item.path } })
  }
}

function openRaw(gamePath: string) {
  router.push({ name: 'raw', query: { path: gamePath } })
}

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

    <!-- 加载中 -->
    <div v-if="loading" class="text-center py-20" style="color: var(--color-text-muted);">
      加载中...
    </div>

    <template v-else>
      <!-- 头部信息 -->
      <div class="card p-6 mb-4">
        <div class="flex items-start gap-5">
          <div
            v-if="iconUrl"
            class="shrink-0 w-20 h-20 rounded-xl flex items-center justify-center overflow-hidden"
            style="background: var(--color-surface-3);"
          >
            <img
              :src="iconUrl"
              :alt="displayName"
              class="w-16 h-16 object-contain"
              @error="($event.target as HTMLImageElement).parentElement!.style.display='none'"
            />
          </div>

          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-2 flex-wrap">
              <span
                class="tag text-xs"
                :style="{ background: (categoryInfo?.color ?? '#64748b') + '20', color: categoryInfo?.color ?? '#64748b' }"
              >
                {{ categoryInfo?.label ?? category }}
              </span>
              <span v-if="indexEntry?.sub_type" class="tag text-xs" style="color: var(--color-text-muted);">
                {{ categoryInfo?.subTypeLabels?.[indexEntry.sub_type] ?? indexEntry.sub_type }}
              </span>
            </div>
            <h1 class="text-2xl font-bold mb-1" style="color: var(--color-text);">
              {{ displayName }}
            </h1>
            <p v-if="indexEntry?.name_en && langStore.currentLang !== 'en'" class="text-xs mb-2" style="color: var(--color-text-muted);">
              {{ indexEntry.name_en }}
            </p>
            <p v-if="description" class="text-sm leading-relaxed" style="color: var(--color-text-muted);">
              {{ description }}
            </p>
          </div>
        </div>

        <!-- 内部路径 -->
        <div
          class="mt-4 px-3 py-2 rounded-lg font-mono text-xs flex items-center justify-between gap-2"
          style="background: var(--color-surface-3); color: var(--color-text-muted);"
        >
          <span class="truncate">{{ path }}</span>
          <button
            class="shrink-0 text-xs px-2 py-0.5 rounded transition-colors"
            style="background: var(--color-surface); color: var(--color-primary);"
            @click="openRaw(path)"
          >
            原始引擎文件
          </button>
        </div>
      </div>

      <!-- 关联文件列表 -->
      <div class="card p-5 mb-4">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-sm font-semibold" style="color: var(--color-primary);">
            关联文件
            <span v-if="allRelatedItems.length > 0" class="ml-1 font-normal" style="color: var(--color-text-muted);">
              ({{ allRelatedItems.length }})
            </span>
          </h2>
          <span v-if="reverseLoading" class="text-xs" style="color: var(--color-text-muted);">扫描引用中...</span>
        </div>

        <div v-if="allRelatedItems.length === 0 && !reverseLoading" class="text-sm" style="color: var(--color-text-muted);">
          暂无关联文件
        </div>

        <div v-else class="space-y-1">
          <div
            v-for="item in allRelatedItems"
            :key="item.path"
            class="flex items-center gap-3 py-2 px-3 rounded-lg group transition-colors"
            style="background: var(--color-surface-3);"
          >
            <!-- 字段名 -->
            <span
              class="shrink-0 text-xs font-mono w-32 truncate"
              style="color: var(--color-text-muted);"
              :title="item.field"
            >
              {{ item.field }}
            </span>

            <!-- 名称 + 路径 -->
            <div class="flex-1 min-w-0">
              <p
                v-if="item.nameZh || item.nameEn"
                class="text-sm font-medium truncate"
                style="color: var(--color-text);"
              >
                {{ getPathDisplayName(item) }}
              </p>
              <p class="text-xs font-mono truncate" style="color: var(--color-text-muted);">
                {{ item.path }}
              </p>
            </div>

            <!-- 操作按钮 -->
            <div class="flex gap-1.5 shrink-0">
              <button
                v-if="item.inIndex"
                class="text-xs px-2.5 py-1 rounded-lg transition-colors font-medium"
                style="background: var(--color-primary-dim, rgba(0,180,216,0.15)); color: var(--color-primary);"
                @click="navigateToItem(item)"
              >
                图鉴
              </button>
              <button
                class="text-xs px-2.5 py-1 rounded-lg transition-colors"
                style="background: var(--color-surface); color: var(--color-text-muted);"
                @click="openRaw(item.path)"
              >
                原始文件
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 完整 Export 数据（可折叠） -->
      <div class="card">
        <button
          class="w-full flex items-center justify-between p-5 text-left"
          @click="showRawJson = !showRawJson"
        >
          <span class="text-sm font-semibold" style="color: var(--color-primary);">
            Export 完整数据
          </span>
          <svg
            class="w-4 h-4 transition-transform"
            :class="{ 'rotate-180': showRawJson }"
            fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
            style="color: var(--color-text-muted);"
          >
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
          </svg>
        </button>
        <div v-if="showRawJson" class="px-5 pb-5">
          <pre
            class="text-xs overflow-auto p-4 rounded-xl"
            style="background: var(--color-surface); color: var(--color-text-muted); max-height: 600px; white-space: pre-wrap; word-break: break-all;"
          >{{ JSON.stringify(entryData, null, 2) }}</pre>
        </div>
      </div>

      <div v-if="!entryData" class="text-center py-10" style="color: var(--color-text-muted);">
        暂无 Export 数据（可能仅存在于原始引擎文件中）
      </div>
    </template>
  </div>
</template>
