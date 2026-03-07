use std::path::PathBuf;

/// 整个项目的路径配置，运行前检查这些路径是否正确
pub struct Config {
    /// warframe-packages-bin-data 根目录
    pub raw_data_root: PathBuf,
    /// warframe-public-export-plus 目录
    pub export_root: PathBuf,
    /// 输出的 SQLite 数据库路径
    pub db_path: PathBuf,
}

impl Config {
    pub fn from_env_or_default() -> Self {
        let raw_data_root = std::env::var("WF_RAW_DATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(r"D:\xh\warframe-packages-bin-data"));

        let export_root = std::env::var("WF_EXPORT_DATA")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                raw_data_root.join("warframe-public-export-plus")
            });

        let db_path = std::env::var("WF_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(r"D:\xh\warframe-codex\warframe.db"));

        Self {
            raw_data_root,
            export_root,
            db_path,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if !self.raw_data_root.exists() {
            anyhow::bail!("raw_data_root 不存在: {:?}", self.raw_data_root);
        }
        if !self.export_root.exists() {
            anyhow::bail!("export_root 不存在: {:?}", self.export_root);
        }
        Ok(())
    }
}
