// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The top-up port: where a verified purchase becomes credits. The real port is Cairn's, W3's sealed `topup_chirho`,
//! still to be defined in W3's re-validation (Cairn #25979). This trait and its stub are PFE's, so the intake can be
//! built and tested now; their shape is offered as input to W3 (PFE #25980). Any implementation must keep two rules:
//! - **One posting per event id.** A replay of an event answers with its first posting; the same id carrying a
//!   different purchase is refused.
//! - **One posting per Checkout Session.** Stripe sends separate Event objects for one session (two deliveries with
//!   distinct ids, or a completed event and then a delayed payment's success;
//!   <https://docs.stripe.com/webhooks#handle-duplicate-events>). A second event for an already-posted session that
//!   carries the same purchase answers with the first posting; one that changes the payer, pack, credits, amount or
//!   currency is refused.
//!
//! Neither refusal overwrites anything. Both checks and the posting are one atomic step: the stub is single-threaded,
//! and a concurrent real port (W3) must hold the same rule under concurrency.

use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditTopupRequestChirho {
    /// Stripe's event id: one idempotency key.
    pub event_id_chirho: String,
    /// The Checkout Session: the other idempotency key, since one session is one purchase.
    pub checkout_session_id_chirho: String,
    pub account_reference_chirho: String,
    pub pack_id_chirho: String,
    pub credits_chirho: u64,
    pub amount_minor_chirho: u64,
    pub currency_chirho: String,
    /// SHA-256 of the verified event body: the witness W3 asks a top-up to carry.
    pub witness_sha256_chirho: String,
}

impl CreditTopupRequestChirho {
    /// The same purchase from the same event: every field but the witness. Stripe redelivers an event with the same id
    /// and the same purchase, but its raw body can differ in delivery fields (measured 2026-10-03: a `stripe events
    /// resend` was refused as a conflict when the witness was compared), so a redelivery must not be read as a
    /// conflict.
    pub fn same_purchase_chirho(&self, other_chirho: &Self) -> bool {
        self.event_id_chirho == other_chirho.event_id_chirho
            && self.same_session_purchase_chirho(other_chirho)
    }

    /// The same purchase of the same Checkout Session, whichever event reported it: the session, payer, pack, credits,
    /// amount and currency all agree.
    pub fn same_session_purchase_chirho(&self, other_chirho: &Self) -> bool {
        self.checkout_session_id_chirho == other_chirho.checkout_session_id_chirho
            && self.account_reference_chirho == other_chirho.account_reference_chirho
            && self.pack_id_chirho == other_chirho.pack_id_chirho
            && self.credits_chirho == other_chirho.credits_chirho
            && self.amount_minor_chirho == other_chirho.amount_minor_chirho
            && self.currency_chirho == other_chirho.currency_chirho
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreditTopupOutcomeChirho {
    PostedChirho { posting_id_chirho: String },
    AlreadyPostedChirho { posting_id_chirho: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreditTopupRefusalChirho {
    /// The same event id arrived with a different request: never post it, never overwrite the first.
    ConflictingReplayChirho,
    /// Another event for an already-posted Checkout Session carries a different purchase: never post it, never
    /// overwrite the first.
    ConflictingSessionChirho,
}

pub trait CreditTopupPortChirho {
    fn topup_chirho(
        &mut self,
        request_chirho: &CreditTopupRequestChirho,
    ) -> Result<CreditTopupOutcomeChirho, CreditTopupRefusalChirho>;
}

/// The development stub: in memory, one posting per Checkout Session, every event id remembered. Not a ledger: it
/// holds no balance authority and grows with every purchase, which is acceptable only because it lives for one local
/// test session.
#[derive(Debug, Default)]
pub struct StubTopupPortChirho {
    /// Every event id seen, with the request it carried and the posting it was answered with.
    events_chirho: BTreeMap<String, (CreditTopupRequestChirho, String)>,
    /// One posting per Checkout Session: its first request and its posting id.
    sessions_chirho: BTreeMap<String, (CreditTopupRequestChirho, String)>,
}

impl StubTopupPortChirho {
    pub fn credits_for_account_chirho(&self, account_reference_chirho: &str) -> u64 {
        self.sessions_chirho
            .values()
            .filter(|(request_chirho, _)| {
                request_chirho.account_reference_chirho == account_reference_chirho
            })
            .map(|(request_chirho, _)| request_chirho.credits_chirho)
            .sum()
    }

    pub fn posting_count_chirho(&self) -> usize {
        self.sessions_chirho.len()
    }
}

impl CreditTopupPortChirho for StubTopupPortChirho {
    fn topup_chirho(
        &mut self,
        request_chirho: &CreditTopupRequestChirho,
    ) -> Result<CreditTopupOutcomeChirho, CreditTopupRefusalChirho> {
        if let Some((first_chirho, posting_id_chirho)) =
            self.events_chirho.get(&request_chirho.event_id_chirho)
        {
            // The first delivery's witness is kept; a redelivery of the same purchase answers with the first posting.
            return if first_chirho.same_purchase_chirho(request_chirho) {
                Ok(CreditTopupOutcomeChirho::AlreadyPostedChirho {
                    posting_id_chirho: posting_id_chirho.clone(),
                })
            } else {
                Err(CreditTopupRefusalChirho::ConflictingReplayChirho)
            };
        }
        if let Some((first_chirho, posting_id_chirho)) = self
            .sessions_chirho
            .get(&request_chirho.checkout_session_id_chirho)
        {
            if !first_chirho.same_session_purchase_chirho(request_chirho) {
                return Err(CreditTopupRefusalChirho::ConflictingSessionChirho);
            }
            // A second event for the same purchase: remembered under its own id, so a later reuse of that id with
            // other content is a conflicting replay, and answered with the session's one posting.
            let posting_id_chirho = posting_id_chirho.clone();
            self.events_chirho.insert(
                request_chirho.event_id_chirho.clone(),
                (request_chirho.clone(), posting_id_chirho.clone()),
            );
            return Ok(CreditTopupOutcomeChirho::AlreadyPostedChirho { posting_id_chirho });
        }
        let posting_id_chirho = format!("stub-posting-{}", self.sessions_chirho.len() + 1);
        self.sessions_chirho.insert(
            request_chirho.checkout_session_id_chirho.clone(),
            (request_chirho.clone(), posting_id_chirho.clone()),
        );
        self.events_chirho.insert(
            request_chirho.event_id_chirho.clone(),
            (request_chirho.clone(), posting_id_chirho.clone()),
        );
        Ok(CreditTopupOutcomeChirho::PostedChirho { posting_id_chirho })
    }
}
