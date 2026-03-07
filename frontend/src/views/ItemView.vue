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

// 从搜索索引找到基本信息
const indexEntry = computed(() =>
  dataStore.searchIndex.find(e => e.path === props.path),
)

const displayName = computed(() => {
  const lang = langStore.currentLang
  if (lang === 'zh' || lang === 'tc') return indexEntry.value?.name_zh ?? indexEntry.value?.name_en ?? props.path.split('/').pop()
  return indexEntry.value?.name_en ?? indexEntry.value?.name_zh ?? props.path.split('/').pop()
})

const categoryInfo = computed(() =>
  CATEGORIES.find(c => c.key === props.category),
)

const iconUrl = computed(() => getIconUrl(indexEntry.value?.icon))

async function load() {
  loading.value = true
  try {
    const data = await dataStore.loadExport(props.category)
    // 尝试直接取，或从嵌套分类取
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
}

// 渲染字段值：如果是语言 key 则翻译，如果是游戏路径则显示为链接
function renderValue(val: unknown): string {
  if (typeof val === 'string') {
    if (val.startsWith('/Lotus/Language/')) {
      const translated = langStore.t(val)
      return translated || val
    }
    return val
  }
  if (typeof val === 'number' || typeof val === 'boolean') return String(val)
  return JSON.stringify(val)
}

function isGamePath(val: unknown): boolean {
  return typeof val === 'string' && (val.startsWith('/Lotus/') || val.startsWith('/EE/'))
    && !val.startsWith('/Lotus/Language/')
}

function isLangKey(val: unknown): boolean {
  return typeof val === 'string' && val.startsWith('/Lotus/Language/')
}

// 将扁平的字段对象转为分组展示
function groupFields(data: Record<string, unknown>) {
  const basic: [string, unknown][] = []
  const refs: [string, unknown][] = []
  const complex: [string, unknown][] = []

  for (const [k, v] of Object.entries(data)) {
    if (k === 'name' || k === 'description' || k === 'icon') continue
    if (typeof v === 'object' && v !== null && !Array.isArray(v)) {
      complex.push([k, v])
    } else if (Array.isArray(v) && v.length > 0 && typeof v[0] === 'object') {
      complex.push([k, v])
    } else if (isGamePath(v)) {
      refs.push([k, v])
    } else {
      basic.push([k, v])
    }
  }
  return { basic, refs, complex }
}

function navigateTo(path: string) {
  const entry = dataStore.searchIndex.find(e => e.path === path)
  if (entry) {
    router.push({ name: 'item', query: { path, category: entry.category } })
  }
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

    <!-- 加载状态 -->
    <div v-if="loading" class="text-center py-20" style="color: var(--color-text-muted);">
      加载中...
    </div>

    <template v-else>
      <!-- 头部信息 -->
      <div class="card p-6 mb-6">
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
            <div class="flex items-center gap-2 mb-1 flex-wrap">
              <span
                class="tag text-xs"
                :style="{ background: (categoryInfo?.color ?? '#64748b') + '20', color: categoryInfo?.color ?? '#64748b' }"
              >
                {{ categoryInfo?.label ?? category }}
              </span>
              <span v-if="indexEntry?.sub_category" class="tag text-xs">
                {{ indexEntry.sub_category }}
              </span>
            </div>
            <h1 class="text-2xl font-bold mb-1" style="color: var(--color-text);">
              {{ displayName }}
            </h1>
            <!-- 其他语言名字 -->
            <p v-if="indexEntry?.name_en && langStore.currentLang !== 'en'" class="text-sm" style="color: var(--color-text-muted);">
              {{ indexEntry.name_en }}
            </p>
            <!-- 描述 -->
            <p
              v-if="entryData?.description"
              class="mt-3 text-sm leading-relaxed"
              style="color: var(--color-text-muted);"
            >
              {{ langStore.t(entryData.description as string) || entryData.description }}
            </p>
          </div>
        </div>
      </div>

      <!-- 内部路径 -->
      <div class="card p-4 mb-4">
        <p class="text-xs font-mono" style="color: var(--color-text-muted);">
          {{ path }}
        </p>
      </div>

      <!-- 数据字段 -->
      <template v-if="entryData">
        <div v-if="groupFields(entryData) as { basic: [string, unknown][], refs: [string, unknown][], complex: [string, unknown][] }">
          <!-- 基础属性 -->
          <div
            v-if="groupFields(entryData).basic.length"
            class="card p-5 mb-4"
          >
            <h2 class="text-sm font-semibold mb-4" style="color: var(--color-primary);">基础属性</h2>
            <dl class="grid grid-cols-2 sm:grid-cols-3 gap-x-4 gap-y-3">
              <template v-for="[key, val] in groupFields(entryData).basic" :key="key">
                <div>
                  <dt class="text-xs mb-0.5" style="color: var(--color-text-muted);">{{ key }}</dt>
                  <dd class="text-sm font-medium" style="color: var(--color-text);">
                    <span v-if="isLangKey(val)" style="color: var(--color-gold);">
                      {{ langStore.t(val as string) || val }}
                    </span>
                    <span v-else>{{ renderValue(val) }}</span>
                  </dd>
                </div>
              </template>
            </dl>
          </div>

          <!-- 引用关系 -->
          <div
            v-if="groupFields(entryData).refs.length"
            class="card p-5 mb-4"
          >
            <h2 class="text-sm font-semibold mb-4" style="color: var(--color-primary);">关联数据</h2>
            <dl class="space-y-2">
              <template v-for="[key, val] in groupFields(entryData).refs" :key="key">
                <div class="flex items-start gap-3">
                  <dt class="text-xs w-32 shrink-0 pt-0.5" style="color: var(--color-text-muted);">{{ key }}</dt>
                  <dd>
                    <button
                      v-if="isGamePath(val) && dataStore.searchIndex.find(e => e.path === val)"
                      class="text-sm text-left transition-colors"
                      style="color: var(--color-primary);"
                      @click="navigateTo(val as string)"
                    >
                      {{ dataStore.searchIndex.find(e => e.path === val)?.name_zh ?? val }}
                    </button>
                    <span v-else class="text-sm font-mono" style="color: var(--color-text-muted); font-size: 0.7rem;">
                      {{ val }}
                    </span>
                  </dd>
                </div>
              </template>
            </dl>
          </div>

          <!-- 复杂结构（折叠） -->
          <details
            v-if="groupFields(entryData).complex.length"
            class="card p-5"
          >
            <summary class="text-sm font-semibold cursor-pointer" style="color: var(--color-primary);">
              原始数据 ({{ groupFields(entryData).complex.length }} 个字段)
            </summary>
            <pre
              class="mt-4 text-xs overflow-auto p-3 rounded-lg"
              style="background: var(--color-surface); color: var(--color-text-muted); max-height: 400px;"
            >{{ JSON.stringify(Object.fromEntries(groupFields(entryData).complex), null, 2) }}</pre>
          </details>
        </div>
      </template>

      <div v-else class="text-center py-10" style="color: var(--color-text-muted);">
        暂无详细数据
      </div>
    </template>
  </div>
</template>
