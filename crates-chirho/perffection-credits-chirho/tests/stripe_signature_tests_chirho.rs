// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! The Stripe signature check. The expected `v1` below was computed with Python's `hmac` (2026-10-02), not by this
//! crate, over "{t}.{body}" with the fixture secret.

use perffection_credits_chirho::stripe_signature_chirho::{
    verify_stripe_signature_chirho, SignatureRefusalChirho,
};

const SECRET_CHIRHO: &str = "whsec_test_fixture_chirho";
const BODY_CHIRHO: &str =
    r#"{"id":"evt_fixture_chirho","type":"checkout.session.completed","livemode":false}"#;
const T_CHIRHO: u64 = 1_790_955_000;
const V1_CHIRHO: &str = "0fbe0f05f5c29ccccbd43b6829caeede7475cf9d3b300c3d45169db95e7aa855";

fn header_chirho(extra_chirho: &str) -> String {
    format!("t={T_CHIRHO},v1={V1_CHIRHO}{extra_chirho}")
}

#[test]
fn a_correct_signature_inside_the_tolerance_verifies_chirho() {
    assert_eq!(
        verify_stripe_signature_chirho(
            BODY_CHIRHO.as_bytes(),
            &header_chirho(""),
            SECRET_CHIRHO,
            T_CHIRHO + 299,
            300
        ),
        Ok(T_CHIRHO)
    );
    // Any one matching v1 is enough, and a v0 beside it is ignored.
    let mixed_chirho = format!(
        "t={T_CHIRHO},v1={},v0=ignored,v1={V1_CHIRHO}",
        "0".repeat(64)
    );
    assert_eq!(
        verify_stripe_signature_chirho(
            BODY_CHIRHO.as_bytes(),
            &mixed_chirho,
            SECRET_CHIRHO,
            T_CHIRHO,
            300
        ),
        Ok(T_CHIRHO)
    );
}

#[test]
fn a_changed_body_another_secret_or_another_time_is_refused_chirho() {
    let tampered_chirho = BODY_CHIRHO.replace("false", "true ");
    assert_eq!(
        verify_stripe_signature_chirho(
            tampered_chirho.as_bytes(),
            &header_chirho(""),
            SECRET_CHIRHO,
            T_CHIRHO,
            300
        ),
        Err(SignatureRefusalChirho::NoMatchingSignatureChirho)
    );
    assert_eq!(
        verify_stripe_signature_chirho(
            BODY_CHIRHO.as_bytes(),
            &header_chirho(""),
            "whsec_other_chirho",
            T_CHIRHO,
            300
        ),
        Err(SignatureRefusalChirho::NoMatchingSignatureChirho)
    );
    // A replay outside the tolerance, from the past or the future, is refused before any HMAC.
    assert_eq!(
        verify_stripe_signature_chirho(
            BODY_CHIRHO.as_bytes(),
            &header_chirho(""),
            SECRET_CHIRHO,
            T_CHIRHO + 301,
            300
        ),
        Err(SignatureRefusalChirho::OutsideToleranceChirho)
    );
    assert_eq!(
        verify_stripe_signature_chirho(
            BODY_CHIRHO.as_bytes(),
            &header_chirho(""),
            SECRET_CHIRHO,
            T_CHIRHO - 301,
            300
        ),
        Err(SignatureRefusalChirho::OutsideToleranceChirho)
    );
}

#[test]
fn malformed_headers_and_an_empty_secret_are_refused_chirho() {
    for header_chirho in [
        "",
        "v1=abc",
        &format!("t={T_CHIRHO}"),
        &format!("t={T_CHIRHO},v0={V1_CHIRHO}"),
        "t=soon,v1=abc",
    ] {
        assert_eq!(
            verify_stripe_signature_chirho(
                BODY_CHIRHO.as_bytes(),
                header_chirho,
                SECRET_CHIRHO,
                T_CHIRHO,
                300
            ),
            Err(SignatureRefusalChirho::MalformedHeaderChirho),
            "{header_chirho}"
        );
    }
    assert_eq!(
        verify_stripe_signature_chirho(
            BODY_CHIRHO.as_bytes(),
            &header_chirho(""),
            "",
            T_CHIRHO,
            300
        ),
        Err(SignatureRefusalChirho::EmptySecretChirho)
    );
}
