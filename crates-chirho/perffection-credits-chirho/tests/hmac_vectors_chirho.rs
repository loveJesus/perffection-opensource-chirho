// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! RFC 4231's HMAC-SHA256 vectors (cases 1-4, 6, 7; case 5 is a truncation test), cross-checked 2026-10-02 with
//! Python's standard `hmac` module, so the hand-built HMAC is proved by values it did not produce.

use perffection_credits_chirho::hmac_sha256_chirho::{hex_lower_chirho, hmac_sha256_chirho};

fn hmac_hex_chirho(key_chirho: &[u8], data_chirho: &[u8]) -> String {
    hex_lower_chirho(&hmac_sha256_chirho(key_chirho, data_chirho))
}

#[test]
fn rfc_4231_vectors_chirho() {
    let long_key_chirho = [0xaau8; 131];
    let cases_chirho: [(&[u8], &[u8], &str); 6] = [
        (&[0x0b; 20], b"Hi There", "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"),
        (b"Jefe", b"what do ya want for nothing?", "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"),
        (&[0xaa; 20], &[0xdd; 50], "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe"),
        (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25], &[0xcd; 50],
            "82558a389a443c0ea4cc819899f2083a85f0faa3e578f8077a2e3ff46729665b"),
        (&long_key_chirho, b"Test Using Larger Than Block-Size Key - Hash Key First",
            "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"),
        (&long_key_chirho, b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.",
            "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2"),
    ];
    for (key_chirho, data_chirho, expected_chirho) in cases_chirho {
        assert_eq!(hmac_hex_chirho(key_chirho, data_chirho), expected_chirho);
    }
}
