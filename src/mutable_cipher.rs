//! A wrapper for [`AesCipher`] to make it easier to update.

use std::fmt::{Debug, Formatter};
use tcp_handler::common::AesCipher;
use tokio::sync::{Mutex, MutexGuard};
use crate::network::NetworkError;

/// A wrapper for [`AesCipher`].
/// Used in [`ClientBase`][crate::client_base::ClientBase].
pub struct MutableCipher {
    cipher: Mutex<Option<AesCipher>>,
}

impl Debug for MutableCipher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutableCipher")
            .field("cipher", self.cipher.try_lock()
                .map_or_else(|_| &"<locked>",
                             |inner| if (*inner).is_some() { &"<unlocked>" } else { &"<broken>" }))
            .finish()
    }
}

impl MutableCipher {
    /// Create a wrapped [`AesCipher`].
    pub fn new(cipher: AesCipher) -> Self {
        Self {
            cipher: Mutex::new(Some(cipher))
        }
    }

    /// Get the inner [`AesCipher`].
    /// Not recommended to use.
    ///
    /// If it returns [`None`], it means the client is broken.
    pub fn into_inner(self) -> Option<AesCipher> {
        self.cipher.into_inner()
    }

    pub(crate) async fn get(&self) -> Result<(AesCipher, MutexGuard<Option<AesCipher>>), NetworkError> {
        let mut guard = self.cipher.lock().await;
        let cipher = (*guard).take().ok_or(NetworkError::BrokenCipher())?;
        Ok((cipher, guard))
    }

    #[inline]
    pub(crate) fn reset(&self, mut guard: MutexGuard<Option<AesCipher>>, cipher: AesCipher) {
        (*guard).replace(cipher);
    }
}
