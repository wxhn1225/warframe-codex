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
  /**
   * sub_type值 → 语言键（从 warframe-languages-bin-data 运行时翻译）
   * 对应 /data/dict/zh.json 等语言文件中的 key
   */
  subTypeLangKeys?: Record<string, string>
  /**
   * sub_type值 → 回退显示名（无语言键时使用，或语言键查不到时）
   * 仅用于无对应语言键的分类（如武器 productCategory）
   */
  subTypeFallback?: Record<string, string>
}

export const CATEGORIES: Category[] = [
  { key: 'ExportWarframes', label: '战甲', labelEn: 'Warframes', color: '#00b4d8' },
  {
    key: 'ExportWeapons',
    label: '武器',
    labelEn: 'Weapons',
    color: '#ff6b35',
    // 武器 productCategory 语言键来自 warframe-languages-bin-data Menu/Loadout_*
    subTypeLangKeys: {
      LongGuns: '/Lotus/Language/Menu/Loadout_LongGun',       // 主要武器
      Pistols: '/Lotus/Language/Menu/Loadout_Pistol',         // 次要武器
      Melee: '/Lotus/Language/Menu/Loadout_Melee',            // 近战
      SpaceGuns: '/Lotus/Language/Menu/Loadout_SpaceGun',     // 曲翼枪械
      SpaceMelee: '/Lotus/Language/Menu/Loadout_SpaceMelee',  // 曲翼近战武器
      SentinelWeapons: '/Lotus/Language/Menu/Codex_SentinelWeapons', // 守护武器
    },
    subTypeFallback: {
      OperatorAmps: '指挥官增幅器',   // Menu/ItemInventory_SlotsOperatorWeapon 过长，取语义
      DrifterMelee: '漂泊者近战',
      SpecialItems: '特殊道具',
    },
  },
  { key: 'ExportUpgrades', label: 'Mod', labelEn: 'Mods', color: '#7b2d8b' },
  {
    key: 'ExportEnemies',
    label: '敌人',
    labelEn: 'Enemies',
    color: '#e63946',
    defaultSubCategory: 'avatars',
    // 派系名称来自 warframe-languages-bin-data，运行时从加载的语言字典查询
    subTypeLangKeys: {
      Corpus: '/Lotus/Language/Game/Faction_CorpusUC',
      Grineer: '/Lotus/Language/Game/Faction_GrineerUC',
      Infestation: '/Lotus/Language/Game/Faction_InfestationUC',
      Infested: '/Lotus/Language/Game/Faction_InfestationUC',
      Sentient: '/Lotus/Language/Game/Faction_SentientUC',
      Orokin: '/Lotus/Language/Game/Faction_OrokinUC',
      OrokinEmpire: '/Lotus/Language/Game/Faction_OrokinUC',
      'Orokin Empire': '/Lotus/Language/Game/Faction_OrokinUC',
      Narmer: '/Lotus/Language/Game/Faction_NarmerUC',
      NarmerVeil: '/Lotus/Language/Game/Faction_NarmerUC',
      MITW: '/Lotus/Language/Game/Faction_MITW',
      Scaldra: '/Lotus/Language/1999/Faction_Scaldra',
      Techrot: '/Lotus/Language/1999/Faction_Techrot',
      Duviri: '/Lotus/Language/Duviri/Duviri',
      'Red Veil': '/Lotus/Language/Game/Faction_RedVeilUC',
    },
  },
  { key: 'ExportResources', label: '资源', labelEn: 'Resources', color: '#2d9c55' },
  { key: 'ExportRecipes', label: '设计图', labelEn: 'Recipes', color: '#ffd60a' },
  {
    key: 'ExportRelics',
    label: '遗物',
    labelEn: 'Relics',
    color: '#8b5cf6',
    // 遗物时代名称来自 warframe-languages-bin-data /Lotus/Language/Relics/Era_*
    subTypeLangKeys: {
      Lith: '/Lotus/Language/Relics/Era_LITH',
      Meso: '/Lotus/Language/Relics/Era_MESO',
      Neo: '/Lotus/Language/Relics/Era_NEO',
      Axi: '/Lotus/Language/Relics/Era_AXI',
      Requiem: '/Lotus/Language/Items/RequiemRelicName',
    },
    subTypeFallback: {
      Requiem: 'Requiem',
      Vanguard: 'Vanguard',
    },
  },
  { key: 'ExportCustoms', label: '外观', labelEn: 'Customization', color: '#ec4899' },
  { key: 'ExportArcanes', label: '赋能', labelEn: 'Arcanes', color: '#f59e0b' },
  { key: 'ExportSentinels', label: '同伴', labelEn: 'Companions', color: '#64748b' },
  { key: 'ExportAbilities', label: '技能', labelEn: 'Abilities', color: '#06b6d4' },
  { key: 'ExportRegions', label: '任务', labelEn: 'Missions', color: '#84cc16' },
]

/**
 * 获取 sub_type 的显示名称。
 * 优先从 warframe-languages-bin-data 加载的语言字典中查找，
 * 其次使用 subTypeFallback，最后直接返回原始值。
 */
export function getSubTypeLabel(
  category: Category,
  subType: string,
  t: (key: string) => string,
): string {
  const langKey = category.subTypeLangKeys?.[subType]
  if (langKey) {
    const translated = t(langKey)
    if (translated) return translated
  }
  return category.subTypeFallback?.[subType] ?? subType
}

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
