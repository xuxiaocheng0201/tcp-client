//! Global configuration for this crate.
//!
//! You may change the configuration by calling `set_config` function.
//!
//! # Example
//! ```rust
//! use tcp_client::config::{ClientConfig, set_config};
//!
//! # fn main() {
//! set_config(ClientConfig::default());
//! # }
//! ```

use std::sync::RwLock;

/// Global configuration.
///
/// # Example
/// ```rust
/// use tcp_client::config::ClientConfig;
///
/// # fn main() {
/// let config = ClientConfig::default();
/// # let _ = config;
/// # }
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ClientConfig {
    /// `connect_sec` is the timeout of connecting to the server.
    /// Default value is `30`.
    ///
    /// # Example
    /// ```rust
    /// use tcp_client::config::ClientConfig;
    ///
    /// # fn main() {
    /// let config = ClientConfig {
    ///     connect_sec: 30,
    ///     ..ClientConfig::default()
    /// };
    /// # let _ = config;
    /// # }
    /// ```
    pub connect_sec: u64,

    /// `idle_sec` is the timeout of sending/receiving from the server.
    /// This is only used after sending a packet.
    /// Default value is `30`.
    ///
    /// # Example
    /// ```rust
    /// use tcp_client::config::ClientConfig;
    ///
    /// # fn main() {
    /// let config = ClientConfig {
    ///     idle_sec: 30,
    ///     ..ClientConfig::default()
    /// };
    /// # let _ = config;
    /// # }
    /// ```
    pub idle_sec: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            connect_sec: 30,
            idle_sec: 30,
        }
    }
}

static CONFIG: RwLock<ClientConfig> = RwLock::new(ClientConfig {
    connect_sec: 30,
    idle_sec: 30,
});

/// Sets the global configuration.
///
/// This function is recommended to only be called once during initialization.
///
/// # Example
/// ```rust
/// use tcp_client::config::{ClientConfig, set_config};
///
/// # fn main() {
/// set_config(ClientConfig::default());
/// # }
/// ```
#[inline]
pub fn set_config(config: ClientConfig) {
    let mut c = CONFIG.write().unwrap();
    *c = config;
}

/// Gets the global configuration.
///
/// # Example
/// ```rust
/// use tcp_client::config::get_config;
///
/// # fn main() {
/// let config = get_config();
/// # let _ = config;
/// # }
/// ```
#[inline]
pub fn get_config() -> ClientConfig {
    let c = CONFIG.read().unwrap();
    (*c).clone()
}

/// A cheaper shortcut of
/// ```rust,ignore
/// get_config().connect_sec
/// ```
#[inline]
pub fn get_connect_sec() -> u64 {
    let c = CONFIG.read().unwrap();
    (*c).connect_sec
}

/// A cheaper shortcut of
/// ```rust,ignore
/// get_config().idle_sec
/// ```
#[inline]
pub fn get_idle_sec() -> u64 {
    let c = CONFIG.read().unwrap();
    (*c).idle_sec
}
