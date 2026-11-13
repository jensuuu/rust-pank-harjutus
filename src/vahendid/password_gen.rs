
use std::io::{self, Write};

use sha2::{Digest, Sha256, Sha512};
use sha3::Sha3_256;
use blake2::{Blake2b512, Blake2s256};

use argon2::{
    Argon2,
    password_hash::{
        PasswordHasher,
        SaltString,
    },
};

use rand::{RngExt};


use password_hash::rand_core::OsRng;

use md5::{
    Md5,
    Digest as Md5Digest,
};

pub struct PasswordGenerator;

impl PasswordGenerator {

    pub fn input_password() -> io::Result<String> {
        let mut password = String::new();

        print!("Enter password: ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut password)?;

        Ok(password.trim().to_string())
    }

    pub fn generate_base64(password: &str) -> String {
        base64::encode(password)
    }

    // MD5
    pub fn generate_md5(password: &str) -> String {
        format!("{:x}", Md5::digest(password.as_bytes()))
    }

    // SHA-256
    pub fn generate_sha256(password: &str) -> String {
        hex::encode(Sha256::digest(password.as_bytes()))
    }

    // SHA-512
    pub fn generate_sha512(password: &str) -> String {
        hex::encode(Sha512::digest(password.as_bytes()))
    }

    // SHA3-256
    pub fn generate_sha3(password: &str) -> String {
        hex::encode(Sha3_256::digest(password.as_bytes()))
    }

    // BLAKE2b-512
    pub fn generate_blake2b(password: &str) -> String {
        hex::encode(Blake2b512::digest(password.as_bytes()))
    }

    // BLAKE2s-256
    pub fn generate_blake2s(password: &str) -> String {
        hex::encode(Blake2s256::digest(password.as_bytes()))
    }

    // Argon2id
    pub fn generate_argon2(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);

        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("Argon2 hashing failed")
            .to_string()
    }

    // BCrypt
    pub fn generate_bcrypt(password: &str) -> String {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .expect("BCrypt hashing failed")
    }

    // Scrypt
    pub fn generate_scrypt(password: &str) -> String {
        let mut salt = [0u8; 16];
        rand::rng().fill(&mut salt);

        let params = scrypt::Params::recommended();
        let mut output = [0u8; 32];

        scrypt::scrypt(
            password.as_bytes(),
            &salt,
            &params,
            &mut output,
        )
        .expect("Scrypt hashing failed");

        format!(
            "scrypt${}${}",
            hex::encode(salt),
            hex::encode(output)
        )
    }



    pub fn show() -> std::io::Result<()> {
        let password = Self::input_password()?;

        println!("\n========== PASSWORD HASHES ==========");

        println!("Base64:    {}", Self::generate_base64(&password));
        println!("MD5:       {}", Self::generate_md5(&password));
        println!("SHA256:    {}", Self::generate_sha256(&password));
        println!("SHA512:    {}", Self::generate_sha512(&password));
        println!("SHA3:      {}", Self::generate_sha3(&password));
        println!("BLAKE2b:   {}", Self::generate_blake2b(&password));
        println!("BLAKE2s:   {}", Self::generate_blake2s(&password));
        println!("Argon2id:  {}", Self::generate_argon2(&password));
        println!("BCrypt:    {}", Self::generate_bcrypt(&password));
        println!("Scrypt:    {}", Self::generate_scrypt(&password));

        println!("=====================================");

        Ok(())
    }
}
