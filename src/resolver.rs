use std::collections::HashMap;

use anyhow::Result;
use dashmap::DashMap;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::{Map, Value};
use tracing::{info, warn};

use crate::scanner::RawNode;

/// 合并后的完整节点：包含继承链上所有 data 字段的合并结果
#[derive(Debug, Clone)]
pub struct ResolvedNode {
    pub path: String,
    pub parent_path: Option<String>,
    pub file_type: String,
    /// 文件自身的 data（未合并）
    pub own_data: Option<Value>,
    /// 沿继承链合并后的完整 data（子覆盖父）
    pub merged_data: Value,
}

/// 递归合并两个 JSON 对象，child 的字段覆盖 parent 的字段
fn merge_objects(parent: &Value, child: &Value) -> Value {
    match (parent, child) {
        (Value::Object(p), Value::Object(c)) => {
            let mut result: Map<String, Value> = p.clone();
            for (k, v) in c {
                result.insert(k.clone(), v.clone());
            }
            Value::Object(result)
        }
        // 如果类型不匹配，child 直接覆盖 parent
        (_, c) => c.clone(),
    }
}

/// 对整个节点图进行继承链解析
/// 采用记忆化递归：解析每个节点时沿 parent 链向上，合并所有 data
pub fn resolve_all(raw_map: &DashMap<String, RawNode>) -> Result<HashMap<String, ResolvedNode>> {
    info!("开始继承链解析，共 {} 个节点...", raw_map.len());

    // 记忆化缓存：已解析的路径 → 合并后的 merged_data
    let cache: DashMap<String, Value> = DashMap::new();

    let pb = ProgressBar::new(raw_map.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50.green/white} {pos}/{len} {msg}")
            .unwrap(),
    );

    // 用迭代而非递归，避免深继承链导致栈溢出
    // 对每个节点：收集从当前节点到根节点的完整链，然后从根向下合并
    let get_merged = |start_path: &str| -> Value {
        if let Some(cached) = cache.get(start_path) {
            return cached.clone();
        }

        // 收集继承链（从当前节点向上到根）
        let mut chain: Vec<String> = Vec::new();
        let mut current = start_path.to_string();
        let mut visited = std::collections::HashSet::new();

        loop {
            if visited.contains(&current) {
                warn!("检测到循环继承: {}", current);
                break;
            }
            visited.insert(current.clone());
            chain.push(current.clone());

            match raw_map.get(&current) {
                Some(node) => match &node.parent {
                    Some(p) => current = p.clone(),
                    None => break,
                },
                None => break,
            }
        }

        // 从根节点向下合并（chain 是从子到根，所以反转）
        chain.reverse();
        let mut merged = Value::Object(serde_json::Map::new());
        for path in &chain {
            if let Some(node) = raw_map.get(path) {
                if let Some(data) = &node.own_data {
                    merged = merge_objects(&merged, data);
                }
            }
        }

        merged
    };

    let mut result: HashMap<String, ResolvedNode> = HashMap::with_capacity(raw_map.len());

    for entry in raw_map.iter() {
        pb.inc(1);
        let node = entry.value();
        let merged = get_merged(&node.path);

        // 写入缓存
        cache.insert(node.path.clone(), merged.clone());

        result.insert(
            node.path.clone(),
            ResolvedNode {
                path: node.path.clone(),
                parent_path: node.parent.clone(),
                file_type: node.file_type.clone(),
                own_data: node.own_data.clone(),
                merged_data: merged,
            },
        );
    }

    pb.finish_with_message("继承链解析完成");
    info!("解析完成，共 {} 个节点", result.len());
    Ok(result)
}
