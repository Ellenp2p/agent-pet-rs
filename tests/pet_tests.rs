use bevy::prelude::*;
use pet_rs::prelude::*;

#[cfg(test)]
mod hook_tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_hook_register_and_trigger() {
        let registry = HookRegistry::default();
        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();

        registry.register_fn("on_spawn", move |_ctx| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        registry.trigger(
            "on_spawn",
            &HookContext {
                entity: Entity::from_raw(0),
            },
        );

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_hook_multiple_subscribers() {
        let registry = HookRegistry::default();
        let counter = Arc::new(AtomicU32::new(0));

        for _ in 0..3 {
            let c = counter.clone();
            registry.register_fn("on_tick", move |_ctx| {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        registry.trigger(
            "on_tick",
            &HookContext {
                entity: Entity::from_raw(0),
            },
        );

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_hook_separate_keys() {
        let registry = HookRegistry::default();
        let a = Arc::new(AtomicU32::new(0));
        let b = Arc::new(AtomicU32::new(0));

        let a2 = a.clone();
        let b2 = b.clone();

        registry.register_fn("key_a", move |_| {
            a2.fetch_add(1, Ordering::SeqCst);
        });
        registry.register_fn("key_b", move |_| {
            b2.fetch_add(10, Ordering::SeqCst);
        });

        let ctx = HookContext {
            entity: Entity::from_raw(0),
        };
        registry.trigger("key_a", &ctx);
        registry.trigger("key_b", &ctx);

        assert_eq!(a.load(Ordering::SeqCst), 1);
        assert_eq!(b.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_hook_clear() {
        let registry = HookRegistry::default();
        registry.register_fn("test", |_ctx| {});
        assert_eq!(registry.count("test"), 1);

        registry.clear("test");
        assert_eq!(registry.count("test"), 0);
    }

    #[test]
    fn test_hook_no_trigger_wrong_key() {
        let registry = HookRegistry::default();
        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();

        registry.register_fn("real_key", move |_| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        registry.trigger(
            "wrong_key",
            &HookContext {
                entity: Entity::from_raw(0),
            },
        );

        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
}

#[cfg(test)]
mod network_channel_tests {
    use pet_rs::network::NetworkChannel;

    #[test]
    fn test_generic_channel_i32() {
        let channel: NetworkChannel<i32> = NetworkChannel::default();
        channel.send(42).unwrap();
        let msgs = channel.drain_outgoing();
        assert_eq!(msgs, vec![42]);
    }

    #[test]
    fn test_generic_channel_string() {
        let channel: NetworkChannel<String> = NetworkChannel::default();
        channel.send("hello".into()).unwrap();
        channel.send("world".into()).unwrap();
        let msgs = channel.drain_outgoing();
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0], "hello");
        assert_eq!(msgs[1], "world");
    }

    #[test]
    fn test_channel_incoming() {
        let channel: NetworkChannel<u64> = NetworkChannel::default();
        channel.inject_incoming(100).unwrap();
        channel.inject_incoming(200).unwrap();
        let msgs = channel.drain_incoming();
        assert_eq!(msgs, vec![100, 200]);
    }

    #[test]
    fn test_channel_drain_clears() {
        let channel: NetworkChannel<&str> = NetworkChannel::default();
        channel.send("a").unwrap();
        let first = channel.drain_outgoing();
        assert_eq!(first.len(), 1);

        let second = channel.drain_outgoing();
        assert!(second.is_empty());
    }

    #[test]
    fn test_channel_with_struct() {
        #[derive(Debug, Clone, PartialEq)]
        struct Dto {
            id: u64,
            value: f32,
        }

        let channel: NetworkChannel<Dto> = NetworkChannel::default();
        channel.send(Dto { id: 1, value: 3.14 }).unwrap();
        let msgs = channel.drain_outgoing();
        assert_eq!(msgs[0].id, 1);
    }
}

#[cfg(test)]
mod config_tests {
    use pet_rs::network::NetworkConfig;

    #[test]
    fn test_default_config() {
        let config = NetworkConfig::default();
        assert_eq!(config.server_url, "http://localhost:3000");
        assert!(!config.use_websocket);
    }
}
