//! API Key 加密工具

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::Rng;

const NONCE_SIZE: usize = 12;

pub fn encrypt_api_key(api_key: &str) -> String {
    if api_key.is_empty() {
        return String::new();
    }

    let key = get_encryption_key();
    let cipher = Aes256Gcm::new_from_slice(&key).expect("valid key size");

    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, api_key.as_bytes())
        .expect("encryption should not fail");

    let mut result = nonce_bytes.to_vec();
    result.extend(ciphertext);
    BASE64.encode(&result)
}

pub fn decrypt_api_key(encrypted: &str) -> String {
    if encrypted.is_empty() {
        return String::new();
    }

    let data = match BASE64.decode(encrypted) {
        Ok(d) => d,
        Err(_) => return String::new(),
    };

    if data.len() < NONCE_SIZE {
        return String::new();
    }

    let key = get_encryption_key();
    let cipher = Aes256Gcm::new_from_slice(&key).expect("valid key size");

    let nonce = Nonce::from_slice(&data[..NONCE_SIZE]);
    let ciphertext = &data[NONCE_SIZE..];

    match cipher.decrypt(nonce, ciphertext) {
        Ok(plaintext) => String::from_utf8_lossy(&plaintext).to_string(),
        Err(_) => String::new(),
    }
}

fn get_encryption_key() -> [u8; 32] {
    let env_key = std::env::var("PET_API_KEY_SECRET")
        .unwrap_or_else(|_| "default-fallback-key-change-in-prod".to_string());

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    env_key.hash(&mut hasher);
    let hash = hasher.finish();

    let mut key = [0u8; 32];
    let hash_bytes = hash.to_le_bytes();
    for (i, byte) in hash_bytes.iter().enumerate() {
        key[i] = *byte;
        key[i + 4] = byte.wrapping_add(i as u8);
        key[i + 8] = byte.wrapping_mul(2);
        key[i + 12] = byte.wrapping_sub(i as u8);
    }
    key
}
