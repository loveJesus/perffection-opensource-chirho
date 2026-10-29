// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Checkout Session a credit purchase asks Stripe for, as plain parameters. This is pure: it builds the exact form
//! fields from a catalog pack and refuses anything else, and it owns no network and no key. Whoever sends the request
//! owns both, in TEST mode only, until the money gate opens.
//!
//! The paid session's verified event (`checkout.session.completed`) carries what the intake reads back: the mode, the
//! client reference (the payer), the amount and currency, and `metadata.pack_id_chirho`. The line item is priced at
//! exactly the pack's cents, so the intake's price check (`PackCatalogChirho::credits_for_chirho`) accepts it.
//!
//! Workflow: spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md

use crate::credit_view_port_chirho::PayerRefChirho;
use crate::pack_catalog_chirho::{PackCatalogChirho, PACK_CURRENCY_CHIRHO};
use crate::stripe_event_chirho::PACK_METADATA_KEY_CHIRHO;
use sha2::{Digest, Sha256};

/// The longest attempt id a caller may supply.
pub const CHECKOUT_ATTEMPT_ID_MAX_BYTES_CHIRHO: usize = 64;

/// One session request: the form fields for `POST /v1/checkout/sessions`, in a stable order, and an `Idempotency-Key`
/// per payer, pack and attempt. Stripe may prune a key after at least 24 hours. While it retains the key, a repeat
/// gets the saved result (including an error), and changed arguments conflict. A pruned key can create a new request;
/// this is not a lifetime or ledger guarantee. The ledger posts once per verified event id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutRequestChirho {
    pub fields_chirho: Vec<(String, String)>,
    pub idempotency_key_chirho: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckoutRequestRefusalChirho {
    UnknownPackChirho,
    /// The attempt id is empty, too long, or not only ASCII letters, digits, '-' and '_'.
    InvalidAttemptChirho,
    /// A return address is not https to a well-formed host, nor http to exactly this machine.
    InvalidReturnUrlChirho,
}

/// Where Checkout sends her back: after paying, and after cancelling.
#[derive(Debug, Clone, Copy)]
pub struct CheckoutReturnUrlsChirho<'urls_chirho> {
    pub paid_url_chirho: &'urls_chirho str,
    pub cancelled_url_chirho: &'urls_chirho str,
}

/// The session request for one pack, for one payer, for one purchase attempt.
pub fn checkout_request_chirho(
    catalog_chirho: &PackCatalogChirho,
    pack_id_chirho: &str,
    payer_chirho: &PayerRefChirho,
    returns_chirho: CheckoutReturnUrlsChirho<'_>,
    attempt_id_chirho: &str,
) -> Result<CheckoutRequestChirho, CheckoutRequestRefusalChirho> {
    let (_, pack_chirho) = catalog_chirho
        .packs_chirho()
        .find(|(id_chirho, _)| *id_chirho == pack_id_chirho)
        .ok_or(CheckoutRequestRefusalChirho::UnknownPackChirho)?;
    let attempt_ok_chirho = !attempt_id_chirho.is_empty()
        && attempt_id_chirho.len() <= CHECKOUT_ATTEMPT_ID_MAX_BYTES_CHIRHO
        && attempt_id_chirho.bytes().all(|byte_chirho| {
            byte_chirho.is_ascii_alphanumeric() || byte_chirho == b'-' || byte_chirho == b'_'
        });
    if !attempt_ok_chirho {
        return Err(CheckoutRequestRefusalChirho::InvalidAttemptChirho);
    }
    if !return_url_ok_chirho(returns_chirho.paid_url_chirho)
        || !return_url_ok_chirho(returns_chirho.cancelled_url_chirho)
    {
        return Err(CheckoutRequestRefusalChirho::InvalidReturnUrlChirho);
    }
    let field_chirho =
        |name_chirho: &str, value_chirho: String| (name_chirho.to_string(), value_chirho);
    let fields_chirho = vec![
        field_chirho("mode", "payment".to_string()),
        field_chirho(
            "client_reference_id",
            payer_chirho.as_str_chirho().to_string(),
        ),
        field_chirho(
            &format!("metadata[{PACK_METADATA_KEY_CHIRHO}]"),
            pack_id_chirho.to_string(),
        ),
        field_chirho("line_items[0][quantity]", "1".to_string()),
        field_chirho(
            "line_items[0][price_data][currency]",
            PACK_CURRENCY_CHIRHO.to_string(),
        ),
        field_chirho(
            "line_items[0][price_data][unit_amount]",
            pack_chirho.amount_minor_chirho.to_string(),
        ),
        field_chirho(
            "line_items[0][price_data][product_data][name]",
            format!("{} Perffection credits", pack_chirho.credits_chirho),
        ),
        field_chirho("success_url", returns_chirho.paid_url_chirho.to_string()),
        field_chirho(
            "cancel_url",
            returns_chirho.cancelled_url_chirho.to_string(),
        ),
    ];
    Ok(CheckoutRequestChirho {
        fields_chirho,
        idempotency_key_chirho: idempotency_key_chirho(
            payer_chirho,
            pack_id_chirho,
            attempt_id_chirho,
        ),
    })
}

