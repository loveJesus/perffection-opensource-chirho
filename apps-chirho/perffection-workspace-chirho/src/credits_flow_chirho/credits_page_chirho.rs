// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Credits page: what she can draft with, the credit packs, and her recent credit activity.
//!
//! Workflow: `spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md`.
//!
//! It reads only through `credit_view_port_chirho`, for the payer the engine resolves from her session, and shows
//! only what that port reports. It never writes a balance. An unavailable read shows a plain notice and no number:
//! unavailable is never zero. While purchases are held it renders no purchase control at all, only the packs and a
//! note.

use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
    WORKSPACE_HOME_HREF_CHIRHO,
};
use perffection_credits_chirho::credit_view_port_chirho::{
    CreditEntryChirho, CreditEntryKindChirho, CreditViewChirho, CreditViewPortChirho,
    PayerRefChirho,
};
use perffection_credits_chirho::pack_catalog_chirho::PackCatalogChirho;
use perffection_ui_chirho::{
    CreditsCopyChirho, SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho,
};

/// The page's address once it is routed (held).
pub const WORKSPACE_CREDITS_HREF_CHIRHO: &str = "/app-chirho/perffection-workspace/credits-chirho";

const MS_PER_DAY_CHIRHO: u64 = 86_400_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditsEntryViewChirho {
    pub(crate) date_chirho: String,
    pub(crate) kind_chirho: String,
    pub(crate) amount_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditsPageV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: CreditsCopyChirho,
    /// `None` when the ledger's answer is unavailable: the page says so, and shows no number.
    pub(crate) balance_chirho: Option<String>,
    pub(crate) packs_chirho: Vec<String>,
    /// `None` when unavailable; empty when the ledger answered with no entries.
    pub(crate) history_chirho: Option<Vec<CreditsEntryViewChirho>>,
    pub(crate) history_more_chirho: Option<String>,
    pub(crate) home_href_chirho: &'static str,
}

impl CreditsPageV1Chirho {
    /// One read through the port, for the payer the engine resolved from her session, then the page.
    pub fn read_chirho(
        port_chirho: &impl CreditViewPortChirho,
        payer_chirho: &PayerRefChirho,
        catalog_chirho: &PackCatalogChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        Self::from_view_chirho(
            &port_chirho.read_credit_view_chirho(payer_chirho),
            catalog_chirho,
            locale_chirho,
        )
    }

    pub fn from_view_chirho(
        view_chirho: &CreditViewChirho,
        catalog_chirho: &PackCatalogChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = CreditsCopyChirho::for_locale_chirho(locale_chirho);
        let chrome_chirho = workspace_chrome_chirho(
            &WorkspaceCopyChirho::for_locale_chirho(locale_chirho),
            &copy_chirho.page_title_chirho,
            &copy_chirho.page_description_chirho,
            WorkspaceNavPageChirho::CreditsChirho,
            WorkspaceNavRoadsChirho::deployed_chirho(),
        );
        let packs_chirho = catalog_chirho
            .packs_chirho()
            .map(|(_, pack_chirho)| {
                copy_chirho.pack_chirho(pack_chirho.credits_chirho, pack_chirho.amount_minor_chirho)
            })
            .collect();
        let (balance_chirho, history_chirho, history_more_chirho) = match view_chirho {
            CreditViewChirho::KnownChirho(statement_chirho) => {
                let entries_chirho = statement_chirho.recent_entries_chirho();
                (
                    Some(
                        copy_chirho
                            .balance_chirho(statement_chirho.available_chirho().whole_chirho()),
                    ),
                    Some(
                        entries_chirho
                            .iter()
                            .map(|entry_chirho| entry_view_chirho(&copy_chirho, entry_chirho))
                            .collect(),
                    ),
                    statement_chirho
                        .more_entries_chirho()
                        .then(|| copy_chirho.history_more_chirho(entries_chirho.len())),
                )
            }
            CreditViewChirho::UnavailableChirho(_) => (None, None, None),
        };
        Self {
            chrome_chirho,
            copy_chirho,
            balance_chirho,
            packs_chirho,
            history_chirho,
            history_more_chirho,
            home_href_chirho: WORKSPACE_HOME_HREF_CHIRHO,
        }
    }

    /// Her balance as the page shows it, or `None` when the read was unavailable.
    pub fn balance_chirho(&self) -> Option<&str> {
        self.balance_chirho.as_deref()
    }

    /// How many history rows the page shows, or `None` when the read was unavailable.
    pub fn history_rows_chirho(&self) -> Option<usize> {
        self.history_chirho.as_ref().map(Vec::len)
    }
}

fn entry_view_chirho(
    copy_chirho: &CreditsCopyChirho,
    entry_chirho: &CreditEntryChirho,
) -> CreditsEntryViewChirho {
    let (kind_chirho, adds_chirho) = match entry_chirho.kind_chirho {
        CreditEntryKindChirho::PurchaseChirho => (&copy_chirho.entry_purchase_chirho, true),
        CreditEntryKindChirho::GrantChirho => (&copy_chirho.entry_grant_chirho, true),
        CreditEntryKindChirho::SpendChirho => (&copy_chirho.entry_spend_chirho, false),
        CreditEntryKindChirho::ReturnChirho => (&copy_chirho.entry_return_chirho, true),
    };
    CreditsEntryViewChirho {
        date_chirho: utc_date_chirho(entry_chirho.at_ms_chirho),
        kind_chirho: kind_chirho.clone(),
        amount_chirho: copy_chirho
            .entry_amount_chirho(adds_chirho, entry_chirho.credits_chirho.whole_chirho()),
    }
}

/// The UTC calendar date of a time in milliseconds since the Unix epoch, as YYYY-MM-DD, in whole-number arithmetic
/// (Howard Hinnant's `civil_from_days`).
pub fn utc_date_chirho(at_ms_chirho: u64) -> String {
    let days_chirho = (at_ms_chirho / MS_PER_DAY_CHIRHO) as i64 + 719_468;
    let era_chirho = days_chirho.div_euclid(146_097);
    let day_of_era_chirho = days_chirho.rem_euclid(146_097);
    let year_of_era_chirho = (day_of_era_chirho - day_of_era_chirho / 1_460
        + day_of_era_chirho / 36_524
        - day_of_era_chirho / 146_096)
        / 365;
    let day_of_year_chirho = day_of_era_chirho
        - (365 * year_of_era_chirho + year_of_era_chirho / 4 - year_of_era_chirho / 100);
    let month_index_chirho = (5 * day_of_year_chirho + 2) / 153;
    let day_chirho = day_of_year_chirho - (153 * month_index_chirho + 2) / 5 + 1;
    let month_chirho = if month_index_chirho < 10 {
        month_index_chirho + 3
    } else {
        month_index_chirho - 9
    };
    let year_chirho = year_of_era_chirho + era_chirho * 400 + i64::from(month_chirho <= 2);
    format!("{year_chirho:04}-{month_chirho:02}-{day_chirho:02}")
}
