use bevy::prelude::*;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Resource, Clone)]
pub struct NetworkConfig {
    pub server_url: String,
    pub poll_interval_secs: f32,
    pub use_websocket: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            server_url: String::from("http://localhost:3000"),
            poll_interval_secs: 5.0,
            use_websocket: false,
        }
    }
}

#[derive(Resource)]
pub struct NetworkChannel<T>
where
    T: Send + Sync + 'static,
{
    outgoing_tx: Arc<mpsc::UnboundedSender<T>>,
    outgoing_rx: Arc<std::sync::Mutex<mpsc::UnboundedReceiver<T>>>,
    incoming_tx: Arc<mpsc::UnboundedSender<T>>,
    incoming_rx: Arc<std::sync::Mutex<mpsc::UnboundedReceiver<T>>>,
    _marker: PhantomData<T>,
}

impl<T> Default for NetworkChannel<T>
where
    T: Send + Sync + 'static,
{
    fn default() -> Self {
        let (out_tx, out_rx) = mpsc::unbounded_channel();
        let (in_tx, in_rx) = mpsc::unbounded_channel();
        Self {
            outgoing_tx: Arc::new(out_tx),
            outgoing_rx: Arc::new(std::sync::Mutex::new(out_rx)),
            incoming_tx: Arc::new(in_tx),
            incoming_rx: Arc::new(std::sync::Mutex::new(in_rx)),
            _marker: PhantomData,
        }
    }
}

impl<T> NetworkChannel<T>
where
    T: Send + Sync + 'static,
{
    pub fn send(&self, msg: T) -> Result<(), String> {
        self.outgoing_tx
            .send(msg)
            .map_err(|e| format!("Send failed: {}", e))
    }

    pub fn drain_outgoing(&self) -> Vec<T> {
        let mut rx = self.outgoing_rx.lock().unwrap();
        let mut msgs = Vec::new();
        while let Ok(msg) = rx.try_recv() {
            msgs.push(msg);
        }
        msgs
    }

    pub fn inject_incoming(&self, msg: T) -> Result<(), String> {
        self.incoming_tx
            .send(msg)
            .map_err(|e| format!("Inject failed: {}", e))
    }

    pub fn drain_incoming(&self) -> Vec<T> {
        let mut rx = self.incoming_rx.lock().unwrap();
        let mut msgs = Vec::new();
        while let Ok(msg) = rx.try_recv() {
            msgs.push(msg);
        }
        msgs
    }
}
