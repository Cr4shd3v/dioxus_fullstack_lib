use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub struct PasswordUtil;

impl PasswordUtil {
    pub fn hash_pw(raw_pw: &str) -> Result<String, argon2::password_hash::Error> {
        let argon = Argon2::default();
        argon.hash_password(raw_pw.as_ref()).map(|v| v.to_string())
    }

    pub fn verify_pw(raw_pw: &str, pw_hash: &str) -> Result<(), argon2::password_hash::Error> {
        let argon = Argon2::default();
        argon.verify_password(raw_pw.as_ref(), &PasswordHash::new(pw_hash)?)
    }
}
