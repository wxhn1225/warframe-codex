use std::path::{Path, PathBuf};

use anyhow::Result;
use dashmap::DashMap;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde_json::Value;
use tracing::{info, warn};
use walkdir::WalkDir;

/// 从原始文件解析出的节点
#[derive(Debug, Clone)]
pub struct RawNode {
    /// 对应的游戏内路径，如 /Lotus/Types/Enemies/.../ArachnoidCamperAgent
    pub path: String,
    /// 磁盘上的文件路径
    pub file_path: PathBuf,
    /// parent 字段（若有）
    pub parent: Option<String>,
    /// 文件自己的 data 字段（若有）
    pub own_data: Option<Value>,
    /// 根据文件名推断的类型，如 Avatar / Agent / DamageController
    pub file_type: String,
}

/// 把文件系统路径转换成游戏内路径
/// D:\xh\warframe-packages-bin-data\Lotus\Types\Enemies\...\ArachnoidCamperAgent.json
///   → /Lotus/Types/Enemies/.../ArachnoidCamperAgent
fn file_path_to_game_path(file_path: &Path, root: &Path) -> Option<String> {
    let rel = file_path.strip_prefix(root).ok()?;
    let without_ext = rel.with_extension("");
    let game_path = "/".to_string()
        + without_ext
            .to_string_lossy()
            .replace('\\', "/")
            .as_str();
    Some(game_path)
}

/// 从文件名推断实体类型
fn infer_file_type(file_name: &str) -> String {
    let name = file_name.to_lowercase();
    // 先匹配更具体的后缀
    if name.ends_with("damagecontroller") {
        "DamageController"
    } else if name.ends_with("agent") {
        "Agent"
    } else if name.ends_with("avatar") {
        "Avatar"
    } else if name.ends_with("suit") || name.ends_with("powersuit") {
        "Suit"
    } else if name.ends_with("weapon") || name.ends_with("weap") {
        "Weapon"
    } else if name.ends_with("ability") {
        "Ability"
    } else if name.ends_with("blueprint") {
        "Blueprint"
    } else if name.ends_with(".animfsm") || name.ends_with("animfsm") {
        "AnimFSM"
    } else if name.ends_with(".animtree") || name.ends_with("animtree") {
        "AnimTree"
    } else {
        "Other"
    }
    .to_string()
}

/// 并行扫描整个 raw data 目录，返回所有节点的 DashMap（path → RawNode）
pub fn scan_all(root: &Path) -> Result<DashMap<String, RawNode>> {
    info!("开始扫描原始数据目录: {:?}", root);

    // 先收集所有文件路径
    let all_files: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            // 跳过子模块目录和 .git
            if p.to_string_lossy().contains("warframe-public-export-plus")
                || p.to_string_lossy().contains(".git")
            {
                return false;
            }
            // 只要 .json 文件，跳过 .animfsm.json 等双扩展名文件中非 json 部分
            p.extension().map(|e| e == "json").unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    let total = all_files.len();
    info!("找到 {} 个 JSON 文件，开始并行解析...", total);

    let pb = ProgressBar::new(total as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos}/{len} {msg}")
            .unwrap(),
    );

    let map: DashMap<String, RawNode> = DashMap::with_capacity(total);

    all_files.par_iter().for_each(|file_path| {
        pb.inc(1);

        let game_path = match file_path_to_game_path(file_path, root) {
            Some(p) => p,
            None => return,
        };

        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                warn!("读取文件失败 {:?}: {}", file_path, e);
                return;
            }
        };

        let json: Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                warn!("解析 JSON 失败 {:?}: {}", file_path, e);
                return;
            }
        };

        let parent = json
            .get("parent")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let own_data = json.get("data").cloned();

        let stem = file_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // 处理双扩展名，如 FsmCamperRoot.animfsm.json → stem = FsmCamperRoot.animfsm
        let file_type = infer_file_type(&stem);

        let node = RawNode {
            path: game_path.clone(),
            file_path: file_path.clone(),
            parent,
            own_data,
            file_type,
        };

        map.insert(game_path, node);
    });

    pb.finish_with_message("扫描完成");
    info!("成功解析 {} 个节点", map.len());
    Ok(map)
}
