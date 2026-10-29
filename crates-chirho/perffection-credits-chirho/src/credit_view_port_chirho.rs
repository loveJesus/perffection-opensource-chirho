// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The credit view port: what the signed-in owner can draft with, and her own recent credit entries. The Credits page
//! and the first-draft work both read through this one port; neither keeps a balance of its own (gpt #28745).
//!
//! The rules it keeps (PFE #28741, accepted #28742):
//! - **Payer-neutral.** It answers for an opaque payer reference that the engine resolves from her session. Whether
//!   that payer is an account pool, a project wallet or a donor is the engine's to decide (gpt #24220), so nothing
//!   here names one.
//! - **Unavailable is never zero.** A read that fails, or an answer that cannot be trusted, is
//!   [`CreditViewChirho::UnavailableChirho`]; only a ledger that answered can say 0.
//! - **Whole credits, bounded history.** No fractions, and at most [`CREDIT_HISTORY_MAX_ENTRIES_CHIRHO`] of her own
//!   entries, newest first.
//! - **The fixture is test-only.** [`FixtureCreditViewChirho`] exists only with the non-default `test-stub-chirho`
//!   feature. That is an explicit opt-in: default builds exclude it, while a build that turns it on, `--all-features`
//!   among them, includes it. Served wiring must use [`UnconnectedCreditViewChirho`], which answers unavailable,
//!   until Cairn's ledger port exists (W3, on hold).
//! - **Who pays, and whose entries these are, stay the engine's.** The engine resolves the payer from her session and
//!   authorizes the read upstream; this port answers for the reference it is handed and owns neither duty.
//!
//! Workflow: spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md

use std::fmt;

/// At most this many of her entries come back, newest first; the page says when older ones exist.
pub const CREDIT_HISTORY_MAX_ENTRIES_CHIRHO: usize = 20;
/// An opaque payer reference's largest size.
pub const PAYER_REF_MAX_BYTES_CHIRHO: usize = 128;

/// Who pays for this owner's drafts, as the engine names it. The product never parses, shows or derives it; it only
/// hands it back to the port. It comes from the engine's view of her session, never from a request.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PayerRefChirho(String);

impl PayerRefChirho {
    /// 1 to [`PAYER_REF_MAX_BYTES_CHIRHO`] visible ASCII characters; anything else is no payer reference.
    pub fn new_chirho(value_chirho: &str) -> Option<Self> {
        let well_formed_chirho = !value_chirho.is_empty()
            && value_chirho.len() <= PAYER_REF_MAX_BYTES_CHIRHO
            && value_chirho
                .bytes()
                .all(|byte_chirho| byte_chirho.is_ascii_graphic());
        well_formed_chirho.then(|| Self(value_chirho.to_string()))
    }

    /// For the port's own lookup, and as the checkout's client reference so a paid event names its payer; never shown.
    pub fn as_str_chirho(&self) -> &str {
        &self.0
    }
}

/// Never printed: a payer reference is not display data.
impl fmt::Debug for PayerRefChirho {
    fn fmt(&self, f_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        f_chirho.write_str("PayerRefChirho(..)")
    }
}

/// Whole credits. One credit is one US cent (`pack_catalog_chirho`), and there are no fractions of one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreditsChirho(u64);

impl CreditsChirho {
    pub const fn new_chirho(whole_chirho: u64) -> Self {
        Self(whole_chirho)
    }

    pub const fn whole_chirho(self) -> u64 {
        self.0
    }
}

/// Which way an entry moved her credits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditEntryKindChirho {
    /// A paid pack: the ledger posts it only from a verified Stripe event.
    PurchaseChirho,
    /// Credits an admin granted.
    GrantChirho,
    /// Credits a paid effect spent, such as an AI draft.
    SpendChirho,
    /// Credits given back, such as a refund or a reversed spend.
    ReturnChirho,
}

/// One of her credit entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditEntryChirho {
    pub kind_chirho: CreditEntryKindChirho,
    /// How many credits moved, always at least one; the kind says which way.
    pub credits_chirho: CreditsChirho,
    /// When the ledger recorded it, in milliseconds since the Unix epoch.
    pub at_ms_chirho: u64,
    /// The pack bought, for a purchase and only a purchase.
    pub pack_id_chirho: Option<String>,
}

/// What the ledger answered for one payer, checked on the way in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditStatementChirho {
    available_chirho: CreditsChirho,
    recent_entries_chirho: Vec<CreditEntryChirho>,
    more_entries_chirho: bool,
}

impl CreditStatementChirho {
    /// A statement only when it keeps the port's rules: at most [`CREDIT_HISTORY_MAX_ENTRIES_CHIRHO`] entries, newest
    /// first, each moving at least one credit, and a pack named on purchases and nowhere else. Any other answer is
    /// `None`, and the port reports it as unverifiable rather than guessing.
    pub fn new_chirho(
        available_chirho: CreditsChirho,
        recent_entries_chirho: Vec<CreditEntryChirho>,
        more_entries_chirho: bool,
    ) -> Option<Self> {
        let within_bound_chirho = recent_entries_chirho.len() <= CREDIT_HISTORY_MAX_ENTRIES_CHIRHO;
        let newest_first_chirho = recent_entries_chirho
            .windows(2)
            .all(|pair_chirho| pair_chirho[0].at_ms_chirho >= pair_chirho[1].at_ms_chirho);
        let entries_whole_chirho = recent_entries_chirho.iter().all(|entry_chirho| {
            entry_chirho.credits_chirho.whole_chirho() >= 1
                && (entry_chirho.kind_chirho == CreditEntryKindChirho::PurchaseChirho)
                    == entry_chirho.pack_id_chirho.is_some()
        });
        (within_bound_chirho && newest_first_chirho && entries_whole_chirho).then_some(Self {
            available_chirho,
            recent_entries_chirho,
            more_entries_chirho,
        })
    }

