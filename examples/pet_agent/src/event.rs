//! 事件处理模块
//!
//! 处理键盘和鼠标事件。

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use std::time::Duration;

/// 应用事件
pub enum AppEvent {
    /// 键盘事件
    Key(KeyEvent),
    /// 鼠标事件
    Mouse(MouseEvent),
    /// 无事件
    None,
}

/// 事件处理器
pub struct EventHandler {
    /// 事件轮询间隔
    tick_rate: Duration,
}

impl EventHandler {
    /// 创建新事件处理器
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }

    /// 读取事件
    pub fn next(&self) -> anyhow::Result<AppEvent> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                Event::Key(key) => Ok(AppEvent::Key(key)),
                Event::Mouse(mouse) => Ok(AppEvent::Mouse(mouse)),
                _ => Ok(AppEvent::None),
            }
        } else {
            Ok(AppEvent::None)
        }
    }
}

/// 处理键盘事件
pub async fn handle_key_event(key: KeyEvent, app: &mut crate::app::App) -> anyhow::Result<()> {
    match key.code {
        // 退出
        KeyCode::Esc => {
            app.should_quit = true;
        }
        // Ctrl+C 退出
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        // 回车发送消息
        KeyCode::Enter => {
            if !app.input.is_empty() {
                // 检查是否是命令
                if app.input.starts_with('/') {
                    let command = crate::commands::parse_command(&app.input);
                    let response = crate::commands::execute_command(app, command).await;
                    app.messages
                        .push(crate::app::DisplayMessage::system(&response));
                    app.input.clear();
                } else {
                    // 发送消息给 AI
                    app.send_message().await?;
                }
            }
        }
        // 退格删除字符
        KeyCode::Backspace => {
            app.input.pop();
        }
        // 删除字符
        KeyCode::Delete => {
            // 暂时不处理
        }
        // 左右移动光标
        KeyCode::Left => {
            // 暂时不处理
        }
        KeyCode::Right => {
            // 暂时不处理
        }
        // Tab 切换位置
        KeyCode::Tab => {
            app.switch_location();
        }
        // F1 喂食
        KeyCode::F(1) => {
            app.feed();
        }
        // F2 玩耍
        KeyCode::F(2) => {
            app.play();
        }
        // F3 休息
        KeyCode::F(3) => {
            app.rest();
        }
        // F4 探索
        KeyCode::F(4) => {
            app.explore();
        }
        // F5 设置
        KeyCode::F(5) => {
            app.messages
                .push(crate::app::DisplayMessage::system("设置功能待实现"));
        }
        // 数字键切换位置
        KeyCode::Char('1') => {
            app.set_location(0);
        }
        KeyCode::Char('2') => {
            app.set_location(1);
        }
        KeyCode::Char('3') => {
            app.set_location(2);
        }
        KeyCode::Char('4') => {
            app.set_location(3);
        }
        // 普通字符输入
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        _ => {}
    }
    Ok(())
}

/// 处理鼠标事件
pub async fn handle_mouse_event(
    mouse: MouseEvent,
    app: &mut crate::app::App,
) -> anyhow::Result<()> {
    match mouse.kind {
        MouseEventKind::Down(crossterm::event::MouseButton::Left) => {
            let (x, y) = (mouse.column, mouse.row);

            // 检测位置标签点击 (第 3 行)
            if y == 2 {
                if x >= 2 && x <= 10 {
                    app.set_location(0); // 屋里
                } else if x >= 12 && x <= 22 {
                    app.set_location(1); // 工作室
                } else if x >= 24 && x <= 32 {
                    app.set_location(2); // 前院
                } else if x >= 34 && x <= 46 {
                    app.set_location(3); // 后院农场
                }
            }

            // 检测快捷键点击 (最后一行)
            // 这里需要根据实际布局调整
        }
        MouseEventKind::ScrollUp => {
            // 向上滚动消息历史
        }
        MouseEventKind::ScrollDown => {
            // 向下滚动消息历史
        }
        _ => {}
    }
    Ok(())
}
