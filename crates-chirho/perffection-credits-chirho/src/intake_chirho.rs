// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The intake, in its one order: signature on the raw bytes, then the event read (live mode refused), then the pack's
//! exact price, then ONE top-up per event id. Each step refuses with nothing posted; only the last step posts.
//! Workflow: spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md

use crate::hmac_sha256_chirho::hex_lower_chirho;
use crate::pack_catalog_chirho::{PackCatalogChirho, PackRefusalChirho};
use crate::stripe_event_chirho::{read_stripe_event_chirho, EventReadChirho, EventRefusalChirho};
use crate::stripe_signature_chirho::{verify_stripe_signature_chirho, SignatureRefusalChirho};
use crate::topup_port_chirho::{
    CreditTopupOutcomeChirho, CreditTopupPortChirho, CreditTopupRefusalChirho,
    CreditTopupRequestChirho,
};
use sha2::{Digest, Sha256};

pub struct IntakeConfigChirho<'a> {
    /// The endpoint's signing secret (`whsec_...`, from the Stripe CLI in development).
    pub signing_secret_chirho: &'a str,
    pub tolerance_seconds_chirho: u64,
    pub catalog_chirho: &'a PackCatalogChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntakeOutcomeChirho {
    CreditedChirho {
        posting_id_chirho: String,
        credits_chirho: u64,
    },
    /// A replay of an event already credited: answered as success, posted nothing.
    AlreadyCreditedChirho { posting_id_chirho: String },
    /// Not a purchase; Stripe gets a 2xx so it stops retrying.
    IgnoredChirho(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntakeRefusalChirho {
    SignatureChirho(SignatureRefusalChirho),
    EventChirho(EventRefusalChirho),
    PackChirho(PackRefusalChirho),
    PortChirho(CreditTopupRefusalChirho),
}

pub fn intake_stripe_event_chirho(
    raw_body_chirho: &[u8],
    signature_header_chirho: &str,
    now_unix_seconds_chirho: u64,
    config_chirho: &IntakeConfigChirho<'_>,
    port_chirho: &mut dyn CreditTopupPortChirho,
) -> Result<IntakeOutcomeChirho, IntakeRefusalChirho> {
    verify_stripe_signature_chirho(
        raw_body_chirho,
        signature_header_chirho,
        config_chirho.signing_secret_chirho,
        now_unix_seconds_chirho,
        config_chirho.tolerance_seconds_chirho,
    )
    .map_err(IntakeRefusalChirho::SignatureChirho)?;
    let paid_chirho = match read_stripe_event_chirho(raw_body_chirho)
        .map_err(IntakeRefusalChirho::EventChirho)?
    {
        EventReadChirho::PaidChirho(paid_chirho) => paid_chirho,
        EventReadChirho::IgnoredChirho(reason_chirho) => {
            return Ok(IntakeOutcomeChirho::IgnoredChirho(reason_chirho))
        }
    };
    let credits_chirho = config_chirho
        .catalog_chirho
        .credits_for_chirho(
            &paid_chirho.pack_id_chirho,
            paid_chirho.amount_minor_chirho,
            &paid_chirho.currency_chirho,
        )
        .map_err(IntakeRefusalChirho::PackChirho)?;
    let request_chirho = CreditTopupRequestChirho {
        event_id_chirho: paid_chirho.event_id_chirho,
        checkout_session_id_chirho: paid_chirho.checkout_session_id_chirho,
        account_reference_chirho: paid_chirho.account_reference_chirho,
        pack_id_chirho: paid_chirho.pack_id_chirho,
        credits_chirho,
        amount_minor_chirho: paid_chirho.amount_minor_chirho,
        currency_chirho: paid_chirho.currency_chirho,
        witness_sha256_chirho: hex_lower_chirho(&Sha256::digest(raw_body_chirho)),
    };
    match port_chirho
        .topup_chirho(&request_chirho)
        .map_err(IntakeRefusalChirho::PortChirho)?
    {
        CreditTopupOutcomeChirho::PostedChirho { posting_id_chirho } => {
            Ok(IntakeOutcomeChirho::CreditedChirho {
                posting_id_chirho,
                credits_chirho,
            })
        }
        CreditTopupOutcomeChirho::AlreadyPostedChirho { posting_id_chirho } => {
            Ok(IntakeOutcomeChirho::AlreadyCreditedChirho { posting_id_chirho })
        }
    }
}
