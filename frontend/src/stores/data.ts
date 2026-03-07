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

  /** 在 Export 数据中查找指定路径的条目（处理嵌套分类结构） */
  async function getEntry(path: string, category: string): Promise<unknown> {
    const data = await loadExport(category)
    if (data[path] !== undefined) return data[path]
    for (const val of Object.values(data)) {
      if (val && typeof val === 'object' && !Array.isArray(val)) {
        const obj = val as Record<string, unknown>
        if (path in obj) return obj[path]
      }
    }
    return null
  }

  /** 搜索，不限制结果数量 */
  function search(query: string, category?: string, subType?: string): SearchEntry[] {
    if (!query.trim()) return []
    const q = query.toLowerCase()
    return searchIndex.value.filter(e => {
      if (category && e.category !== category) return false
      if (subType && e.sub_type !== subType) return false
      return (
        (e.name_zh?.toLowerCase().includes(q) ?? false) ||
        (e.name_en?.toLowerCase().includes(q) ?? false) ||
        e.path.toLowerCase().includes(q)
      )
    })
  }

  /**
   * 按分类获取条目。
   * - forceSubCategory: 强制 sub_category 过滤（ExportEnemies 默认仅展示 avatars）
   * - subType: sub_type 过滤（faction / era / productCategory）
   */
  function getByCategory(category: string, forceSubCategory?: string, subType?: string): SearchEntry[] {
    return searchIndex.value.filter(e => {
      if (e.category !== category) return false
      if (forceSubCategory && e.sub_category !== forceSubCategory) return false
      if (subType && e.sub_type !== subType) return false
      return true
    })
  }

  /** 获取某分类下所有可用的 sub_type 值（去重排序） */
  function getSubTypes(category: string, forceSubCategory?: string): string[] {
    const types = new Set<string>()
    searchIndex.value.forEach(e => {
      if (e.category !== category) return
      if (forceSubCategory && e.sub_category !== forceSubCategory) return
      if (e.sub_type) types.add(e.sub_type)
    })
    return Array.from(types).sort()
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
    getSubTypes,
  }
})
