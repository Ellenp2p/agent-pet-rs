//! AI API 集成模块
//!
//! 调用 OpenRouter API 进行对话。

use serde::{Deserialize, Serialize};

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// 聊天请求
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
}

/// 聊天响应
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

/// 选择
#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

/// 调用 OpenRouter API
pub async fn call_openrouter(
    messages: Vec<ChatMessage>,
    api_key: &str,
    model: &str,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    let request = ChatRequest {
        model: model.to_string(),
        messages,
        max_tokens: 1000,
        temperature: 0.7,
    };

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://github.com/Ellenp2p/agent-pet-rs")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        anyhow::bail!("API error: {}", error_text);
    }

    let chat_response: ChatResponse = response.json().await?;

    if chat_response.choices.is_empty() {
        anyhow::bail!("No response from API");
    }

    Ok(chat_response.choices[0].message.content.clone())
}

/// 创建系统提示
pub fn create_system_prompt(location: &str, state: &str, pet_name: &str) -> String {
    format!(
        r#"你是一只可爱的智能小狗，名字叫 {}。你有以下特点：

1. 性格：友好、忠诚、聪明、有点调皮
2. 能力：
   - 回答问题（对话）
   - 执行任务（提醒、查询等）
   - 自主行为（学习、记忆、决策）
   - 帮助主人处理日常事务

3. 行为：
   - 用可爱的语气回复
   - 记住主人的偏好
   - 在不同位置做不同的事
   - 有自己的想法和个性

4. 回复格式：
   - 简洁友好（1-3句话）
   - 适当使用 emoji
   - 保持小狗的性格

当前位置：{}
当前状态：{}

请用中文回复，保持可爱友好的语气。"#,
        pet_name, location, state
    )
}

/// 创建对话消息
pub fn create_messages(
    system_prompt: &str,
    history: Vec<ChatMessage>,
    user_message: &str,
) -> Vec<ChatMessage> {
    let mut messages = vec![ChatMessage {
        role: "system".to_string(),
        content: system_prompt.to_string(),
    }];

    // 添加历史消息（最多 10 条）
    let history_limit = history.len().min(10);
    let start = if history.len() > history_limit {
        history.len() - history_limit
    } else {
        0
    };
    messages.extend(history[start..].to_vec());

    // 添加用户消息
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: user_message.to_string(),
    });

    messages
}

/// 解析 AI 响应，提取任务信息
pub fn parse_task_from_response(response: &str) -> Option<TaskInfo> {
    // 简单的任务解析
    if response.contains("设置提醒") || response.contains("提醒") {
        Some(TaskInfo {
            task_type: TaskType::Reminder,
            content: response.to_string(),
        })
    } else if response.contains("查天气") || response.contains("天气") {
        Some(TaskInfo {
            task_type: TaskType::Weather,
            content: response.to_string(),
        })
    } else if response.contains("搜索") || response.contains("查找") {
        Some(TaskInfo {
            task_type: TaskType::Search,
            content: response.to_string(),
        })
    } else {
        None
    }
}

/// 任务信息
pub struct TaskInfo {
    pub task_type: TaskType,
    pub content: String,
}

/// 任务类型
pub enum TaskType {
    Reminder,
    Weather,
    Search,
    Other,
}
