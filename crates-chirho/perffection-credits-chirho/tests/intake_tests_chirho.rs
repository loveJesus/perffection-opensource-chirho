// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! The intake end to end, against the stub port: a paid test-mode checkout credits once and only once, and every
//! refusal posts nothing. Signatures are made with the crate's HMAC, which hmac_vectors_chirho.rs proves against
//! RFC 4231 independently.

use perffection_credits_chirho::hmac_sha256_chirho::{hex_lower_chirho, hmac_sha256_chirho};
use perffection_credits_chirho::intake_chirho::{
    intake_stripe_event_chirho, IntakeConfigChirho, IntakeOutcomeChirho, IntakeRefusalChirho,
};
use perffection_credits_chirho::pack_catalog_chirho::{
    CreditPackChirho, PackCatalogChirho, PackRefusalChirho,
};
use perffection_credits_chirho::stripe_event_chirho::EventRefusalChirho;
use perffection_credits_chirho::stripe_signature_chirho::SignatureRefusalChirho;
use perffection_credits_chirho::topup_port_chirho::{
    CreditTopupRefusalChirho, StubTopupPortChirho,
};
use serde_json::json;

const SECRET_CHIRHO: &str = "whsec_test_fixture_chirho";
const NOW_CHIRHO: u64 = 1_790_955_000;

fn catalog_chirho() -> PackCatalogChirho {
    PackCatalogChirho::default().with_pack_chirho(
        "pack-small-chirho",
        CreditPackChirho {
            amount_minor_chirho: 500,
            currency_chirho: "usd".into(),
            credits_chirho: 500,
        },
    )
}

fn event_chirho(
    event_id_chirho: &str,
    livemode_chirho: bool,
    payment_status_chirho: &str,
    amount_chirho: u64,
) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "id": event_id_chirho, "object": "event", "type": "checkout.session.completed", "livemode": livemode_chirho,
        "data": {"object": {
            "id": "cs_test_fixture_chirho", "object": "checkout.session", "mode": "payment",
            "payment_status": payment_status_chirho, "client_reference_id": "account-fixture-chirho",
            "amount_total": amount_chirho, "currency": "usd", "metadata": {"pack_id_chirho": "pack-small-chirho"}
        }}
    })).unwrap()
}

fn signed_header_chirho(body_chirho: &[u8]) -> String {
    let mut payload_chirho = format!("{NOW_CHIRHO}.").into_bytes();
    payload_chirho.extend_from_slice(body_chirho);
    format!(
        "t={NOW_CHIRHO},v1={}",
        hex_lower_chirho(&hmac_sha256_chirho(
            SECRET_CHIRHO.as_bytes(),
            &payload_chirho
        ))
    )
}

fn intake_chirho(
    body_chirho: &[u8],
    header_chirho: &str,
    port_chirho: &mut StubTopupPortChirho,
) -> Result<IntakeOutcomeChirho, IntakeRefusalChirho> {
    let catalog_chirho = catalog_chirho();
    let config_chirho = IntakeConfigChirho {
        signing_secret_chirho: SECRET_CHIRHO,
        tolerance_seconds_chirho: 300,
        catalog_chirho: &catalog_chirho,
    };
    intake_stripe_event_chirho(
        body_chirho,
        header_chirho,
        NOW_CHIRHO,
        &config_chirho,
        port_chirho,
    )
}

#[test]
fn a_paid_test_checkout_credits_once_and_a_replay_credits_nothing_more_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let body_chirho = event_chirho("evt_one_chirho", false, "paid", 500);
    let header_chirho = signed_header_chirho(&body_chirho);
    assert_eq!(
        intake_chirho(&body_chirho, &header_chirho, &mut port_chirho),
        Ok(IntakeOutcomeChirho::CreditedChirho {
            posting_id_chirho: "stub-posting-1".into(),
            credits_chirho: 500
        })
    );
    // Stripe retries deliveries; the same event must not credit twice.
    assert_eq!(
        intake_chirho(&body_chirho, &header_chirho, &mut port_chirho),
        Ok(IntakeOutcomeChirho::AlreadyCreditedChirho {
            posting_id_chirho: "stub-posting-1".into()
        })
    );
    assert_eq!(
        (
            port_chirho.posting_count_chirho(),
            port_chirho.credits_for_account_chirho("account-fixture-chirho")
        ),
        (1, 500)
    );
}

