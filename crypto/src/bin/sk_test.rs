use ed25519_dalek::SigningKey;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;

fn main() {
    let bytes = [33, 238, 240, 99, 177, 254, 194, 94, 46, 221, 28, 12, 46, 19, 85, 180, 27, 66, 64, 109, 202, 206, 224, 140, 21, 229, 170, 149, 53, 25, 105, 166];

    println!("{:?}", bytes);

    let signing_key = SigningKey::from_bytes(&bytes);

    let message: &[u8] = b"abc";

    let signature: Signature = signing_key.sign(message);
    let sig_bytes = signature.to_bytes();
    
    println!("{:?}", sig_bytes);
}
