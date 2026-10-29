// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The held Credits page and its return pages (gpt #28745): payer isolation, unavailable never zero, recovery after
//! a failed read, the bounded history, no purchase control while purchases are held, and no route until activation.

use perffection_credits_chirho::credit_view_port_chirho::{
    CreditEntryChirho, CreditEntryKindChirho, CreditViewPortChirho, CreditsChirho,
    FixtureCreditViewChirho, PayerRefChirho, UnconnectedCreditViewChirho,
    CREDIT_HISTORY_MAX_ENTRIES_CHIRHO,
};
use perffection_credits_chirho::pack_catalog_chirho::PackCatalogChirho;
use perffection_ui_chirho::{CreditsCopyChirho, UiLocaleChirho};
use perffection_workspace_chirho::credits_flow_chirho::credits_page_chirho::{
    utc_date_chirho, CreditsPageV1Chirho, WORKSPACE_CREDITS_HREF_CHIRHO,
};
use perffection_workspace_chirho::credits_flow_chirho::credits_render_chirho::{
    render_credits_chirho, render_credits_return_chirho,
};
use perffection_workspace_chirho::credits_flow_chirho::credits_return_chirho::{
    CreditsReturnKindChirho, CreditsReturnV1Chirho,
};

#[path = "../support_chirho/mod.rs"]
mod support_chirho;

/// A fixed instant (2026-10-26 19:52 UTC), so the page's dates are deterministic.
const MOMENT_MS_CHIRHO: u64 = 1_793_044_340_798;
const DAY_MS_CHIRHO: u64 = 86_400_000;

fn payer_chirho(value_chirho: &str) -> PayerRefChirho {
    PayerRefChirho::new_chirho(value_chirho).expect("a well-formed fixture payer")
}

fn entry_chirho(
    kind_chirho: CreditEntryKindChirho,
    credits_chirho: u64,
    at_ms_chirho: u64,
) -> CreditEntryChirho {
    CreditEntryChirho {
        kind_chirho,
        credits_chirho: CreditsChirho::new_chirho(credits_chirho),
        at_ms_chirho,
        pack_id_chirho: (kind_chirho == CreditEntryKindChirho::PurchaseChirho)
            .then(|| format!("credits-{credits_chirho}-chirho")),
    }
}

/// Two owners' ledgers side by side, plus one that answered with nothing.
fn ledger_chirho() -> FixtureCreditViewChirho {
    FixtureCreditViewChirho::default()
        .with_payer_chirho(
            payer_chirho("account/alpha-chirho"),
            CreditsChirho::new_chirho(1_250),
            vec![
                entry_chirho(
                    CreditEntryKindChirho::PurchaseChirho,
                    1_000,
                    MOMENT_MS_CHIRHO - 2 * DAY_MS_CHIRHO,
                ),
                entry_chirho(
                    CreditEntryKindChirho::GrantChirho,
                    500,
                    MOMENT_MS_CHIRHO - DAY_MS_CHIRHO,
                ),
                entry_chirho(CreditEntryKindChirho::SpendChirho, 250, MOMENT_MS_CHIRHO),
            ],
        )
        .with_payer_chirho(
            payer_chirho("account/beta-chirho"),
            CreditsChirho::new_chirho(7_777),
            vec![
                entry_chirho(
                    CreditEntryKindChirho::PurchaseChirho,
                    5_000,
                    MOMENT_MS_CHIRHO,
                ),
                entry_chirho(CreditEntryKindChirho::SpendChirho, 3_333, MOMENT_MS_CHIRHO),
            ],
        )
        .with_payer_chirho(
            payer_chirho("account/empty-chirho"),
            CreditsChirho::new_chirho(0),
            Vec::new(),
        )
}

fn english_page_chirho(
    port_chirho: &impl CreditViewPortChirho,
    payer_value_chirho: &str,
) -> CreditsPageV1Chirho {
    CreditsPageV1Chirho::read_chirho(
        port_chirho,
        &payer_chirho(payer_value_chirho),
        &PackCatalogChirho::starting_chirho(),
        UiLocaleChirho::EnglishChirho,
    )
}

fn page_html_chirho(page_chirho: &CreditsPageV1Chirho) -> String {
    let html_chirho = render_credits_chirho(page_chirho).expect("the Credits page renders");
    support_chirho::assert_page_wiring_chirho("credits", &html_chirho);
    html_chirho
}

/// Copy as the page renders it: escaped by the template engine itself, so an apostrophe reads `&#39;`.
fn shown_chirho(text_chirho: &str) -> String {
    askama::filters::escape(text_chirho, askama::filters::Html)
        .expect("HTML escaping cannot fail")
        .to_string()
}

/// The balance panel alone, from its heading to where the packs' section opens: where a balance would be, and
/// nowhere else.
fn balance_panel_chirho(html_chirho: &str) -> &str {
    let start_chirho = html_chirho
        .find("id=\"credits-balance-heading-chirho\"")
        .expect("the balance panel");
    let end_chirho = html_chirho
        .find("aria-labelledby=\"credits-packs-heading-chirho\"")
        .expect("the packs panel");
    &html_chirho[start_chirho..end_chirho]
}

