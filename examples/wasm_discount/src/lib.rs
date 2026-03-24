//! WASM Discount Plugin
//!
//! 功能：
//! 1. 概率性折扣 - 每次购买有概率获得折扣
//! 2. VIP 等级影响 - VIP 等级越高，折扣概率越大
//! 3. 连续购买奖励 - 连续购买增加折扣概率
//! 4. 折扣幅度变化 - 根据 VIP 等级调整折扣幅度

#![no_std]
#![no_main]

// 简单的 panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// 插件名称
const PLUGIN_NAME: &[u8] = b"DiscountPlugin\0";

#[no_mangle]
pub extern "C" fn wasm_plugin_name() -> *const u8 {
    PLUGIN_NAME.as_ptr()
}

#[no_mangle]
pub extern "C" fn wasm_plugin_name_len() -> usize {
    PLUGIN_NAME.len() - 1
}

// 插件版本
const PLUGIN_VERSION: &[u8] = b"1.0.0\0";

#[no_mangle]
pub extern "C" fn wasm_plugin_version() -> *const u8 {
    PLUGIN_VERSION.as_ptr()
}

#[no_mangle]
pub extern "C" fn wasm_plugin_version_len() -> usize {
    PLUGIN_VERSION.len() - 1
}

// 折扣状态
struct DiscountState {
    vip_level: u32,
    consecutive_purchases: u32,
    total_purchases: u32,
    last_discount_amount: u32, // 0-100 表示百分比
}

impl DiscountState {
    const fn new() -> Self {
        Self {
            vip_level: 1,
            consecutive_purchases: 0,
            total_purchases: 0,
            last_discount_amount: 0,
        }
    }

    // 计算折扣概率 (0.0-1.0)
    fn calculate_discount_chance(&self) -> u32 {
        // 基础概率 20%
        let mut chance = 20;

        // VIP 加成: 每级 +5%
        chance += self.vip_level * 5;

        // 连续购买加成: 每次 +2%
        chance += self.consecutive_purchases * 2;

        // 最大概率 80%
        if chance > 80 {
            chance = 80;
        }

        chance
    }

    // 计算折扣幅度 (10-50%)
    fn calculate_discount_amount(&self) -> u32 {
        // 基础折扣 10%
        let mut amount = 10;

        // VIP 加成: 每级 +5%
        amount += self.vip_level * 5;

        // 最大折扣 50%
        if amount > 50 {
            amount = 50;
        }

        amount
    }

    // 尝试获取折扣
    fn try_get_discount(&mut self, random_value: u32) -> Option<u32> {
        let chance = self.calculate_discount_chance();

        // random_value 是 0-99 的随机数
        if random_value < chance {
            let amount = self.calculate_discount_amount();
            self.last_discount_amount = amount;
            Some(amount)
        } else {
            self.last_discount_amount = 0;
            None
        }
    }

    // 增加 VIP 等级
    fn increase_vip(&mut self) {
        self.vip_level += 1;
    }

    // 记录购买
    fn record_purchase(&mut self) {
        self.total_purchases += 1;
        self.consecutive_purchases += 1;

        // 每 10 次购买自动升级 VIP
        if self.total_purchases % 10 == 0 {
            self.increase_vip();
        }
    }

    // 重置连续购买（比如治疗后）
    fn reset_consecutive(&mut self) {
        self.consecutive_purchases = 0;
    }
}

// 全局状态
static mut DISCOUNT_STATE: DiscountState = DiscountState::new();

// ABI 函数声明
extern "C" {
    fn wasm_plugin_set_data(
        key_ptr: *const u8,
        key_len: usize,
        data_ptr: *const u8,
        data_len: usize,
    );
}

// 导出折扣状态到主机
fn export_discount_state() {
    unsafe {
        // 导出 VIP 等级
        let key = b"vip_level";
        let value = DISCOUNT_STATE.vip_level.to_le_bytes();
        wasm_plugin_set_data(key.as_ptr(), key.len(), value.as_ptr(), value.len());

        // 导出连续购买次数
        let key = b"consecutive_purchases";
        let value = DISCOUNT_STATE.consecutive_purchases.to_le_bytes();
        wasm_plugin_set_data(key.as_ptr(), key.len(), value.as_ptr(), value.len());

        // 导出总购买次数
        let key = b"total_purchases";
        let value = DISCOUNT_STATE.total_purchases.to_le_bytes();
        wasm_plugin_set_data(key.as_ptr(), key.len(), value.as_ptr(), value.len());

        // 导出折扣概率
        let key = b"discount_chance";
        let value = DISCOUNT_STATE.calculate_discount_chance().to_le_bytes();
        wasm_plugin_set_data(key.as_ptr(), key.len(), value.as_ptr(), value.len());

        // 导出当前折扣幅度
        let key = b"discount_amount";
        let value = DISCOUNT_STATE.last_discount_amount.to_le_bytes();
        wasm_plugin_set_data(key.as_ptr(), key.len(), value.as_ptr(), value.len());
    }
}

/// Called every frame
#[no_mangle]
pub extern "C" fn wasm_plugin_on_tick(_entity_id: u64) {
    // 定期导出状态
    static mut TICK_COUNT: u64 = 0;
    unsafe {
        TICK_COUNT += 1;
        if TICK_COUNT % 30 == 0 {
            export_discount_state();
        }
    }
}

/// Called when an event occurs
#[no_mangle]
pub extern "C" fn wasm_plugin_on_event(
    entity_id: u64,
    event_ptr: *const u8,
    event_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) {
    // 读取事件字符串
    let event_str = if !event_ptr.is_null() && event_len > 0 {
        unsafe {
            let bytes = core::slice::from_raw_parts(event_ptr, event_len);
            core::str::from_utf8(bytes).unwrap_or("")
        }
    } else {
        ""
    };

    // 读取数据字符串
    let data_str = if !data_ptr.is_null() && data_len > 0 {
        unsafe {
            let bytes = core::slice::from_raw_parts(data_ptr, data_len);
            core::str::from_utf8(bytes).unwrap_or("")
        }
    } else {
        ""
    };

    // 处理事件
    unsafe {
        if event_str == "purchase" {
            // 记录购买
            DISCOUNT_STATE.record_purchase();

            // 生成随机数（简单伪随机，基于时间或实体ID）
            let random_value =
                (entity_id.wrapping_mul(1103515245).wrapping_add(12345) % 100) as u32;

            // 尝试获取折扣
            if let Some(discount) = DISCOUNT_STATE.try_get_discount(random_value) {
                // 导出折扣信息
                let key = b"last_discount";
                let value = discount.to_le_bytes();
                wasm_plugin_set_data(key.as_ptr(), key.len(), value.as_ptr(), value.len());
            }

            export_discount_state();
        } else if event_str == "heal" {
            // 治疗重置连续购买
            DISCOUNT_STATE.reset_consecutive();
            export_discount_state();
        } else if event_str == "check_discount" {
            // 查询当前折扣状态
            export_discount_state();
        }
    }

    let _ = data_str;
}

/// Called when the plugin is loaded
#[no_mangle]
pub extern "C" fn wasm_plugin_on_load() {
    // 初始化折扣状态
    unsafe {
        DISCOUNT_STATE = DiscountState::new();
    }
    export_discount_state();
}

/// Called when the plugin is unloaded
#[no_mangle]
pub extern "C" fn wasm_plugin_on_unload() {
    // 清理逻辑
}

/// Called when an error occurs
#[no_mangle]
pub extern "C" fn wasm_plugin_on_error(error_code: u32) {
    let _ = error_code;
}
