<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useLangStore } from '@/stores/lang'
import { LANGUAGES } from '@/types'

const router = useRouter()
const langStore = useLangStore()
const showLangMenu = ref(false)

let searchQ = ''
function onSearch(e: Event) {
  searchQ = (e.target as HTMLInputElement).value
}
function submitSearch(e: Event) {
  e.preventDefault()
  if (searchQ.trim()) {
    router.push({ name: 'home', query: { q: searchQ.trim() } })
  }
}
function selectLang(code: string) {
  langStore.setLang(code)
  showLangMenu.value = false
}
</script>

<template>
  <header
    class="sticky top-0 z-50 flex items-center gap-4 px-6 h-14"
    style="background: var(--color-surface); border-bottom: 1px solid var(--color-border); box-shadow: 0 1px 3px rgba(0,0,0,0.04);"
  >
    <!-- Logo -->
    <RouterLink
      to="/"
      class="flex items-center gap-1.5 shrink-0 no-underline"
      style="text-decoration: none;"
    >
      <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none">
        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" stroke="var(--color-primary)" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span class="font-bold text-base tracking-wide" style="color: var(--color-text); letter-spacing: 0.04em;">
        Warframe <span style="color: var(--color-primary);">Codex</span>
      </span>
    </RouterLink>

    <!-- 搜索栏 -->
    <form class="flex-1 max-w-xl mx-auto" @submit="submitSearch">
      <div class="relative">
        <svg
          class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 pointer-events-none"
          style="color: var(--color-text-subtle);"
          fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
        >
          <circle cx="11" cy="11" r="8"/><path stroke-linecap="round" d="M21 21l-4.35-4.35"/>
        </svg>
        <input
          class="input-search text-sm"
          style="padding: 0.45rem 0.85rem 0.45rem 2.25rem;"
          placeholder="搜索武器、战甲、敌人、Mod..."
          @input="onSearch"
          @keydown.enter="submitSearch"
        />
      </div>
    </form>

    <!-- 语言切换 -->
    <div class="relative shrink-0">
      <button
        class="flex items-center gap-1 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors"
        style="background: var(--color-surface-2); color: var(--color-text-muted); border: 1px solid var(--color-border);"
        @click="showLangMenu = !showLangMenu"
      >
        {{ langStore.language.nativeName }}
        <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      <Transition name="fade">
        <div
          v-if="showLangMenu"
          class="absolute right-0 top-full mt-1.5 py-1 rounded-xl z-50"
          style="background: var(--color-surface); border: 1px solid var(--color-border); min-width: 160px; box-shadow: 0 6px 20px rgba(0,0,0,0.08);"
        >
          <button
            v-for="lang in LANGUAGES"
            :key="lang.code"
            class="w-full text-left px-4 py-1.5 text-sm transition-colors hover:bg-slate-50"
            :style="{
              color: lang.code === langStore.currentLang ? 'var(--color-primary)' : 'var(--color-text)',
              fontWeight: lang.code === langStore.currentLang ? '600' : '400',
            }"
            @click="selectLang(lang.code)"
          >
            {{ lang.nativeName }}
          </button>
        </div>
      </Transition>
    </div>
  </header>

  <div v-if="showLangMenu" class="fixed inset-0 z-40" @click="showLangMenu = false" />
</template>
