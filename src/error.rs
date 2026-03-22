/// Errors that can occur within the pet-rs framework.
#[derive(Debug, thiserror::Error)]
pub enum FrameworkError {
    /// A mutex was poisoned due to a panic in another thread.
    #[error("resource lock poisoned")]
    LockPoisoned,

    /// A network channel receiver has been dropped.
    #[error("channel closed: {0}")]
    ChannelClosed(String),

    /// An error occurred within a WASM plugin.
    #[error("plugin error: {0}")]
    Plugin(String),
}
