//! The basic trait for client.

use std::io::{Error, ErrorKind};
use async_trait::async_trait;
use tcp_handler::bytes::{Buf, BufMut, BytesMut};
use tcp_handler::flate2::Compression;
use tcp_handler::variable_len_reader::{VariableReader, VariableWriter};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::mutable_cipher::MutableCipher;
use crate::network::{NetworkError, recv, send};

/// The basic trait for client.
/// See [`ClientFactory`][crate::ClientFactory] for example.
#[async_trait]
pub trait ClientBase<R, W> where R: AsyncReadExt + Unpin + Send, W: AsyncWriteExt + Unpin + Send {
    /// Get the receiver and mutable cipher.
    /// # Note:
    /// This should be a const expr.
    fn get_receiver<'a>(&'a mut self) -> (&'a mut R, &MutableCipher);

    /// Get the sender and mutable cipher.
    /// # Note:
    /// This should be a const expr.
    fn get_sender<'a>(&'a mut self) -> (&'a mut W, &MutableCipher);

    /// Send a message to the server.
    async fn send<B: Buf + Send>(&mut self, message: &mut B) -> Result<(), NetworkError> {
        let (sender, mutable_cipher) = self.get_sender();
        let (cipher, guard) = mutable_cipher.get().await?;
        match send(sender, message, cipher, Compression::default()).await {
            Ok(cipher) => { mutable_cipher.reset(guard, Some(cipher)); Ok(()) }
            Err(e) => { mutable_cipher.reset(guard, None); Err(e) }
        }
    }

    /// Recv a message from the server.
    async fn recv(&mut self) -> Result<BytesMut, NetworkError> {
        let (receiver, mutable_cipher) = self.get_receiver();
        let (cipher, guard) = mutable_cipher.get().await?;
        match recv(receiver, cipher).await {
            Ok((response, cipher)) => { mutable_cipher.reset(guard, Some(cipher)); Ok(response) }
            Err(e) => { mutable_cipher.reset(guard, None); Err(e) }
        }
    }

    /// A shortcut of send and recv message.
    async fn send_recv<B: Buf + Send>(&mut self, message: &mut B) -> Result<BytesMut, NetworkError> {
        self.send(message).await?;
        self.recv().await
    }

    /// Check if the function is supported by the server.
    ///
    /// This is corresponding to `tcp-server` crate.
    async fn check_func(&mut self, func: &str) -> Result<(), NetworkError> {
        let mut writer = BytesMut::new().writer();
        writer.write_string(func)?;
        let mut reader = self.send_recv(&mut writer.into_inner()).await?.reader();
        if reader.read_bool()? {
            Ok(())
        } else {
            Err(NetworkError::BufError(Error::new(ErrorKind::Other, format!("func is not available: {}", func))))
        }
    }
}
