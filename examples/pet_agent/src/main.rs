//! 桌面宠物 Agent - TUI 示例
//!
//! 一个基于 TUI 的桌面宠物，可以与 AI 对话、执行任务、记住偏好。
//!
//! ## 运行
//!
//! ```bash
//! cargo run --bin pet-agent
//! ```

mod ai;
mod animation;
mod app;
mod commands;
mod config;
mod event;
mod location;
mod memory;
mod pet;
mod tui;
mod ui;

use tui::{Event, Tui};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化应用
    let mut app = app::App::new()?;

    // 如果需要设置，先进行设置
    if app.needs_setup {
        // 先退出 TUI 模式
        println!("欢迎使用桌面宠物 Agent！");
        println!("请输入你的 OpenRouter API Key:");
        println!("(可以在 https://openrouter.ai/keys 获取)");
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout())?;

        let mut api_key = String::new();
        std::io::stdin().read_line(&mut api_key)?;
        let api_key = api_key.trim();

        if api_key.is_empty() {
            println!("API Key 不能为空！");
            return Ok(());
        }

        // 创建提供商配置
        let provider_config = crate::ai::provider::ProviderConfig::new(
            crate::ai::provider::ProviderType::OpenRouter,
            api_key,
        );
        app.config.ai.providers.push(provider_config);
        app.config.save()?;
        println!("配置已保存！启动宠物...\n");
    }

    // 创建 TUI
    let mut tui = Tui::new()?
        .tick_rate(4.0) // 4 ticks per second
        .frame_rate(30.0) // 30 frames per second
        .mouse(true); // 启用鼠标

    // 进入终端模式
    tui.enter()?;

    // 添加欢迎消息
    app.messages.push(app::DisplayMessage::system(&format!(
        "欢迎！我是 {}，你的智能宠物助手！🐕",
        app.pet.name
    )));
    app.messages.push(app::DisplayMessage::system(
        "输入消息和我聊天，或者按 /help 查看帮助",
    ));

    // 主循环
    loop {
        // 处理事件
        if let Some(event) = tui.next().await {
            match event {
                Event::Init => {
                    // 初始化
                }
                Event::Quit => {
                    break;
                }
                Event::Error => {
                    // 错误处理
                }
                Event::Tick => {
                    // 更新游戏状态
                }
                Event::Render => {
                    // 渲染 UI
                    tui.draw(|f| ui::render(f, &app))?;
                }
                Event::Key(key) => {
                    event::handle_key_event(key, &mut app).await?;
                }
                Event::Mouse(mouse) => {
                    event::handle_mouse_event(mouse, &mut app).await?;
                }
                Event::Resize(_, _) => {
                    // 处理窗口大小变化
                }
            }
        }

        // 检查是否退出
        if app.should_quit {
            break;
        }
    }

    // 退出终端模式
    tui.exit()?;

    // 保存状态
    if let Err(e) = app.save() {
        eprintln!("保存状态失败: {}", e);
    }

    Ok(())
}
