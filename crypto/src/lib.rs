use std::error::Error;
use ed25519_dalek::{SecretKey, Signature, SigningKey, Verifier, VerifyingKey, SECRET_KEY_LENGTH, SIGNATURE_LENGTH};
use rand::rngs::OsRng;
use rand::RngCore;
use bip39::Mnemonic;
use signature::SignerMut;

pub type SignatureBytes = [u8; SIGNATURE_LENGTH];

pub struct KeyPair {
    sk: SigningKey,
    vk: VerifyingKey
}

impl KeyPair {
    pub fn new() -> (Self, Mnemonic) {
        let mut entropy = [0; 32];
        OsRng.fill_bytes(&mut entropy);
        
        let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
        let kp = Self::from_mnemonic(&mnemonic);

        (kp, mnemonic.clone())
    }

    pub fn from_mnemonic(mnemonic: &Mnemonic) -> Self {
        let seed = mnemonic.to_seed("");

        Self::from_bytes(seed[..32].try_into().expect("Fatal Neon error: failed casting [u8; 64] into [u8; 32]"))
    }

    pub fn from_bytes(bytes: &SecretKey) -> Self {
        let sk = SigningKey::from_bytes(bytes);
        let vk = sk.verifying_key();

        Self {
            sk,
            vk
        }
    }

    pub fn from_seed_phrase(phrase: String) -> Result<Self, Box<dyn Error>> {
        let mnemonic = Mnemonic::parse(phrase)?;

        Ok(Self::from_mnemonic(&mnemonic))
    }

    pub fn to_bytes(&self) -> [u8; SECRET_KEY_LENGTH] {
        self.sk.to_bytes()
    }

    pub fn as_verifyer(&self) -> Verifyer {
        Verifyer::new(self.vk.clone())
    }

    pub fn get_address(&self) -> String {
        self.as_verifyer().to_address()
    }

    pub fn sign(&mut self, message: &[u8]) -> Vec<u8> {
        self.sk.sign(message).to_vec()
    }
}

pub struct Verifyer {
    vk: VerifyingKey
}

impl Verifyer {
    pub fn new(vk: VerifyingKey) -> Self {
        Self {
            vk
        }
    }

    pub fn verify(&self, message: &[u8], signature: &SignatureBytes) -> bool {
        let sig = Signature::from_bytes(signature);

        self.vk.verify(message, &sig).is_ok()
    }

    pub fn to_address(&self) -> String {
        bs58::encode(self.vk.to_bytes()).into_string()
    }
}

pub fn mnemonic_to_string(mnemonic: Mnemonic) -> String {
    let words: Vec<&str> = mnemonic.words().collect();
    let mut word_string = String::new();

    for (i, word) in words.iter().enumerate() {
        if i > 0 {
            word_string.push_str(" ");
        }

        word_string.push_str(word);
    }

    word_string
}
