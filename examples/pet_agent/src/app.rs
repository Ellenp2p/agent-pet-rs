//! 应用状态管理模块

use crate::ai::{self, ChatMessage, ProviderManager};
use crate::animation::Animation;
use crate::config::Config;
use crate::location::Location;
use crate::memory::Memory;
use crate::pet::{Pet, PetState};

pub struct App {
    pub config: Config,
    pub pet: Pet,
    pub memory: Memory,
    pub animation: Animation,
    pub provider_manager: Option<ProviderManager>,
    pub input: String,
    pub character_index: usize,  // 光标位置
    pub messages: Vec<DisplayMessage>,
    pub should_quit: bool,
    pub is_thinking: bool,
    pub location_index: usize,
    pub needs_setup: bool,
}

pub struct DisplayMessage {
    pub sender: String,
    pub content: String,
    pub is_system: bool,
}

impl DisplayMessage {
    pub fn user(content: &str) -> Self {
        Self { sender: "你".to_string(), content: content.to_string(), is_system: false }
    }
    pub fn pet(name: &str, content: &str) -> Self {
        Self { sender: name.to_string(), content: content.to_string(), is_system: false }
    }
    pub fn system(content: &str) -> Self {
        Self { sender: "系统".to_string(), content: content.to_string(), is_system: true }
    }
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let config = Config::load()?;
        let memory = Memory::load(&config.memory_path())?;
        let animation = Animation::new(config.settings.animation_speed);
        let mut pet = Pet::new("Buddy");
        pet.size = config.settings.dog_size;

        let needs_setup = config.needs_setup();

        let provider_manager = if !needs_setup {
            let ai_config = ai::manager::AIConfig {
                auto_switch: config.ai.auto_switch,
                switch_order: config.ai.switch_order.clone(),
                providers: config.ai.providers.clone(),
            };
            ProviderManager::new(&ai_config, &config.usage_path(), config.ai.budget.clone()).ok()
        } else {
            None
        };

        Ok(Self {
            config,
            pet,
            memory,
            animation,
            provider_manager,
            input: String::new(),
            character_index: 0,  // 初始化光标位置
            messages: Vec::new(),
            should_quit: false,
            is_thinking: false,
            location_index: 0,
            needs_setup,
        })
    }

    pub async fn send_message(&mut self) -> anyhow::Result<()> {
        if self.input.is_empty() {
            return Ok(());
        }

        let user_message = self.input.clone();
        self.input.clear();
        self.messages.push(DisplayMessage::user(&user_message));
        self.pet.set_state(PetState::Thinking);
        self.is_thinking = true;

        let system_prompt = ai::create_system_prompt(self.pet.location.name(), self.pet.state.name(), &self.pet.name);
        let history = self.memory.get_recent_context(10);
        let messages = ai::create_messages(&system_prompt, history, &user_message);

        if let Some(ref mut manager) = self.provider_manager {
            match manager.chat_with_fallback(messages).await {
                Ok(response) => {
                    self.messages.push(DisplayMessage::pet(&self.pet.name, &response));
                    self.memory.add_conversation(self.pet.location.name(), &user_message, &response);
                    self.pet.set_state(PetState::Happy);
                    self.pet.boost_happiness(5.0);
                    self.messages.push(DisplayMessage::system(&format!("📊 {}", manager.usage_summary())));
                }
                Err(e) => {
                    self.messages.push(DisplayMessage::system(&format!("AI 错误: {}", e)));
                    self.pet.set_state(PetState::Idle);
                }
            }
        } else {
            self.messages.push(DisplayMessage::system("请先配置 AI 提供商 (/setup)"));
            self.pet.set_state(PetState::Idle);
        }

        self.is_thinking = false;
        Ok(())
    }

    pub fn switch_location(&mut self) {
        self.location_index = (self.location_index + 1) % Location::all().len();
        let loc = Location::from_index(self.location_index);
        self.pet.move_to(loc);
        self.messages.push(DisplayMessage::system(&format!("{} 移动到了 {} {}", self.pet.name, loc.emoji(), loc.name())));
    }

    pub fn set_location(&mut self, index: usize) {
        self.location_index = index;
        let loc = Location::from_index(index);
        self.pet.move_to(loc);
        self.messages.push(DisplayMessage::system(&format!("{} 移动到了 {} {}", self.pet.name, loc.emoji(), loc.name())));
    }

    pub fn feed(&mut self) {
        self.pet.restore_energy(20.0);
        self.pet.boost_happiness(10.0);
        self.pet.set_state(PetState::Happy);
        self.messages.push(DisplayMessage::system(&format!("你喂了 {}，它很开心！🍖", self.pet.name)));
    }

    pub fn play(&mut self) {
        self.pet.boost_happiness(20.0);
        self.pet.restore_energy(-10.0);
        self.pet.set_state(PetState::Playing);
        self.messages.push(DisplayMessage::system(&format!("你和 {} 玩耍了！🎾", self.pet.name)));
    }

    pub fn rest(&mut self) {
        self.pet.restore_energy(30.0);
        self.pet.set_state(PetState::Sleeping);
        self.messages.push(DisplayMessage::system(&format!("{} 正在休息...💤", self.pet.name)));
    }

    pub fn explore(&mut self) {
        self.pet.restore_energy(-15.0);
        self.pet.boost_happiness(15.0);
        self.pet.set_state(PetState::Working);
        self.messages.push(DisplayMessage::system(&format!("{} 开始探索周围...🔍", self.pet.name)));
    }

    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    // ========== 输入处理方法 ==========

    /// 移动光标向左
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    /// 移动光标向右
    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    /// 在光标位置插入字符
    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// 删除光标前的字符
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    /// 计算字节索引（基于字符位置）
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    /// 限制光标位置在有效范围内
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    /// 重置光标位置到开头
    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    /// 提交消息
    pub fn submit_message(&mut self) {
        self.messages.push(DisplayMessage::user(&self.input));
        self.input.clear();
        self.reset_cursor();
    }

    // ========== 其他方法 ==========

    pub fn save(&self) -> anyhow::Result<()> {
        self.memory.save(&self.config.memory_path())
    }

    pub fn provider_status(&self) -> String {
        if let Some(ref pm) = self.provider_manager {
            format!("{} | {}", pm.current_provider_name(), pm.rate_limit_status())
        } else {
            "未配置".to_string()
        }
    }

    pub fn usage_stats(&self) -> String {
        if let Some(ref pm) = self.provider_manager {
            pm.usage_summary()
        } else {
            "无数据".to_string()
        }
    }

    pub fn export_usage(&self, format: &str) -> Result<String, String> {
        if let Some(ref pm) = self.provider_manager {
            match format {
                "json" => pm.export_usage_json().map_err(|e| format!("{}", e)),
                "csv" => Ok(pm.export_usage_csv()),
                _ => Err("不支持的格式".to_string()),
            }
        } else {
            Err("未配置".to_string())
        }
    }
}
