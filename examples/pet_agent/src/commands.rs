//! 命令处理模块
//!
//! 处理用户输入的命令。

use crate::app::App;

/// 命令类型
pub enum Command {
    /// 显示帮助
    Help,
    /// 显示状态
    Status,
    /// 切换位置
    Location,
    /// 查看记忆
    Memory,
    /// 清屏
    Clear,
    /// 退出
    Quit,
    /// 设置 API Key
    SetApiKey(String),
    /// 设置动画速度
    SetSpeed(u64),
    /// 喂食
    Feed,
    /// 玩耍
    Play,
    /// 休息
    Rest,
    /// 探索
    Explore,
    /// 未知命令
    Unknown(String),
}

/// 解析命令
pub fn parse_command(input: &str) -> Command {
    let input = input.trim();

    if !input.starts_with('/') {
        return Command::Unknown(input.to_string());
    }

    let parts: Vec<&str> = input[1..].splitn(2, ' ').collect();
    let cmd = parts[0].to_lowercase();
    let args = if parts.len() > 1 { parts[1].trim() } else { "" };

    match cmd.as_str() {
        "help" | "h" | "?" => Command::Help,
        "status" | "s" => Command::Status,
        "location" | "loc" | "l" => Command::Location,
        "memory" | "mem" | "m" => Command::Memory,
        "clear" | "c" => Command::Clear,
        "quit" | "q" | "exit" => Command::Quit,
        "apikey" | "key" => Command::SetApiKey(args.to_string()),
        "speed" => {
            if let Ok(speed) = args.parse::<u64>() {
                Command::SetSpeed(speed)
            } else {
                Command::Unknown(input.to_string())
            }
        }
        "feed" | "f" => Command::Feed,
        "play" | "p" => Command::Play,
        "rest" | "r" => Command::Rest,
        "explore" | "e" => Command::Explore,
        _ => Command::Unknown(input.to_string()),
    }
}

/// 执行命令
pub async fn execute_command(app: &mut App, command: Command) -> String {
    match command {
        Command::Help => r#"
可用命令:
  /help, /h, /?       - 显示帮助
  /status, /s         - 显示状态
  /location, /loc, /l - 切换位置
  /memory, /mem, /m   - 查看记忆
  /clear, /c          - 清空消息
  /quit, /q, /exit    - 退出
  /apikey <key>       - 设置 API Key
  /speed <ms>         - 设置动画速度 (默认 200)
  /feed, /f           - 喂食
  /play, /p           - 玩耍
  /rest, /r           - 休息
  /explore, /e        - 探索

快捷键:
  F1 - 喂食
  F2 - 玩耍
  F3 - 休息
  F4 - 探索
  Tab - 切换位置
  Esc - 退出
"#
        .to_string(),
        Command::Status => app.status_summary(),
        Command::Location => {
            app.switch_location();
            format!(
                "切换到: {} {}",
                app.pet.location.emoji(),
                app.pet.location.name()
            )
        }
        Command::Memory => {
            format!(
                "记忆状态:\n\
                 - 对话数量: {}\n\
                 - 偏好数量: {}\n\
                 - 知识数量: {}\n\
                 - 待办任务: {}",
                app.memory.conversations.len(),
                app.memory.preferences.len(),
                app.memory.knowledge.len(),
                app.memory.pending_task_count()
            )
        }
        Command::Clear => {
            app.clear_messages();
            "消息已清空".to_string()
        }
        Command::Quit => {
            app.should_quit = true;
            "再见！".to_string()
        }
        Command::SetApiKey(key) => {
            if key.is_empty() {
                "请提供 API Key: /apikey <your_key>".to_string()
            } else {
                match app.set_api_key(&key) {
                    Ok(_) => "API Key 已保存".to_string(),
                    Err(e) => format!("保存失败: {}", e),
                }
            }
        }
        Command::SetSpeed(speed) => {
            app.animation.set_speed(speed);
            format!("动画速度设置为 {}ms", speed)
        }
        Command::Feed => {
            app.feed();
            format!("喂了 {}", app.pet.name)
        }
        Command::Play => {
            app.play();
            format!("和 {} 玩耍", app.pet.name)
        }
        Command::Rest => {
            app.rest();
            format!("{} 在休息", app.pet.name)
        }
        Command::Explore => {
            app.explore();
            format!("{} 在探索", app.pet.name)
        }
        Command::Unknown(cmd) => {
            format!("未知命令: {} (输入 /help 查看帮助)", cmd)
        }
    }
}
