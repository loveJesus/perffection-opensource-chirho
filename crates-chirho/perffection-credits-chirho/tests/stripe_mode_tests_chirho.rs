// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! TEST mode only: a live key is refused whatever else is true.

use perffection_credits_chirho::stripe_mode_chirho::{
    require_test_key_chirho, StripeModeRefusalChirho,
};

#[test]
fn only_test_keys_pass_chirho() {
    for key_chirho in [
        "sk_test_51abc",
        "rk_test_51abc",
        "pk_test_51abc",
        "  sk_test_51abc  ",
    ] {
        assert_eq!(require_test_key_chirho(key_chirho), Ok(()), "{key_chirho}");
    }
    for key_chirho in ["sk_live_51abc", "rk_live_51abc", "pk_live_51abc"] {
        assert_eq!(
            require_test_key_chirho(key_chirho),
            Err(StripeModeRefusalChirho::LiveKeyChirho),
            "{key_chirho}"
        );
    }
    for key_chirho in ["", "sk_test_", "whsec_abc", "test", "sk_abc"] {
        assert_eq!(
            require_test_key_chirho(key_chirho),
            Err(StripeModeRefusalChirho::NotATestKeyChirho),
            "{key_chirho}"
        );
    }
}
