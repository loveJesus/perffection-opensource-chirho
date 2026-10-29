// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Prints the Stripe CLI command for one TEST-mode Checkout Session, built by `checkout_request_chirho`. It never runs
//! the command, reads no key and calls nothing. The Stripe CLI would use its own paired TEST key, and running what this
//! prints is not released (credit-purchase-chirho.md).
//!
//! Usage: `local_checkout_chirho <pack-id> <payer-reference> <attempt-id> [<paid-url> <cancelled-url>]`

use perffection_credits_chirho::checkout_request_chirho::{
    checkout_request_chirho, CheckoutReturnUrlsChirho,
};
use perffection_credits_chirho::credit_view_port_chirho::PayerRefChirho;
use perffection_credits_chirho::pack_catalog_chirho::PackCatalogChirho;

/// The local stand-in's addresses, for a loopback TEST journey.
const LOCAL_PAID_URL_CHIRHO: &str = "http://127.0.0.1:4242/credits-paid-chirho";
const LOCAL_CANCELLED_URL_CHIRHO: &str = "http://127.0.0.1:4242/credits-cancelled-chirho";
const USAGE_CHIRHO: &str =
    "usage: local_checkout_chirho <pack-id> <payer-reference> <attempt-id> [<paid-url> <cancelled-url>]";

fn main() -> Result<(), String> {
    let arguments_chirho: Vec<String> = std::env::args().skip(1).collect();
    let [pack_id_chirho, payer_value_chirho, attempt_id_chirho, rest_chirho @ ..] =
        arguments_chirho.as_slice()
    else {
        return Err(USAGE_CHIRHO.to_string());
    };
    let (paid_url_chirho, cancelled_url_chirho) = match rest_chirho {
        [] => (LOCAL_PAID_URL_CHIRHO, LOCAL_CANCELLED_URL_CHIRHO),
        [paid_chirho, cancelled_chirho] => (paid_chirho.as_str(), cancelled_chirho.as_str()),
        _ => return Err(USAGE_CHIRHO.to_string()),
    };
    let payer_chirho = PayerRefChirho::new_chirho(payer_value_chirho)
        .ok_or("the payer reference is not 1 to 128 visible ASCII characters")?;
    let request_chirho = checkout_request_chirho(
        &PackCatalogChirho::starting_chirho(),
        pack_id_chirho,
        &payer_chirho,
        CheckoutReturnUrlsChirho {
            paid_url_chirho,
            cancelled_url_chirho,
        },
        attempt_id_chirho,
    )
    .map_err(|refusal_chirho| format!("refused: {refusal_chirho:?}"))?;
    let mut command_chirho: Vec<String> = [
        "stripe",
        "post",
        "/v1/checkout/sessions",
        "--idempotency",
        &request_chirho.idempotency_key_chirho,
    ]
    .iter()
    .map(|part_chirho| part_chirho.to_string())
    .collect();
    for (name_chirho, value_chirho) in &request_chirho.fields_chirho {
        command_chirho.push("-d".to_string());
        command_chirho.push(format!("{name_chirho}={value_chirho}"));
    }
    let quoted_chirho: Vec<String> = command_chirho
        .iter()
        .map(|part_chirho| shell_quote_chirho(part_chirho))
        .collect();
    println!("{}", quoted_chirho.join(" "));
    Ok(())
}

/// One argument, quoted for a POSIX shell when it needs to be.
fn shell_quote_chirho(part_chirho: &str) -> String {
    let plain_chirho = part_chirho
        .bytes()
        .all(|byte_chirho| byte_chirho.is_ascii_alphanumeric() || b"-_./:=".contains(&byte_chirho));
    if plain_chirho {
        part_chirho.to_string()
    } else {
        format!("'{}'", part_chirho.replace('\'', "'\\''"))
    }
}
