//! A wrapper for [`AesCipher`] to make it easier to update.

use tcp_handler::common::AesCipher;
use tokio::sync::{Mutex, MutexGuard};
use crate::network::NetworkError;

pub struct MutableCipher {
    cipher: Mutex<Option<AesCipher>>,
}

impl MutableCipher {
    pub fn new(cipher: AesCipher) -> MutableCipher {
        MutableCipher {
            cipher: Mutex::new(Some(cipher))
        }
    }

    pub fn into_inner(self) -> AesCipher {
        self.cipher.into_inner().unwrap()
    }

    pub(crate) async fn get<'a>(&'a self) -> Result<(AesCipher, MutexGuard<Option<AesCipher>>), NetworkError> {
        let mut guard = self.cipher.lock().await;
        let cipher = (*guard).take().ok_or(NetworkError::BrokenCipher())?;
        Ok((cipher, guard))
    }

    #[inline]
    pub(crate) fn reset(&self, mut guard: MutexGuard<Option<AesCipher>>, cipher: Option<AesCipher>) {
        (*guard) = cipher;
    }
}
