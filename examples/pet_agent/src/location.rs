//! 位置系统模块
//!
//! 定义宠物可以去的不同位置。

/// 位置枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Location {
    /// 屋里
    Indoor,
    /// 工作室
    Workshop,
    /// 前院
    FrontYard,
    /// 后院农场
    BackYard,
}

impl Location {
    /// 获取位置名称
    pub fn name(&self) -> &'static str {
        match self {
            Location::Indoor => "屋里",
            Location::Workshop => "工作室",
            Location::FrontYard => "前院",
            Location::BackYard => "后院农场",
        }
    }

    /// 获取位置 emoji
    pub fn emoji(&self) -> &'static str {
        match self {
            Location::Indoor => "🏠",
            Location::Workshop => "🔧",
            Location::FrontYard => "🌳",
            Location::BackYard => "🌾",
        }
    }

    /// 获取位置描述
    pub fn description(&self) -> &'static str {
        match self {
            Location::Indoor => "温暖舒适的房间",
            Location::Workshop => "充满工具的工作室",
            Location::FrontYard => "阳光明媚的前院",
            Location::BackYard => "生机勃勃的农场",
        }
    }

    /// 获取该位置可以做的活动
    pub fn activities(&self) -> Vec<&'static str> {
        match self {
            Location::Indoor => vec!["休息", "学习", "吃饭", "睡觉"],
            Location::Workshop => vec!["工作", "修理", "制作", "学习技能"],
            Location::FrontYard => vec!["玩耍", "晒太阳", "散步", "接待"],
            Location::BackYard => vec!["种植", "养殖", "探索", "收获"],
        }
    }

    /// 获取所有位置
    pub fn all() -> Vec<Location> {
        vec![
            Location::Indoor,
            Location::Workshop,
            Location::FrontYard,
            Location::BackYard,
        ]
    }

    /// 获取下一个位置
    pub fn next(&self) -> Location {
        match self {
            Location::Indoor => Location::Workshop,
            Location::Workshop => Location::FrontYard,
            Location::FrontYard => Location::BackYard,
            Location::BackYard => Location::Indoor,
        }
    }

    /// 获取位置索引
    pub fn index(&self) -> usize {
        match self {
            Location::Indoor => 0,
            Location::Workshop => 1,
            Location::FrontYard => 2,
            Location::BackYard => 3,
        }
    }

    /// 从索引获取位置
    pub fn from_index(index: usize) -> Location {
        match index {
            0 => Location::Indoor,
            1 => Location::Workshop,
            2 => Location::FrontYard,
            3 => Location::BackYard,
            _ => Location::Indoor,
        }
    }
}