/// A return address Checkout may send her to: `https://` to a well-formed DNS host, or `http://` to exactly this
/// machine (`127.0.0.1` or `localhost`) for a local journey, each with an optional port from 1 to 65535. The authority
/// is parsed, never prefix-matched. So userinfo (`http://127.0.0.1:80@evil.example/` goes to `evil.example`), a missing
/// or malformed host or port, brackets, a backslash and any whitespace or control character are all refused.
fn return_url_ok_chirho(url_chirho: &str) -> bool {
    if !url_chirho
        .bytes()
        .all(|byte_chirho| byte_chirho.is_ascii_graphic())
        || url_chirho.contains('\\')
    {
        return false;
    }
    let (loopback_only_chirho, rest_chirho) =
        if let Some(rest_chirho) = url_chirho.strip_prefix("https://") {
            (false, rest_chirho)
        } else if let Some(rest_chirho) = url_chirho.strip_prefix("http://") {
            (true, rest_chirho)
        } else {
            return false;
        };
    let authority_end_chirho = rest_chirho
        .find(['/', '?', '#'])
        .unwrap_or(rest_chirho.len());
    let authority_chirho = &rest_chirho[..authority_end_chirho];
    if authority_chirho.contains(['@', '[', ']']) {
        return false;
    }
    let (host_chirho, port_chirho) = match authority_chirho.split_once(':') {
        Some((host_chirho, port_chirho)) => (host_chirho, Some(port_chirho)),
        None => (authority_chirho, None),
    };
    let port_ok_chirho = port_chirho.is_none_or(|port_chirho| {
        !port_chirho.starts_with('0')
            && port_chirho.len() <= 5
            && port_chirho
                .bytes()
                .all(|byte_chirho| byte_chirho.is_ascii_digit())
            && port_chirho
                .parse::<u32>()
                .is_ok_and(|number_chirho| (1..=65_535).contains(&number_chirho))
    });
    let host_ok_chirho = if loopback_only_chirho {
        host_chirho == "127.0.0.1" || host_chirho == "localhost"
    } else {
        dns_host_ok_chirho(host_chirho)
    };
    port_ok_chirho && host_ok_chirho
}

/// A DNS name of at least two labels, each 1 to 63 letters, digits or hyphens with no hyphen at either end, ending in
/// a label that is not all digits (so not a bare IP address), at most 253 characters in all.
fn dns_host_ok_chirho(host_chirho: &str) -> bool {
    let labels_chirho: Vec<&str> = host_chirho.split('.').collect();
    host_chirho.len() <= 253
        && labels_chirho.len() >= 2
        && labels_chirho.iter().all(|label_chirho| {
            (1..=63).contains(&label_chirho.len())
                && !label_chirho.starts_with('-')
                && !label_chirho.ends_with('-')
                && label_chirho
                    .bytes()
                    .all(|byte_chirho| byte_chirho.is_ascii_alphanumeric() || byte_chirho == b'-')
        })
        && labels_chirho.last().is_some_and(|last_chirho| {
            !last_chirho
                .bytes()
                .all(|byte_chirho| byte_chirho.is_ascii_digit())
        })
}

/// Stable per payer, pack and attempt; a hash, so the payer reference is not in it.
fn idempotency_key_chirho(
    payer_chirho: &PayerRefChirho,
    pack_id_chirho: &str,
    attempt_id_chirho: &str,
) -> String {
    let digest_chirho = Sha256::digest(
        format!(
            "{}\n{pack_id_chirho}\n{attempt_id_chirho}",
            payer_chirho.as_str_chirho()
        )
        .as_bytes(),
    );
    let hex_chirho: String = digest_chirho
        .iter()
        .map(|byte_chirho| format!("{byte_chirho:02x}"))
        .collect();
    format!("checkout-{}", &hex_chirho[..32])
}
