import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { LANGUAGES, type Language } from '@/types'

const DATA_BASE = import.meta.env.BASE_URL + 'data'

export const useLangStore = defineStore('lang', () => {
  const currentLang = ref(localStorage.getItem('wf-lang') ?? 'zh')
  const dicts = ref<Record<string, Record<string, string>>>({})
  const loading = ref(false)

  const language = computed<Language>(
    () => LANGUAGES.find(l => l.code === currentLang.value) ?? LANGUAGES[0],
  )

  async function loadDict(lang: string) {
    if (dicts.value[lang]) return
    loading.value = true
    try {
      const res = await fetch(`${DATA_BASE}/dict/${lang}.json`)
      dicts.value[lang] = await res.json() as Record<string, string>
    } finally {
      loading.value = false
    }
  }

  async function setLang(lang: string) {
    currentLang.value = lang
    localStorage.setItem('wf-lang', lang)
    await loadDict(lang)
  }

  function t(key?: string | null): string {
    if (!key) return ''
    return dicts.value[currentLang.value]?.[key]
      ?? dicts.value['en']?.[key]
      ?? ''
  }

  async function init() {
    await loadDict(currentLang.value)
    if (currentLang.value !== 'en') loadDict('en')
  }

  return { currentLang, language, loading, setLang, t, init }
})
