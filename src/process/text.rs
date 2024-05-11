use crate::{get_reader, process_genpass, TextSignFormat};

use anyhow::{Ok, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

pub trait TextSign {
    // 动态分派
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    // 静态分派
    // fn verify<R: Read>(&self, reader: R, sig: &[u8]) -> Result<bool>;
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>; // 等价于上面的写法
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)
        }
    }
}

pub fn process_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        Ok(Blake3::new(key))
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(blake3::keyed_hash(&self.key, &buffer).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let hash = blake3::keyed_hash(&self.key, &buffer);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Blake3::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Ed25519Signer::new(key))
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let sig = self.key.sign(&buffer);
        Ok(sig.to_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Ed25519Signer::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let sk = signing_key.to_bytes().to_vec();
        let pk = signing_key.verifying_key().to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Ed25519Verifier::new(key))
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buffer, &sig).is_ok())
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Ed25519Verifier::try_new(&key)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"hello world";
        let sig = blake3.sign(&mut &data[..])?;
        assert!(blake3.verify(&data[..], &sig)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;
        let data = b"hello world";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&data[..], &sig)?);
        Ok(())
    }
}
