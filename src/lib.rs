#![allow(clippy::type_complexity)]

pub mod hooks;
pub mod network;
pub mod plugins;

pub mod components;
pub mod events;
pub mod systems;

#[cfg(feature = "wasm-plugin")]
pub mod wasm;

pub mod prelude;

use bevy::prelude::SystemSet;
use bevy::prelude::*;

use hooks::HookRegistry;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FrameworkSet {
    Input,
    Process,
    Output,
}

pub fn configure_backend() {
    if std::env::var("WGPU_BACKEND").is_err() {
        std::env::set_var("WGPU_BACKEND", "vulkan");
    }
}

pub struct FrameworkPlugin;

impl Plugin for FrameworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HookRegistry>()
            .init_resource::<network::NetworkConfig>()
            .configure_sets(
                Update,
                (
                    FrameworkSet::Input,
                    FrameworkSet::Process,
                    FrameworkSet::Output,
                )
                    .chain(),
            );
    }
}
