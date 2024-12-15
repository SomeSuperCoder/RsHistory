use crypto::{mnemonic_to_string, KeyPair};

fn main() {
    loop {
        let (_, mnemonic) = KeyPair::new();
        println!("{}", mnemonic_to_string(mnemonic));
    }
}
