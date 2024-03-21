//! Global configuration for this [`crate`].
//!
//! You may change the configuration by calling [`set_config`] function.
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
use std::time::Duration;
use once_cell::sync::Lazy;

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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(default))]
pub struct ClientConfig {
    /// `connect_timeout` is the timeout of connecting to the server.
    ///
    /// The default value is `10s`.
    ///
    /// # Example
    /// ```rust
    /// use tcp_client::config::ClientConfig;
    ///
    /// # fn main() {
    /// use std::time::Duration;
    /// let config = ClientConfig {
    ///     connect_timeout: Duration::from_secs(10),
    ///     ..ClientConfig::default()
    /// };
    /// # let _ = config;
    /// # }
    /// ```
    pub connect_timeout: Duration,

    /// `receive_timeout` is the timeout of receiving from the server.
    ///
    /// The default value is `30s`.
    ///
    /// # Example
    /// ```rust
    /// use tcp_client::config::ClientConfig;
    ///
    /// # fn main() {
    /// use std::time::Duration;
    /// let config = ClientConfig {
    ///     receive_timeout: Duration::from_secs(30),
    ///     ..ClientConfig::default()
    /// };
    /// # let _ = config;
    /// # }
    /// ```
    pub receive_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            receive_timeout: Duration::from_secs(30),
        }
    }
}

static CONFIG: Lazy<RwLock<ClientConfig>> = Lazy::new(|| RwLock::new(ClientConfig::default()));

/// Set the global configuration.
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

/// Get the global configuration.
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
/// get_config().connect_timeout
/// ```
#[inline]
pub fn get_connect_timeout() -> Duration {
    let c = CONFIG.read().unwrap();
    (*c).connect_timeout
}

/// A cheaper shortcut of
/// ```rust,ignore
/// get_config().idle_sec
/// ```
#[inline]
pub fn get_receive_timeout() -> Duration {
    let c = CONFIG.read().unwrap();
    (*c).receive_timeout
}
