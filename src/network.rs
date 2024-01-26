use std::time::Duration;
use tcp_handler::bytes::{Buf, BytesMut};
use tcp_handler::common::{AesCipher, PacketError, StarterError};
use tcp_handler::compress_encrypt::{client_init, client_start};
use tcp_handler::flate2::Compression;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::time::timeout;
use crate::ClientFactory;
use crate::config::{get_connect_sec, get_idle_sec};

/// Error in send/recv message.
#[derive(Error, Debug)]
pub enum NetworkError {
    /// Sending/receiving timeout. See [`get_idle_sec`].
    #[error("Network timeout: {} after {1} sec.", match .0 { 1 => "Sending", 2 => "Receiving", _ => "Connecting" })]
    Timeout(u8, u64),

    /// During init protocol. From [`tcp_handler`][crate::tcp_handler].
    #[error("During io packet: {0:?}")]
    StarterError(#[from] StarterError),

    /// During io packet. From [`tcp_handler`][crate::tcp_handler].
    #[error("During io packet: {0:?}")]
    PacketError(#[from] PacketError),

    /// During read/write data from [`bytes`][crate::bytes].
    #[error("During read/write data: {0:?}")]
    BufError(#[from] std::io::Error),

    /// Broken cipher. This is a fatal error.
    ///
    /// When another error returned during send/recv, the client is broken because no [`AesCipher`] received.
    /// In order not to panic, the client marks as broken and this error is returned.
    #[error("Broken client.")]
    BrokenCipher(),
}

#[inline]
pub(crate) async fn send<W: AsyncWriteExt + Unpin + Send, B: Buf + Send>(stream: &mut W, message: &mut B, cipher: AesCipher, level: Compression) -> Result<AesCipher, NetworkError> {
    let idle = get_idle_sec();
    timeout(Duration::from_secs(idle), tcp_handler::compress_encrypt::send(stream, message, cipher, level)).await
        .map_err(|_| NetworkError::Timeout(1, idle))?.map_err(|e| e.into())
}

#[inline]
pub(crate) async fn recv<R: AsyncReadExt + Unpin + Send>(stream: &mut R, cipher: AesCipher) -> Result<(BytesMut, AesCipher), NetworkError> {
    let idle = get_idle_sec();
    timeout(Duration::from_secs(idle), tcp_handler::compress_encrypt::recv(stream, cipher)).await
        .map_err(|_| NetworkError::Timeout(2, idle))?.map_err(|e| e.into())
}

pub(super) async fn start_client<C: ClientFactory<T> + ?Sized, A: ToSocketAddrs + Send, T: From<(TcpStream, AesCipher)>>(c: &C, addr: A) -> Result<(TcpStream, AesCipher), NetworkError> {
    let mut stream = TcpStream::connect(addr).await.map_err(|e| StarterError::from(e))?;
    let connect = get_connect_sec();
    let cipher = timeout(Duration::from_secs(connect), async {
        let init = client_init(&mut stream, c.get_identifier(), c.get_version()).await;
        client_start(&mut stream, init).await
    }).await.map_err(|_| NetworkError::Timeout(3, connect))??;
    Ok((stream, cipher))
}
