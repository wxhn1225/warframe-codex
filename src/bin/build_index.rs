use anyhow::Result;
use tracing::info;
use warframe_codex::{
    config::Config,
    db,
    exporter,
    localizer,
    relations::{extract_relations_from_export, extract_relations_from_node},
    resolver,
    scanner,
};

fn main() -> Result<()> {
    // 初始化日志输出
    tracing_subscriber::fmt()
        .with_env_filter("warframe_codex=info")
        .init();

    info!("=== Warframe Codex 索引构建器 ===");

    let config = Config::from_env_or_default();
    config.validate()?;

    info!("原始数据目录: {:?}", config.raw_data_root);
    info!("Export 目录:  {:?}", config.export_root);
    info!("输出数据库:   {:?}", config.db_path);

    // 1. 打开/创建数据库
    let conn = db::open(&config.db_path)?;
    db::insert_languages(&conn)?;

    // 2. 扫描 44 万个引擎文件
    let raw_map = scanner::scan_all(&config.raw_data_root)?;

    // 3. 解析继承链
    let resolved_map = resolver::resolve_all(&raw_map)?;

    // 4. 从所有节点提取关系图
    info!("提取引擎文件关系图...");
    let mut all_relations: Vec<warframe_codex::relations::Relation> = resolved_map
        .values()
        .flat_map(|node| {
            extract_relations_from_node(
                &node.path,
                node.parent_path.as_deref(),
                node.own_data.as_ref(),
            )
        })
        .collect();
    info!("引擎文件关系: {} 条", all_relations.len());

    // 5. 写入实体表
    db::insert_entities(&conn, &resolved_map)?;

    // 6. 解析所有 Export*.json
    let export_entries = exporter::parse_all_exports(&config.export_root)?;

    // 7. 从 Export 提取关系
    let export_relations: Vec<_> = export_entries
        .iter()
        .flat_map(|e| extract_relations_from_export(&e.path, &e.data))
        .collect();
    info!("Export 关系: {} 条", export_relations.len());
    all_relations.extend(export_relations);

    // 8. 写入 Export 条目
    db::insert_export_entries(&conn, &export_entries)?;

    // 9. 写入全部关系
    db::insert_relations(&conn, &all_relations)?;

    // 10. 加载并写入 15 种语言字典
    let all_dicts = localizer::load_all_dicts(&config.export_root)?;
    db::insert_localizations(&conn, &all_dicts)?;

    info!("=== 构建完成！数据库: {:?} ===", config.db_path);
    print_stats(&conn)?;

    Ok(())
}

fn print_stats(conn: &rusqlite::Connection) -> Result<()> {
    let entity_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM entities", [], |r| r.get(0))?;
    let export_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM export_entries", [], |r| r.get(0))?;
    let relation_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM relations", [], |r| r.get(0))?;
    let loc_count: i64 =
        conn.query_row("SELECT COUNT(*) FROM localizations", [], |r| r.get(0))?;

    info!("┌─────────────────────────────┐");
    info!("│         数据库统计          │");
    info!("├─────────────────────────────┤");
    info!("│ 引擎实体      {:>10} 条 │", entity_count);
    info!("│ Export 条目   {:>10} 条 │", export_count);
    info!("│ 关系图        {:>10} 条 │", relation_count);
    info!("│ 本地化文本    {:>10} 条 │", loc_count);
    info!("└─────────────────────────────┘");

    Ok(())
}
