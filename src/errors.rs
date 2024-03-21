//! The common errors.

use std::time::Duration;
use tcp_handler::protocols::common::{PacketError, StarterError};
use thiserror::Error;

/// Error when send/recv messages.
#[derive(Error, Debug)]
pub enum Error {
    /// Sending/receiving timeout. See [`get_receive_timeout`].
    #[error("Network timeout: {} after {1:?}.", if *.0 { "Connecting" } else { "Receiving" })]
    Timeout(bool, Duration),

    /// During init protocol. From [`tcp_handler`][crate::tcp_handler].
    #[error("During io packet: {0}")]
    StarterError(#[from] StarterError),

    /// During io packet. From [`tcp_handler`][crate::tcp_handler].
    #[error("During io packet: {0}")]
    PacketError(#[from] PacketError),

    /// During read/write data from [`bytes`][crate::bytes].
    #[error("During read/write data: {0}")]
    DataError(#[from] std::io::Error),

    /// During check if the server supports the function.
    #[error("The server is not available.")]
    ServerDenied,
}

/// Type alias of `std::result::Result<T, Error>`
pub type Result<T> = std::result::Result<T, Error>;
