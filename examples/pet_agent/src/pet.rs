//! 小狗显示和状态模块
//!
//! 管理小狗的显示、状态和动画。

use crate::config::DogSize;
use crate::location::Location;

/// 小狗状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PetState {
    /// 空闲
    Idle,
    /// 思考中
    Thinking,
    /// 开心
    Happy,
    /// 睡觉
    Sleeping,
    /// 工作中
    Working,
    /// 玩耍中
    Playing,
}

impl PetState {
    pub fn name(&self) -> &'static str {
        match self {
            PetState::Idle => "发呆",
            PetState::Thinking => "思考",
            PetState::Happy => "开心",
            PetState::Sleeping => "睡觉",
            PetState::Working => "工作",
            PetState::Playing => "玩耍",
        }
    }
}

/// 小狗结构
pub struct Pet {
    /// 名字
    pub name: String,
    /// 当前状态
    pub state: PetState,
    /// 当前位置
    pub location: Location,
    /// 精力 (0-100)
    pub energy: f32,
    /// 心情 (0-100)
    pub happiness: f32,
    /// 小狗大小
    pub size: DogSize,
}

impl Pet {
    /// 创建新小狗
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: PetState::Idle,
            location: Location::Indoor,
            energy: 100.0,
            happiness: 100.0,
            size: DogSize::Medium,
        }
    }

    /// 获取 ASCII Art
    pub fn ascii_art(&self) -> Vec<String> {
        match self.size {
            DogSize::Small => self.small_art(),
            DogSize::Medium => self.medium_art(),
            DogSize::Large => self.large_art(),
        }
    }

    /// 小号 ASCII Art (5 行)
    fn small_art(&self) -> Vec<String> {
        match self.state {
            PetState::Idle => vec![
                "  /\\_/\\   ".to_string(),
                " ( o.o )  ".to_string(),
                "  > ^ <   ".to_string(),
                " /|   |\\  ".to_string(),
                "(_|   |_) ".to_string(),
            ],
            PetState::Thinking => vec![
                "  /\\_/\\   ".to_string(),
                " ( o.o )  ".to_string(),
                "  > ? <   ".to_string(),
                " /|   |\\  ".to_string(),
                "(_|   |_) 💭".to_string(),
            ],
            PetState::Happy => vec![
                "  /\\_/\\   ".to_string(),
                " ( ^.^ )  ".to_string(),
                "  > w <   ".to_string(),
                " /|   |\\  ".to_string(),
                "(_|   |_) ".to_string(),
            ],
            PetState::Sleeping => vec![
                "  /\\_/\\   ".to_string(),
                " ( -.- )  ".to_string(),
                "  > ^ <   ".to_string(),
                " /|   |\\  ".to_string(),
                "(_|   |_) zZz".to_string(),
            ],
            PetState::Working => vec![
                "  /\\_/\\   ".to_string(),
                " ( o.o )  ".to_string(),
                "  > ! <   ".to_string(),
                " /|   |\\  ".to_string(),
                "(_|   |_) 🔧".to_string(),
            ],
            PetState::Playing => vec![
                "  /\\_/\\   ".to_string(),
                " ( ^.^ )  ".to_string(),
                "  > ~ <   ".to_string(),
                " /|   |\\  ".to_string(),
                "(_|   |_) ⚽".to_string(),
            ],
        }
    }

    /// 中号 ASCII Art (7 行)
    fn medium_art(&self) -> Vec<String> {
        match self.state {
            PetState::Idle => vec![
                "    /\\_/\\     ".to_string(),
                "   ( o.o )    ".to_string(),
                "    > ^ <     ".to_string(),
                "   /|   |\\    ".to_string(),
                "  (_|   |_)   ".to_string(),
                "".to_string(),
                format!("  {} 正在发呆...", self.name),
            ],
            PetState::Thinking => vec![
                "    /\\_/\\     ".to_string(),
                "   ( o.o )    ".to_string(),
                "    > ? <     ".to_string(),
                "   /|   |\\    ".to_string(),
                "  (_|   |_)   ".to_string(),
                "    💭         ".to_string(),
                format!("  {} 正在思考...", self.name),
            ],
            PetState::Happy => vec![
                "    /\\_/\\     ".to_string(),
                "   ( ^.^ )    ".to_string(),
                "    > w <     ".to_string(),
                "   /|   |\\    ".to_string(),
                "  (_|   |_)   ".to_string(),
                "".to_string(),
                format!("  {} 很开心！", self.name),
            ],
            PetState::Sleeping => vec![
                "    /\\_/\\     ".to_string(),
                "   ( -.- ) zZz".to_string(),
                "    > ^ <     ".to_string(),
                "   /|   |\\    ".to_string(),
                "  (_|   |_)   ".to_string(),
                "".to_string(),
                format!("  {} 正在睡觉", self.name),
            ],
            PetState::Working => vec![
                "    /\\_/\\     ".to_string(),
                "   ( o.o )    ".to_string(),
                "    > ! <     ".to_string(),
                "   /|   |\\    ".to_string(),
                "  (_|   |_) 🔧".to_string(),
                "".to_string(),
                format!("  {} 正在工作...", self.name),
            ],
            PetState::Playing => vec![
                "    /\\_/\\     ".to_string(),
                "   ( ^.^ )    ".to_string(),
                "    > ~ <  ⚽ ".to_string(),
                "   /|   |\\    ".to_string(),
                "  (_|   |_)   ".to_string(),
                "".to_string(),
                format!("  {} 正在玩耍！", self.name),
            ],
        }
    }

    /// 大号 ASCII Art (10 行)
    fn large_art(&self) -> Vec<String> {
        match self.state {
            PetState::Idle => vec![
                "       /\\_/\\      ".to_string(),
                "      ( o.o )     ".to_string(),
                "       > ^ <      ".to_string(),
                "      /|   |\\     ".to_string(),
                "     (_|   |_)    ".to_string(),
                "      /     \\     ".to_string(),
                "     (       )    ".to_string(),
                "".to_string(),
                format!("    {} 正在发呆...", self.name),
                "      状态: 空闲  ".to_string(),
            ],
            PetState::Thinking => vec![
                "       /\\_/\\      ".to_string(),
                "      ( o.o )     ".to_string(),
                "       > ? <      ".to_string(),
                "      /|   |\\     ".to_string(),
                "     (_|   |_)    ".to_string(),
                "      /     \\     ".to_string(),
                "     (   💭   )   ".to_string(),
                "".to_string(),
                format!("    {} 正在思考...", self.name),
                "      状态: 思考中".to_string(),
            ],
            PetState::Happy => vec![
                "       /\\_/\\      ".to_string(),
                "      ( ^.^ )     ".to_string(),
                "       > w <      ".to_string(),
                "      /|   |\\     ".to_string(),
                "     (_|   |_)    ".to_string(),
                "      /     \\     ".to_string(),
                "     (  ^_^  )    ".to_string(),
                "".to_string(),
                format!("    {} 很开心！", self.name),
                "      状态: 开心  ".to_string(),
            ],
            PetState::Sleeping => vec![
                "       /\\_/\\      ".to_string(),
                "      ( -.- ) zZz ".to_string(),
                "       > ^ <      ".to_string(),
                "      /|   |\\     ".to_string(),
                "     (_|   |_)    ".to_string(),
                "      /     \\     ".to_string(),
                "     (  - -  )    ".to_string(),
                "".to_string(),
                format!("    {} 正在睡觉", self.name),
                "      状态: 睡觉  ".to_string(),
            ],
            PetState::Working => vec![
                "       /\\_/\\      ".to_string(),
                "      ( o.o )     ".to_string(),
                "       > ! <      ".to_string(),
                "      /|   |\\     ".to_string(),
                "     (_|   |_) 🔧 ".to_string(),
                "      /     \\     ".to_string(),
                "     (  ! !  )    ".to_string(),
                "".to_string(),
                format!("    {} 正在工作...", self.name),
                "      状态: 工作中".to_string(),
            ],
            PetState::Playing => vec![
                "       /\\_/\\      ".to_string(),
                "      ( ^.^ )     ".to_string(),
                "       > ~ <  ⚽  ".to_string(),
                "      /|   |\\     ".to_string(),
                "     (_|   |_)    ".to_string(),
                "      /     \\     ".to_string(),
                "     (  ~ ~  )    ".to_string(),
                "".to_string(),
                format!("    {} 正在玩耍！", self.name),
                "      状态: 玩耍中".to_string(),
            ],
        }
    }

    /// 获取状态文本
    pub fn status_text(&self) -> String {
        format!(
            "{} 位置: {} | 精力: {:.0}% | 心情: {:.0}%",
            self.name,
            self.location.name(),
            self.energy,
            self.happiness
        )
    }

    /// 设置状态
    pub fn set_state(&mut self, state: PetState) {
        self.state = state;
    }

    /// 移动到新位置
    pub fn move_to(&mut self, location: Location) {
        self.location = location;
        // 移动消耗精力
        self.energy = (self.energy - 5.0).max(0.0);
    }

    /// 恢复精力
    pub fn restore_energy(&mut self, amount: f32) {
        self.energy = (self.energy + amount).min(100.0);
    }

    /// 提升心情
    pub fn boost_happiness(&mut self, amount: f32) {
        self.happiness = (self.happiness + amount).min(100.0);
    }
}