    /// What she can draft with now.
    pub fn available_chirho(&self) -> CreditsChirho {
        self.available_chirho
    }

    /// Her own entries, newest first.
    pub fn recent_entries_chirho(&self) -> &[CreditEntryChirho] {
        &self.recent_entries_chirho
    }

    /// True when older entries exist beyond the bound.
    pub fn more_entries_chirho(&self) -> bool {
        self.more_entries_chirho
    }
}

/// Why the credits cannot be shown. None of these is a balance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditViewUnavailableChirho {
    /// No ledger port is connected on this build.
    NotConnectedChirho,
    /// The ledger did not answer, or answered with an error.
    ReadFailedChirho,
    /// An answer came but could not be trusted: it broke a rule above, or named no payer the ledger knows.
    UnverifiableChirho,
}

/// One read's result: a checked statement, or a typed reason it is unavailable. There is no third state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CreditViewChirho {
    KnownChirho(CreditStatementChirho),
    UnavailableChirho(CreditViewUnavailableChirho),
}

pub trait CreditViewPortChirho {
    /// One read for one payer. A failed or untrusted read answers `UnavailableChirho`, never a statement of 0.
    fn read_credit_view_chirho(&self, payer_chirho: &PayerRefChirho) -> CreditViewChirho;
}

/// The only implementation a served build has until Cairn's ledger port exists: every read is unavailable.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnconnectedCreditViewChirho;

impl CreditViewPortChirho for UnconnectedCreditViewChirho {
    fn read_credit_view_chirho(&self, _payer_chirho: &PayerRefChirho) -> CreditViewChirho {
        CreditViewChirho::UnavailableChirho(CreditViewUnavailableChirho::NotConnectedChirho)
    }
}

#[cfg(feature = "test-stub-chirho")]
pub use fixture_chirho::FixtureCreditViewChirho;

#[cfg(feature = "test-stub-chirho")]
mod fixture_chirho {
    use super::{
        CreditEntryChirho, CreditStatementChirho, CreditViewChirho, CreditViewPortChirho,
        CreditViewUnavailableChirho, CreditsChirho, PayerRefChirho,
        CREDIT_HISTORY_MAX_ENTRIES_CHIRHO,
    };
    use std::cell::Cell;
    use std::collections::BTreeMap;

    /// TESTS ONLY: a deterministic, in-memory ledger keyed by payer. It exists only with the opt-in `test-stub-chirho`
    /// feature, which default builds exclude; served wiring uses `UnconnectedCreditViewChirho` instead.
    #[derive(Debug, Default)]
    pub struct FixtureCreditViewChirho {
        payers_chirho: BTreeMap<PayerRefChirho, (CreditsChirho, Vec<CreditEntryChirho>)>,
        failing_reads_chirho: Cell<u32>,
    }

    impl FixtureCreditViewChirho {
        /// One payer's balance and full history, oldest first.
        pub fn with_payer_chirho(
            mut self,
            payer_chirho: PayerRefChirho,
            available_chirho: CreditsChirho,
            entries_oldest_first_chirho: Vec<CreditEntryChirho>,
        ) -> Self {
            self.payers_chirho.insert(
                payer_chirho,
                (available_chirho, entries_oldest_first_chirho),
            );
            self
        }

        /// The next `count_chirho` reads fail as the ledger not answering, then reads recover.
        pub fn fail_next_reads_chirho(&self, count_chirho: u32) {
            self.failing_reads_chirho.set(count_chirho);
        }
    }

    impl CreditViewPortChirho for FixtureCreditViewChirho {
        fn read_credit_view_chirho(&self, payer_chirho: &PayerRefChirho) -> CreditViewChirho {
            let failing_chirho = self.failing_reads_chirho.get();
            if failing_chirho > 0 {
                self.failing_reads_chirho.set(failing_chirho - 1);
                return CreditViewChirho::UnavailableChirho(
                    CreditViewUnavailableChirho::ReadFailedChirho,
                );
            }
            // A payer this ledger does not know has no statement; it is never shown as 0.
            let Some((available_chirho, entries_chirho)) = self.payers_chirho.get(payer_chirho)
            else {
                return CreditViewChirho::UnavailableChirho(
                    CreditViewUnavailableChirho::UnverifiableChirho,
                );
            };
            let recent_chirho: Vec<CreditEntryChirho> = entries_chirho
                .iter()
                .rev()
                .take(CREDIT_HISTORY_MAX_ENTRIES_CHIRHO)
                .cloned()
                .collect();
            let more_chirho = entries_chirho.len() > CREDIT_HISTORY_MAX_ENTRIES_CHIRHO;
            match CreditStatementChirho::new_chirho(*available_chirho, recent_chirho, more_chirho) {
                Some(statement_chirho) => CreditViewChirho::KnownChirho(statement_chirho),
                None => CreditViewChirho::UnavailableChirho(
                    CreditViewUnavailableChirho::UnverifiableChirho,
                ),
            }
        }
    }
}
