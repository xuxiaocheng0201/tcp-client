use tcp_handler::common::AesCipher;
use tokio::sync::{Mutex, MutexGuard};

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

    pub(crate) async fn get<'a>(&'a self) -> (AesCipher, MutexGuard<Option<AesCipher>>) {
        let mut guard = self.cipher.lock().await;
        let cipher = (*guard).take().unwrap();
        (cipher, guard)
    }

    pub(crate) fn reset(&self, mut guard: MutexGuard<Option<AesCipher>>, cipher: AesCipher) {
        (*guard).replace(cipher);
    }
}
