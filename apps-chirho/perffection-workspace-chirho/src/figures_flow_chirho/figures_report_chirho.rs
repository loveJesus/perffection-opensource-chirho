// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The one report her saved figures give: "which work pays its way?"
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. A pure function of
//! one saved version: the same version always gives byte-identical JSON. Integer money only; shares
//! are basis points and money per job is rounded half up. A total that depends on an unknown figure
//! is unknown, never a partial sum dressed as whole: costs or jobs missing in any month she has money
//! in make that kind's costs, margin or money per job `None`, and a flag names the months; with nothing
//! given at all they are `None` too, never an empty sum of 0. Shares are of the money in she gave;
//! `complete_chirho` says whether every kind has a figure for every month. The falling-share trend
//! compares totals across months, so it is declined unless the figures are complete: an unknown
//! figure is never read as zero (gpt #26569). There is no forecast and no advice; flags state facts.

use crate::figures_flow_chirho::figures_document_chirho::{
    parse_month_chirho, FiguresDocumentV1Chirho,
};
use serde::Serialize;

pub const FIGURES_REPORT_SCHEMA_CHIRHO: &str = "perffection-figures-report-v1-chirho";
/// A kind's share in the latest month this far below its share across the earlier months is flagged.
pub const FALLING_SHARE_BASIS_POINTS_CHIRHO: i128 = 500;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FiguresReportV1Chirho {
    pub schema_chirho: &'static str,
    /// The saved version this report reads.
    pub version_chirho: u64,
    pub currency_chirho: String,
    pub note_chirho: String,
    pub first_month_chirho: String,
    /// The latest month in her figures: the report is "as of" it, never live.
    pub as_of_month_chirho: String,
    pub money_in_minor_chirho: i64,
    /// Every kind has a money-in figure for every month. When false, shares and totals are of the
    /// figures she gave, and no trend is drawn.
    pub complete_chirho: bool,
    pub kinds_chirho: Vec<KindLineChirho>,
    pub months_chirho: Vec<MonthLineChirho>,
    pub flags_chirho: Vec<FiguresFlagChirho>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct KindLineChirho {
    pub kind_chirho: String,
    /// The sum of the months she gave a figure for.
    pub money_in_minor_chirho: i64,
    pub months_with_figures_chirho: usize,
    pub costs_minor_chirho: Option<i64>,
    pub margin_minor_chirho: Option<i64>,
    /// Of all her money in, in basis points (1/100 of a percent), rounded half up.
    pub share_basis_points_chirho: u32,
    pub jobs_chirho: Option<u64>,
    pub money_per_job_minor_chirho: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MonthLineChirho {
    pub month_chirho: String,
    pub money_in_minor_chirho: i64,
    pub costs_minor_chirho: Option<i64>,
    pub margin_minor_chirho: Option<i64>,
    /// Kinds with no figure this month.
    pub kinds_without_figures_chirho: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FiguresFlagChirho {
    pub code_chirho: &'static str,
    pub kind_chirho: Option<String>,
    pub months_chirho: Vec<String>,
}

pub fn figures_report_chirho(
    version_chirho: u64,
    document_chirho: &FiguresDocumentV1Chirho,
) -> FiguresReportV1Chirho {
    let months_chirho = &document_chirho.months_chirho;
    let money_chirho = &document_chirho.money_in_chirho;
    let total_chirho: i64 = money_chirho.iter().flatten().flatten().sum();
    let mut flags_chirho = Vec::new();
    let complete_chirho = money_chirho.iter().flatten().all(Option::is_some);
    let mut kinds_chirho = Vec::new();
    for (index_chirho, kind_chirho) in document_chirho.kinds_chirho.iter().enumerate() {
        let money_row_chirho = &money_chirho[index_chirho];
        let given_chirho: Vec<usize> = (0..months_chirho.len())
            .filter(|month_chirho| money_row_chirho[*month_chirho].is_some())
            .collect();
        let money_in_chirho: i64 = given_chirho
            .iter()
            .filter_map(|month_chirho| money_row_chirho[*month_chirho])
            .sum();
        let costs_row_chirho = &document_chirho.costs_chirho[index_chirho];
        let jobs_row_chirho = &document_chirho.jobs_chirho[index_chirho];
        let costs_unknown_chirho: Vec<String> = given_chirho
            .iter()
            .filter(|month_chirho| costs_row_chirho[**month_chirho].is_none())
            .map(|month_chirho| months_chirho[*month_chirho].clone())
            .collect();
        let costs_chirho =
            (!given_chirho.is_empty() && costs_unknown_chirho.is_empty()).then(|| {
                given_chirho
                    .iter()
                    .filter_map(|month_chirho| costs_row_chirho[*month_chirho])
                    .sum::<i64>()
            });
        let jobs_chirho = (!given_chirho.is_empty()
            && given_chirho
                .iter()
                .all(|month_chirho| jobs_row_chirho[*month_chirho].is_some()))
        .then(|| {
            given_chirho
                .iter()
                .filter_map(|month_chirho| jobs_row_chirho[*month_chirho])
                .map(u64::from)
                .sum::<u64>()
        });
        let margin_chirho = costs_chirho.map(|costs_chirho| money_in_chirho - costs_chirho);
        let line_chirho = KindLineChirho {
            kind_chirho: kind_chirho.clone(),
            money_in_minor_chirho: money_in_chirho,
            months_with_figures_chirho: given_chirho.len(),
            costs_minor_chirho: costs_chirho,
            margin_minor_chirho: margin_chirho,
            share_basis_points_chirho: basis_points_chirho(money_in_chirho, total_chirho),
            jobs_chirho,
            money_per_job_minor_chirho: jobs_chirho.filter(|jobs_chirho| *jobs_chirho > 0).map(
                |jobs_chirho| {
                    round_half_up_chirho(i128::from(money_in_chirho), i128::from(jobs_chirho))
                        as i64
                },
            ),
        };
        if margin_chirho.is_some_and(|margin_chirho| margin_chirho < 0) {
            flags_chirho.push(flag_chirho(
                "negative-margin-chirho",
                kind_chirho,
                Vec::new(),
            ));
        }
        if complete_chirho && falling_share_chirho(document_chirho, index_chirho) {
            flags_chirho.push(flag_chirho(
                "falling-share-chirho",
                kind_chirho,
                vec![months_chirho[months_chirho.len() - 1].clone()],
            ));
        }
        if !costs_unknown_chirho.is_empty() {
            flags_chirho.push(flag_chirho(
                "costs-unknown-chirho",
                kind_chirho,
                costs_unknown_chirho,
            ));
        }
        let missing_chirho: Vec<String> = (0..months_chirho.len())
            .filter(|month_chirho| money_row_chirho[*month_chirho].is_none())
            .map(|month_chirho| months_chirho[month_chirho].clone())
            .collect();
        if !missing_chirho.is_empty() {
            flags_chirho.push(flag_chirho(
                "missing-months-chirho",
                kind_chirho,
                missing_chirho,
            ));
        }
        kinds_chirho.push(line_chirho);
    }
    let gaps_chirho = calendar_gaps_chirho(months_chirho);
    if !gaps_chirho.is_empty() {
        flags_chirho.push(FiguresFlagChirho {
            code_chirho: "gap-in-months-chirho",
            kind_chirho: None,
            months_chirho: gaps_chirho,
        });
    }
    FiguresReportV1Chirho {
        schema_chirho: FIGURES_REPORT_SCHEMA_CHIRHO,
        version_chirho,
        currency_chirho: document_chirho.currency_chirho.clone(),
        note_chirho: document_chirho.note_chirho.clone(),
        first_month_chirho: months_chirho.first().cloned().unwrap_or_default(),
        as_of_month_chirho: months_chirho.last().cloned().unwrap_or_default(),
        money_in_minor_chirho: total_chirho,
        complete_chirho,
        kinds_chirho,
        months_chirho: month_lines_chirho(document_chirho),
        flags_chirho,
    }
}

fn month_lines_chirho(document_chirho: &FiguresDocumentV1Chirho) -> Vec<MonthLineChirho> {
    (0..document_chirho.months_chirho.len())
        .map(|month_chirho| {
            let kinds_chirho = 0..document_chirho.kinds_chirho.len();
            let given_chirho: Vec<usize> = kinds_chirho
                .clone()
                .filter(|kind_chirho| {
                    document_chirho.money_in_chirho[*kind_chirho][month_chirho].is_some()
                })
                .collect();
            let money_in_chirho: i64 = given_chirho
                .iter()
                .filter_map(|kind_chirho| {
                    document_chirho.money_in_chirho[*kind_chirho][month_chirho]
                })
                .sum();
            let costs_chirho = given_chirho
                .iter()
                .map(|kind_chirho| document_chirho.costs_chirho[*kind_chirho][month_chirho])
                .collect::<Option<Vec<i64>>>()
                .filter(|costs_chirho| !costs_chirho.is_empty())
                .map(|costs_chirho| costs_chirho.iter().sum::<i64>());
            MonthLineChirho {
                month_chirho: document_chirho.months_chirho[month_chirho].clone(),
                money_in_minor_chirho: money_in_chirho,
                costs_minor_chirho: costs_chirho,
                margin_minor_chirho: costs_chirho
                    .map(|costs_chirho| money_in_chirho - costs_chirho),
                kinds_without_figures_chirho: kinds_chirho.len() - given_chirho.len(),
            }
        })
        .collect()
}

/// The kind's share of the latest month, against its share of all the earlier months together.
fn falling_share_chirho(document_chirho: &FiguresDocumentV1Chirho, kind_chirho: usize) -> bool {
    let month_count_chirho = document_chirho.months_chirho.len();
    if month_count_chirho < 2 {
        return false;
    }
    let latest_chirho = month_count_chirho - 1;
    let Some(latest_money_chirho) = document_chirho.money_in_chirho[kind_chirho][latest_chirho]
    else {
        return false;
    };
    let month_total_chirho = |month_chirho: usize| -> i128 {
        document_chirho
            .money_in_chirho
            .iter()
            .filter_map(|row_chirho| row_chirho[month_chirho])
            .map(i128::from)
            .sum()
    };
    let latest_total_chirho = month_total_chirho(latest_chirho);
    let earlier_total_chirho: i128 = (0..latest_chirho).map(month_total_chirho).sum();
    let earlier_kind_chirho: i128 = document_chirho.money_in_chirho[kind_chirho][..latest_chirho]
        .iter()
        .flatten()
        .map(|money_chirho| i128::from(*money_chirho))
        .sum();
    if latest_total_chirho == 0 || earlier_total_chirho == 0 {
        return false;
    }
    let latest_share_chirho = round_half_up_chirho(
        i128::from(latest_money_chirho) * 10_000,
        latest_total_chirho,
    );
    let earlier_share_chirho =
        round_half_up_chirho(earlier_kind_chirho * 10_000, earlier_total_chirho);
    earlier_share_chirho - latest_share_chirho >= FALLING_SHARE_BASIS_POINTS_CHIRHO
}

/// Calendar months between her first and last month that her figures leave out entirely.
fn calendar_gaps_chirho(months_chirho: &[String]) -> Vec<String> {
    let parsed_chirho: Vec<(u32, u32)> = months_chirho
        .iter()
        .filter_map(|month_chirho| parse_month_chirho(month_chirho))
        .collect();
    let (Some(first_chirho), Some(last_chirho)) = (parsed_chirho.first(), parsed_chirho.last())
    else {
        return Vec::new();
    };
    let mut gaps_chirho = Vec::new();
    let (mut year_chirho, mut month_chirho) = *first_chirho;
    while (year_chirho, month_chirho) < *last_chirho {
        if !parsed_chirho.contains(&(year_chirho, month_chirho)) {
            gaps_chirho.push(format!("{year_chirho:04}-{month_chirho:02}"));
        }
        (year_chirho, month_chirho) = if month_chirho == 12 {
            (year_chirho + 1, 1)
        } else {
            (year_chirho, month_chirho + 1)
        };
    }
    gaps_chirho
}

fn flag_chirho(
    code_chirho: &'static str,
    kind_chirho: &str,
    months_chirho: Vec<String>,
) -> FiguresFlagChirho {
    FiguresFlagChirho {
        code_chirho,
        kind_chirho: Some(kind_chirho.to_string()),
        months_chirho,
    }
}

fn basis_points_chirho(part_chirho: i64, total_chirho: i64) -> u32 {
    if total_chirho <= 0 {
        return 0;
    }
    round_half_up_chirho(i128::from(part_chirho) * 10_000, i128::from(total_chirho)) as u32
}

/// `numerator / denominator` rounded half up, for a non-negative numerator and a positive denominator.
fn round_half_up_chirho(numerator_chirho: i128, denominator_chirho: i128) -> i128 {
    (2 * numerator_chirho + denominator_chirho) / (2 * denominator_chirho)
}
