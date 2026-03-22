use super::WasmPlugin;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct WasmPluginHost {
    plugins: Arc<Mutex<Vec<Box<dyn WasmPlugin>>>>,
}

impl Default for WasmPluginHost {
    fn default() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WasmPluginHost {
    pub fn register(&self, plugin: Box<dyn WasmPlugin>) {
        let mut plugins = self.plugins.lock().unwrap();
        info!("Registered WASM plugin: {}", plugin.name());
        plugins.push(plugin);
    }

    pub fn trigger_on_tick(&self, entity_id: u64) {
        let plugins = self.plugins.lock().unwrap();
        for plugin in plugins.iter() {
            plugin.on_tick(entity_id);
        }
    }

    pub fn trigger_on_event(&self, entity_id: u64, event: &str, data: &str) {
        let plugins = self.plugins.lock().unwrap();
        for plugin in plugins.iter() {
            plugin.on_event(entity_id, event, data);
        }
    }

    pub fn plugin_count(&self) -> usize {
        self.plugins.lock().unwrap().len()
    }
}
