// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! HMAC-SHA256 (RFC 2104) over the workspace's pinned `sha2`, so the signature check adds no new dependency.
//! Checked against RFC 4231's published vectors (tests/hmac_vectors_chirho.rs).

use sha2::{Digest, Sha256};

const BLOCK_BYTES_CHIRHO: usize = 64;

pub fn hmac_sha256_chirho(key_chirho: &[u8], message_chirho: &[u8]) -> [u8; 32] {
    // A key longer than one block is first hashed; a shorter one is zero-padded to the block.
    let mut block_key_chirho = [0u8; BLOCK_BYTES_CHIRHO];
    if key_chirho.len() > BLOCK_BYTES_CHIRHO {
        block_key_chirho[..32].copy_from_slice(&Sha256::digest(key_chirho));
    } else {
        block_key_chirho[..key_chirho.len()].copy_from_slice(key_chirho);
    }
    let mut inner_chirho = Sha256::new();
    inner_chirho.update(block_key_chirho.map(|byte_chirho| byte_chirho ^ 0x36));
    inner_chirho.update(message_chirho);
    let mut outer_chirho = Sha256::new();
    outer_chirho.update(block_key_chirho.map(|byte_chirho| byte_chirho ^ 0x5c));
    outer_chirho.update(inner_chirho.finalize());
    outer_chirho.finalize().into()
}

/// Lowercase hex, the form Stripe's `v1` signatures use.
pub fn hex_lower_chirho(bytes_chirho: &[u8]) -> String {
    const DIGITS_CHIRHO: &[u8; 16] = b"0123456789abcdef";
    let mut text_chirho = String::with_capacity(bytes_chirho.len() * 2);
    for byte_chirho in bytes_chirho {
        text_chirho.push(DIGITS_CHIRHO[(byte_chirho >> 4) as usize] as char);
        text_chirho.push(DIGITS_CHIRHO[(byte_chirho & 0x0f) as usize] as char);
    }
    text_chirho
}
