// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures as she saves them: monthly totals by kind of work, in one currency.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. The document is
//! columnar (three kinds-by-months integer matrices) so a maximal one fits the viewer-state door's
//! 16 KiB value; one object per row would not. Money is integer minor units (cents). A cell with no
//! figure is `None`, never zero: an unknown stays unknown all the way to the report. The validator
//! measures the exact stored bytes against the door's bound and refuses past it by name; it never
//! truncates. No customer names or other people's details are part of the shape.

use serde::{Deserialize, Serialize};

pub const FIGURES_SCHEMA_CHIRHO: &str = "perffection-figures-v1-chirho";
pub const FIGURES_MAX_MONTHS_CHIRHO: usize = 24;
pub const FIGURES_MAX_KINDS_CHIRHO: usize = 12;
pub const FIGURES_MAX_KIND_CHARS_CHIRHO: usize = 60;
pub const FIGURES_MAX_NOTE_CHARS_CHIRHO: usize = 120;
/// 999,999,999.99 in minor units: eleven digits, as the measured maximal document assumed.
pub const FIGURES_MAX_AMOUNT_MINOR_CHIRHO: i64 = 99_999_999_999;
pub const FIGURES_MAX_JOBS_CHIRHO: u32 = 999_999;
/// Cairn's viewer-state value bound (`VIEWER_STATE_MAX_VALUE_BYTES_CHIRHO`, 16 KiB).
pub const FIGURES_MAX_STORED_BYTES_CHIRHO: usize = 16 * 1024;
/// The currencies her figures support: ISO 4217 codes whose minor unit is two decimal places and
/// whose cents are in everyday use (gpt #26569: a closed, verified list, not "three capitals minus a
/// blocklist"). Any other code is refused by name; adding one is a reviewed change.
pub const FIGURES_CURRENCIES_CHIRHO: [&str; 30] = [
    "AED", "AUD", "BBD", "BRL", "BSD", "CAD", "CHF", "CNY", "CZK", "DKK", "EUR", "GBP", "GHS",
    "HKD", "ILS", "INR", "JMD", "KES", "MXN", "MYR", "NGN", "NOK", "NZD", "PEN", "PHP", "PLN",
    "SEK", "SGD", "USD", "ZAR",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FiguresDocumentV1Chirho {
    pub schema_chirho: String,
    /// ISO 4217 code, three capital letters, a two-decimal currency.
    pub currency_chirho: String,
    /// Her own source note ("from my bookkeeping export"); may be empty. Text, never authority.
    pub note_chirho: String,
    /// `YYYY-MM`, strictly ascending.
    pub months_chirho: Vec<String>,
    /// Kinds of work in her order; unique ignoring case.
    pub kinds_chirho: Vec<String>,
    /// `[kind][month]` money in, minor units; `None` where she gave no figure.
    pub money_in_chirho: Vec<Vec<Option<i64>>>,
    /// `[kind][month]` direct costs, minor units; `None` where unknown.
    pub costs_chirho: Vec<Vec<Option<i64>>>,
    /// `[kind][month]` jobs done; `None` where unknown.
    pub jobs_chirho: Vec<Vec<Option<u32>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiguresValidationErrorChirho {
    UnsupportedSchemaChirho,
    InvalidCurrencyChirho,
    UnsupportedCurrencyChirho,
    NoteTooLongChirho,
    NoMonthsChirho,
    TooManyMonthsChirho,
    InvalidMonthChirho,
    MonthsOutOfOrderChirho,
    NoKindsChirho,
    TooManyKindsChirho,
    InvalidKindChirho,
    DuplicateKindChirho,
    WrongShapeChirho,
    AmountOutOfRangeChirho,
    JobsOutOfRangeChirho,
    NoMoneyInChirho,
    /// A kind with no money-in figure in any month: its totals could only be invented.
    KindWithoutFiguresChirho,
    /// A month with no money-in figure for any kind.
    MonthWithoutFiguresChirho,
    /// Costs or jobs where she gave no money in: a cell the report could not place honestly.
    CostsWithoutMoneyChirho,
    TooLargeToStoreChirho,
}

impl FiguresValidationErrorChirho {
    /// The stable code a refusal names; the page and the agent answer both carry it.
    pub fn code_chirho(self) -> &'static str {
        match self {
            Self::UnsupportedSchemaChirho => "unsupported-schema-chirho",
            Self::InvalidCurrencyChirho => "invalid-currency-chirho",
            Self::UnsupportedCurrencyChirho => "unsupported-currency-chirho",
            Self::NoteTooLongChirho => "note-too-long-chirho",
            Self::NoMonthsChirho => "no-months-chirho",
            Self::TooManyMonthsChirho => "too-many-months-chirho",
            Self::InvalidMonthChirho => "invalid-month-chirho",
            Self::MonthsOutOfOrderChirho => "months-out-of-order-chirho",
            Self::NoKindsChirho => "no-kinds-chirho",
            Self::TooManyKindsChirho => "too-many-kinds-chirho",
            Self::InvalidKindChirho => "invalid-kind-chirho",
            Self::DuplicateKindChirho => "duplicate-kind-chirho",
            Self::WrongShapeChirho => "wrong-shape-chirho",
            Self::AmountOutOfRangeChirho => "amount-out-of-range-chirho",
            Self::JobsOutOfRangeChirho => "jobs-out-of-range-chirho",
            Self::NoMoneyInChirho => "no-money-in-chirho",
            Self::KindWithoutFiguresChirho => "kind-without-figures-chirho",
            Self::MonthWithoutFiguresChirho => "month-without-figures-chirho",
            Self::CostsWithoutMoneyChirho => "costs-without-money-chirho",
            Self::TooLargeToStoreChirho => "too-large-to-store-chirho",
        }
    }
}

/// A document that passed every check, with the exact bytes the door will store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedFiguresChirho {
    document_chirho: FiguresDocumentV1Chirho,
    stored_bytes_chirho: Vec<u8>,
}

