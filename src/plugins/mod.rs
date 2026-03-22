#[cfg(feature = "wasm-plugin")]
pub mod wasm_plugin;

#[cfg(feature = "wasm-plugin")]
pub use wasm_plugin::WasmPluginBevy;
