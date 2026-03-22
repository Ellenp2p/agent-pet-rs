//! WASM Reader Plugin
//!
//! This plugin demonstrates inter-plugin communication by reading data from other plugins.

#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const PLUGIN_NAME: &[u8] = b"ReaderPlugin\0";

#[no_mangle]
pub extern "C" fn wasm_plugin_name() -> *const u8 {
    PLUGIN_NAME.as_ptr()
}

#[no_mangle]
pub extern "C" fn wasm_plugin_name_len() -> usize {
    PLUGIN_NAME.len() - 1
}

// Plugin state to track requests
struct ReaderState {
    last_purchase_count: u32,
    request_sent: bool,
}

static mut STATE: ReaderState = ReaderState {
    last_purchase_count: 0,
    request_sent: false,
};

#[no_mangle]
pub extern "C" fn wasm_plugin_on_tick(entity_id: u64) {
    // Every few ticks, request data from stats_plugin
    unsafe {
        if !STATE.request_sent && entity_id % 10 == 0 {
            // Request purchase count from stats_plugin
            // This would be done via a special event in a real implementation
            STATE.request_sent = true;
        }
    }
}

#[no_mangle]
pub extern "C" fn wasm_plugin_on_event(
    entity_id: u64,
    event_ptr: *const u8,
    event_len: usize,
    data_ptr: *const u8,
    data_len: usize,
) {
    // Read event string
    let event_str = if event_ptr != core::ptr::null() && event_len > 0 {
        unsafe {
            let bytes = core::slice::from_raw_parts(event_ptr, event_len);
            core::str::from_utf8(bytes).unwrap_or("")
        }
    } else {
        ""
    };

    // Check if this is a response from stats_plugin
    if event_str.starts_with("plugin_response:stats_plugin:") {
        // Parse the response data (hex string)
        let data_str = if data_ptr != core::ptr::null() && data_len > 0 {
            unsafe {
                let bytes = core::slice::from_raw_parts(data_ptr, data_len);
                core::str::from_utf8(bytes).unwrap_or("")
            }
        } else {
            ""
        };

        // In a real implementation, you would parse the hex data
        // For now, we just log that we received a response
        unsafe {
            STATE.last_purchase_count = 0; // Would parse from data_str
        }
    }

    let _ = entity_id;
}
