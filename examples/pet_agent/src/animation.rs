//! 动画系统模块
//!
//! 提供思考动画、旋转符号等效果。

use std::time::{Duration, Instant};

/// 动画系统
pub struct Animation {
    /// 旋转符号
    spinner: Vec<char>,
    /// 当前帧索引
    index: usize,
    /// 上次更新时间
    last_update: Instant,
    /// 动画速度
    speed: Duration,
    /// 思考消息
    thinking_messages: Vec<String>,
    /// 当前消息索引
    message_index: usize,
}

impl Animation {
    /// 创建新动画系统
    pub fn new(speed_ms: u64) -> Self {
        Self {
            spinner: vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            index: 0,
            last_update: Instant::now(),
            speed: Duration::from_millis(speed_ms),
            thinking_messages: vec![
                "正在思考".to_string(),
                "在回忆".to_string(),
                "在分析".to_string(),
                "在组织语言".to_string(),
                "在寻找答案".to_string(),
            ],
            message_index: 0,
        }
    }

    /// 更新动画帧
    pub fn update(&mut self) -> bool {
        if self.last_update.elapsed() >= self.speed {
            self.index = (self.index + 1) % self.spinner.len();
            // 每 3 帧切换一次消息
            if self.index % 3 == 0 {
                self.message_index = (self.message_index + 1) % self.thinking_messages.len();
            }
            self.last_update = Instant::now();
            true
        } else {
            false
        }
    }

    /// 获取当前旋转符号
    pub fn current_frame(&self) -> char {
        self.spinner[self.index]
    }

    /// 获取思考消息
    pub fn thinking_message(&self) -> String {
        format!(
            "{} {}",
            self.thinking_messages[self.message_index],
            self.current_frame()
        )
    }

    /// 获取带动画的思考消息
    pub fn animated_thinking(&self) -> String {
        let dots = ".".repeat((self.index % 4) + 1);
        format!("小狗正在思考{} {}", dots, self.current_frame())
    }

    /// 设置动画速度
    pub fn set_speed(&mut self, speed_ms: u64) {
        self.speed = Duration::from_millis(speed_ms);
    }

    /// 获取当前速度
    pub fn speed(&self) -> u64 {
        self.speed.as_millis() as u64
    }

    /// 添加自定义消息
    pub fn add_message(&mut self, message: String) {
        self.thinking_messages.push(message);
    }

    /// 重置消息列表
    pub fn reset_messages(&mut self) {
        self.thinking_messages = vec![
            "正在思考".to_string(),
            "在回忆".to_string(),
            "在分析".to_string(),
            "在组织语言".to_string(),
            "在寻找答案".to_string(),
        ];
    }
}

/// 加载动画
pub struct LoadingAnimation {
    /// 进度条符号
    bar_chars: Vec<char>,
    /// 当前进度
    progress: usize,
    /// 总长度
    length: usize,
}

impl LoadingAnimation {
    /// 创建新加载动画
    pub fn new(length: usize) -> Self {
        Self {
            bar_chars: vec!['▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'],
            progress: 0,
            length,
        }
    }

    /// 更新进度
    pub fn update(&mut self) {
        self.progress = (self.progress + 1) % (self.length * self.bar_chars.len());
    }

    /// 获取进度条字符串
    pub fn bar(&self) -> String {
        let full_blocks = self.progress / self.bar_chars.len();
        let partial = self.progress % self.bar_chars.len();

        let mut bar = String::new();
        bar.push('[');

        // 完整块
        for _ in 0..full_blocks {
            bar.push('█');
        }

        // 部分块
        if partial > 0 && full_blocks < self.length {
            bar.push(self.bar_chars[partial - 1]);
        }

        // 空白
        let remaining = self.length - full_blocks - if partial > 0 { 1 } else { 0 };
        for _ in 0..remaining {
            bar.push(' ');
        }

        bar.push(']');
        bar
    }

    /// 重置进度
    pub fn reset(&mut self) {
        self.progress = 0;
    }
}
