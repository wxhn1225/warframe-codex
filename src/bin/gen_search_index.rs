/// 生成前端用的搜索索引 search-index.json
/// 格式: [{path, name_zh, name_en, category, sub_category, icon}]
/// 约 44k 条，压缩后约 1-2MB
use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct SearchEntry {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_zh: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_en: Option<String>,
    category: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    sub_category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
}

fn load_dict(lang_root: &PathBuf, lang: &str) -> HashMap<String, String> {
    let path = lang_root.join(format!("{}.json", lang));
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let val: Value = serde_json::from_str(&content).unwrap_or(Value::Null);
    val.as_object()
        .map(|o| {
            o.iter()
                .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn extract_lang_key(val: &Value, field: &str) -> Option<String> {
    val.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| s.starts_with("/Lotus/Language/"))
        .map(|s| s.to_string())
}

fn extract_icon(val: &Value) -> Option<String> {
    val.get("icon")
        .or_else(|| val.get("Icon"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn main() -> Result<()> {
    let export_root = std::env::var("WF_EXPORT_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(r"warframe-public-export-plus"));

    // 语言字典来自 warframe-languages-bin-data（文件名：zh.json / en.json）
    let lang_root = std::env::var("WF_LANG_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(r"warframe-languages-bin-data"));

    let out_path = std::env::var("WF_SEARCH_INDEX_OUT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("frontend/public/data/search-index.json"));

    eprintln!("加载语言字典...");
    let dict_zh = load_dict(&lang_root, "zh");
    let dict_en = load_dict(&lang_root, "en");

    let mut entries: Vec<SearchEntry> = Vec::with_capacity(50_000);

    let export_files: Vec<_> = std::fs::read_dir(&export_root)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("Export") && n.ends_with(".json"))
                .unwrap_or(false)
        })
        .collect();

    eprintln!("处理 {} 个 Export 文件...", export_files.len());

    for file_path in &export_files {
        let category = file_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let content = std::fs::read_to_string(file_path)?;
        let root: Value = serde_json::from_str(&content)?;
        let obj = match root.as_object() {
            Some(o) => o,
            None => continue,
        };

        for (cat_or_path, cat_value) in obj {
            if let Some(inner) = cat_value.as_object() {
                let first_key = inner.keys().next().map(|s| s.as_str()).unwrap_or("");
                if first_key.starts_with('/') {
                    // 嵌套分类（ExportEnemies）
                    for (item_path, item_data) in inner {
                        let name_key = extract_lang_key(item_data, "name");
                        entries.push(SearchEntry {
                            path: item_path.clone(),
                            name_zh: name_key.as_ref().and_then(|k| dict_zh.get(k)).cloned(),
                            name_en: name_key.as_ref().and_then(|k| dict_en.get(k)).cloned(),
                            category: category.clone(),
                            sub_category: cat_or_path.clone(),
                            icon: extract_icon(item_data),
                        });
                    }
                } else {
                    // 直接 path → data
                    let name_key = extract_lang_key(cat_value, "name");
                    entries.push(SearchEntry {
                        path: cat_or_path.clone(),
                        name_zh: name_key.as_ref().and_then(|k| dict_zh.get(k)).cloned(),
                        name_en: name_key.as_ref().and_then(|k| dict_en.get(k)).cloned(),
                        category: category.clone(),
                        sub_category: String::new(),
                        icon: extract_icon(cat_value),
                    });
                }
            }
        }
    }

    eprintln!("生成 {} 条索引，写入 {:?}...", entries.len(), out_path);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string(&entries)?;
    std::fs::write(&out_path, json)?;
    eprintln!("完成！文件大小: {} KB", std::fs::metadata(&out_path)?.len() / 1024);
    Ok(())
}
