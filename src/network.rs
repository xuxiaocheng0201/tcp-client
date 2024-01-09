use std::io::{Error, ErrorKind};
use std::time::Duration;
use tcp_handler::bytes::{Buf, BytesMut};
use tcp_handler::common::{AesCipher, PacketError, StarterError};
use tcp_handler::compress_encrypt::{client_init, client_start};
use tcp_handler::flate2::Compression;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::time::timeout;
use crate::ClientFactory;
use crate::configuration::get_connect_sec;

#[inline]
pub async fn send<W: AsyncWriteExt + Unpin + Send, B: Buf>(stream: &mut W, message: &mut B, cipher: AesCipher, level: Compression) -> Result<AesCipher, PacketError> {
    tcp_handler::compress_encrypt::send(stream, message, cipher, level).await
}

#[inline]
pub async fn recv<R: AsyncReadExt + Unpin + Send>(stream: &mut R, cipher: AesCipher, time: Option<Duration>) -> Result<(BytesMut, AesCipher), PacketError> {
    if let Some(time) = time {
        timeout(time, tcp_handler::compress_encrypt::recv(stream, cipher)).await
            .map_err(|_| PacketError::IO(Error::new(ErrorKind::TimedOut, format!("Recv timeout: {:?}", time))))?
    } else {
        tcp_handler::compress_encrypt::recv(stream, cipher).await
    }
}

pub(super) async fn start_client<C: ClientFactory<T> + ?Sized, A: ToSocketAddrs + Send, T: From<(TcpStream, AesCipher)>>(c: &C, addr: A) -> Result<(TcpStream, AesCipher), StarterError> {
    let mut stream = TcpStream::connect(addr).await?;
    let connect_sec = get_connect_sec();
    let cipher = timeout(Duration::from_secs(connect_sec), async {
        let init = client_init(&mut stream, c.get_identifier(), c.get_version()).await;
        client_start(&mut stream, init).await
    }).await.map_err(|_| Error::new(ErrorKind::TimedOut, format!("Connect timeout {}, {} sec.", stream.peer_addr()?, connect_sec)))??;
    Ok((stream, cipher))
}
