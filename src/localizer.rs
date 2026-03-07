use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use rayon::prelude::*;
use serde_json::Value;
use tracing::info;

pub const LANG_CODES: &[&str] = &[
    "en", "de", "es", "fr", "it", "ja", "ko", "pl", "pt", "ru", "tr", "uk", "zh", "tc", "th",
];

/// 单种语言的字典：lang_key → 翻译文本
pub type LangDict = HashMap<String, String>;

/// 所有语言的字典集合
pub type AllDicts = HashMap<String, LangDict>;

/// 并行加载所有语言字典
/// lang_root 指向 warframe-languages-bin-data 目录（文件名格式：zh.json / en.json）
pub fn load_all_dicts(lang_root: &Path) -> Result<AllDicts> {
    info!("加载 {} 种语言字典...", LANG_CODES.len());

    let dicts: Vec<(String, LangDict)> = LANG_CODES
        .par_iter()
        .filter_map(|&code| {
            let path = lang_root.join(format!("{}.json", code));
            if !path.exists() {
                return None;
            }
            let content = std::fs::read_to_string(&path).ok()?;
            let raw: Value = serde_json::from_str(&content).ok()?;
            let obj = raw.as_object()?;
            let dict: LangDict = obj
                .iter()
                .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
                .collect();
            info!("  {} ({} 条)", code, dict.len());
            Some((code.to_string(), dict))
        })
        .collect();

    let all: AllDicts = dicts.into_iter().collect();
    info!("语言字典加载完成，共 {} 种语言", all.len());
    Ok(all)
}

/// 用指定语言查询翻译，找不到时回退到英文，再找不到返回 key 本身
pub fn lookup<'a>(dicts: &'a AllDicts, lang: &str, key: &str) -> Option<&'a str> {
    dicts
        .get(lang)
        .and_then(|d| d.get(key))
        .map(|s| s.as_str())
        .or_else(|| {
            // 回退到英文
            dicts
                .get("en")
                .and_then(|d| d.get(key))
                .map(|s| s.as_str())
        })
}
