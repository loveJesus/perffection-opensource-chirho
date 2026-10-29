// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her pasted CSV, read strictly into figures she reviews before she saves.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. The header is
//! exactly `month,kind,money_in,costs,jobs`. Amounts are plain decimals with at most two places (no
//! symbols, no thousands separators, no minus); `costs` and `jobs` may be empty, meaning unknown. A
//! row that cannot be read is refused with its line and reason and kept out. Two rows for the same
//! month and kind are combined, and the review says so: money in is summed, and costs or jobs stay
//! known only if every combined row knew them. Past a bound the whole import is refused by name;
//! nothing is ever truncated. The parser never sees or keeps anyone's personal details.

use crate::figures_flow_chirho::figures_document_chirho::{
    parse_month_chirho, valid_kind_chirho, FiguresDocumentV1Chirho, FiguresValidationErrorChirho,
    ValidatedFiguresChirho, FIGURES_MAX_AMOUNT_MINOR_CHIRHO, FIGURES_MAX_JOBS_CHIRHO,
    FIGURES_MAX_KINDS_CHIRHO, FIGURES_MAX_MONTHS_CHIRHO, FIGURES_SCHEMA_CHIRHO,
};

pub const FIGURES_CSV_HEADER_CHIRHO: &str = "month,kind,money_in,costs,jobs";
/// A month-by-kind export of the largest saveable figures is a few KB; 64 KiB leaves room for
/// combined duplicate rows without letting a paste grow without bound.
pub const FIGURES_CSV_MAX_BYTES_CHIRHO: usize = 64 * 1024;
pub const FIGURES_CSV_MAX_ROWS_CHIRHO: usize = 1_000;

/// Why a row was kept out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowRefusalChirho {
    WrongColumnCountChirho,
    UnclosedQuoteChirho,
    /// A quote inside a plain field, or anything but a comma after a closing quote.
    MalformedQuoteChirho,
    InvalidMonthChirho,
    InvalidKindChirho,
    InvalidMoneyInChirho,
    InvalidCostsChirho,
    InvalidJobsChirho,
}

impl RowRefusalChirho {
    pub fn code_chirho(self) -> &'static str {
        match self {
            Self::WrongColumnCountChirho => "wrong-column-count-chirho",
            Self::UnclosedQuoteChirho => "unclosed-quote-chirho",
            Self::MalformedQuoteChirho => "malformed-quote-chirho",
            Self::InvalidMonthChirho => "invalid-month-chirho",
            Self::InvalidKindChirho => "invalid-kind-chirho",
            Self::InvalidMoneyInChirho => "invalid-money-in-chirho",
            Self::InvalidCostsChirho => "invalid-costs-chirho",
            Self::InvalidJobsChirho => "invalid-jobs-chirho",
        }
    }
}

/// Why nothing could be kept at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiguresImportErrorChirho {
    EmptyChirho,
    TooLargeChirho,
    TooManyRowsChirho,
    WrongHeaderChirho,
    NoRowsKeptChirho,
    InvalidChirho(FiguresValidationErrorChirho),
}

