<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useDataStore } from '@/stores/data'
import { useLangStore } from '@/stores/lang'
import { getIconUrl, getSubTypeLabel, CATEGORIES } from '@/types'
import VueJsonPretty from 'vue-json-pretty'
import 'vue-json-pretty/lib/styles.css'

const props = defineProps<{ path: string; category: string }>()
const router = useRouter()
const dataStore = useDataStore()
const langStore = useLangStore()

const entryData = ref<Record<string, unknown> | null>(null)
const loading = ref(true)
const showRawJson = ref(false)

const indexEntry = computed(() =>
  dataStore.searchIndex.find(e => e.path === props.path),
)
const displayName = computed(() => {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') return indexEntry.value?.name_zh ?? indexEntry.value?.name_en ?? props.path.split('/').pop()
  return indexEntry.value?.name_en ?? indexEntry.value?.name_zh ?? props.path.split('/').pop()
})
const categoryInfo = computed(() => CATEGORIES.find(c => c.key === props.category))
const iconUrl = computed(() => getIconUrl(indexEntry.value?.icon))
const description = computed(() => {
  if (!entryData.value?.description) return ''
  const v = entryData.value.description as string
  return v.startsWith('/Lotus/Language/') ? langStore.t(v) || v : v
})

// ── 关联文件 ──────────────────────────────────────────────
interface RelatedItem {
  field: string; path: string
  nameZh?: string; nameEn?: string
  inIndex: boolean; indexCategory?: string
}

function extractGamePaths(val: unknown, prefix: string, out: RelatedItem[], seen: Set<string>) {
  if (typeof val === 'string') {
    if ((val.startsWith('/Lotus/') || val.startsWith('/EE/')) && !val.startsWith('/Lotus/Language/') && !seen.has(val)) {
      seen.add(val)
      const hit = dataStore.searchIndex.find(e => e.path === val)
      out.push({ field: prefix, path: val, nameZh: hit?.name_zh, nameEn: hit?.name_en, inIndex: !!hit, indexCategory: hit?.category })
    }
    return
  }
  if (Array.isArray(val)) { val.forEach((v, i) => extractGamePaths(v, `${prefix}[${i}]`, out, seen)); return }
  if (val && typeof val === 'object') {
    for (const [k, v] of Object.entries(val as Record<string, unknown>)) extractGamePaths(v, prefix ? `${prefix}.${k}` : k, out, seen)
  }
}

const forwardRefs = computed<RelatedItem[]>(() => {
  if (!entryData.value) return []
  const out: RelatedItem[] = []; const seen = new Set([props.path])
  for (const [k, v] of Object.entries(entryData.value)) { if (k !== 'icon') extractGamePaths(v, k, out, seen) }
  return out
})

const reverseRefs = ref<RelatedItem[]>([])
const reverseLoading = ref(false)

async function loadReverseRefs() {
  reverseLoading.value = true
  try {
    const data = await dataStore.loadExport(props.category)
    const found: RelatedItem[] = []; const seen = new Set<string>()
    function has(val: unknown, target: string): boolean {
      if (typeof val === 'string') return val === target
      if (Array.isArray(val)) return val.some(v => has(v, target))
      if (val && typeof val === 'object') return Object.values(val as Record<string, unknown>).some(v => has(v, target))
      return false
    }
    function scan(obj: Record<string, unknown>) {
      for (const [k, v] of Object.entries(obj)) {
        if (k === props.path) continue
        if (k.startsWith('/') && v && typeof v === 'object' && has(v, props.path) && !seen.has(k)) {
          seen.add(k); const hit = dataStore.searchIndex.find(e => e.path === k)
          found.push({ field: '被引用', path: k, nameZh: hit?.name_zh, nameEn: hit?.name_en, inIndex: !!hit, indexCategory: hit?.category })
        } else if (v && typeof v === 'object' && !Array.isArray(v)) scan(v as Record<string, unknown>)
      }
    }
    scan(data); reverseRefs.value = found
  } finally { reverseLoading.value = false }
}

const allRelatedItems = computed<RelatedItem[]>(() => {
  const seen = new Set(forwardRefs.value.map(r => r.path))
  return [...forwardRefs.value, ...reverseRefs.value.filter(r => !seen.has(r.path))]
})

async function load() {
  loading.value = true; reverseRefs.value = []
  try {
    const data = await dataStore.loadExport(props.category)
    let found: unknown = data[props.path]
    if (!found) for (const v of Object.values(data)) {
      if (v && typeof v === 'object' && !Array.isArray(v)) { const inner = (v as Record<string, unknown>)[props.path]; if (inner) { found = inner; break } }
    }
    entryData.value = (found as Record<string, unknown>) ?? null
  } finally { loading.value = false }
  loadReverseRefs()
}

function getName(item: RelatedItem) {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') return item.nameZh ?? item.nameEn ?? item.path.split('/').pop() ?? item.path
  return item.nameEn ?? item.nameZh ?? item.path.split('/').pop() ?? item.path
}
function goToItem(item: RelatedItem) {
  if (item.inIndex && item.indexCategory) router.push({ name: 'item', query: { path: item.path, category: item.indexCategory } })
  else router.push({ name: 'raw', query: { path: item.path } })
}
function openRaw(p: string) { router.push({ name: 'raw', query: { path: p } }) }

onMounted(load)
watch(() => props.path, load)
</script>