/// What a reader sees of a piece of markup: its text, with every tag left out and every character reference (such
/// as `&#39;`, an apostrophe) read as one mark rather than as the digits that spell it.
fn visible_text_chirho(html_chirho: &str) -> String {
    let mut text_chirho = String::with_capacity(html_chirho.len());
    let mut in_tag_chirho = false;
    let mut in_reference_chirho = false;
    for character_chirho in html_chirho.chars() {
        match character_chirho {
            '<' => in_tag_chirho = true,
            '>' => in_tag_chirho = false,
            '&' if !in_tag_chirho => in_reference_chirho = true,
            ';' if in_reference_chirho => {
                in_reference_chirho = false;
                text_chirho.push('\u{FFFD}');
            }
            _ if !in_tag_chirho && !in_reference_chirho => text_chirho.push(character_chirho),
            _ => {}
        }
    }
    text_chirho
}

#[test]
fn the_page_shows_her_balance_the_packs_and_only_her_own_activity_chirho() {
    let copy_chirho = CreditsCopyChirho::english_chirho();
    let page_chirho = english_page_chirho(&ledger_chirho(), "account/alpha-chirho");
    let html_chirho = page_html_chirho(&page_chirho);
    assert_eq!(
        page_chirho.balance_chirho(),
        Some(copy_chirho.balance_chirho(1_250).as_str())
    );
    assert!(html_chirho.contains(&copy_chirho.balance_chirho(1_250)));
    assert_eq!(page_chirho.history_rows_chirho(), Some(3));
    let spent_at_chirho = html_chirho
        .find(&copy_chirho.entry_amount_chirho(false, 250))
        .unwrap();
    let granted_at_chirho = html_chirho
        .find(&copy_chirho.entry_amount_chirho(true, 500))
        .unwrap();
    let bought_at_chirho = html_chirho
        .find(&copy_chirho.entry_amount_chirho(true, 1_000))
        .unwrap();
    assert!(spent_at_chirho < granted_at_chirho && granted_at_chirho < bought_at_chirho);
    assert!(html_chirho.contains(&format!(
        "datetime=\"{}\"",
        utc_date_chirho(MOMENT_MS_CHIRHO)
    )));
    for foreign_chirho in [
        copy_chirho.balance_chirho(7_777),
        copy_chirho.entry_amount_chirho(false, 3_333),
    ] {
        assert!(!html_chirho.contains(&foreign_chirho), "{foreign_chirho}");
    }
    for (_, pack_chirho) in PackCatalogChirho::starting_chirho().packs_chirho() {
        assert!(html_chirho.contains(
            &copy_chirho.pack_chirho(pack_chirho.credits_chirho, pack_chirho.amount_minor_chirho)
        ));
    }
}

#[test]
fn an_unavailable_read_shows_a_notice_and_never_a_zero_chirho() {
    let copy_chirho = CreditsCopyChirho::english_chirho();
    let unconnected_chirho =
        english_page_chirho(&UnconnectedCreditViewChirho, "account/alpha-chirho");
    let unknown_payer_chirho = english_page_chirho(&ledger_chirho(), "account/unknown-chirho");
    for page_chirho in [&unconnected_chirho, &unknown_payer_chirho] {
        let html_chirho = page_html_chirho(page_chirho);
        assert_eq!(page_chirho.balance_chirho(), None);
        assert_eq!(page_chirho.history_rows_chirho(), None);
        let panel_chirho = balance_panel_chirho(&html_chirho);
        assert!(panel_chirho.contains(&shown_chirho(&copy_chirho.unavailable_heading_chirho)));
        let panel_text_chirho = visible_text_chirho(panel_chirho);
        assert!(
            !panel_text_chirho
                .chars()
                .any(|character_chirho| character_chirho.is_ascii_digit()),
            "{panel_text_chirho}"
        );
        assert!(html_chirho.contains(&shown_chirho(&copy_chirho.history_unavailable_chirho)));
        assert!(!html_chirho.contains(&shown_chirho(&copy_chirho.history_empty_chirho)));
    }
}

#[test]
fn a_known_zero_is_shown_as_zero_with_no_activity_chirho() {
    let copy_chirho = CreditsCopyChirho::english_chirho();
    let page_chirho = english_page_chirho(&ledger_chirho(), "account/empty-chirho");
    let html_chirho = page_html_chirho(&page_chirho);
    assert_eq!(
        page_chirho.balance_chirho(),
        Some(copy_chirho.balance_chirho(0).as_str())
    );
    assert_eq!(page_chirho.history_rows_chirho(), Some(0));
    assert!(
        balance_panel_chirho(&html_chirho).contains(&shown_chirho(&copy_chirho.balance_chirho(0)))
    );
    assert!(html_chirho.contains(&shown_chirho(&copy_chirho.history_empty_chirho)));
    assert!(!html_chirho.contains(&shown_chirho(&copy_chirho.unavailable_heading_chirho)));
}