#[test]
fn live_mode_a_bad_signature_and_a_wrong_price_post_nothing_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let live_chirho = event_chirho("evt_live_chirho", true, "paid", 500);
    assert_eq!(
        intake_chirho(
            &live_chirho,
            &signed_header_chirho(&live_chirho),
            &mut port_chirho
        ),
        Err(IntakeRefusalChirho::EventChirho(
            EventRefusalChirho::LiveModeChirho
        ))
    );
    let body_chirho = event_chirho("evt_two_chirho", false, "paid", 500);
    let forged_chirho = format!("t={NOW_CHIRHO},v1={}", "0".repeat(64));
    assert_eq!(
        intake_chirho(&body_chirho, &forged_chirho, &mut port_chirho),
        Err(IntakeRefusalChirho::SignatureChirho(
            SignatureRefusalChirho::NoMatchingSignatureChirho
        ))
    );
    let cheap_chirho = event_chirho("evt_three_chirho", false, "paid", 499);
    assert_eq!(
        intake_chirho(
            &cheap_chirho,
            &signed_header_chirho(&cheap_chirho),
            &mut port_chirho
        ),
        Err(IntakeRefusalChirho::PackChirho(
            PackRefusalChirho::PriceMismatchChirho
        ))
    );
    let unknown_chirho = String::from_utf8(event_chirho("evt_four_chirho", false, "paid", 500))
        .unwrap()
        .replace("pack-small-chirho", "pack-free-chirho");
    assert_eq!(
        intake_chirho(
            unknown_chirho.as_bytes(),
            &signed_header_chirho(unknown_chirho.as_bytes()),
            &mut port_chirho
        ),
        Err(IntakeRefusalChirho::PackChirho(
            PackRefusalChirho::UnknownPackChirho
        ))
    );
    assert_eq!(port_chirho.posting_count_chirho(), 0);
}

#[test]
fn unpaid_sessions_and_other_events_are_ignored_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let unpaid_chirho = event_chirho("evt_unpaid_chirho", false, "unpaid", 500);
    assert_eq!(
        intake_chirho(
            &unpaid_chirho,
            &signed_header_chirho(&unpaid_chirho),
            &mut port_chirho
        ),
        Ok(IntakeOutcomeChirho::IgnoredChirho(
            "the session is not paid"
        ))
    );
    let other_chirho = String::from_utf8(event_chirho("evt_other_chirho", false, "paid", 500))
        .unwrap()
        .replace("checkout.session.completed", "customer.created");
    assert_eq!(
        intake_chirho(
            other_chirho.as_bytes(),
            &signed_header_chirho(other_chirho.as_bytes()),
            &mut port_chirho
        ),
        Ok(IntakeOutcomeChirho::IgnoredChirho(
            "not a completed checkout"
        ))
    );
    assert_eq!(port_chirho.posting_count_chirho(), 0);
}

#[test]
fn the_same_event_id_with_a_different_request_is_refused_not_overwritten_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let first_chirho = event_chirho("evt_same_chirho", false, "paid", 500);
    intake_chirho(
        &first_chirho,
        &signed_header_chirho(&first_chirho),
        &mut port_chirho,
    )
    .unwrap();
    // A validly signed body reusing the id with other content (here, another account): never a second posting.
    let second_chirho = String::from_utf8(first_chirho.clone())
        .unwrap()
        .replace("account-fixture-chirho", "account-other-chirho");
    assert_eq!(
        intake_chirho(
            second_chirho.as_bytes(),
            &signed_header_chirho(second_chirho.as_bytes()),
            &mut port_chirho
        ),
        Err(IntakeRefusalChirho::PortChirho(
            CreditTopupRefusalChirho::ConflictingReplayChirho
        ))
    );
    assert_eq!(
        (
            port_chirho.posting_count_chirho(),
            port_chirho.credits_for_account_chirho("account-other-chirho")
        ),
        (1, 0)
    );
}

#[test]
fn a_redelivery_with_different_delivery_fields_is_already_credited_not_a_conflict_chirho() {
    // Stripe resends an event with the same id and purchase but a changed body (measured with `stripe events resend`).
    let mut port_chirho = StubTopupPortChirho::default();
    let first_chirho = event_chirho("evt_resent_chirho", false, "paid", 500);
    let mut value_chirho: serde_json::Value = serde_json::from_slice(&first_chirho).unwrap();
    value_chirho["pending_webhooks"] = serde_json::json!(2);
    let redelivered_chirho = serde_json::to_vec(&value_chirho).unwrap();
    assert_ne!(first_chirho, redelivered_chirho);
    assert_eq!(
        intake_chirho(
            &first_chirho,
            &signed_header_chirho(&first_chirho),
            &mut port_chirho
        ),
        Ok(IntakeOutcomeChirho::CreditedChirho {
            posting_id_chirho: "stub-posting-1".into(),
            credits_chirho: 500
        })
    );
    assert_eq!(
        intake_chirho(
            &redelivered_chirho,
            &signed_header_chirho(&redelivered_chirho),
            &mut port_chirho
        ),
        Ok(IntakeOutcomeChirho::AlreadyCreditedChirho {
            posting_id_chirho: "stub-posting-1".into()
        })
    );
    assert_eq!(
        (
            port_chirho.posting_count_chirho(),
            port_chirho.credits_for_account_chirho("account-fixture-chirho")
        ),
        (1, 500)
    );
}

