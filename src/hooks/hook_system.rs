use bevy::prelude::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type HookKey = Cow<'static, str>;

#[derive(Clone)]
pub struct HookContext {
    pub entity: Entity,
}

pub type HookCallback = Arc<dyn Fn(&HookContext) + Send + Sync>;

#[derive(Resource, Clone)]
pub struct HookRegistry {
    hooks: Arc<Mutex<HashMap<HookKey, Vec<HookCallback>>>>,
}

impl Default for HookRegistry {
    fn default() -> Self {
        Self {
            hooks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl HookRegistry {
    pub fn register(&self, key: impl Into<HookKey>, callback: HookCallback) {
        let mut hooks = self.hooks.lock().unwrap();
        hooks.entry(key.into()).or_default().push(callback);
    }

    pub fn register_fn<F>(&self, key: impl Into<HookKey>, f: F)
    where
        F: Fn(&HookContext) + Send + Sync + 'static,
    {
        self.register(key, Arc::new(f));
    }

    pub fn trigger(&self, key: &str, ctx: &HookContext) {
        let hooks = self.hooks.lock().unwrap();
        if let Some(callbacks) = hooks.get(key) {
            for cb in callbacks {
                cb(ctx);
            }
        }
    }

    pub fn clear(&self, key: &str) {
        let mut hooks = self.hooks.lock().unwrap();
        hooks.remove(key);
    }

    pub fn clear_all(&self) {
        let mut hooks = self.hooks.lock().unwrap();
        hooks.clear();
    }

    pub fn count(&self, key: &str) -> usize {
        let hooks = self.hooks.lock().unwrap();
        hooks.get(key).map_or(0, |v| v.len())
    }
}