impl FiguresImportErrorChirho {
    pub fn code_chirho(self) -> &'static str {
        match self {
            Self::EmptyChirho => "empty-chirho",
            Self::TooLargeChirho => "too-large-chirho",
            Self::TooManyRowsChirho => "too-many-rows-chirho",
            Self::WrongHeaderChirho => "wrong-header-chirho",
            Self::NoRowsKeptChirho => "no-rows-kept-chirho",
            Self::InvalidChirho(error_chirho) => error_chirho.code_chirho(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefusedRowChirho {
    /// 1-based line number in what she pasted, the header being line 1.
    pub line_chirho: usize,
    pub refusal_chirho: RowRefusalChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombinedRowChirho {
    pub month_chirho: String,
    pub kind_chirho: String,
    /// Every line that went into this one cell, in order.
    pub lines_chirho: Vec<usize>,
}

/// What a paste comes to: the figures that would be kept, every combination and every refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiguresImportChirho {
    pub kept_chirho: Result<ValidatedFiguresChirho, FiguresImportErrorChirho>,
    pub kept_rows_chirho: usize,
    pub combined_chirho: Vec<CombinedRowChirho>,
    pub refused_chirho: Vec<RefusedRowChirho>,
}

struct RowChirho {
    month_chirho: String,
    kind_chirho: String,
    money_in_chirho: i64,
    costs_chirho: Option<i64>,
    jobs_chirho: Option<u32>,
}

pub fn import_figures_csv_chirho(
    text_chirho: &str,
    currency_chirho: &str,
    note_chirho: &str,
) -> FiguresImportChirho {
    let refuse_all_chirho = |error_chirho| FiguresImportChirho {
        kept_chirho: Err(error_chirho),
        kept_rows_chirho: 0,
        combined_chirho: Vec::new(),
        refused_chirho: Vec::new(),
    };
    if text_chirho.len() > FIGURES_CSV_MAX_BYTES_CHIRHO {
        return refuse_all_chirho(FiguresImportErrorChirho::TooLargeChirho);
    }
    let lines_chirho: Vec<&str> = text_chirho
        .split('\n')
        .map(|line_chirho| line_chirho.strip_suffix('\r').unwrap_or(line_chirho))
        .collect();
    let Some(header_index_chirho) = lines_chirho
        .iter()
        .position(|line_chirho| !line_chirho.trim().is_empty())
    else {
        return refuse_all_chirho(FiguresImportErrorChirho::EmptyChirho);
    };
    if lines_chirho[header_index_chirho]
        .trim()
        .trim_start_matches('\u{feff}')
        != FIGURES_CSV_HEADER_CHIRHO
    {
        return refuse_all_chirho(FiguresImportErrorChirho::WrongHeaderChirho);
    }
    let body_chirho: Vec<(usize, &str)> = lines_chirho
        .iter()
        .enumerate()
        .skip(header_index_chirho + 1)
        .filter(|(_, line_chirho)| !line_chirho.trim().is_empty())
        .map(|(index_chirho, line_chirho)| (index_chirho + 1, *line_chirho))
        .collect();
    if body_chirho.len() > FIGURES_CSV_MAX_ROWS_CHIRHO {
        return refuse_all_chirho(FiguresImportErrorChirho::TooManyRowsChirho);
    }
    let mut refused_chirho = Vec::new();
    let mut rows_chirho: Vec<(usize, RowChirho)> = Vec::new();
    for (line_chirho, text_line_chirho) in body_chirho {
        match read_row_chirho(text_line_chirho) {
            Ok(row_chirho) => rows_chirho.push((line_chirho, row_chirho)),
            Err(refusal_chirho) => refused_chirho.push(RefusedRowChirho {
                line_chirho,
                refusal_chirho,
            }),
        }
    }
    let kept_rows_chirho = rows_chirho.len();
    if rows_chirho.is_empty() {
        return FiguresImportChirho {
            kept_chirho: Err(FiguresImportErrorChirho::NoRowsKeptChirho),
            kept_rows_chirho,
            combined_chirho: Vec::new(),
            refused_chirho,
        };
    }
    let (document_chirho, combined_chirho) =
        assemble_chirho(&rows_chirho, currency_chirho, note_chirho);
    let kept_chirho = match document_chirho {
        Ok(document_chirho) => document_chirho
            .validate_chirho()
            .map_err(FiguresImportErrorChirho::InvalidChirho),
        Err(error_chirho) => Err(error_chirho),
    };
    FiguresImportChirho {
        kept_chirho,
        kept_rows_chirho,
        combined_chirho,
        refused_chirho,
    }
}

/// Months ascending, kinds in first-seen order, duplicate cells combined.
fn assemble_chirho(
    rows_chirho: &[(usize, RowChirho)],
    currency_chirho: &str,
    note_chirho: &str,
) -> (
    Result<FiguresDocumentV1Chirho, FiguresImportErrorChirho>,
    Vec<CombinedRowChirho>,
) {
    let mut months_chirho: Vec<String> = rows_chirho
        .iter()
        .map(|(_, row_chirho)| row_chirho.month_chirho.clone())
        .collect();
    months_chirho.sort_by_key(|month_chirho| parse_month_chirho(month_chirho));
    months_chirho.dedup();
    let mut kinds_chirho: Vec<String> = Vec::new();
    for (_, row_chirho) in rows_chirho {
        if !kinds_chirho
            .iter()
            .any(|kind_chirho| kind_chirho.to_lowercase() == row_chirho.kind_chirho.to_lowercase())
        {
            kinds_chirho.push(row_chirho.kind_chirho.clone());
        }
    }
    if months_chirho.len() > FIGURES_MAX_MONTHS_CHIRHO {
        return (
            Err(FiguresImportErrorChirho::InvalidChirho(
                FiguresValidationErrorChirho::TooManyMonthsChirho,
            )),
            Vec::new(),
        );
    }
    if kinds_chirho.len() > FIGURES_MAX_KINDS_CHIRHO {
        return (
            Err(FiguresImportErrorChirho::InvalidChirho(
                FiguresValidationErrorChirho::TooManyKindsChirho,
            )),
            Vec::new(),
        );
    }
    let (kind_count_chirho, month_count_chirho) = (kinds_chirho.len(), months_chirho.len());
    let mut money_in_chirho = vec![vec![None::<i64>; month_count_chirho]; kind_count_chirho];
    let mut costs_chirho = vec![vec![None::<i64>; month_count_chirho]; kind_count_chirho];
    let mut jobs_chirho = vec![vec![None::<u32>; month_count_chirho]; kind_count_chirho];
    // Per cell: the lines it came from, and whether costs and jobs stayed known across them.
    let mut cells_chirho: Vec<Vec<(Vec<usize>, bool, bool)>> =
        vec![vec![(Vec::new(), true, true); month_count_chirho]; kind_count_chirho];
    let mut out_of_range_chirho = false;
    for (line_chirho, row_chirho) in rows_chirho {
        let kind_index_chirho = kinds_chirho
            .iter()
            .position(|kind_chirho| {
                kind_chirho.to_lowercase() == row_chirho.kind_chirho.to_lowercase()
            })
            .unwrap_or(0);
        let month_index_chirho = months_chirho
            .iter()
            .position(|month_chirho| *month_chirho == row_chirho.month_chirho)
            .unwrap_or(0);
        let cell_chirho = &mut cells_chirho[kind_index_chirho][month_index_chirho];
        cell_chirho.0.push(*line_chirho);
        let money_chirho = &mut money_in_chirho[kind_index_chirho][month_index_chirho];
        *money_chirho = Some(money_chirho.unwrap_or(0) + row_chirho.money_in_chirho);
        cell_chirho.1 &= row_chirho.costs_chirho.is_some();
        cell_chirho.2 &= row_chirho.jobs_chirho.is_some();
        let cost_cell_chirho = &mut costs_chirho[kind_index_chirho][month_index_chirho];
        *cost_cell_chirho = if cell_chirho.1 {
            Some(cost_cell_chirho.unwrap_or(0) + row_chirho.costs_chirho.unwrap_or(0))
        } else {
            None
        };
        let jobs_cell_chirho = &mut jobs_chirho[kind_index_chirho][month_index_chirho];
        *jobs_cell_chirho = if cell_chirho.2 {
            jobs_cell_chirho
                .unwrap_or(0)
                .checked_add(row_chirho.jobs_chirho.unwrap_or(0))
        } else {
            None
        };
        out_of_range_chirho |= money_chirho
            .is_some_and(|sum_chirho| sum_chirho > FIGURES_MAX_AMOUNT_MINOR_CHIRHO)
            || cost_cell_chirho
                .is_some_and(|sum_chirho| sum_chirho > FIGURES_MAX_AMOUNT_MINOR_CHIRHO)
            || jobs_cell_chirho.is_some_and(|sum_chirho| sum_chirho > FIGURES_MAX_JOBS_CHIRHO)
            || (cell_chirho.2 && jobs_cell_chirho.is_none());
    }
    let mut combined_chirho = Vec::new();
    for (kind_index_chirho, kind_cells_chirho) in cells_chirho.into_iter().enumerate() {
        for (month_index_chirho, (lines_chirho, _, _)) in kind_cells_chirho.into_iter().enumerate()
        {
            if lines_chirho.len() > 1 {
                combined_chirho.push(CombinedRowChirho {
                    month_chirho: months_chirho[month_index_chirho].clone(),
                    kind_chirho: kinds_chirho[kind_index_chirho].clone(),
                    lines_chirho,
                });
            }
        }
    }
    if out_of_range_chirho {
        return (
            Err(FiguresImportErrorChirho::InvalidChirho(
                FiguresValidationErrorChirho::AmountOutOfRangeChirho,
            )),
            combined_chirho,
        );
    }
    let document_chirho = FiguresDocumentV1Chirho {
        schema_chirho: FIGURES_SCHEMA_CHIRHO.to_string(),
        currency_chirho: currency_chirho.to_string(),
        note_chirho: note_chirho.to_string(),
        months_chirho,
        kinds_chirho,
        money_in_chirho,
        costs_chirho,
        jobs_chirho,
    };
    (Ok(document_chirho), combined_chirho)
}

fn read_row_chirho(line_chirho: &str) -> Result<RowChirho, RowRefusalChirho> {
    let fields_chirho = split_fields_chirho(line_chirho)?;
    let [month_chirho, kind_chirho, money_in_chirho, costs_chirho, jobs_chirho] =
        fields_chirho.as_slice()
    else {
        return Err(RowRefusalChirho::WrongColumnCountChirho);
    };
    let month_chirho = month_chirho.trim();
    parse_month_chirho(month_chirho).ok_or(RowRefusalChirho::InvalidMonthChirho)?;
    let kind_chirho = kind_chirho.trim();
    if !valid_kind_chirho(kind_chirho) {
        return Err(RowRefusalChirho::InvalidKindChirho);
    }
    let money_in_chirho = parse_amount_chirho(money_in_chirho.trim())
        .ok_or(RowRefusalChirho::InvalidMoneyInChirho)?;
    let costs_chirho = optional_chirho(costs_chirho, parse_amount_chirho)
        .map_err(|()| RowRefusalChirho::InvalidCostsChirho)?;
    let jobs_chirho = optional_chirho(jobs_chirho, parse_jobs_chirho)
        .map_err(|()| RowRefusalChirho::InvalidJobsChirho)?;
    Ok(RowChirho {
        month_chirho: month_chirho.to_string(),
        kind_chirho: kind_chirho.to_string(),
        money_in_chirho,
        costs_chirho,
        jobs_chirho,
    })
}

fn optional_chirho<T>(
    field_chirho: &str,
    parse_chirho: fn(&str) -> Option<T>,
) -> Result<Option<T>, ()> {
    let field_chirho = field_chirho.trim();
    if field_chirho.is_empty() {
        return Ok(None);
    }
    parse_chirho(field_chirho).map(Some).ok_or(())
}

/// RFC 4180 fields, strictly. A field is either plain (no quote anywhere in it) or wholly quoted, where
/// a doubled quote stands for one quote and a comma is just text. After a closing quote only a comma or
/// the line's end may follow: `"1"0` is refused, never read as 10 (gpt #26569). Spaces before an
/// opening quote are allowed.
fn split_fields_chirho(line_chirho: &str) -> Result<Vec<String>, RowRefusalChirho> {
    #[expect(
        clippy::enum_variant_names,
        reason = "every identifier we name carries the Chirho suffix; private enums are the only ones clippy checks for it"
    )]
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum StateChirho {
        StartChirho,
        PlainChirho,
        QuotedChirho,
        AfterQuoteChirho,
    }
    let mut fields_chirho = Vec::new();
    let mut field_chirho = String::new();
    let mut state_chirho = StateChirho::StartChirho;
    let mut characters_chirho = line_chirho.chars().peekable();
    while let Some(character_chirho) = characters_chirho.next() {
        state_chirho = match (state_chirho, character_chirho) {
            (StateChirho::QuotedChirho, '"') if characters_chirho.peek() == Some(&'"') => {
                characters_chirho.next();
                field_chirho.push('"');
                StateChirho::QuotedChirho
            }
            (StateChirho::QuotedChirho, '"') => StateChirho::AfterQuoteChirho,
            (StateChirho::QuotedChirho, other_chirho) => {
                field_chirho.push(other_chirho);
                StateChirho::QuotedChirho
            }
            (StateChirho::StartChirho, '"') => {
                field_chirho.clear();
                StateChirho::QuotedChirho
            }
            (
                StateChirho::StartChirho | StateChirho::PlainChirho | StateChirho::AfterQuoteChirho,
                ',',
            ) => {
                fields_chirho.push(std::mem::take(&mut field_chirho));
                StateChirho::StartChirho
            }
            (StateChirho::StartChirho, space_chirho) if space_chirho.is_whitespace() => {
                field_chirho.push(space_chirho);
                StateChirho::StartChirho
            }
            (StateChirho::PlainChirho, '"') | (StateChirho::AfterQuoteChirho, _) => {
                return Err(RowRefusalChirho::MalformedQuoteChirho);
            }
            (StateChirho::StartChirho | StateChirho::PlainChirho, other_chirho) => {
                field_chirho.push(other_chirho);
                StateChirho::PlainChirho
            }
        };
    }
    if state_chirho == StateChirho::QuotedChirho {
        return Err(RowRefusalChirho::UnclosedQuoteChirho);
    }
    fields_chirho.push(field_chirho);
    Ok(fields_chirho)
}

/// `1234`, `1234.5` or `1234.50` into minor units; nothing else.
pub fn parse_amount_chirho(text_chirho: &str) -> Option<i64> {
    let (whole_chirho, fraction_chirho) = text_chirho.split_once('.').unwrap_or((text_chirho, ""));
    if whole_chirho.is_empty()
        || whole_chirho.len() > 9
        || fraction_chirho.len() > 2
        || (text_chirho.contains('.') && fraction_chirho.is_empty())
    {
        return None;
    }
    if !whole_chirho
        .bytes()
        .chain(fraction_chirho.bytes())
        .all(|byte_chirho| byte_chirho.is_ascii_digit())
    {
        return None;
    }
    let cents_chirho = format!("{fraction_chirho:0<2}");
    Some(whole_chirho.parse::<i64>().ok()? * 100 + cents_chirho.parse::<i64>().ok()?)
}

fn parse_jobs_chirho(text_chirho: &str) -> Option<u32> {
    (text_chirho.len() <= 6
        && text_chirho
            .bytes()
            .all(|byte_chirho| byte_chirho.is_ascii_digit()))
    .then(|| text_chirho.parse::<u32>().ok())
    .flatten()
}
