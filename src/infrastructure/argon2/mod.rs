use ::anyhow::Result;
use anyhow::Ok;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let byte = password.as_bytes();

    let argon2 = Argon2::default();

    let result = argon2
        .hash_password(byte, &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(result.to_string())
}

pub fn verify(password: String, hash: String) -> Result<bool> {
    let password_hash = PasswordHash::new(&hash).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let byte = password.as_bytes();

    Ok(Argon2::default()
        .verify_password(byte, &password_hash)
        .is_ok())
}