#[test]
fn a_failed_read_shows_unavailable_and_the_next_read_recovers_chirho() {
    let ledger_failing_once_chirho = ledger_chirho();
    ledger_failing_once_chirho.fail_next_reads_chirho(1);
    let failed_chirho = english_page_chirho(&ledger_failing_once_chirho, "account/alpha-chirho");
    assert_eq!(failed_chirho.balance_chirho(), None);
    let recovered_chirho = english_page_chirho(&ledger_failing_once_chirho, "account/alpha-chirho");
    assert_eq!(
        recovered_chirho.balance_chirho(),
        Some(
            CreditsCopyChirho::english_chirho()
                .balance_chirho(1_250)
                .as_str()
        )
    );
}

#[test]
fn a_long_history_shows_her_latest_entries_and_says_so_chirho() {
    let total_chirho = CREDIT_HISTORY_MAX_ENTRIES_CHIRHO as u64 + 3;
    let busy_ledger_chirho = FixtureCreditViewChirho::default().with_payer_chirho(
        payer_chirho("account/busy-chirho"),
        CreditsChirho::new_chirho(42),
        (1..=total_chirho)
            .map(|step_chirho| {
                entry_chirho(
                    CreditEntryKindChirho::SpendChirho,
                    1,
                    MOMENT_MS_CHIRHO + step_chirho,
                )
            })
            .collect(),
    );
    let page_chirho = english_page_chirho(&busy_ledger_chirho, "account/busy-chirho");
    let html_chirho = page_html_chirho(&page_chirho);
    assert_eq!(
        page_chirho.history_rows_chirho(),
        Some(CREDIT_HISTORY_MAX_ENTRIES_CHIRHO)
    );
    assert!(html_chirho.contains(&shown_chirho(
        &CreditsCopyChirho::english_chirho().history_more_chirho(CREDIT_HISTORY_MAX_ENTRIES_CHIRHO)
    )));
}

#[test]
fn no_purchase_control_while_purchases_are_held_chirho() {
    let html_chirho = page_html_chirho(&english_page_chirho(
        &ledger_chirho(),
        "account/alpha-chirho",
    ));
    for control_chirho in ["<form", "<button", "<input", "type=\"submit\""] {
        assert!(!html_chirho.contains(control_chirho), "{control_chirho}");
    }
    assert!(html_chirho.contains(&shown_chirho(
        &CreditsCopyChirho::english_chirho().purchases_held_chirho
    )));
}

#[test]
fn the_pseudolocale_keeps_her_numbers_and_turns_the_page_rtl_chirho() {
    let page_chirho = CreditsPageV1Chirho::read_chirho(
        &ledger_chirho(),
        &payer_chirho("account/alpha-chirho"),
        &PackCatalogChirho::starting_chirho(),
        UiLocaleChirho::PseudolocaleRtlChirho,
    );
    let html_chirho = page_html_chirho(&page_chirho);
    assert!(html_chirho.contains("lang=\"ar-XB\" dir=\"rtl\""));
    assert!(page_chirho
        .balance_chirho()
        .is_some_and(|balance_chirho| balance_chirho.contains("1,250")));
}

#[test]
fn the_return_pages_point_to_her_credits_and_carry_no_balance_chirho() {
    let copy_chirho = CreditsCopyChirho::english_chirho();
    for (kind_chirho, heading_chirho) in [
        (
            CreditsReturnKindChirho::PaidChirho,
            &copy_chirho.paid_heading_chirho,
        ),
        (
            CreditsReturnKindChirho::CancelledChirho,
            &copy_chirho.cancelled_heading_chirho,
        ),
    ] {
        let html_chirho = render_credits_return_chirho(&CreditsReturnV1Chirho::for_kind_chirho(
            kind_chirho,
            UiLocaleChirho::EnglishChirho,
        ))
        .expect("a return page renders");
        support_chirho::assert_page_wiring_chirho("credits return", &html_chirho);
        assert!(html_chirho.contains(&shown_chirho(heading_chirho)));
        assert!(html_chirho.contains(&format!("href=\"{WORKSPACE_CREDITS_HREF_CHIRHO}\"")));
        assert!(!html_chirho.contains(&shown_chirho(&copy_chirho.balance_heading_chirho)));
        assert!(!html_chirho.contains("<form"));
    }
}

/// The hold (gpt #28745): no manifest route until Cairn's ledger port and L.J.'s word. Activation removes this test
/// on purpose.
#[test]
fn the_credits_pages_stay_unrouted_until_activation_chirho() {
    let manifest_chirho = include_str!("../../app-chirho.toml");
    for route_chirho in [
        "credits-chirho",
        "credits-paid-chirho",
        "credits-cancelled-chirho",
    ] {
        assert!(
            !manifest_chirho.contains(&format!("route_chirho = \"{route_chirho}\"")),
            "{route_chirho}"
        );
    }
}
