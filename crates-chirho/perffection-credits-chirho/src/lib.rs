// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The credit purchase intake, Stripe TEST mode only (L.J., 2026-10-02: "Yes, build in test mode").
//!
//! A purchase is proven by one thing: a Stripe event whose signature verifies, that says it is a test-mode, paid
//! Checkout Session. The return page proves nothing. The intake turns that verified event into one top-up request and
//! hands it to a [`topup_port_chirho::CreditTopupPortChirho`], idempotent on the event id. The real port is Cairn's,
//! to be defined in W3's re-validation; until then the intake runs against the stub port this crate owns, whose
//! shape is offered as input to W3 (PFE #25980).
//!
//! No card data is ever seen here (Stripe's hosted Checkout), no live key is accepted, and nothing here opens a
//! public webhook: in development the Stripe CLI forwards test events to a local stand-in.
//!
//! Workflow: spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md

pub mod checkout_request_chirho;
pub mod credit_view_port_chirho;
pub mod hmac_sha256_chirho;
pub mod intake_chirho;
pub mod pack_catalog_chirho;
pub mod stripe_event_chirho;
pub mod stripe_mode_chirho;
pub mod stripe_signature_chirho;
pub mod topup_port_chirho;
