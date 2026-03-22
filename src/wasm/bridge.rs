use super::WasmPlugin;
use crate::error::FrameworkError;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

/// Host-side container for registered WASM plugins.
///
/// Plugins are registered once at startup and called during gameplay.
/// Uses `std::sync::Mutex` because all access is from synchronous
/// Bevy systems (never across an `.await`).
#[derive(Resource, Default)]
pub struct WasmPluginHost {
    plugins: Arc<Mutex<Vec<Box<dyn WasmPlugin>>>>,
}

impl WasmPluginHost {
    pub fn register(&self, plugin: Box<dyn WasmPlugin>) -> Result<(), FrameworkError> {
        let mut plugins = self
            .plugins
            .lock()
            .map_err(|_| FrameworkError::LockPoisoned)?;
        info!("Registered WASM plugin: {}", plugin.name());
        plugins.push(plugin);
        Ok(())
    }

    pub fn trigger_on_tick(&self, entity_id: u64) -> Result<(), FrameworkError> {
        let plugins = self
            .plugins
            .lock()
            .map_err(|_| FrameworkError::LockPoisoned)?;
        let id = super::WasmEntityId(entity_id);
        for plugin in plugins.iter() {
            plugin.on_tick(id);
        }
        Ok(())
    }

    pub fn trigger_on_event(
        &self,
        entity_id: u64,
        event: &str,
        data: &str,
    ) -> Result<(), FrameworkError> {
        let plugins = self
            .plugins
            .lock()
            .map_err(|_| FrameworkError::LockPoisoned)?;
        let id = super::WasmEntityId(entity_id);
        for plugin in plugins.iter() {
            plugin.on_event(id, event, data);
        }
        Ok(())
    }

    pub fn plugin_count(&self) -> Result<usize, FrameworkError> {
        let plugins = self
            .plugins
            .lock()
            .map_err(|_| FrameworkError::LockPoisoned)?;
        Ok(plugins.len())
    }
}