impl ValidatedFiguresChirho {
    pub fn document_chirho(&self) -> &FiguresDocumentV1Chirho {
        &self.document_chirho
    }

    pub fn stored_bytes_chirho(&self) -> &[u8] {
        &self.stored_bytes_chirho
    }
}

impl FiguresDocumentV1Chirho {
    pub fn validate_chirho(self) -> Result<ValidatedFiguresChirho, FiguresValidationErrorChirho> {
        use FiguresValidationErrorChirho as E;
        if self.schema_chirho != FIGURES_SCHEMA_CHIRHO {
            return Err(E::UnsupportedSchemaChirho);
        }
        check_currency_chirho(&self.currency_chirho)?;
        if self.note_chirho.chars().count() > FIGURES_MAX_NOTE_CHARS_CHIRHO
            || has_control_chirho(&self.note_chirho)
        {
            return Err(E::NoteTooLongChirho);
        }
        if self.months_chirho.is_empty() {
            return Err(E::NoMonthsChirho);
        }
        if self.months_chirho.len() > FIGURES_MAX_MONTHS_CHIRHO {
            return Err(E::TooManyMonthsChirho);
        }
        let mut previous_chirho: Option<(u32, u32)> = None;
        for month_chirho in &self.months_chirho {
            let parsed_chirho = parse_month_chirho(month_chirho).ok_or(E::InvalidMonthChirho)?;
            if previous_chirho.is_some_and(|earlier_chirho| earlier_chirho >= parsed_chirho) {
                return Err(E::MonthsOutOfOrderChirho);
            }
            previous_chirho = Some(parsed_chirho);
        }
        if self.kinds_chirho.is_empty() {
            return Err(E::NoKindsChirho);
        }
        if self.kinds_chirho.len() > FIGURES_MAX_KINDS_CHIRHO {
            return Err(E::TooManyKindsChirho);
        }
        let mut seen_chirho: Vec<String> = Vec::new();
        for kind_chirho in &self.kinds_chirho {
            if !valid_kind_chirho(kind_chirho) {
                return Err(E::InvalidKindChirho);
            }
            let folded_chirho = kind_chirho.to_lowercase();
            if seen_chirho.contains(&folded_chirho) {
                return Err(E::DuplicateKindChirho);
            }
            seen_chirho.push(folded_chirho);
        }
        let (kinds_chirho, months_chirho) = (self.kinds_chirho.len(), self.months_chirho.len());
        if !shaped_chirho(&self.money_in_chirho, kinds_chirho, months_chirho)
            || !shaped_chirho(&self.costs_chirho, kinds_chirho, months_chirho)
            || !shaped_chirho(&self.jobs_chirho, kinds_chirho, months_chirho)
        {
            return Err(E::WrongShapeChirho);
        }
        let amount_ok_chirho = |cell_chirho: &Option<i64>| {
            cell_chirho.is_none_or(|minor_chirho| {
                (0..=FIGURES_MAX_AMOUNT_MINOR_CHIRHO).contains(&minor_chirho)
            })
        };
        if !self
            .money_in_chirho
            .iter()
            .chain(&self.costs_chirho)
            .flatten()
            .all(amount_ok_chirho)
        {
            return Err(E::AmountOutOfRangeChirho);
        }
        if !self.jobs_chirho.iter().flatten().all(|cell_chirho| {
            cell_chirho.is_none_or(|jobs_chirho| jobs_chirho <= FIGURES_MAX_JOBS_CHIRHO)
        }) {
            return Err(E::JobsOutOfRangeChirho);
        }
        if self.money_in_chirho.iter().flatten().all(Option::is_none) {
            return Err(E::NoMoneyInChirho);
        }
        if self
            .money_in_chirho
            .iter()
            .any(|row_chirho| row_chirho.iter().all(Option::is_none))
        {
            return Err(E::KindWithoutFiguresChirho);
        }
        if (0..months_chirho).any(|month_chirho| {
            self.money_in_chirho
                .iter()
                .all(|row_chirho| row_chirho[month_chirho].is_none())
        }) {
            return Err(E::MonthWithoutFiguresChirho);
        }
        let stray_chirho = (0..kinds_chirho).any(|kind_chirho| {
            (0..months_chirho).any(|month_chirho| {
                self.money_in_chirho[kind_chirho][month_chirho].is_none()
                    && (self.costs_chirho[kind_chirho][month_chirho].is_some()
                        || self.jobs_chirho[kind_chirho][month_chirho].is_some())
            })
        });
        if stray_chirho {
            return Err(E::CostsWithoutMoneyChirho);
        }
        let stored_bytes_chirho =
            serde_json::to_vec(&self).map_err(|_| E::TooLargeToStoreChirho)?;
        within_stored_bound_chirho(&stored_bytes_chirho, FIGURES_MAX_STORED_BYTES_CHIRHO)?;
        Ok(ValidatedFiguresChirho {
            document_chirho: self,
            stored_bytes_chirho,
        })
    }
}