// ---- One purchase per Checkout Session, across events (gpt #28846) ----------------------------------------------

/// Two packs, so a changed pack can still pass its own price check and reach the port.
fn two_pack_catalog_chirho() -> PackCatalogChirho {
    catalog_chirho().with_pack_chirho(
        "pack-large-chirho",
        CreditPackChirho {
            amount_minor_chirho: 2_500,
            currency_chirho: "usd".into(),
            credits_chirho: 2_500,
        },
    )
}

/// One signed-ready event, every binding chosen by the test.
struct SessionEventChirho<'event_chirho> {
    event_id_chirho: &'event_chirho str,
    kind_chirho: &'event_chirho str,
    livemode_chirho: bool,
    session_id_chirho: &'event_chirho str,
    payer_chirho: &'event_chirho str,
    pack_id_chirho: &'event_chirho str,
    amount_chirho: u64,
    payment_status_chirho: &'event_chirho str,
}

const BASE_EVENT_CHIRHO: SessionEventChirho<'static> = SessionEventChirho {
    event_id_chirho: "evt_base_chirho",
    kind_chirho: "checkout.session.completed",
    livemode_chirho: false,
    session_id_chirho: "cs_one_purchase_chirho",
    payer_chirho: "account/alpha-chirho",
    pack_id_chirho: "pack-small-chirho",
    amount_chirho: 500,
    payment_status_chirho: "paid",
};

fn session_event_chirho(spec_chirho: &SessionEventChirho<'_>) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "id": spec_chirho.event_id_chirho, "object": "event", "type": spec_chirho.kind_chirho,
        "livemode": spec_chirho.livemode_chirho,
        "data": {"object": {
            "id": spec_chirho.session_id_chirho, "object": "checkout.session", "mode": "payment",
            "payment_status": spec_chirho.payment_status_chirho, "client_reference_id": spec_chirho.payer_chirho,
            "amount_total": spec_chirho.amount_chirho, "currency": "usd",
            "metadata": {"pack_id_chirho": spec_chirho.pack_id_chirho}
        }}
    }))
    .unwrap()
}

fn deliver_chirho(
    spec_chirho: &SessionEventChirho<'_>,
    port_chirho: &mut StubTopupPortChirho,
) -> Result<IntakeOutcomeChirho, IntakeRefusalChirho> {
    let body_chirho = session_event_chirho(spec_chirho);
    let catalog_chirho = two_pack_catalog_chirho();
    intake_stripe_event_chirho(
        &body_chirho,
        &signed_header_chirho(&body_chirho),
        NOW_CHIRHO,
        &IntakeConfigChirho {
            signing_secret_chirho: SECRET_CHIRHO,
            tolerance_seconds_chirho: 300,
            catalog_chirho: &catalog_chirho,
        },
        port_chirho,
    )
}

#[test]
fn another_event_for_a_posted_session_with_a_changed_purchase_is_refused_not_overwritten_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    assert!(matches!(
        deliver_chirho(&BASE_EVENT_CHIRHO, &mut port_chirho),
        Ok(IntakeOutcomeChirho::CreditedChirho { .. })
    ));
    let changed_payer_chirho = SessionEventChirho {
        event_id_chirho: "evt_changed_payer_chirho",
        payer_chirho: "account/other-chirho",
        ..BASE_EVENT_CHIRHO
    };
    let changed_pack_chirho = SessionEventChirho {
        event_id_chirho: "evt_changed_pack_chirho",
        pack_id_chirho: "pack-large-chirho",
        amount_chirho: 2_500,
        ..BASE_EVENT_CHIRHO
    };
    for changed_chirho in [&changed_payer_chirho, &changed_pack_chirho] {
        assert_eq!(
            deliver_chirho(changed_chirho, &mut port_chirho),
            Err(IntakeRefusalChirho::PortChirho(
                CreditTopupRefusalChirho::ConflictingSessionChirho
            ))
        );
    }
    assert_eq!(port_chirho.posting_count_chirho(), 1);
    assert_eq!(
        port_chirho.credits_for_account_chirho("account/alpha-chirho"),
        500
    );
    assert_eq!(
        port_chirho.credits_for_account_chirho("account/other-chirho"),
        0
    );
}

