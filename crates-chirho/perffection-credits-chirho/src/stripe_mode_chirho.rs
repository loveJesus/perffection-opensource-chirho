// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! TEST mode only, enforced in code: a live key is refused before any Stripe call is made. The .env already holds
//! LIVE keys under other names (checked 2026-10-02); real money needs the money gate and L.J.'s separate word.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StripeModeRefusalChirho {
    /// A `sk_live_`/`rk_live_`/`pk_live_` key: real money, refused here whatever the configuration says.
    LiveKeyChirho,
    /// Not a recognisable Stripe test key at all.
    NotATestKeyChirho,
}

/// Accepts only Stripe test-mode keys (secret, restricted or publishable). Never echoes the key.
pub fn require_test_key_chirho(key_chirho: &str) -> Result<(), StripeModeRefusalChirho> {
    let key_chirho = key_chirho.trim();
    if ["sk_live_", "rk_live_", "pk_live_"]
        .iter()
        .any(|prefix_chirho| key_chirho.starts_with(prefix_chirho))
    {
        return Err(StripeModeRefusalChirho::LiveKeyChirho);
    }
    let is_test_chirho = ["sk_test_", "rk_test_", "pk_test_"]
        .iter()
        .any(|prefix_chirho| {
            key_chirho.len() > prefix_chirho.len() && key_chirho.starts_with(prefix_chirho)
        });
    if is_test_chirho {
        Ok(())
    } else {
        Err(StripeModeRefusalChirho::NotATestKeyChirho)
    }
}
