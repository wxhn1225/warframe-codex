export interface SearchEntry {
  path: string
  name_zh?: string
  name_en?: string
  category: string
  sub_category: string
  icon?: string
}

export interface Language {
  code: string
  nativeName: string
  enName: string
}

export const LANGUAGES: Language[] = [
  { code: 'zh', nativeName: '简体中文', enName: 'Simplified Chinese' },
  { code: 'en', nativeName: 'English', enName: 'English' },
  { code: 'tc', nativeName: '繁體中文', enName: 'Traditional Chinese' },
  { code: 'ja', nativeName: '日本語', enName: 'Japanese' },
  { code: 'ko', nativeName: '한국어', enName: 'Korean' },
  { code: 'de', nativeName: 'Deutsch', enName: 'German' },
  { code: 'es', nativeName: 'Español', enName: 'Spanish' },
  { code: 'fr', nativeName: 'Français', enName: 'French' },
  { code: 'it', nativeName: 'Italiano', enName: 'Italian' },
  { code: 'pl', nativeName: 'Polski', enName: 'Polish' },
  { code: 'pt', nativeName: 'Português', enName: 'Portuguese' },
  { code: 'ru', nativeName: 'Русский', enName: 'Russian' },
  { code: 'tr', nativeName: 'Türkçe', enName: 'Turkish' },
  { code: 'uk', nativeName: 'Українська', enName: 'Ukrainian' },
  { code: 'th', nativeName: 'แบบไทย', enName: 'Thai' },
]

export interface Category {
  key: string
  label: string
  labelEn: string
  color: string
  subCategories?: string[]
}

export const CATEGORIES: Category[] = [
  { key: 'ExportWarframes', label: '战甲', labelEn: 'Warframes', color: '#00b4d8' },
  { key: 'ExportWeapons', label: '武器', labelEn: 'Weapons', color: '#ff6b35' },
  { key: 'ExportUpgrades', label: 'Mod', labelEn: 'Mods', color: '#7b2d8b' },
  { key: 'ExportEnemies', label: '敌人', labelEn: 'Enemies', color: '#e63946', subCategories: ['avatars'] },
  { key: 'ExportResources', label: '素材', labelEn: 'Resources', color: '#2d9c55' },
  { key: 'ExportRecipes', label: '配方', labelEn: 'Recipes', color: '#ffd60a' },
  { key: 'ExportRelics', label: '遗物', labelEn: 'Relics', color: '#8b5cf6' },
  { key: 'ExportCustoms', label: '外观', labelEn: 'Customization', color: '#ec4899' },
  { key: 'ExportArcanes', label: '奥义', labelEn: 'Arcanes', color: '#f59e0b' },
  { key: 'ExportSentinels', label: '哨兵', labelEn: 'Sentinels', color: '#64748b' },
  { key: 'ExportAbilities', label: '技能', labelEn: 'Abilities', color: '#06b6d4' },
  { key: 'ExportRegions', label: '任务', labelEn: 'Missions', color: '#84cc16' },
]

export const IMAGE_CDN = 'https://browse.wf'

export function getIconUrl(iconPath?: string): string {
  if (!iconPath) return ''
  return `${IMAGE_CDN}${iconPath}`
}
