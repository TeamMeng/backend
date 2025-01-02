use crate::AppError;
use jwt_simple::prelude::*;

#[allow(unused)]
pub struct EncodingKey(Ed25519KeyPair);

#[allow(unused)]
pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn new(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }
}

impl DecodingKey {
    pub fn new(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }
}
