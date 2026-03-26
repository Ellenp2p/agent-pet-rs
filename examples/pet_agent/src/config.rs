//! 配置管理模块
//!
//! 管理 API Key、模型、记忆路径等配置。

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// OpenRouter API Key
    pub api_key: String,
    /// AI 模型
    pub model: String,
    /// 记忆文件路径
    pub memory_path: PathBuf,
    /// 窗口大小
    pub window_size: WindowSize,
    /// 小狗大小
    pub dog_size: DogSize,
    /// 动画速度 (毫秒)
    pub animation_speed: u64,
}

/// 窗口大小
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: u16,
    pub height: u16,
}

/// 小狗大小
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DogSize {
    /// 小 (5 行)
    Small,
    /// 中 (7 行)
    Medium,
    /// 大 (10 行)
    Large,
}

impl DogSize {
    pub fn lines(&self) -> usize {
        match self {
            DogSize::Small => 5,
            DogSize::Medium => 7,
            DogSize::Large => 10,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        Self {
            api_key: String::new(),
            model: "google/gemma-3-27b-it:free".to_string(),
            memory_path: home.join(".pet_agent").join("memory.json"),
            window_size: WindowSize {
                width: 80,
                height: 24,
            },
            dog_size: DogSize::Medium,
            animation_speed: 200,
        }
    }
}

impl Config {
    /// 获取配置文件路径
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_default()
            .join("pet_agent")
            .join("config.toml")
    }

    /// 加载配置
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let config: Self = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// 保存配置
    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    /// 检查是否需要配置
    pub fn needs_setup(&self) -> bool {
        self.api_key.is_empty()
    }
}
