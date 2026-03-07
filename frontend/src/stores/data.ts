import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { SearchEntry } from '@/types'

const DATA_BASE = import.meta.env.BASE_URL + 'data'

export const useDataStore = defineStore('data', () => {
  const searchIndex = ref<SearchEntry[]>([])
  const exportCache = ref<Record<string, Record<string, unknown>>>({})
  const indexLoaded = ref(false)
  const indexLoading = ref(false)

  async function loadSearchIndex() {
    if (indexLoaded.value || indexLoading.value) return
    indexLoading.value = true
    try {
      const res = await fetch(`${DATA_BASE}/search-index.json`)
      searchIndex.value = await res.json() as SearchEntry[]
      indexLoaded.value = true
    } finally {
      indexLoading.value = false
    }
  }

  async function loadExport(category: string): Promise<Record<string, unknown>> {
    if (exportCache.value[category]) return exportCache.value[category]
    const res = await fetch(`${DATA_BASE}/export/${category}.json`)
    const data = await res.json() as Record<string, unknown>
    exportCache.value[category] = data
    return data
  }

  async function getEntry(path: string, category: string): Promise<unknown> {
    const data = await loadExport(category)
    // 处理 ExportEnemies 的嵌套分类结构
    for (const val of Object.values(data)) {
      if (val && typeof val === 'object' && !Array.isArray(val)) {
        const obj = val as Record<string, unknown>
        if (path in obj) return obj[path]
        // 如果顶层就是 path → data 格式
      }
    }
    return data[path] ?? null
  }

  function search(query: string, category?: string): SearchEntry[] {
    if (!query.trim()) return []
    const q = query.toLowerCase()
    return searchIndex.value
      .filter(e => {
        if (category && e.category !== category) return false
        return (
          (e.name_zh?.toLowerCase().includes(q) ?? false) ||
          (e.name_en?.toLowerCase().includes(q) ?? false) ||
          e.path.toLowerCase().includes(q)
        )
      })
      .slice(0, 100)
  }

  function getByCategory(category: string, subCategory?: string): SearchEntry[] {
    return searchIndex.value.filter(e => {
      if (e.category !== category) return false
      if (subCategory && e.sub_category !== subCategory) return false
      return true
    })
  }

  return {
    searchIndex,
    indexLoaded,
    indexLoading,
    loadSearchIndex,
    loadExport,
    getEntry,
    search,
    getByCategory,
  }
})
