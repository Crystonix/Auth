use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use aes_gcm::aead::rand_core::RngCore;
use anyhow::{Result, anyhow};

pub fn encrypt_token(key: &[u8; 32], refresh_token: &str) -> (Vec<u8>, [u8; 12]) {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
      .encrypt(nonce, refresh_token.as_bytes())
      .expect("encryption failed"); // This is unlikely to fail
    (ciphertext, nonce_bytes)
}

pub fn decrypt_token(key: &[u8; 32], encrypted: &[u8], nonce_bytes: &[u8; 12]) -> Result<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
      .decrypt(nonce, encrypted)
      .map_err(|_| anyhow!("Decryption failed"))?;
    String::from_utf8(plaintext).map_err(|_| anyhow!("Invalid UTF-8 in decrypted token"))
}
