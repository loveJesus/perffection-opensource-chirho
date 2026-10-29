// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Stripe's webhook signature: the `Stripe-Signature` header carries `t=<unix seconds>` and one or more
//! `v1=<hex HMAC-SHA256>` over `"{t}.{raw body}"` with the endpoint's signing secret. The body is verified as the
//! exact bytes received, before any parsing; the timestamp must sit inside the tolerance, so a captured event cannot
//! be replayed later; every `v1` is compared in constant time, and any other scheme (`v0`) is ignored.

use crate::hmac_sha256_chirho::{hex_lower_chirho, hmac_sha256_chirho};

/// Stripe's own libraries default to five minutes.
pub const DEFAULT_TOLERANCE_SECONDS_CHIRHO: u64 = 300;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureRefusalChirho {
    /// No `t=`, no `v1=`, or a value that does not parse.
    MalformedHeaderChirho,
    /// The timestamp is further than the tolerance from now, in either direction.
    OutsideToleranceChirho,
    /// No `v1` signature matches.
    NoMatchingSignatureChirho,
    /// An empty signing secret would make every signature forgeable.
    EmptySecretChirho,
}

/// Verifies `raw_body_chirho` against the header; returns the signed timestamp on success.
pub fn verify_stripe_signature_chirho(
    raw_body_chirho: &[u8],
    header_chirho: &str,
    signing_secret_chirho: &str,
    now_unix_seconds_chirho: u64,
    tolerance_seconds_chirho: u64,
) -> Result<u64, SignatureRefusalChirho> {
    if signing_secret_chirho.is_empty() {
        return Err(SignatureRefusalChirho::EmptySecretChirho);
    }
    let mut timestamp_chirho: Option<u64> = None;
    let mut signatures_chirho: Vec<&str> = Vec::new();
    for part_chirho in header_chirho.split(',') {
        match part_chirho.trim().split_once('=') {
            Some(("t", value_chirho)) => {
                timestamp_chirho = Some(
                    value_chirho
                        .parse()
                        .map_err(|_| SignatureRefusalChirho::MalformedHeaderChirho)?,
                );
            }
            Some(("v1", value_chirho)) => signatures_chirho.push(value_chirho),
            _ => {}
        }
    }
    let timestamp_chirho = timestamp_chirho.ok_or(SignatureRefusalChirho::MalformedHeaderChirho)?;
    if signatures_chirho.is_empty() {
        return Err(SignatureRefusalChirho::MalformedHeaderChirho);
    }
    if now_unix_seconds_chirho.abs_diff(timestamp_chirho) > tolerance_seconds_chirho {
        return Err(SignatureRefusalChirho::OutsideToleranceChirho);
    }
    let mut signed_payload_chirho = format!("{timestamp_chirho}.").into_bytes();
    signed_payload_chirho.extend_from_slice(raw_body_chirho);
    let expected_chirho = hex_lower_chirho(&hmac_sha256_chirho(
        signing_secret_chirho.as_bytes(),
        &signed_payload_chirho,
    ));
    // Every candidate is compared, so the time taken does not say which one (if any) matched.
    let matched_chirho = signatures_chirho
        .iter()
        .fold(false, |found_chirho, candidate_chirho| {
            constant_time_eq::constant_time_eq(
                candidate_chirho.as_bytes(),
                expected_chirho.as_bytes(),
            ) | found_chirho
        });
    if matched_chirho {
        Ok(timestamp_chirho)
    } else {
        Err(SignatureRefusalChirho::NoMatchingSignatureChirho)
    }
}
