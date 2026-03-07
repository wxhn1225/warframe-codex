use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use rusqlite::{params, Connection};
use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;

use crate::exporter::ExportEntry;
use crate::localizer::AllDicts;
use crate::relations::Relation;
use crate::resolver::ResolvedNode;

/// 建立所有表结构
pub fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA cache_size = -64000;
        PRAGMA temp_store = MEMORY;

        CREATE TABLE IF NOT EXISTS languages (
            code        TEXT PRIMARY KEY,
            native_name TEXT NOT NULL,
            en_name     TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS localizations (
            lang_code   TEXT NOT NULL,
            lang_key    TEXT NOT NULL,
            text        TEXT NOT NULL,
            PRIMARY KEY (lang_code, lang_key)
        );

        CREATE TABLE IF NOT EXISTS entities (
            path         TEXT PRIMARY KEY,
            parent_path  TEXT,
            file_type    TEXT NOT NULL,
            own_data     TEXT,
            merged_data  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS export_entries (
            path         TEXT PRIMARY KEY,
            export_file  TEXT NOT NULL,
            category     TEXT NOT NULL,
            name_key     TEXT,
            desc_key     TEXT,
            export_data  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS relations (
            from_path    TEXT NOT NULL,
            rel_type     TEXT NOT NULL,
            to_path      TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_relations_from ON relations(from_path);
        CREATE INDEX IF NOT EXISTS idx_relations_to   ON relations(to_path);
        CREATE INDEX IF NOT EXISTS idx_loc_lang       ON localizations(lang_code, lang_key);
        CREATE INDEX IF NOT EXISTS idx_export_file    ON export_entries(export_file);
        CREATE INDEX IF NOT EXISTS idx_entity_type    ON entities(file_type);
    ")?;
    Ok(())
}

/// 写入语言注册表
pub fn insert_languages(conn: &Connection) -> Result<()> {
    let languages = [
        ("en", "English", "English"),
        ("de", "Deutsch", "German"),
        ("es", "Español", "Spanish"),
        ("fr", "Français", "French"),
        ("it", "Italiano", "Italian"),
        ("ja", "日本語", "Japanese"),
        ("ko", "한국어", "Korean"),
        ("pl", "Polski", "Polish"),
        ("pt", "Português", "Portuguese"),
        ("ru", "Русский", "Russian"),
        ("tr", "Türkçe", "Turkish"),
        ("uk", "Українська", "Ukrainian"),
        ("zh", "简体中文", "Simplified Chinese"),
        ("tc", "繁體中文", "Traditional Chinese"),
        ("th", "แบบไทย", "Thai"),
    ];
    let tx = conn.unchecked_transaction()?;
    for (code, native, en) in &languages {
        tx.execute(
            "INSERT OR REPLACE INTO languages VALUES (?, ?, ?)",
            params![code, native, en],
        )?;
    }
    tx.commit()?;
    info!("语言表写入完成");
    Ok(())
}

/// 批量写入本地化字典（15种语言 × 34028 条）
pub fn insert_localizations(conn: &Connection, all_dicts: &AllDicts) -> Result<()> {
    let total: usize = all_dicts.values().map(|d| d.len()).sum();
    info!("写入 {} 条本地化记录...", total);

    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.yellow/white} {pos}/{len} localizations")
            .unwrap(),
    );

    let tx = conn.unchecked_transaction()?;
    let mut stmt = tx.prepare_cached(
        "INSERT OR REPLACE INTO localizations (lang_code, lang_key, text) VALUES (?, ?, ?)",
    )?;

    for (lang_code, dict) in all_dicts {
        for (key, text) in dict {
            stmt.execute(params![lang_code, key, text])?;
            pb.inc(1);
        }
    }
    drop(stmt);
    tx.commit()?;
    pb.finish_with_message("本地化写入完成");
    Ok(())
}

/// 批量写入实体（引擎原始节点）
pub fn insert_entities(conn: &Connection, nodes: &HashMap<String, ResolvedNode>) -> Result<()> {
    info!("写入 {} 个实体...", nodes.len());

    let pb = ProgressBar::new(nodes.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/white} {pos}/{len} entities")
            .unwrap(),
    );

    let tx = conn.unchecked_transaction()?;
    let mut stmt = tx.prepare_cached(
        "INSERT OR REPLACE INTO entities (path, parent_path, file_type, own_data, merged_data)
         VALUES (?, ?, ?, ?, ?)",
    )?;

    for node in nodes.values() {
        let own_data_str = node
            .own_data
            .as_ref()
            .map(|v| v.to_string())
            .unwrap_or_default();
        let merged_str = node.merged_data.to_string();

        stmt.execute(params![
            node.path,
            node.parent_path,
            node.file_type,
            if own_data_str.is_empty() { None } else { Some(own_data_str) },
            merged_str,
        ])?;
        pb.inc(1);
    }
    drop(stmt);
    tx.commit()?;
    pb.finish_with_message("实体写入完成");
    Ok(())
}

/// 批量写入 Export 条目
pub fn insert_export_entries(conn: &Connection, entries: &[ExportEntry]) -> Result<()> {
    info!("写入 {} 条 Export 条目...", entries.len());

    let pb = ProgressBar::new(entries.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.magenta/white} {pos}/{len} exports")
            .unwrap(),
    );

    let tx = conn.unchecked_transaction()?;
    let mut stmt = tx.prepare_cached(
        "INSERT OR REPLACE INTO export_entries
         (path, export_file, category, name_key, desc_key, export_data)
         VALUES (?, ?, ?, ?, ?, ?)",
    )?;

    for entry in entries {
        stmt.execute(params![
            entry.path,
            entry.export_file,
            entry.category,
            entry.name_key,
            entry.desc_key,
            entry.data.to_string(),
        ])?;
        pb.inc(1);
    }
    drop(stmt);
    tx.commit()?;
    pb.finish_with_message("Export 条目写入完成");
    Ok(())
}

/// 批量写入关系图
pub fn insert_relations(conn: &Connection, relations: &[Relation]) -> Result<()> {
    info!("写入 {} 条关系...", relations.len());

    let pb = ProgressBar::new(relations.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.green/white} {pos}/{len} relations")
            .unwrap(),
    );

    let tx = conn.unchecked_transaction()?;
    let mut stmt = tx.prepare_cached(
        "INSERT INTO relations (from_path, rel_type, to_path) VALUES (?, ?, ?)",
    )?;

    for r in relations {
        stmt.execute(params![r.from_path, r.rel_type, r.to_path])?;
        pb.inc(1);
    }
    drop(stmt);
    tx.commit()?;
    pb.finish_with_message("关系写入完成");
    Ok(())
}

/// 打开数据库连接并初始化 schema
pub fn open(db_path: &Path) -> Result<Connection> {
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(db_path)?;
    create_schema(&conn)?;
    Ok(conn)
}
