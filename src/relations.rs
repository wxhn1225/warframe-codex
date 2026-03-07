use serde_json::Value;

/// 两个游戏路径之间的引用关系
#[derive(Debug, Clone)]
pub struct Relation {
    pub from_path: String,
    pub rel_type: String,
    pub to_path: String,
}

/// 判断一个字符串是否像游戏内路径
fn is_game_path(s: &str) -> bool {
    (s.starts_with("/Lotus/") || s.starts_with("/EE/") || s.starts_with("/DS/"))
        && !s.starts_with("/Lotus/Language/")
}

/// 从 JSON Value 中递归提取所有游戏路径引用
fn extract_paths_from_value(value: &Value, field_hint: &str, results: &mut Vec<(String, String)>) {
    match value {
        Value::String(s) if is_game_path(s) => {
            results.push((field_hint.to_string(), s.clone()));
        }
        Value::Object(obj) => {
            for (k, v) in obj {
                extract_paths_from_value(v, k, results);
            }
        }
        Value::Array(arr) => {
            for item in arr {
                extract_paths_from_value(item, field_hint, results);
            }
        }
        _ => {}
    }
}

/// 从一个节点的 own_data 中提取所有关系
/// 使用字段名作为 rel_type（parent / avatarTypes / damageController / ingredients 等）
pub fn extract_relations_from_node(
    from_path: &str,
    parent_path: Option<&str>,
    own_data: Option<&Value>,
) -> Vec<Relation> {
    let mut relations = Vec::new();

    // parent 关系
    if let Some(p) = parent_path {
        if is_game_path(p) {
            relations.push(Relation {
                from_path: from_path.to_string(),
                rel_type: "parent".to_string(),
                to_path: p.to_string(),
            });
        }
    }

    // own_data 中的所有路径引用
    if let Some(data) = own_data {
        let mut found: Vec<(String, String)> = Vec::new();
        extract_paths_from_value(data, "ref", &mut found);
        for (rel_type, to_path) in found {
            if to_path != from_path {
                relations.push(Relation {
                    from_path: from_path.to_string(),
                    rel_type,
                    to_path,
                });
            }
        }
    }

    relations
}

/// 从 Export 条目数据中提取关系
pub fn extract_relations_from_export(from_path: &str, data: &Value) -> Vec<Relation> {
    let mut relations = Vec::new();
    let mut found: Vec<(String, String)> = Vec::new();
    extract_paths_from_value(data, "ref", &mut found);
    for (rel_type, to_path) in found {
        if to_path != from_path {
            relations.push(Relation {
                from_path: from_path.to_string(),
                rel_type,
                to_path,
            });
        }
    }
    relations
}
