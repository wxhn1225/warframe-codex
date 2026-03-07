export interface SearchEntry {
  path: string
  name_zh?: string
  name_en?: string
  category: string
  sub_category: string
  /** 细分类型：遗物的era、敌人的faction、武器的productCategory */
  sub_type?: string
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
  /** 默认只展示的 sub_category（例如 ExportEnemies 只展示 avatars） */
  defaultSubCategory?: string
  /** sub_type 的中文显示名（key=sub_type值, value=显示名） */
  subTypeLabels?: Record<string, string>
}

export const CATEGORIES: Category[] = [
  { key: 'ExportWarframes', label: '战甲', labelEn: 'Warframes', color: '#00b4d8' },
  {
    key: 'ExportWeapons',
    label: '武器',
    labelEn: 'Weapons',
    color: '#ff6b35',
    subTypeLabels: {
      LongGuns: '主武器',
      Pistols: '副武器',
      Melee: '近战',
      SpaceGuns: '惰空主武',
      SpaceMelee: '惰空近战',
      OperatorAmps: '聚能灯',
      SentinelWeapons: '哨兵武器',
      DrifterMelee: '漂流者近战',
      SpecialItems: '特殊',
    },
  },
  { key: 'ExportUpgrades', label: 'Mod', labelEn: 'Mods', color: '#7b2d8b' },
  {
    key: 'ExportEnemies',
    label: '敌人',
    labelEn: 'Enemies',
    color: '#e63946',
    defaultSubCategory: 'avatars',
    subTypeLabels: {
      Corpus: '星际公司',
      Grineer: '星裂帝国',
      Infestation: '感染体',
      Infested: '感染体',
      Sentient: '感知体',
      Orokin: '黄金时代',
      OrokinEmpire: '黄金时代',
      'Orokin Empire': '黄金时代',
      Narmer: '纳尔玛',
      NarmerVeil: '纳尔玛遮幕',
      Stalker: '追猎者',
      Duviri: '杜维里',
      Scaldra: 'Scaldra',
      Techrot: 'Techrot',
      Anarch: 'Anarch',
      TENNO: '星际战士',
      Tenno: '星际战士',
      MITW: 'MITW',
      ENEMY: '通用敌人',
      Neutral: '中立',
      None: '无',
      Dummy: '测试',
      Prey: '猎物',
      'Red Veil': '赤幕',
      Special: '特殊',
    },
  },
  { key: 'ExportResources', label: '资源', labelEn: 'Resources', color: '#2d9c55' },
  { key: 'ExportRecipes', label: '设计图', labelEn: 'Blueprints', color: '#ffd60a' },
  {
    key: 'ExportRelics',
    label: '遗物',
    labelEn: 'Relics',
    color: '#8b5cf6',
    subTypeLabels: {
      Lith: '铸造',
      Meso: '中子',
      Neo: '新纪',
      Axi: '轴心',
      Requiem: '安魂',
      Vanguard: '前锋',
    },
  },
  { key: 'ExportCustoms', label: '外观', labelEn: 'Customization', color: '#ec4899' },
  { key: 'ExportArcanes', label: '赋能', labelEn: 'Arcanes', color: '#f59e0b' },
  { key: 'ExportSentinels', label: '同伴', labelEn: 'Companions', color: '#64748b' },
  { key: 'ExportAbilities', label: '技能', labelEn: 'Abilities', color: '#06b6d4' },
  { key: 'ExportRegions', label: '任务', labelEn: 'Missions', color: '#84cc16' },
]

export const IMAGE_CDN = 'https://browse.wf'

export function getIconUrl(iconPath?: string): string {
  if (!iconPath) return ''
  return `${IMAGE_CDN}${iconPath}`
}

/** warframe-packages-bin-data 原始文件的 GitHub raw 地址 */
export const RAW_DATA_BASE = 'https://raw.githubusercontent.com/Sainan/warframe-packages-bin-data/senpai'

export function getRawFileUrl(gamePath: string): string {
  // gamePath 格式: /Lotus/Types/... → 去掉开头 /，加 .json
  const clean = gamePath.startsWith('/') ? gamePath.slice(1) : gamePath
  return `${RAW_DATA_BASE}/${clean}.json`
}
