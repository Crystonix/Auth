use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use aes_gcm::aead::rand_core::RngCore;
use anyhow::{Result, anyhow};

pub fn encrypt_token(key: &[u8;32], token: &str) -> Result<(Vec<u8>, Vec<u8>)> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let ciphertext = cipher
      .encrypt(Nonce::from_slice(&nonce_bytes), token.as_bytes())
      .map_err(|_| anyhow!("Encryption failed"))?;
    Ok((ciphertext, nonce_bytes.to_vec()))
}

pub fn decrypt_token(key: &[u8;32], encrypted: &[u8], nonce: &[u8]) -> Result<String> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce: [u8; 12] = nonce.try_into().map_err(|_| anyhow!("Invalid nonce length"))?;
    let plaintext = cipher.decrypt(Nonce::from_slice(&nonce), encrypted)
      .map_err(|_| anyhow!("Decryption failed"))?;
    Ok(String::from_utf8(plaintext)?)
}

