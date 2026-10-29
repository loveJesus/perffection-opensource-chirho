// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
//! The checkout builder and verified intake must agree on the payer and one purchase, including delayed settlement.

use perffection_credits_chirho::checkout_request_chirho::{
    checkout_request_chirho, CheckoutReturnUrlsChirho,
};
use perffection_credits_chirho::credit_view_port_chirho::PayerRefChirho;
use perffection_credits_chirho::hmac_sha256_chirho::{hex_lower_chirho, hmac_sha256_chirho};
use perffection_credits_chirho::intake_chirho::{
    intake_stripe_event_chirho, IntakeConfigChirho, IntakeOutcomeChirho, IntakeRefusalChirho,
};
use perffection_credits_chirho::pack_catalog_chirho::PackCatalogChirho;
use perffection_credits_chirho::topup_port_chirho::StubTopupPortChirho;

const FIXTURE_SECRET_CHIRHO: &str = "local-synthetic-webhook-fixture-chirho";
const NOW_CHIRHO: u64 = 1_793_045_000;

fn purchase_event_chirho(
    payer_chirho: &str,
    event_chirho: &str,
    kind_chirho: &str,
    paid_chirho: bool,
) -> Vec<u8> {
    let request_chirho = checkout_request_chirho(
        &PackCatalogChirho::starting_chirho(),
        "credits-1000-chirho",
        &PayerRefChirho::new_chirho(payer_chirho).unwrap(),
        CheckoutReturnUrlsChirho {
            paid_url_chirho: "http://127.0.0.1:4242/credits-paid-chirho",
            cancelled_url_chirho: "http://127.0.0.1:4242/credits-cancelled-chirho",
        },
        "attempt-fixture-chirho",
    )
    .unwrap();
    let field_chirho = |name_chirho: &str| {
        request_chirho
            .fields_chirho
            .iter()
            .find(|(key_chirho, _)| key_chirho == name_chirho)
            .unwrap()
            .1
            .clone()
    };
    serde_json::to_vec(&serde_json::json!({
        "id": event_chirho, "type": kind_chirho, "livemode": false,
        "data": {"object": {
            "id": "cs_same_purchase_chirho", "object": "checkout.session",
            "mode": field_chirho("mode"),
            "payment_status": if paid_chirho { "paid" } else { "unpaid" },
            "client_reference_id": field_chirho("client_reference_id"),
            "amount_total": field_chirho("line_items[0][price_data][unit_amount]").parse::<u64>().unwrap(),
            "currency": field_chirho("line_items[0][price_data][currency]"),
            "metadata": {"pack_id_chirho": field_chirho("metadata[pack_id_chirho]")}
        }}
    })).unwrap()
}

fn deliver_chirho(
    body_chirho: &[u8],
    port_chirho: &mut StubTopupPortChirho,
) -> Result<IntakeOutcomeChirho, IntakeRefusalChirho> {
    let mut signed_chirho = format!("{NOW_CHIRHO}.").into_bytes();
    signed_chirho.extend_from_slice(body_chirho);
    let header_chirho = format!(
        "t={NOW_CHIRHO},v1={}",
        hex_lower_chirho(&hmac_sha256_chirho(
            FIXTURE_SECRET_CHIRHO.as_bytes(),
            &signed_chirho
        ))
    );
    let catalog_chirho = PackCatalogChirho::starting_chirho();
    intake_stripe_event_chirho(
        body_chirho,
        &header_chirho,
        NOW_CHIRHO,
        &IntakeConfigChirho {
            signing_secret_chirho: FIXTURE_SECRET_CHIRHO,
            tolerance_seconds_chirho: 300,
            catalog_chirho: &catalog_chirho,
        },
        port_chirho,
    )
}

#[test]
fn an_opaque_payer_from_the_builder_survives_the_paid_event_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let event_chirho = purchase_event_chirho(
        "account/alpha-chirho",
        "evt_payer_chirho",
        "checkout.session.completed",
        true,
    );
    assert!(matches!(
        deliver_chirho(&event_chirho, &mut port_chirho),
        Ok(IntakeOutcomeChirho::CreditedChirho { .. })
    ));
    assert_eq!(
        port_chirho.credits_for_account_chirho("account/alpha-chirho"),
        1000
    );
}

#[test]
fn delayed_settlement_credits_only_after_the_paid_event_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let pending_chirho = purchase_event_chirho(
        "payer-alpha-chirho",
        "evt_pending_chirho",
        "checkout.session.completed",
        false,
    );
    assert!(matches!(
        deliver_chirho(&pending_chirho, &mut port_chirho),
        Ok(IntakeOutcomeChirho::IgnoredChirho(_))
    ));
    let settled_chirho = purchase_event_chirho(
        "payer-alpha-chirho",
        "evt_settled_chirho",
        "checkout.session.async_payment_succeeded",
        true,
    );
    assert!(matches!(
        deliver_chirho(&settled_chirho, &mut port_chirho),
        Ok(IntakeOutcomeChirho::CreditedChirho { .. })
    ));
    assert_eq!(
        port_chirho.credits_for_account_chirho("payer-alpha-chirho"),
        1000
    );
}

#[test]
fn two_event_ids_for_one_paid_checkout_post_one_purchase_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    for event_id_chirho in ["evt_first_chirho", "evt_second_chirho"] {
        let event_chirho = purchase_event_chirho(
            "payer-alpha-chirho",
            event_id_chirho,
            "checkout.session.completed",
            true,
        );
        deliver_chirho(&event_chirho, &mut port_chirho).unwrap();
    }
    assert_eq!(port_chirho.posting_count_chirho(), 1);
    assert_eq!(
        port_chirho.credits_for_account_chirho("payer-alpha-chirho"),
        1000
    );
}
