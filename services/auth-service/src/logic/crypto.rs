// src/logic/crypto.rs
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use aes_gcm::aead::rand_core::RngCore;

pub fn encrypt_token(key: &[u8; 32], refresh_token: &str) -> (Vec<u8>, [u8; 12]) {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, refresh_token.as_bytes())
        .expect("encryption failed");
    (ciphertext, nonce_bytes)
}

pub fn decrypt_token(key: &[u8; 32], encrypted: &[u8], nonce_bytes: &[u8; 12]) -> String {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);
    String::from_utf8(cipher.decrypt(nonce, encrypted).expect("decryption failed")).unwrap()
}
