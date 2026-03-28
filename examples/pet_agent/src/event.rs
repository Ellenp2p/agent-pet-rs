//! 事件处理模块
//!
//! 处理键盘和鼠标事件。
//! 使用 tui-textarea 处理输入。

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

/// 处理键盘事件
pub async fn handle_key_event(key: KeyEvent, app: &mut crate::app::App) -> anyhow::Result<()> {
    // 特殊按键处理
    match key.code {
        // 退出
        KeyCode::Esc => {
            app.should_quit = true;
            return Ok(());
        }
        // Ctrl+C 退出
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
            return Ok(());
        }
        // 回车发送消息
        KeyCode::Enter => {
            let input = app.textarea.lines().join("\n");
            if !input.is_empty() {
                // 检查是否是命令
                if input.starts_with('/') {
                    let command = crate::commands::parse_command(&input);
                    let response = crate::commands::execute_command(app, command).await;
                    app.messages
                        .push(crate::app::DisplayMessage::system(&response));
                    // 清空输入
                    app.textarea = tui_textarea::TextArea::default();
                    app.textarea.set_block(
                        ratatui::widgets::Block::default()
                            .borders(ratatui::widgets::Borders::ALL)
                            .title("输入消息")
                    );
                } else {
                    // 发送消息给 AI
                    app.send_message().await?;
                }
            }
            return Ok(());
        }
        // Tab 切换位置
        KeyCode::Tab => {
            app.switch_location();
            return Ok(());
        }
        // F1 喂食
        KeyCode::F(1) => {
            app.feed();
            return Ok(());
        }
        // F2 玩耍
        KeyCode::F(2) => {
            app.play();
            return Ok(());
        }
        // F3 休息
        KeyCode::F(3) => {
            app.rest();
            return Ok(());
        }
        // F4 探索
        KeyCode::F(4) => {
            app.explore();
            return Ok(());
        }
        // F5 设置
        KeyCode::F(5) => {
            app.messages
                .push(crate::app::DisplayMessage::system("设置功能待实现"));
            return Ok(());
        }
        // 数字键切换位置（仅在非编辑模式下）
        KeyCode::Char('1') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.set_location(0);
            return Ok(());
        }
        KeyCode::Char('2') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.set_location(1);
            return Ok(());
        }
        KeyCode::Char('3') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.set_location(2);
            return Ok(());
        }
        KeyCode::Char('4') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.set_location(3);
            return Ok(());
        }
        _ => {}
    }

    // 其他按键交给 tui-textarea 处理
    app.textarea.input(key);
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
