// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The credit view port's rules (PFE #28741): payer isolation, unavailable never zero, whole credits, a bounded
//! history of her own entries newest first, and recovery after a failed read.

use perffection_credits_chirho::credit_view_port_chirho::{
    CreditEntryChirho, CreditEntryKindChirho, CreditStatementChirho, CreditViewChirho,
    CreditViewPortChirho, CreditViewUnavailableChirho, CreditsChirho, FixtureCreditViewChirho,
    PayerRefChirho, UnconnectedCreditViewChirho, CREDIT_HISTORY_MAX_ENTRIES_CHIRHO,
    PAYER_REF_MAX_BYTES_CHIRHO,
};

fn payer_chirho(value_chirho: &str) -> PayerRefChirho {
    PayerRefChirho::new_chirho(value_chirho).expect("a well-formed fixture payer")
}

fn spend_chirho(credits_chirho: u64, at_ms_chirho: u64) -> CreditEntryChirho {
    CreditEntryChirho {
        kind_chirho: CreditEntryKindChirho::SpendChirho,
        credits_chirho: CreditsChirho::new_chirho(credits_chirho),
        at_ms_chirho,
        pack_id_chirho: None,
    }
}

fn purchase_chirho(credits_chirho: u64, at_ms_chirho: u64) -> CreditEntryChirho {
    CreditEntryChirho {
        kind_chirho: CreditEntryKindChirho::PurchaseChirho,
        credits_chirho: CreditsChirho::new_chirho(credits_chirho),
        at_ms_chirho,
        pack_id_chirho: Some(format!("credits-{credits_chirho}-chirho")),
    }
}

fn known_chirho(view_chirho: CreditViewChirho) -> CreditStatementChirho {
    match view_chirho {
        CreditViewChirho::KnownChirho(statement_chirho) => statement_chirho,
        other_chirho => panic!("expected a statement, got {other_chirho:?}"),
    }
}

#[test]
fn a_payer_sees_only_her_own_balance_and_entries_chirho() {
    let ledger_chirho = FixtureCreditViewChirho::default()
        .with_payer_chirho(
            payer_chirho("account/alpha-chirho"),
            CreditsChirho::new_chirho(950),
            vec![purchase_chirho(1_000, 10), spend_chirho(50, 20)],
        )
        .with_payer_chirho(
            payer_chirho("account/beta-chirho"),
            CreditsChirho::new_chirho(2_500),
            vec![purchase_chirho(2_500, 15)],
        );
    let alpha_chirho =
        known_chirho(ledger_chirho.read_credit_view_chirho(&payer_chirho("account/alpha-chirho")));
    let beta_chirho =
        known_chirho(ledger_chirho.read_credit_view_chirho(&payer_chirho("account/beta-chirho")));
    assert_eq!(alpha_chirho.available_chirho().whole_chirho(), 950);
    assert_eq!(
        alpha_chirho.recent_entries_chirho(),
        &[spend_chirho(50, 20), purchase_chirho(1_000, 10)]
    );
    assert_eq!(beta_chirho.available_chirho().whole_chirho(), 2_500);
    assert_eq!(
        beta_chirho.recent_entries_chirho(),
        &[purchase_chirho(2_500, 15)]
    );
}

#[test]
fn an_unknown_payer_is_unavailable_and_a_known_zero_is_zero_chirho() {
    let ledger_chirho = FixtureCreditViewChirho::default().with_payer_chirho(
        payer_chirho("account/empty-chirho"),
        CreditsChirho::new_chirho(0),
        Vec::new(),
    );
    assert_eq!(
        ledger_chirho.read_credit_view_chirho(&payer_chirho("account/unknown-chirho")),
        CreditViewChirho::UnavailableChirho(CreditViewUnavailableChirho::UnverifiableChirho)
    );
    let empty_chirho =
        known_chirho(ledger_chirho.read_credit_view_chirho(&payer_chirho("account/empty-chirho")));
    assert_eq!(empty_chirho.available_chirho().whole_chirho(), 0);
    assert!(empty_chirho.recent_entries_chirho().is_empty());
    assert!(!empty_chirho.more_entries_chirho());
}

#[test]
fn a_served_build_without_a_ledger_is_always_unavailable_chirho() {
    assert_eq!(
        UnconnectedCreditViewChirho.read_credit_view_chirho(&payer_chirho("account/alpha-chirho")),
        CreditViewChirho::UnavailableChirho(CreditViewUnavailableChirho::NotConnectedChirho)
    );
}

