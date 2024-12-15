use sha2::{Sha256, Digest};

pub fn compute_hash(input: &str) -> String {
    // Create a Sha256 hasher instanc
    let mut hasher = Sha256::new();
    
    // Update the hasher with the input string (converted to bytes)
    hasher.update(input.as_bytes());
    
    // Finalize the hash and obtain the result
    let result = hasher.finalize();

    format!("{:x}", result)
}
