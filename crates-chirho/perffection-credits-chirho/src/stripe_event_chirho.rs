// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Reads a VERIFIED Stripe event (the signature check runs first, on the raw bytes). Only a test-mode, PAID,
//! one-time-payment Checkout Session becomes a purchase. It arrives as `checkout.session.completed`, or, for a payment
//! method that settles later, as `checkout.session.async_payment_succeeded` after a completed-but-unpaid event; any
//! unpaid or failed session and any other event type is ignored, so Stripe stops retrying it. Separate events can name
//! one session, so the top-up port posts once per session as well as once per event id. Stripe's own field names are
//! kept byte-for-byte through serde renames; the metadata key `pack_id_chirho` is ours, set when the product creates
//! the Checkout Session.

use crate::credit_view_port_chirho::PayerRefChirho;
use serde::Deserialize;

pub const COMPLETED_EVENT_TYPE_CHIRHO: &str = "checkout.session.completed";
/// A delayed payment method's later success (bank debits and the like): the same session, now paid.
pub const ASYNC_PAYMENT_SUCCEEDED_EVENT_TYPE_CHIRHO: &str =
    "checkout.session.async_payment_succeeded";
pub const PACK_METADATA_KEY_CHIRHO: &str = "pack_id_chirho";

#[derive(Debug, Deserialize)]
struct EventEnvelopeChirho {
    #[serde(rename = "id")]
    id_chirho: String,
    #[serde(rename = "type")]
    type_chirho: String,
    #[serde(rename = "livemode")]
    livemode_chirho: bool,
    #[serde(rename = "data")]
    data_chirho: EventDataChirho,
}

#[derive(Debug, Deserialize)]
struct EventDataChirho {
    #[serde(rename = "object")]
    object_chirho: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct CheckoutSessionChirho {
    #[serde(rename = "id")]
    id_chirho: String,
    #[serde(rename = "object")]
    object_chirho: String,
    #[serde(rename = "mode")]
    mode_chirho: String,
    #[serde(rename = "payment_status")]
    payment_status_chirho: String,
    #[serde(rename = "client_reference_id")]
    client_reference_id_chirho: Option<String>,
    #[serde(rename = "amount_total")]
    amount_total_chirho: Option<u64>,
    #[serde(rename = "currency")]
    currency_chirho: Option<String>,
    #[serde(rename = "metadata", default)]
    metadata_chirho: std::collections::BTreeMap<String, String>,
}

/// A paid test-mode Checkout Session, as the event reported it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaidCheckoutChirho {
    pub event_id_chirho: String,
    pub checkout_session_id_chirho: String,
    pub account_reference_chirho: String,
    pub pack_id_chirho: String,
    pub amount_minor_chirho: u64,
    pub currency_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventReadChirho {
    PaidChirho(PaidCheckoutChirho),
    /// Not a purchase: another event type, or a session that is not (yet) paid.
    IgnoredChirho(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventRefusalChirho {
    /// Not JSON, or not the shape of a Stripe event.
    MalformedChirho,
    /// `livemode: true`: real money, which this intake never accepts.
    LiveModeChirho,
    /// A completed one-time payment without the fields a top-up needs.
    IncompleteSessionChirho(&'static str),
}

pub fn read_stripe_event_chirho(
    raw_body_chirho: &[u8],
) -> Result<EventReadChirho, EventRefusalChirho> {
    let envelope_chirho: EventEnvelopeChirho =
        serde_json::from_slice(raw_body_chirho).map_err(|_| EventRefusalChirho::MalformedChirho)?;
    // Live mode is refused before the type is even looked at: no live event is ever "ignored" into a 200.
    if envelope_chirho.livemode_chirho {
        return Err(EventRefusalChirho::LiveModeChirho);
    }
    if envelope_chirho.type_chirho != COMPLETED_EVENT_TYPE_CHIRHO
        && envelope_chirho.type_chirho != ASYNC_PAYMENT_SUCCEEDED_EVENT_TYPE_CHIRHO
    {
        return Ok(EventReadChirho::IgnoredChirho("not a completed checkout"));
    }
    let session_chirho: CheckoutSessionChirho =
        serde_json::from_value(envelope_chirho.data_chirho.object_chirho)
            .map_err(|_| EventRefusalChirho::MalformedChirho)?;
    if session_chirho.object_chirho != "checkout.session" || session_chirho.mode_chirho != "payment"
    {
        return Ok(EventReadChirho::IgnoredChirho(
            "not a one-time payment session",
        ));
    }
    if session_chirho.payment_status_chirho != "paid" {
        return Ok(EventReadChirho::IgnoredChirho("the session is not paid"));
    }
    // The payer comes back exactly as the checkout sent it, under the payer reference's own law: never re-encoded, and
    // never narrowed to a stricter grammar than the one the builder accepted.
    let account_reference_chirho = session_chirho
        .client_reference_id_chirho
        .filter(|value_chirho| PayerRefChirho::new_chirho(value_chirho).is_some())
        .ok_or(EventRefusalChirho::IncompleteSessionChirho(
            "account reference",
        ))?;
    let pack_id_chirho = session_chirho
        .metadata_chirho
        .get(PACK_METADATA_KEY_CHIRHO)
        .filter(|value_chirho| pack_id_shape_ok_chirho(value_chirho))
        .cloned()
        .ok_or(EventRefusalChirho::IncompleteSessionChirho("pack"))?;
    let amount_minor_chirho = session_chirho
        .amount_total_chirho
        .filter(|amount_chirho| *amount_chirho > 0)
        .ok_or(EventRefusalChirho::IncompleteSessionChirho("amount"))?;
    let currency_chirho = session_chirho
        .currency_chirho
        .filter(|value_chirho| {
            value_chirho.len() == 3
                && value_chirho
                    .bytes()
                    .all(|byte_chirho| byte_chirho.is_ascii_lowercase())
        })
        .ok_or(EventRefusalChirho::IncompleteSessionChirho("currency"))?;
    Ok(EventReadChirho::PaidChirho(PaidCheckoutChirho {
        event_id_chirho: envelope_chirho.id_chirho,
        checkout_session_id_chirho: session_chirho.id_chirho,
        account_reference_chirho,
        pack_id_chirho,
        amount_minor_chirho,
        currency_chirho,
    }))
}

/// A pack id is a short ASCII word (`credits-1000-chirho`): its own rule, apart from the payer reference's.
fn pack_id_shape_ok_chirho(value_chirho: &str) -> bool {
    !value_chirho.is_empty()
        && value_chirho.len() <= 128
        && value_chirho.bytes().all(|byte_chirho| {
            byte_chirho.is_ascii_alphanumeric() || byte_chirho == b'-' || byte_chirho == b'_'
        })
}