fn check_currency_chirho(currency_chirho: &str) -> Result<(), FiguresValidationErrorChirho> {
    if currency_chirho.len() != 3
        || !currency_chirho
            .bytes()
            .all(|byte_chirho| byte_chirho.is_ascii_uppercase())
    {
        return Err(FiguresValidationErrorChirho::InvalidCurrencyChirho);
    }
    if !FIGURES_CURRENCIES_CHIRHO.contains(&currency_chirho) {
        return Err(FiguresValidationErrorChirho::UnsupportedCurrencyChirho);
    }
    Ok(())
}

/// The door's value bound on the exact serialized bytes. Within today's other bounds no document
/// reaches it (the measured maximum is in tests/figures_chirho/figures_core_chirho.rs), so it is the guard that still
/// holds if those bounds ever grow: exactly the limit is stored, one byte past it is refused.
pub fn within_stored_bound_chirho(
    stored_bytes_chirho: &[u8],
    limit_chirho: usize,
) -> Result<(), FiguresValidationErrorChirho> {
    if stored_bytes_chirho.len() > limit_chirho {
        return Err(FiguresValidationErrorChirho::TooLargeToStoreChirho);
    }
    Ok(())
}

/// One row per kind, one cell per month.
fn shaped_chirho<T>(matrix_chirho: &[Vec<T>], kinds_chirho: usize, months_chirho: usize) -> bool {
    matrix_chirho.len() == kinds_chirho
        && matrix_chirho
            .iter()
            .all(|row_chirho| row_chirho.len() == months_chirho)
}

fn has_control_chirho(text_chirho: &str) -> bool {
    text_chirho.chars().any(char::is_control)
}

/// A kind label: 1 to 60 characters, no control characters, no leading or trailing space.
pub fn valid_kind_chirho(kind_chirho: &str) -> bool {
    let count_chirho = kind_chirho.chars().count();
    (1..=FIGURES_MAX_KIND_CHARS_CHIRHO).contains(&count_chirho)
        && kind_chirho.trim() == kind_chirho
        && !has_control_chirho(kind_chirho)
}

/// `YYYY-MM` with a year from 1900 to 2999 and a month from 01 to 12, as `(year, month)`.
pub fn parse_month_chirho(month_chirho: &str) -> Option<(u32, u32)> {
    let bytes_chirho = month_chirho.as_bytes();
    if bytes_chirho.len() != 7 || bytes_chirho[4] != b'-' {
        return None;
    }
    let digits_chirho = |range_chirho: std::ops::Range<usize>| {
        let part_chirho = &month_chirho[range_chirho];
        part_chirho
            .bytes()
            .all(|byte_chirho| byte_chirho.is_ascii_digit())
            .then(|| part_chirho.parse::<u32>().ok())
            .flatten()
    };
    let (year_chirho, number_chirho) = (digits_chirho(0..4)?, digits_chirho(5..7)?);
    ((1900..=2999).contains(&year_chirho) && (1..=12).contains(&number_chirho))
        .then_some((year_chirho, number_chirho))
}