#[test]
fn history_is_bounded_newest_first_and_says_when_more_exist_chirho() {
    let total_chirho = CREDIT_HISTORY_MAX_ENTRIES_CHIRHO as u64 + 5;
    let entries_chirho = (1..=total_chirho)
        .map(|at_chirho| spend_chirho(1, at_chirho))
        .collect();
    let ledger_chirho = FixtureCreditViewChirho::default().with_payer_chirho(
        payer_chirho("account/busy-chirho"),
        CreditsChirho::new_chirho(7),
        entries_chirho,
    );
    let statement_chirho =
        known_chirho(ledger_chirho.read_credit_view_chirho(&payer_chirho("account/busy-chirho")));
    let times_chirho: Vec<u64> = statement_chirho
        .recent_entries_chirho()
        .iter()
        .map(|entry_chirho| entry_chirho.at_ms_chirho)
        .collect();
    assert_eq!(times_chirho.len(), CREDIT_HISTORY_MAX_ENTRIES_CHIRHO);
    assert_eq!(times_chirho.first(), Some(&total_chirho));
    assert!(times_chirho
        .windows(2)
        .all(|pair_chirho| pair_chirho[0] > pair_chirho[1]));
    assert!(statement_chirho.more_entries_chirho());
}

#[test]
fn a_failed_read_is_unavailable_and_the_next_read_recovers_chirho() {
    let ledger_chirho = FixtureCreditViewChirho::default().with_payer_chirho(
        payer_chirho("account/alpha-chirho"),
        CreditsChirho::new_chirho(950),
        vec![purchase_chirho(1_000, 10)],
    );
    ledger_chirho.fail_next_reads_chirho(1);
    let payer_alpha_chirho = payer_chirho("account/alpha-chirho");
    assert_eq!(
        ledger_chirho.read_credit_view_chirho(&payer_alpha_chirho),
        CreditViewChirho::UnavailableChirho(CreditViewUnavailableChirho::ReadFailedChirho)
    );
    let recovered_chirho = known_chirho(ledger_chirho.read_credit_view_chirho(&payer_alpha_chirho));
    assert_eq!(recovered_chirho.available_chirho().whole_chirho(), 950);
}

#[test]
fn a_statement_that_breaks_a_rule_is_refused_not_repaired_chirho() {
    let available_chirho = CreditsChirho::new_chirho(10);
    let too_many_chirho = (0..=CREDIT_HISTORY_MAX_ENTRIES_CHIRHO as u64)
        .rev()
        .map(|at_chirho| spend_chirho(1, at_chirho))
        .collect();
    assert!(CreditStatementChirho::new_chirho(available_chirho, too_many_chirho, true).is_none());
    assert!(CreditStatementChirho::new_chirho(
        available_chirho,
        vec![spend_chirho(1, 10), spend_chirho(1, 20)],
        false
    )
    .is_none());
    assert!(
        CreditStatementChirho::new_chirho(available_chirho, vec![spend_chirho(0, 10)], false)
            .is_none()
    );
    let unnamed_purchase_chirho = CreditEntryChirho {
        pack_id_chirho: None,
        ..purchase_chirho(1_000, 10)
    };
    assert!(CreditStatementChirho::new_chirho(
        available_chirho,
        vec![unnamed_purchase_chirho],
        false
    )
    .is_none());
    let spend_with_pack_chirho = CreditEntryChirho {
        pack_id_chirho: Some("credits-1000-chirho".to_string()),
        ..spend_chirho(5, 10)
    };
    assert!(CreditStatementChirho::new_chirho(
        available_chirho,
        vec![spend_with_pack_chirho],
        false
    )
    .is_none());
    assert!(CreditStatementChirho::new_chirho(
        available_chirho,
        vec![spend_chirho(1, 20), purchase_chirho(1_000, 10)],
        false
    )
    .is_some());
}

#[test]
fn a_payer_reference_is_bounded_visible_ascii_and_never_printed_chirho() {
    assert!(PayerRefChirho::new_chirho("").is_none());
    assert!(PayerRefChirho::new_chirho("account/has space-chirho").is_none());
    assert!(PayerRefChirho::new_chirho("account/é-chirho").is_none());
    assert!(PayerRefChirho::new_chirho(&"a".repeat(PAYER_REF_MAX_BYTES_CHIRHO + 1)).is_none());
    let longest_chirho = "a".repeat(PAYER_REF_MAX_BYTES_CHIRHO);
    assert_eq!(
        PayerRefChirho::new_chirho(&longest_chirho)
            .map(|payer_chirho| payer_chirho.as_str_chirho().len()),
        Some(PAYER_REF_MAX_BYTES_CHIRHO)
    );
    let secret_chirho = payer_chirho("account/private-marker-chirho");
    assert!(!format!("{secret_chirho:?}").contains("private-marker"));
}
