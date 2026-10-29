// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Checkout Session request: the exact money fields for each pack, what the intake will read back, an idempotency
//! key that hides the payer, and refusals for an unknown pack, a bad attempt id or an unsafe return address.

use perffection_credits_chirho::checkout_request_chirho::{
    checkout_request_chirho, CheckoutRequestChirho, CheckoutRequestRefusalChirho,
    CheckoutReturnUrlsChirho, CHECKOUT_ATTEMPT_ID_MAX_BYTES_CHIRHO,
};
use perffection_credits_chirho::credit_view_port_chirho::PayerRefChirho;
use perffection_credits_chirho::pack_catalog_chirho::PackCatalogChirho;

const RETURNS_CHIRHO: CheckoutReturnUrlsChirho<'static> = CheckoutReturnUrlsChirho {
    paid_url_chirho:
        "https://beta.example.test/app-chirho/perffection-workspace/credits-paid-chirho",
    cancelled_url_chirho:
        "https://beta.example.test/app-chirho/perffection-workspace/credits-cancelled-chirho",
};

fn payer_chirho() -> PayerRefChirho {
    PayerRefChirho::new_chirho("account/alpha-chirho").expect("a well-formed payer")
}

fn field_chirho<'request_chirho>(
    request_chirho: &'request_chirho CheckoutRequestChirho,
    name_chirho: &str,
) -> &'request_chirho str {
    let matches_chirho: Vec<&str> = request_chirho
        .fields_chirho
        .iter()
        .filter(|(field_name_chirho, _)| field_name_chirho == name_chirho)
        .map(|(_, value_chirho)| value_chirho.as_str())
        .collect();
    assert_eq!(matches_chirho.len(), 1, "exactly one {name_chirho}");
    matches_chirho[0]
}

#[test]
fn each_pack_asks_for_exactly_its_price_and_names_itself_for_the_intake_chirho() {
    let catalog_chirho = PackCatalogChirho::starting_chirho();
    for (pack_id_chirho, pack_chirho) in catalog_chirho.packs_chirho() {
        let request_chirho = checkout_request_chirho(
            &catalog_chirho,
            pack_id_chirho,
            &payer_chirho(),
            RETURNS_CHIRHO,
            "attempt-1",
        )
        .expect("a catalog pack");
        assert_eq!(field_chirho(&request_chirho, "mode"), "payment");
        assert_eq!(
            field_chirho(&request_chirho, "line_items[0][quantity]"),
            "1"
        );
        let cents_chirho: u64 =
            field_chirho(&request_chirho, "line_items[0][price_data][unit_amount]")
                .parse()
                .expect("whole cents");
        let currency_chirho = field_chirho(&request_chirho, "line_items[0][price_data][currency]");
        assert_eq!(cents_chirho, pack_chirho.amount_minor_chirho);
        // What comes back on the paid event is exactly what the intake's own price check accepts.
        assert_eq!(
            catalog_chirho.credits_for_chirho(pack_id_chirho, cents_chirho, currency_chirho),
            Ok(pack_chirho.credits_chirho)
        );
        assert_eq!(
            field_chirho(&request_chirho, "metadata[pack_id_chirho]"),
            pack_id_chirho
        );
        assert_eq!(
            field_chirho(&request_chirho, "client_reference_id"),
            "account/alpha-chirho"
        );
        assert_eq!(
            field_chirho(&request_chirho, "success_url"),
            RETURNS_CHIRHO.paid_url_chirho
        );
        assert_eq!(
            field_chirho(&request_chirho, "cancel_url"),
            RETURNS_CHIRHO.cancelled_url_chirho
        );
    }
}

#[test]
fn an_unknown_pack_is_refused_chirho() {
    assert_eq!(
        checkout_request_chirho(
            &PackCatalogChirho::starting_chirho(),
            "credits-999-chirho",
            &payer_chirho(),
            RETURNS_CHIRHO,
            "attempt-1"
        ),
        Err(CheckoutRequestRefusalChirho::UnknownPackChirho)
    );
}

#[test]
fn one_attempt_makes_one_key_and_the_key_hides_the_payer_chirho() {
    let catalog_chirho = PackCatalogChirho::starting_chirho();
    let request_chirho = |attempt_chirho: &str| {
        checkout_request_chirho(
            &catalog_chirho,
            "credits-1000-chirho",
            &payer_chirho(),
            RETURNS_CHIRHO,
            attempt_chirho,
        )
        .expect("a catalog pack")
        .idempotency_key_chirho
    };
    assert_eq!(request_chirho("attempt-1"), request_chirho("attempt-1"));
    assert_ne!(request_chirho("attempt-1"), request_chirho("attempt-2"));
    assert!(!request_chirho("attempt-1").contains("alpha"));
}

