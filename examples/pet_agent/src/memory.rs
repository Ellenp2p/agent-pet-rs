//! 记忆持久化模块
//!
//! 保存对话历史、主人偏好等记忆。

use crate::ai::ChatMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 记忆结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// 对话历史
    pub conversations: Vec<Conversation>,
    /// 主人偏好
    pub preferences: HashMap<String, String>,
    /// 学到的知识
    pub knowledge: Vec<String>,
    /// 任务记录
    pub tasks: Vec<TaskRecord>,
    /// 最后更新时间
    pub last_updated: u64,
}

/// 对话记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    /// 时间戳
    pub timestamp: u64,
    /// 位置
    pub location: String,
    /// 用户消息
    pub user: String,
    /// 小狗回复
    pub pet: String,
}

/// 任务记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRecord {
    /// 时间戳
    pub timestamp: u64,
    /// 任务类型
    pub task_type: String,
    /// 任务内容
    pub content: String,
    /// 是否完成
    pub completed: bool,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            conversations: Vec::new(),
            preferences: HashMap::new(),
            knowledge: Vec::new(),
            tasks: Vec::new(),
            last_updated: chrono::Utc::now().timestamp() as u64,
        }
    }
}

impl Memory {
    /// 加载记忆
    pub fn load(path: &PathBuf) -> anyhow::Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let memory: Self = serde_json::from_str(&content)?;
            Ok(memory)
        } else {
            Ok(Self::default())
        }
    }

    /// 保存记忆
    pub fn save(&self, path: &PathBuf) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// 添加对话
    pub fn add_conversation(&mut self, location: &str, user: &str, pet: &str) {
        self.conversations.push(Conversation {
            timestamp: chrono::Utc::now().timestamp() as u64,
            location: location.to_string(),
            user: user.to_string(),
            pet: pet.to_string(),
        });
        self.last_updated = chrono::Utc::now().timestamp() as u64;

        // 限制历史记录数量（最多 1000 条）
        if self.conversations.len() > 1000 {
            self.conversations.remove(0);
        }
    }

    /// 添加任务
    pub fn add_task(&mut self, task_type: &str, content: &str) {
        self.tasks.push(TaskRecord {
            timestamp: chrono::Utc::now().timestamp() as u64,
            task_type: task_type.to_string(),
            content: content.to_string(),
            completed: false,
        });
        self.last_updated = chrono::Utc::now().timestamp() as u64;
    }

    /// 完成任务
    pub fn complete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks[index].completed = true;
            self.last_updated = chrono::Utc::now().timestamp() as u64;
        }
    }

    /// 设置偏好
    pub fn set_preference(&mut self, key: &str, value: &str) {
        self.preferences.insert(key.to_string(), value.to_string());
        self.last_updated = chrono::Utc::now().timestamp() as u64;
    }

    /// 获取偏好
    pub fn get_preference(&self, key: &str) -> Option<&String> {
        self.preferences.get(key)
    }

    /// 添加知识
    pub fn add_knowledge(&mut self, knowledge: &str) {
        if !self.knowledge.contains(&knowledge.to_string()) {
            self.knowledge.push(knowledge.to_string());
            self.last_updated = chrono::Utc::now().timestamp() as u64;
        }
    }

    /// 获取最近的上下文（用于 AI 对话）
    pub fn get_recent_context(&self, count: usize) -> Vec<ChatMessage> {
        self.conversations
            .iter()
            .rev()
            .take(count)
            .rev()
            .flat_map(|conv| {
                vec![
                    ChatMessage {
                        role: "user".to_string(),
                        content: conv.user.clone(),
                    },
                    ChatMessage {
                        role: "assistant".to_string(),
                        content: conv.pet.clone(),
                    },
                ]
            })
            .collect()
    }

    /// 获取对话数量
    pub fn conversation_count(&self) -> usize {
        self.conversations.len()
    }

    /// 获取未完成任务数量
    pub fn pending_task_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.completed).count()
    }

    /// 获取记忆摘要
    pub fn summary(&self) -> String {
        format!(
            "对话: {} | 偏好: {} | 知识: {} | 待办: {}",
            self.conversations.len(),
            self.preferences.len(),
            self.knowledge.len(),
            self.pending_task_count()
        )
    }
}
