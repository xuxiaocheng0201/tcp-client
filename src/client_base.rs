use std::io::{Error, ErrorKind};
use std::time::Duration;
use async_trait::async_trait;
use tcp_handler::bytes::{Buf, BufMut, BytesMut};
use tcp_handler::common::PacketError;
use tcp_handler::flate2::Compression;
use tcp_handler::variable_len_reader::{VariableReadable, VariableWritable};
use tcp_handler::variable_len_reader::asynchronous::AsyncVariableReadable;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::mutable_cipher::MutableCipher;
use crate::network::{recv, send};
use crate::configuration::get_idle_sec;

#[async_trait]
pub trait ClientBase<R, W> where R: AsyncReadExt + Unpin + Send, W: AsyncWriteExt + Unpin + Send {
    /*const*/ fn get_receiver<'a>(&'a mut self) -> (&'a mut R, &MutableCipher);

    /*const*/ fn get_sender<'a>(&'a mut self) -> (&'a mut W, &MutableCipher);

    async fn send<B: Buf + Send>(&mut self, message: &mut B) -> Result<(), PacketError> {
        let (sender, mutable_cipher) = self.get_sender();
        let (cipher, guard) = mutable_cipher.get().await;
        let cipher = send(sender, message, cipher, Compression::default()).await?;
        mutable_cipher.reset(guard, cipher);
        Ok(())
    }

    async fn recv(&mut self) -> Result<BytesMut, PacketError> {
        let (receiver, mutable_cipher) = self.get_receiver();
        let (cipher, guard) = mutable_cipher.get().await;
        let (response, cipher) = recv(receiver, cipher, Some(Duration::from_secs(get_idle_sec()))).await?;
        mutable_cipher.reset(guard, cipher);
        Ok(response)
    }

    async fn send_recv<B: Buf + Send>(&mut self, message: &mut B) -> Result<BytesMut, PacketError> {
        self.send(message).await?;
        Ok(self.recv().await?)
    }

    async fn check_func(&mut self, func: &str) -> Result<(), PacketError> {
        let mut request = BytesMut::new().writer();
        request.write_string(func)?;
        self.send(&mut request.into_inner()).await?;
        if self.get_receiver().0.read_bool()? {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, format!("func is not available: {}", func)).into())
        }
    }
}
