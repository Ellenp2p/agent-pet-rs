use bevy::prelude::*;

pub struct WasmPluginBevy;

impl Plugin for WasmPluginBevy {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::wasm::WasmPluginHost>();
    }
}