#[test]
fn a_second_event_id_once_answered_cannot_be_reused_for_another_purchase_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    deliver_chirho(&BASE_EVENT_CHIRHO, &mut port_chirho).unwrap();
    let second_chirho = SessionEventChirho {
        event_id_chirho: "evt_second_chirho",
        ..BASE_EVENT_CHIRHO
    };
    assert_eq!(
        deliver_chirho(&second_chirho, &mut port_chirho),
        Ok(IntakeOutcomeChirho::AlreadyCreditedChirho {
            posting_id_chirho: "stub-posting-1".into()
        })
    );
    // The same second id, now naming another session: a conflicting replay, never a new posting.
    let reused_chirho = SessionEventChirho {
        session_id_chirho: "cs_another_purchase_chirho",
        ..second_chirho
    };
    assert_eq!(
        deliver_chirho(&reused_chirho, &mut port_chirho),
        Err(IntakeRefusalChirho::PortChirho(
            CreditTopupRefusalChirho::ConflictingReplayChirho
        ))
    );
    assert_eq!(port_chirho.posting_count_chirho(), 1);
    assert_eq!(
        port_chirho.credits_for_account_chirho("account/alpha-chirho"),
        500
    );
}

#[test]
fn an_unpaid_completion_and_a_failed_delayed_payment_post_nothing_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let unpaid_chirho = SessionEventChirho {
        event_id_chirho: "evt_unpaid_chirho",
        payment_status_chirho: "unpaid",
        ..BASE_EVENT_CHIRHO
    };
    let failed_chirho = SessionEventChirho {
        event_id_chirho: "evt_failed_chirho",
        kind_chirho: "checkout.session.async_payment_failed",
        payment_status_chirho: "unpaid",
        ..BASE_EVENT_CHIRHO
    };
    for not_paid_chirho in [&unpaid_chirho, &failed_chirho] {
        assert!(matches!(
            deliver_chirho(not_paid_chirho, &mut port_chirho),
            Ok(IntakeOutcomeChirho::IgnoredChirho(_))
        ));
    }
    assert_eq!(port_chirho.posting_count_chirho(), 0);
}

#[test]
fn a_delayed_success_keeps_every_gate_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let settled_chirho = SessionEventChirho {
        event_id_chirho: "evt_settled_chirho",
        kind_chirho: "checkout.session.async_payment_succeeded",
        ..BASE_EVENT_CHIRHO
    };
    assert_eq!(
        deliver_chirho(
            &SessionEventChirho {
                livemode_chirho: true,
                ..settled_chirho
            },
            &mut port_chirho
        ),
        Err(IntakeRefusalChirho::EventChirho(
            EventRefusalChirho::LiveModeChirho
        ))
    );
    assert_eq!(
        deliver_chirho(
            &SessionEventChirho {
                amount_chirho: 499,
                ..settled_chirho
            },
            &mut port_chirho
        ),
        Err(IntakeRefusalChirho::PackChirho(
            PackRefusalChirho::PriceMismatchChirho
        ))
    );
    assert_eq!(
        deliver_chirho(
            &SessionEventChirho {
                payment_status_chirho: "unpaid",
                ..settled_chirho
            },
            &mut port_chirho
        ),
        Ok(IntakeOutcomeChirho::IgnoredChirho(
            "the session is not paid"
        ))
    );
    assert_eq!(port_chirho.posting_count_chirho(), 0);
    assert!(matches!(
        deliver_chirho(&settled_chirho, &mut port_chirho),
        Ok(IntakeOutcomeChirho::CreditedChirho { .. })
    ));
}

#[test]
fn the_payer_keeps_its_own_law_and_the_pack_id_keeps_its_own_rule_chirho() {
    let mut port_chirho = StubTopupPortChirho::default();
    let too_long_chirho = "a".repeat(129);
    for bad_payer_chirho in [
        "",
        "account/has space-chirho",
        "account/é-chirho",
        too_long_chirho.as_str(),
    ] {
        assert_eq!(
            deliver_chirho(
                &SessionEventChirho {
                    payer_chirho: bad_payer_chirho,
                    ..BASE_EVENT_CHIRHO
                },
                &mut port_chirho
            ),
            Err(IntakeRefusalChirho::EventChirho(
                EventRefusalChirho::IncompleteSessionChirho("account reference")
            )),
            "{bad_payer_chirho:?}"
        );
    }
    assert_eq!(
        deliver_chirho(
            &SessionEventChirho {
                pack_id_chirho: "pack/small-chirho",
                ..BASE_EVENT_CHIRHO
            },
            &mut port_chirho
        ),
        Err(IntakeRefusalChirho::EventChirho(
            EventRefusalChirho::IncompleteSessionChirho("pack")
        ))
    );
    assert_eq!(port_chirho.posting_count_chirho(), 0);
}
