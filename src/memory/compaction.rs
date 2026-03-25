//! 记忆压缩模块
//!
//! 提供记忆压缩和持久化功能。

use super::memory_impl::MemoryEntry;
use crate::error::FrameworkError;
use serde::{Deserialize, Serialize};

/// 压缩策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompactionStrategy {
    /// 基于时间：删除超过指定时间的条目
    TimeBased { max_age_secs: u64 },
    /// 基于数量：只保留最新的 N 条
    CountBased { max_count: usize },
    /// 基于重要性：删除低于阈值的条目
    ImportanceBased { min_importance: f32 },
    /// 混合策略：结合时间和重要性
    Hybrid {
        max_age_secs: u64,
        min_importance: f32,
    },
}

impl Default for CompactionStrategy {
    fn default() -> Self {
        CompactionStrategy::Hybrid {
            max_age_secs: 86400, // 24 hours
            min_importance: 0.3,
        }
    }
}

/// 记忆压缩器
pub struct MemoryCompactor {
    /// 压缩策略
    strategy: CompactionStrategy,
}

impl MemoryCompactor {
    /// 创建新的记忆压缩器
    pub fn new(strategy: CompactionStrategy) -> Self {
        Self { strategy }
    }

    /// 使用默认策略创建
    pub fn default_strategy() -> Self {
        Self::new(CompactionStrategy::default())
    }

    /// 压缩记忆条目
    pub fn compact(&self, entries: Vec<MemoryEntry>, current_time: u64) -> Vec<MemoryEntry> {
        match &self.strategy {
            CompactionStrategy::TimeBased { max_age_secs } => {
                self.compact_by_time(entries, current_time, *max_age_secs)
            }
            CompactionStrategy::CountBased { max_count } => {
                self.compact_by_count(entries, *max_count)
            }
            CompactionStrategy::ImportanceBased { min_importance } => {
                self.compact_by_importance(entries, *min_importance)
            }
            CompactionStrategy::Hybrid {
                max_age_secs,
                min_importance,
            } => {
                let entries = self.compact_by_time(entries, current_time, *max_age_secs);
                self.compact_by_importance(entries, *min_importance)
            }
        }
    }

    /// 按时间压缩
    fn compact_by_time(
        &self,
        entries: Vec<MemoryEntry>,
        current_time: u64,
        max_age_secs: u64,
    ) -> Vec<MemoryEntry> {
        entries
            .into_iter()
            .filter(|e| current_time.saturating_sub(e.timestamp) <= max_age_secs)
            .collect()
    }

    /// 按数量压缩
    fn compact_by_count(
        &self,
        mut entries: Vec<MemoryEntry>,
        max_count: usize,
    ) -> Vec<MemoryEntry> {
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        entries.truncate(max_count);
        entries
    }

    /// 按重要性压缩
    fn compact_by_importance(
        &self,
        entries: Vec<MemoryEntry>,
        min_importance: f32,
    ) -> Vec<MemoryEntry> {
        entries
            .into_iter()
            .filter(|e| e.importance >= min_importance)
            .collect()
    }

    /// 获取压缩策略
    pub fn strategy(&self) -> &CompactionStrategy {
        &self.strategy
    }
}

/// 记忆持久化
pub struct MemoryPersistence {
    /// 存储路径
    storage_path: std::path::PathBuf,
}

impl MemoryPersistence {
    /// 创建新的记忆持久化
    pub fn new(storage_path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            storage_path: storage_path.into(),
        }
    }

    /// 保存记忆条目到文件
    pub fn save(&self, entries: &[MemoryEntry]) -> Result<(), FrameworkError> {
        let content = serde_json::to_string(entries).map_err(|e| {
            FrameworkError::Other(format!("Failed to serialize memory entries: {}", e))
        })?;

        // 确保目录存在
        if let Some(parent) = self.storage_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| FrameworkError::Other(format!("Failed to create directory: {}", e)))?;
        }

        std::fs::write(&self.storage_path, content)
            .map_err(|e| FrameworkError::Other(format!("Failed to write memory file: {}", e)))?;

        Ok(())
    }

    /// 从文件加载记忆条目
    pub fn load(&self) -> Result<Vec<MemoryEntry>, FrameworkError> {
        if !self.storage_path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.storage_path)
            .map_err(|e| FrameworkError::Other(format!("Failed to read memory file: {}", e)))?;

        let entries: Vec<MemoryEntry> = serde_json::from_str(&content)
            .map_err(|e| FrameworkError::Other(format!("Failed to parse memory file: {}", e)))?;

        Ok(entries)
    }

    /// 清空持久化存储
    pub fn clear(&self) -> Result<(), FrameworkError> {
        if self.storage_path.exists() {
            std::fs::remove_file(&self.storage_path).map_err(|e| {
                FrameworkError::Other(format!("Failed to remove memory file: {}", e))
            })?;
        }
        Ok(())
    }

    /// 获取存储路径
    pub fn storage_path(&self) -> &std::path::Path {
        &self.storage_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_compactor_time_based() {
        let compactor = MemoryCompactor::new(CompactionStrategy::TimeBased { max_age_secs: 100 });

        let entries = vec![
            MemoryEntry {
                id: "1".to_string(),
                content: serde_json::json!("old"),
                timestamp: 100,
                tags: vec![],
                importance: 0.5,
            },
            MemoryEntry {
                id: "2".to_string(),
                content: serde_json::json!("new"),
                timestamp: 200,
                tags: vec![],
                importance: 0.5,
            },
        ];

        let compacted = compactor.compact(entries, 250);
        assert_eq!(compacted.len(), 1);
        assert_eq!(compacted[0].id, "2");
    }

    #[test]
    fn test_memory_compactor_importance_based() {
        let compactor = MemoryCompactor::new(CompactionStrategy::ImportanceBased {
            min_importance: 0.5,
        });

        let entries = vec![
            MemoryEntry {
                id: "1".to_string(),
                content: serde_json::json!("low"),
                timestamp: 100,
                tags: vec![],
                importance: 0.3,
            },
            MemoryEntry {
                id: "2".to_string(),
                content: serde_json::json!("high"),
                timestamp: 100,
                tags: vec![],
                importance: 0.8,
            },
        ];

        let compacted = compactor.compact(entries, 200);
        assert_eq!(compacted.len(), 1);
        assert_eq!(compacted[0].id, "2");
    }

    #[test]
    fn test_memory_persistence() {
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join("test_memory.json");
        let persistence = MemoryPersistence::new(&path);

        let entries = vec![MemoryEntry {
            id: "1".to_string(),
            content: serde_json::json!("test"),
            timestamp: 1234567890,
            tags: vec!["test".to_string()],
            importance: 0.8,
        }];

        // Save
        persistence.save(&entries).unwrap();

        // Load
        let loaded = persistence.load().unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, "1");

        // Clean up
        persistence.clear().unwrap();
    }
}