<template>
  <div class="max-w-4xl mx-auto px-4 py-8">
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

    <div v-if="loading" class="text-center py-20" style="color: var(--color-text-muted);">加载中...</div>

    <template v-else>
      <!-- 头部 -->
      <div class="card p-5 mb-4">
        <div class="flex items-start gap-4">
          <div
            v-if="iconUrl"
            class="shrink-0 w-16 h-16 rounded-xl flex items-center justify-center overflow-hidden"
            :style="{ background: (categoryInfo?.color ?? '#0ea5e9') + '40', border: `1px solid ${categoryInfo?.color ?? '#0ea5e9'}50` }"
          >
            <img :src="iconUrl" :alt="displayName" class="w-13 h-13 object-contain"
              @error="($event.target as HTMLImageElement).parentElement!.style.display='none'" />
          </div>
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-2 flex-wrap">
              <span
                class="text-xs font-semibold px-2 py-0.5 rounded-full"
                :style="{ background: (categoryInfo?.color ?? '#0ea5e9') + '15', color: categoryInfo?.color ?? '#0ea5e9' }"
              >
                {{ categoryInfo?.label ?? category }}
              </span>
              <span
                v-if="indexEntry?.sub_type && categoryInfo"
                class="text-xs px-2 py-0.5 rounded-full"
                style="background: var(--color-surface-2); color: var(--color-text-muted);"
              >
                {{ getSubTypeLabel(categoryInfo, indexEntry.sub_type, langStore.t) }}
              </span>
            </div>
            <h1 class="text-xl font-bold mb-1" style="color: var(--color-text);">{{ displayName }}</h1>
            <p v-if="indexEntry?.name_en && langStore.currentLang !== 'en'" class="text-xs mb-2" style="color: var(--color-text-muted);">{{ indexEntry.name_en }}</p>
            <p v-if="description" class="text-sm leading-relaxed" style="color: var(--color-text-muted);">{{ description }}</p>
          </div>
        </div>
        <!-- 内部路径 -->
        <div class="mt-4 px-3 py-2 rounded-lg flex items-center justify-between gap-2"
          style="background: var(--color-surface-2); border: 1px solid var(--color-border);">
          <span class="font-mono text-xs truncate" style="color: var(--color-text-muted);">{{ path }}</span>
          <button
            class="shrink-0 text-xs px-2.5 py-1 rounded-lg transition-colors font-medium"
            style="background: var(--color-surface); border: 1px solid var(--color-border); color: var(--color-primary);"
            @click="openRaw(path)"
          >原始引擎文件</button>
        </div>
      </div>

      <!-- 关联文件 -->
      <div class="card p-5 mb-4">
        <div class="flex items-center justify-between mb-3">
          <h2 class="text-sm font-semibold" style="color: var(--color-text);">
            关联文件
            <span class="font-normal ml-1" style="color: var(--color-text-muted);">({{ allRelatedItems.length }})</span>
          </h2>
          <span v-if="reverseLoading" class="text-xs" style="color: var(--color-text-subtle);">扫描中…</span>
        </div>

        <p v-if="allRelatedItems.length === 0 && !reverseLoading" class="text-sm" style="color: var(--color-text-muted);">暂无关联文件</p>

        <div v-else class="space-y-1.5">
          <div
            v-for="item in allRelatedItems"
            :key="item.path"
            class="flex items-center gap-3 px-3 py-2.5 rounded-lg"
            style="background: var(--color-surface-2); border: 1px solid var(--color-border);"
          >
            <span class="shrink-0 text-xs font-mono w-32 truncate" style="color: var(--color-text-subtle);" :title="item.field">{{ item.field }}</span>
            <div class="flex-1 min-w-0">
              <p v-if="item.nameZh || item.nameEn" class="text-sm font-medium truncate" style="color: var(--color-text);">{{ getName(item) }}</p>
              <p class="text-xs font-mono truncate" style="color: var(--color-text-muted);">{{ item.path }}</p>
            </div>
            <div class="flex gap-1.5 shrink-0">
              <button v-if="item.inIndex"
                class="text-xs px-2.5 py-1 rounded-lg font-medium transition-colors"
                style="background: var(--color-primary); color: #fff;"
                @click="goToItem(item)">图鉴</button>
              <button
                class="text-xs px-2.5 py-1 rounded-lg transition-colors"
                style="background: var(--color-surface); border: 1px solid var(--color-border); color: var(--color-text-muted);"
                @click="openRaw(item.path)">原始文件</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Export JSON（折叠，高亮显示） -->
      <div class="card overflow-hidden">
        <button
          class="w-full flex items-center justify-between px-5 py-4 text-left"
          @click="showRawJson = !showRawJson"
        >
          <span class="text-sm font-semibold" style="color: var(--color-text);">Export 完整数据</span>
          <svg class="w-4 h-4 transition-transform" :class="{ 'rotate-180': showRawJson }"
            fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" style="color: var(--color-text-muted);">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
          </svg>
        </button>
        <div v-if="showRawJson" class="px-4 pb-4">
          <div class="rounded-lg overflow-auto p-3" style="background: var(--color-surface-2); border: 1px solid var(--color-border); max-height: 600px;">
            <VueJsonPretty :data="entryData" :deep="3" :show-line="true" :show-icon="true" />
          </div>
        </div>
      </div>

      <p v-if="!entryData" class="text-center py-8 text-sm" style="color: var(--color-text-muted);">暂无 Export 数据</p>
    </template>
  </div>
</template>
