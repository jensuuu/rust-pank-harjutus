
use sha2::{Digest, Sha256, Sha512};
use sha3::Sha3_256;

use base64::{
    Engine as _,
    engine::general_purpose,
};

use argon2::{
    Argon2,
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
};

use password_hash::{
    SaltString,
    rand_core::OsRng,
};

// SHA-256 hashing
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());

    let result = hasher.finalize();

    format!("{:x}", result)
}

// SHA-512 hashing
pub fn hash_sha512(data: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    format!("{:x}", result)
}

// SHA3-256 hashing
pub fn hash_sha3(data: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data.as_bytes());

    let result = hasher.finalize();

    format!("{:x}", result)
}

// Base64 encoding
pub fn encode_base64(data: &str) -> String {
    general_purpose::STANDARD.encode(data)
}

// Base64 decoding
pub fn decode_base64(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::STANDARD.decode(data)
}

// Argon2 password hashing
pub fn hash_argon2(
    password: &str,
) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?;

    Ok(hash.to_string())
}

// Argon2 password verification
pub fn verify_argon2(
    password: &str,
    hashed_password: &str,
) -> bool {
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
