<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useLangStore } from '@/stores/lang'
import { LANGUAGES } from '@/types'

const router = useRouter()
const langStore = useLangStore()
const showLangMenu = ref(false)

function selectLang(code: string) {
  langStore.setLang(code)
  showLangMenu.value = false
}

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
</script>

<template>
  <header
    class="sticky top-0 z-50 flex items-center gap-4 px-6 h-14"
    style="background: rgba(10,22,40,0.92); backdrop-filter: blur(12px); border-bottom: 1px solid var(--color-border);"
  >
    <!-- Logo -->
    <RouterLink
      to="/"
      class="flex items-center gap-2 shrink-0 text-white font-bold text-lg tracking-wide no-underline"
      style="letter-spacing: 0.05em;"
    >
      <span style="color: var(--color-primary);">WARFRAME</span>
      <span style="color: var(--color-gold);">CODEX</span>
    </RouterLink>

    <!-- 搜索栏（居中） -->
    <form class="flex-1 max-w-xl mx-auto" @submit="submitSearch">
      <input
        class="input-search text-sm"
        placeholder="搜索武器、战甲、敌人、Mod..."
        style="padding: 0.4rem 0.85rem;"
        @input="onSearch"
      />
    </form>

    <!-- 语言切换 -->
    <div class="relative shrink-0">
      <button
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-colors"
        style="background: var(--color-surface-3); color: var(--color-text-muted);"
        @click="showLangMenu = !showLangMenu"
      >
        {{ langStore.language.nativeName }}
        <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
          <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      <Transition name="fade">
        <div
          v-if="showLangMenu"
          class="absolute right-0 top-full mt-1 py-1 rounded-xl shadow-xl z-50"
          style="background: var(--color-surface-2); border: 1px solid var(--color-border); min-width: 160px;"
        >
          <button
            v-for="lang in LANGUAGES"
            :key="lang.code"
            class="w-full text-left px-4 py-2 text-sm transition-colors"
            :style="{
              color: lang.code === langStore.currentLang ? 'var(--color-primary)' : 'var(--color-text)',
              background: lang.code === langStore.currentLang ? 'rgba(0,180,216,0.08)' : 'transparent'
            }"
            @click="selectLang(lang.code)"
          >
            {{ lang.nativeName }}
          </button>
        </div>
      </Transition>
    </div>
  </header>

  <!-- 点击外部关闭语言菜单 -->
  <div
    v-if="showLangMenu"
    class="fixed inset-0 z-40"
    @click="showLangMenu = false"
  />
</template>