#[test]
fn a_bad_attempt_id_or_an_unsafe_return_address_is_refused_chirho() {
    let catalog_chirho = PackCatalogChirho::starting_chirho();
    let too_long_chirho = "a".repeat(CHECKOUT_ATTEMPT_ID_MAX_BYTES_CHIRHO + 1);
    for attempt_chirho in ["", "has space", "slash/no", too_long_chirho.as_str()] {
        assert_eq!(
            checkout_request_chirho(
                &catalog_chirho,
                "credits-1000-chirho",
                &payer_chirho(),
                RETURNS_CHIRHO,
                attempt_chirho
            ),
            Err(CheckoutRequestRefusalChirho::InvalidAttemptChirho),
            "{attempt_chirho:?}"
        );
    }
    for unsafe_url_chirho in [
        "http://beta.example.test/paid",
        "javascript:alert(1)",
        "ftp://x",
    ] {
        let returns_chirho = CheckoutReturnUrlsChirho {
            paid_url_chirho: unsafe_url_chirho,
            ..RETURNS_CHIRHO
        };
        assert_eq!(
            checkout_request_chirho(
                &catalog_chirho,
                "credits-1000-chirho",
                &payer_chirho(),
                returns_chirho,
                "attempt-1"
            ),
            Err(CheckoutRequestRefusalChirho::InvalidReturnUrlChirho),
            "{unsafe_url_chirho}"
        );
    }
    let loopback_chirho = CheckoutReturnUrlsChirho {
        paid_url_chirho: "http://127.0.0.1:4242/credits-paid-chirho",
        cancelled_url_chirho: "http://localhost:4242/credits-cancelled-chirho",
    };
    assert!(checkout_request_chirho(
        &catalog_chirho,
        "credits-1000-chirho",
        &payer_chirho(),
        loopback_chirho,
        "attempt-1"
    )
    .is_ok());
}

/// The return address's authority is parsed, never prefix-matched (gpt #28814): userinfo would send her to another host,
/// so it and every malformed host, port or character is refused, and plain http is only ever this exact machine.
#[test]
fn a_return_address_whose_real_host_is_elsewhere_or_malformed_is_refused_chirho() {
    let catalog_chirho = PackCatalogChirho::starting_chirho();
    let verdict_chirho = |paid_url_chirho: &str| {
        checkout_request_chirho(
            &catalog_chirho,
            "credits-1000-chirho",
            &payer_chirho(),
            CheckoutReturnUrlsChirho {
                paid_url_chirho,
                ..RETURNS_CHIRHO
            },
            "attempt-1",
        )
        .map(|_| ())
    };
    for refused_chirho in [
        "http://localhost:80@evil.example/",
        "http://127.0.0.1:80@evil.example/",
        "https://user@beta.example.test/paid",
        "https://",
        "https:///paid",
        "https://beta.example.test:0/paid",
        "https://beta.example.test:065/paid",
        "https://beta.example.test:65536/paid",
        "https://beta.example.test:/paid",
        "https://beta.example.test:8a/paid",
        "https://beta.example.test:443:443/paid",
        "https://beta example.test/paid",
        "https://beta.example.test/pa\tid",
        "https://beta.example.test/paid\n",
        "https://beta.example.test\\@evil.example/",
        "https://-beta.example.test/paid",
        "https://beta-.example.test/paid",
        "https://beta..example.test/paid",
        "https://example./paid",
        "https://localhost/paid",
        "https://192.0.2.1/paid",
        "https://[::1]/paid",
        "https://exam%70le.test/paid",
        "http://127.0.0.1.evil.example/paid",
        "http://localhost.evil.example/paid",
        "http://10.0.0.1:4242/paid",
        "HTTPS://beta.example.test/paid",
    ] {
        assert_eq!(
            verdict_chirho(refused_chirho),
            Err(CheckoutRequestRefusalChirho::InvalidReturnUrlChirho),
            "{refused_chirho:?}"
        );
    }
    for accepted_chirho in [
        "https://beta.example.test/app-chirho/perffection-workspace/credits-paid-chirho",
        "https://beta.example.test:8443/paid?from=checkout#top",
        "https://beta.example.test",
        "http://127.0.0.1:4242/credits-paid-chirho",
        "http://localhost:4242/credits-paid-chirho",
        "http://localhost/credits-paid-chirho",
    ] {
        assert_eq!(
            verdict_chirho(accepted_chirho),
            Ok(()),
            "{accepted_chirho:?}"
        );
    }
}
