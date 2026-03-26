//! 应用状态管理模块
//!
//! 管理整个应用的状态。

use crate::ai::{self, ChatMessage};
use crate::animation::Animation;
use crate::config::Config;
use crate::location::Location;
use crate::memory::Memory;
use crate::pet::{Pet, PetState};

/// 应用状态
pub struct App {
    /// 配置
    pub config: Config,
    /// 小狗
    pub pet: Pet,
    /// 记忆
    pub memory: Memory,
    /// 动画
    pub animation: Animation,
    /// 用户输入
    pub input: String,
    /// 消息历史（用于显示）
    pub messages: Vec<DisplayMessage>,
    /// 是否退出
    pub should_quit: bool,
    /// 是否正在思考
    pub is_thinking: bool,
    /// 当前位置索引
    pub location_index: usize,
    /// 是否需要设置
    pub needs_setup: bool,
}

/// 显示消息
pub struct DisplayMessage {
    pub sender: String,
    pub content: String,
    pub is_system: bool,
}

impl DisplayMessage {
    pub fn user(content: &str) -> Self {
        Self {
            sender: "你".to_string(),
            content: content.to_string(),
            is_system: false,
        }
    }

    pub fn pet(name: &str, content: &str) -> Self {
        Self {
            sender: name.to_string(),
            content: content.to_string(),
            is_system: false,
        }
    }

    pub fn system(content: &str) -> Self {
        Self {
            sender: "系统".to_string(),
            content: content.to_string(),
            is_system: true,
        }
    }
}

impl App {
    /// 创建新应用
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load()?;
        let memory = Memory::load(&config.memory_path)?;
        let animation = Animation::new(config.animation_speed);
        let mut pet = Pet::new("Buddy");
        pet.size = config.dog_size;

        let needs_setup = config.needs_setup();

        Ok(Self {
            config,
            pet,
            memory,
            animation,
            input: String::new(),
            messages: Vec::new(),
            should_quit: false,
            is_thinking: false,
            location_index: 0,
            needs_setup,
        })
    }

    /// 发送消息
    pub async fn send_message(&mut self) -> anyhow::Result<()> {
        if self.input.is_empty() {
            return Ok(());
        }

        let user_message = self.input.clone();
        self.input.clear();

        // 添加用户消息到显示
        self.messages.push(DisplayMessage::user(&user_message));

        // 设置思考状态
        self.pet.set_state(PetState::Thinking);
        self.is_thinking = true;

        // 准备 AI 请求
        let system_prompt = ai::create_system_prompt(
            self.pet.location.name(),
            self.pet.state.name(),
            &self.pet.name,
        );
        let history = self.memory.get_recent_context(10);
        let messages = ai::create_messages(&system_prompt, history, &user_message);

        // 调用 AI API
        match ai::call_openrouter(messages, &self.config.api_key, &self.config.model).await {
            Ok(response) => {
                // 添加回复到显示
                self.messages
                    .push(DisplayMessage::pet(&self.pet.name, &response));

                // 保存到记忆
                self.memory
                    .add_conversation(self.pet.location.name(), &user_message, &response);

                // 设置开心状态
                self.pet.set_state(PetState::Happy);
                self.pet.boost_happiness(5.0);
            }
            Err(e) => {
                self.messages
                    .push(DisplayMessage::system(&format!("AI 调用失败: {}", e)));
                self.pet.set_state(PetState::Idle);
            }
        }

        self.is_thinking = false;
        Ok(())
    }

    /// 切换位置
    pub fn switch_location(&mut self) {
        self.location_index = (self.location_index + 1) % Location::all().len();
        let new_location = Location::from_index(self.location_index);
        self.pet.move_to(new_location);
        self.messages.push(DisplayMessage::system(&format!(
            "{} 移动到了 {} {}",
            self.pet.name,
            new_location.emoji(),
            new_location.name()
        )));
    }

    /// 设置位置
    pub fn set_location(&mut self, index: usize) {
        self.location_index = index;
        let new_location = Location::from_index(index);
        self.pet.move_to(new_location);
        self.messages.push(DisplayMessage::system(&format!(
            "{} 移动到了 {} {}",
            self.pet.name,
            new_location.emoji(),
            new_location.name()
        )));
    }

    /// 喂食
    pub fn feed(&mut self) {
        self.pet.restore_energy(20.0);
        self.pet.boost_happiness(10.0);
        self.pet.set_state(PetState::Happy);
        self.messages.push(DisplayMessage::system(&format!(
            "你喂了 {}，它很开心！🍖",
            self.pet.name
        )));
    }

    /// 玩耍
    pub fn play(&mut self) {
        self.pet.boost_happiness(20.0);
        self.pet.restore_energy(-10.0);
        self.pet.set_state(PetState::Playing);
        self.messages.push(DisplayMessage::system(&format!(
            "你和 {} 玩耍了！🎾",
            self.pet.name
        )));
    }

    /// 休息
    pub fn rest(&mut self) {
        self.pet.restore_energy(30.0);
        self.pet.set_state(PetState::Sleeping);
        self.messages.push(DisplayMessage::system(&format!(
            "{} 正在休息...💤",
            self.pet.name
        )));
    }

    /// 探索
    pub fn explore(&mut self) {
        self.pet.restore_energy(-15.0);
        self.pet.boost_happiness(15.0);
        self.pet.set_state(PetState::Working);
        self.messages.push(DisplayMessage::system(&format!(
            "{} 开始探索周围...🔍",
            self.pet.name
        )));
    }

    /// 清空消息
    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    /// 保存状态
    pub fn save(&self) -> anyhow::Result<()> {
        self.memory.save(&self.config.memory_path)
    }

    /// 设置 API Key
    pub fn set_api_key(&mut self, api_key: &str) -> anyhow::Result<()> {
        self.config.api_key = api_key.to_string();
        self.config.save()?;
        self.needs_setup = false;
        Ok(())
    }

    /// 获取状态摘要
    pub fn status_summary(&self) -> String {
        format!(
            "位置: {} | 精力: {:.0}% | 心情: {:.0}% | {}",
            self.pet.location.name(),
            self.pet.energy,
            self.pet.happiness,
            self.memory.summary()
        )
    }
}
