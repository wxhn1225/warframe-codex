<script setup lang="ts">
import { ref } from 'vue'
import { useLangStore } from '@/stores/lang'
import { LANGUAGES } from '@/types'

const langStore = useLangStore()
const showLangMenu = ref(false)

function selectLang(code: string) {
  langStore.setLang(code)
  showLangMenu.value = false
}
</script>

<template>
  <header
    class="sticky top-0 z-50 flex items-center justify-between px-6 h-13"
    style="background: var(--color-surface); border-bottom: 1px solid var(--color-border); box-shadow: 0 1px 3px rgba(0,0,0,0.04);"
  >
    <!-- Logo -->
    <RouterLink
      to="/"
      class="flex items-center gap-1.5 no-underline"
      style="text-decoration: none;"
    >
      <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none">
        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" stroke="var(--color-primary)" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span class="font-bold text-base" style="color: var(--color-text); letter-spacing: 0.04em;">
        Warframe <span style="color: var(--color-primary);">Codex</span>
      </span>
    </RouterLink>

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
