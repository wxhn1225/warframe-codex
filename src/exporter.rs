use std::path::Path;

use anyhow::Result;
use rayon::prelude::*;
use serde_json::Value;
use tracing::{info, warn};

/// 一条 Export 条目
#[derive(Debug, Clone)]
pub struct ExportEntry {
    /// 游戏内路径，如 /Lotus/Types/Enemies/.../ArachnoidCamperAvatar
    pub path: String,
    /// 来自哪个 Export 文件，如 "ExportEnemies"
    pub export_file: String,
    /// 在该文件中的分类，如 "avatars" / "agents" / "weapons"
    pub category: String,
    /// name 字段的语言 key（/Lotus/Language/...），若有
    pub name_key: Option<String>,
    /// description 字段的语言 key，若有
    pub desc_key: Option<String>,
    /// 完整的 Export 数据
    pub data: Value,
}

/// 收集 JSON 对象中所有值为 /Lotus/Language/... 的字段
fn extract_lang_key(obj: &Value, field: &str) -> Option<String> {
    obj.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| s.starts_with("/Lotus/Language/"))
        .map(|s| s.to_string())
}

/// 解析单个 Export*.json 文件，返回所有条目
fn parse_export_file(path: &Path) -> Result<Vec<ExportEntry>> {
    let file_stem = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let content = std::fs::read_to_string(path)?;
    let root: Value = serde_json::from_str(&content)?;

    let mut entries = Vec::new();

    let obj = match root.as_object() {
        Some(o) => o,
        None => return Ok(entries),
    };

    for (cat_or_path, cat_value) in obj {
        // ExportEnemies 有嵌套分类（agents / avatars / damageControllers）
        // 其他大多数文件是直接 path → data
        if let Some(inner_obj) = cat_value.as_object() {
            // 判断：如果第一个 key 以 "/" 开头，说明是 path → data 的直接结构
            let first_key = inner_obj.keys().next().map(|s| s.as_str()).unwrap_or("");
            if first_key.starts_with('/') {
                // 嵌套分类（如 ExportEnemies.agents）
                for (item_path, item_data) in inner_obj {
                    entries.push(ExportEntry {
                        path: item_path.clone(),
                        export_file: file_stem.clone(),
                        category: cat_or_path.clone(),
                        name_key: extract_lang_key(item_data, "name"),
                        desc_key: extract_lang_key(item_data, "description"),
                        data: item_data.clone(),
                    });
                }
            } else {
                // 直接 path → data 结构（大多数 Export 文件）
                // cat_or_path 就是 item path
                entries.push(ExportEntry {
                    path: cat_or_path.clone(),
                    export_file: file_stem.clone(),
                    category: "default".to_string(),
                    name_key: extract_lang_key(cat_value, "name"),
                    desc_key: extract_lang_key(cat_value, "description"),
                    data: cat_value.clone(),
                });
            }
        }
    }

    Ok(entries)
}

/// 并行解析所有 Export*.json 文件
pub fn parse_all_exports(export_root: &Path) -> Result<Vec<ExportEntry>> {
    let export_files: Vec<_> = std::fs::read_dir(export_root)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("Export") && n.ends_with(".json"))
                .unwrap_or(false)
        })
        .collect();

    info!("找到 {} 个 Export 文件，开始并行解析...", export_files.len());

    let all_entries: Vec<ExportEntry> = export_files
        .par_iter()
        .flat_map(|path| match parse_export_file(path) {
            Ok(entries) => {
                info!(
                    "  {} → {} 条",
                    path.file_name().unwrap_or_default().to_string_lossy(),
                    entries.len()
                );
                entries
            }
            Err(e) => {
                warn!("解析失败 {:?}: {}", path, e);
                vec![]
            }
        })
        .collect();

    info!("Export 解析完成，共 {} 条条目", all_entries.len());
    Ok(all_entries)
}
