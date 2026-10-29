// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The typed models of her figures pages: the report with the paste form, and the review.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. Every figure is
//! formatted here from integers; an unknown is written as unknown, never as a number. The review shows
//! exactly what would be kept, every line added together and every line left out with its reason, and
//! offers Save only when something can be kept. Nothing on these pages is publishable.

use crate::figures_flow_chirho::figures_csv_chirho::{
    FiguresImportChirho, FIGURES_CSV_HEADER_CHIRHO,
};
use crate::figures_flow_chirho::figures_document_chirho::FiguresDocumentV1Chirho;
use crate::figures_flow_chirho::figures_form_chirho::{
    figures_to_csv_chirho, FiguresFormChirho, FiguresFormRefusalChirho,
};
use crate::figures_flow_chirho::figures_report_chirho::{
    figures_report_chirho, FiguresReportV1Chirho,
};
use crate::viewer_state_chirho::figures_contract_chirho::FiguresStateChirho;
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
};
use perffection_ui_chirho::{
    SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho, WorkspaceFiguresCopyChirho,
};

pub const FIGURES_HREF_CHIRHO: &str = "/app-chirho/perffection-workspace/figures-chirho";
pub const FIGURES_REVIEW_ACTION_CHIRHO: &str =
    "/app-chirho/perffection-workspace/review-figures-chirho";
pub const FIGURES_SAVE_ACTION_CHIRHO: &str =
    "/app-chirho/perffection-workspace/save-figures-chirho";
const DEFAULT_CURRENCY_CHIRHO: &str = "USD";

/// What happened to her last save, when the page answers one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiguresNoticeChirho {
    SavedChirho,
    ConflictChirho,
    UnknownChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindRowViewChirho {
    pub(crate) kind_chirho: String,
    pub(crate) money_in_chirho: String,
    pub(crate) costs_chirho: String,
    pub(crate) margin_chirho: String,
    pub(crate) share_chirho: String,
    pub(crate) per_job_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonthRowViewChirho {
    pub(crate) month_chirho: String,
    pub(crate) money_in_chirho: String,
    pub(crate) costs_chirho: String,
    pub(crate) margin_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagViewChirho {
    pub(crate) sentence_chirho: String,
    /// The kind it is about, empty when it is about her figures as a whole.
    pub(crate) kind_chirho: String,
    pub(crate) months_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportViewChirho {
    pub(crate) version_chirho: u64,
    pub(crate) as_of_chirho: String,
    pub(crate) currency_chirho: String,
    pub(crate) note_chirho: String,
    pub(crate) total_money_in_chirho: String,
    pub(crate) complete_chirho: bool,
    pub(crate) kinds_chirho: Vec<KindRowViewChirho>,
    pub(crate) months_chirho: Vec<MonthRowViewChirho>,
    pub(crate) flags_chirho: Vec<FlagViewChirho>,
}

/// The four fields every figures form posts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiguresFormViewChirho {
    pub(crate) csv_chirho: String,
    pub(crate) currency_chirho: String,
    pub(crate) note_chirho: String,
    pub(crate) expected_version_chirho: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiguresPageV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) status_chirho: String,
    /// Her sentence about the last save, and its kind (`saved`, `conflict` or `unknown`) as a class, so
    /// the page and its checks tell an answered save from a refused one without reading the words.
    pub(crate) notice_chirho: Option<(String, &'static str)>,
    pub(crate) report_chirho: Option<ReportViewChirho>,
    pub(crate) unavailable_chirho: bool,
    pub(crate) form_chirho: FiguresFormViewChirho,
    pub(crate) review_action_chirho: &'static str,
    pub(crate) csv_header_chirho: &'static str,
}

impl FiguresPageV1Chirho {
    /// The page for a read: her saved figures, none yet (`Ok(None)`), or a state that could not be
    /// verified (`Err`), which is never shown as "none".
    pub fn from_state_chirho(
        state_chirho: Result<Option<&FiguresStateChirho>, ()>,
        notice_chirho: Option<FiguresNoticeChirho>,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
        let figures_copy_chirho = &copy_chirho.figures_chirho;
        let (report_chirho, unavailable_chirho, form_chirho) = match state_chirho {
            Ok(Some(state_chirho)) => (
                Some(report_view_chirho(
                    &figures_report_chirho(
                        state_chirho.current_version_chirho,
                        &state_chirho.figures_chirho,
                    ),
                    figures_copy_chirho,
                )),
                false,
                FiguresFormViewChirho {
                    csv_chirho: figures_to_csv_chirho(&state_chirho.figures_chirho),
                    currency_chirho: state_chirho.figures_chirho.currency_chirho.clone(),
                    note_chirho: state_chirho.figures_chirho.note_chirho.clone(),
                    expected_version_chirho: state_chirho.current_version_chirho,
                },
            ),
            Ok(None) => (None, false, empty_form_chirho(0)),
            Err(()) => (None, true, empty_form_chirho(0)),
        };
        let status_chirho = match &report_chirho {
            Some(report_chirho) => format!(
                "{} {} · {} {}",
                figures_copy_chirho.version_label_chirho,
                report_chirho.version_chirho,
                figures_copy_chirho.as_of_label_chirho,
                report_chirho.as_of_chirho
            ),
            None if unavailable_chirho => figures_copy_chirho.unavailable_chirho.clone(),
            None => figures_copy_chirho.nothing_saved_chirho.clone(),
        };
        let notice_chirho = notice_chirho.map(|notice_chirho| {
            let (sentence_chirho, kind_chirho) = match notice_chirho {
                FiguresNoticeChirho::SavedChirho => {
                    (&figures_copy_chirho.saved_notice_chirho, "saved")
                }
                FiguresNoticeChirho::ConflictChirho => {
                    (&figures_copy_chirho.conflict_notice_chirho, "conflict")
                }
                FiguresNoticeChirho::UnknownChirho => {
                    (&figures_copy_chirho.unknown_notice_chirho, "unknown")
                }
            };
            (sentence_chirho.clone(), kind_chirho)
        });
        Self {
            chrome_chirho: figures_chrome_chirho(&copy_chirho),
            status_chirho,
            notice_chirho,
            report_chirho,
            unavailable_chirho,
            form_chirho,
            review_action_chirho: FIGURES_REVIEW_ACTION_CHIRHO,
            csv_header_chirho: FIGURES_CSV_HEADER_CHIRHO,
            copy_chirho,
        }
    }

    /// Someone saved first: the saved figures' report, and her attempt back in the paste box against
    /// the new head, so she can review it again. The whole attempt comes back - its CSV, its currency
    /// and its note - so a re-review never quietly takes the head's units or provenance (gpt #26573).
    pub fn conflict_chirho(
        head_chirho: &FiguresStateChirho,
        attempted_chirho: Option<&FiguresDocumentV1Chirho>,
    ) -> Self {
        let mut page_chirho = Self::from_state_chirho(
            Ok(Some(head_chirho)),
            Some(FiguresNoticeChirho::ConflictChirho),
        );
        if let Some(attempted_chirho) = attempted_chirho {
            page_chirho.form_chirho.csv_chirho = figures_to_csv_chirho(attempted_chirho);
            page_chirho.form_chirho.currency_chirho = attempted_chirho.currency_chirho.clone();
            page_chirho.form_chirho.note_chirho = attempted_chirho.note_chirho.clone();
        }
        page_chirho
    }
}

/// Nothing saved: an empty paste box, its header line shown only as a placeholder, so choosing a file
/// never collides with text she did not write.
fn empty_form_chirho(expected_version_chirho: u64) -> FiguresFormViewChirho {
    FiguresFormViewChirho {
        csv_chirho: String::new(),
        currency_chirho: DEFAULT_CURRENCY_CHIRHO.to_string(),
        note_chirho: String::new(),
        expected_version_chirho,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeptRowViewChirho {
    pub(crate) kind_chirho: String,
    pub(crate) cells_chirho: Vec<String>,
}

/// One of the three things kept for every month and kind: money in, direct costs, jobs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeptTableViewChirho {
    pub(crate) caption_chirho: String,
    pub(crate) rows_chirho: Vec<KeptRowViewChirho>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineViewChirho {
    pub(crate) label_chirho: String,
    pub(crate) sentence_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiguresReviewV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    /// Set when nothing can be kept: her sentence for why, and its code, which the page carries as a
    /// data attribute so its checks tell one refusal from another without reading the words.
    pub(crate) refusal_chirho: Option<(String, &'static str)>,
    pub(crate) kept_summary_chirho: String,
    pub(crate) kept_months_chirho: Vec<String>,
    /// The currency and her note exactly as they will be kept.
    pub(crate) kept_currency_chirho: String,
    pub(crate) kept_note_chirho: String,
    /// The name of the file her figures came from, when they came as a file: display only.
    pub(crate) kept_file_name_chirho: Option<String>,
    pub(crate) kept_tables_chirho: Vec<KeptTableViewChirho>,
    pub(crate) combined_chirho: Vec<LineViewChirho>,
    pub(crate) refused_chirho: Vec<LineViewChirho>,
    /// The exact fields the Save posts, only when something can be kept.
    pub(crate) save_form_chirho: Option<FiguresFormViewChirho>,
    pub(crate) save_action_chirho: &'static str,
    pub(crate) back_href_chirho: &'static str,
}

impl FiguresReviewV1Chirho {
    pub fn from_import_chirho(
        form_chirho: &FiguresFormChirho,
        import_chirho: &FiguresImportChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
        let figures_copy_chirho = &copy_chirho.figures_chirho;
        let combined_chirho = import_chirho
            .combined_chirho
            .iter()
            .map(|row_chirho| LineViewChirho {
                label_chirho: format!("{} · {}", row_chirho.month_chirho, row_chirho.kind_chirho),
                sentence_chirho: format!(
                    "{} {} {}",
                    row_chirho.lines_chirho.len(),
                    figures_copy_chirho.combined_line_chirho,
                    row_chirho
                        .lines_chirho
                        .iter()
                        .map(usize::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            })
            .collect();
        let refused_chirho = import_chirho
            .refused_chirho
            .iter()
            .map(|row_chirho| LineViewChirho {
                label_chirho: format!(
                    "{} {}",
                    figures_copy_chirho.refused_line_chirho, row_chirho.line_chirho
                ),
                sentence_chirho: figures_copy_chirho
                    .row_sentence_chirho(row_chirho.refusal_chirho.code_chirho())
                    .to_string(),
            })
            .collect();
        let (
            refusal_chirho,
            kept_summary_chirho,
            kept_months_chirho,
            kept_tables_chirho,
            save_form_chirho,
        ) = match &import_chirho.kept_chirho {
            Ok(kept_chirho) => {
                let document_chirho = kept_chirho.document_chirho();
                // What is kept for every month: money in where she gave it, and costs and jobs beside it,
                // "unknown" where she left them empty; a month she gave no figure for stays blank.
                let table_chirho =
                    |caption_chirho: &str, cell_chirho: &dyn Fn(usize, usize) -> String| {
                        KeptTableViewChirho {
                            caption_chirho: caption_chirho.to_string(),
                            rows_chirho: document_chirho
                                .kinds_chirho
                                .iter()
                                .enumerate()
                                .map(|(kind_chirho, label_chirho)| KeptRowViewChirho {
                                    kind_chirho: label_chirho.clone(),
                                    cells_chirho: (0..document_chirho.months_chirho.len())
                                        .map(|month_chirho| cell_chirho(kind_chirho, month_chirho))
                                        .collect(),
                                })
                                .collect(),
                        }
                    };
                let given_chirho = |kind_chirho: usize, month_chirho: usize| {
                    document_chirho.money_in_chirho[kind_chirho][month_chirho].is_some()
                };
                let tables_chirho = vec![
                    table_chirho(
                        &figures_copy_chirho.money_in_column_chirho,
                        &|kind_chirho, month_chirho| {
                            document_chirho.money_in_chirho[kind_chirho][month_chirho]
                                .map(money_chirho)
                                .unwrap_or_default()
                        },
                    ),
                    table_chirho(
                        &figures_copy_chirho.costs_column_chirho,
                        &|kind_chirho, month_chirho| match document_chirho.costs_chirho[kind_chirho]
                            [month_chirho]
                        {
                            Some(costs_chirho) => money_chirho(costs_chirho),
                            None if given_chirho(kind_chirho, month_chirho) => {
                                figures_copy_chirho.unknown_chirho.clone()
                            }
                            None => String::new(),
                        },
                    ),
                    table_chirho(
                        &figures_copy_chirho.jobs_column_chirho,
                        &|kind_chirho, month_chirho| match document_chirho.jobs_chirho[kind_chirho]
                            [month_chirho]
                        {
                            Some(jobs_chirho) => jobs_chirho.to_string(),
                            None if given_chirho(kind_chirho, month_chirho) => {
                                figures_copy_chirho.unknown_chirho.clone()
                            }
                            None => String::new(),
                        },
                    ),
                ];
                (
                    None,
                    format!(
                        "{} · {}",
                        figures_copy_chirho.kept_counts_chirho(
                            document_chirho.kinds_chirho.len(),
                            document_chirho.months_chirho.len()
                        ),
                        document_chirho.currency_chirho
                    ),
                    document_chirho.months_chirho.clone(),
                    tables_chirho,
                    Some(FiguresFormViewChirho {
                        csv_chirho: form_chirho.csv_chirho.clone(),
                        currency_chirho: form_chirho.currency_chirho.clone(),
                        note_chirho: form_chirho.note_chirho.clone(),
                        expected_version_chirho: form_chirho.expected_version_chirho,
                    }),
                )
            }
            Err(error_chirho) => (
                Some((
                    figures_copy_chirho
                        .refusal_sentence_chirho(error_chirho.code_chirho())
                        .to_string(),
                    error_chirho.code_chirho(),
                )),
                String::new(),
                Vec::new(),
                Vec::new(),
                None,
            ),
        };
        let (kept_currency_chirho, kept_note_chirho) = match &import_chirho.kept_chirho {
            Ok(kept_chirho) => (
                kept_chirho.document_chirho().currency_chirho.clone(),
                kept_chirho.document_chirho().note_chirho.clone(),
            ),
            Err(_) => (String::new(), String::new()),
        };
        Self {
            chrome_chirho: figures_chrome_chirho(&copy_chirho),
            refusal_chirho,
            kept_summary_chirho,
            kept_months_chirho,
            kept_tables_chirho,
            kept_currency_chirho,
            kept_note_chirho,
            kept_file_name_chirho: form_chirho.file_name_chirho.clone(),
            combined_chirho,
            refused_chirho,
            save_form_chirho,
            save_action_chirho: FIGURES_SAVE_ACTION_CHIRHO,
            back_href_chirho: FIGURES_HREF_CHIRHO,
            copy_chirho,
        }
    }

    pub fn unreadable_form_chirho() -> Self {
        Self::refused_form_chirho(FiguresFormRefusalChirho::UnreadableChirho)
    }

    /// A form that gives nothing to review (an unreadable form, or an upload refused before it is
    /// read): her sentence for why, and no Save.
    pub fn refused_form_chirho(refusal_chirho: FiguresFormRefusalChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
        Self {
            chrome_chirho: figures_chrome_chirho(&copy_chirho),
            refusal_chirho: Some((
                copy_chirho
                    .figures_chirho
                    .refusal_sentence_chirho(refusal_chirho.code_chirho())
                    .to_string(),
                refusal_chirho.code_chirho(),
            )),
            kept_summary_chirho: String::new(),
            kept_months_chirho: Vec::new(),
            kept_tables_chirho: Vec::new(),
            kept_currency_chirho: String::new(),
            kept_note_chirho: String::new(),
            kept_file_name_chirho: None,
            combined_chirho: Vec::new(),
            refused_chirho: Vec::new(),
            save_form_chirho: None,
            save_action_chirho: FIGURES_SAVE_ACTION_CHIRHO,
            back_href_chirho: FIGURES_HREF_CHIRHO,
            copy_chirho,
        }
    }
}

fn figures_chrome_chirho(copy_chirho: &WorkspaceCopyChirho) -> SiteChromeChirho {
    workspace_chrome_chirho(
        copy_chirho,
        &copy_chirho.figures_chirho.page_title_chirho,
        &copy_chirho.figures_chirho.page_description_chirho,
        WorkspaceNavPageChirho::FiguresChirho,
        WorkspaceNavRoadsChirho::deployed_chirho(),
    )
}

fn report_view_chirho(
    report_chirho: &FiguresReportV1Chirho,
    copy_chirho: &WorkspaceFiguresCopyChirho,
) -> ReportViewChirho {
    let known_chirho = |value_chirho: Option<i64>| {
        value_chirho
            .map(money_chirho)
            .unwrap_or_else(|| copy_chirho.unknown_chirho.clone())
    };
    ReportViewChirho {
        version_chirho: report_chirho.version_chirho,
        as_of_chirho: report_chirho.as_of_month_chirho.clone(),
        currency_chirho: report_chirho.currency_chirho.clone(),
        note_chirho: report_chirho.note_chirho.clone(),
        total_money_in_chirho: money_chirho(report_chirho.money_in_minor_chirho),
        complete_chirho: report_chirho.complete_chirho,
        kinds_chirho: report_chirho
            .kinds_chirho
            .iter()
            .map(|line_chirho| KindRowViewChirho {
                kind_chirho: line_chirho.kind_chirho.clone(),
                money_in_chirho: money_chirho(line_chirho.money_in_minor_chirho),
                costs_chirho: known_chirho(line_chirho.costs_minor_chirho),
                margin_chirho: known_chirho(line_chirho.margin_minor_chirho),
                share_chirho: format!(
                    "{}.{:02}%",
                    line_chirho.share_basis_points_chirho / 100,
                    line_chirho.share_basis_points_chirho % 100
                ),
                per_job_chirho: known_chirho(line_chirho.money_per_job_minor_chirho),
            })
            .collect(),
        months_chirho: report_chirho
            .months_chirho
            .iter()
            .map(|line_chirho| MonthRowViewChirho {
                month_chirho: line_chirho.month_chirho.clone(),
                money_in_chirho: money_chirho(line_chirho.money_in_minor_chirho),
                costs_chirho: known_chirho(line_chirho.costs_minor_chirho),
                margin_chirho: known_chirho(line_chirho.margin_minor_chirho),
            })
            .collect(),
        flags_chirho: report_chirho
            .flags_chirho
            .iter()
            .map(|flag_chirho| FlagViewChirho {
                sentence_chirho: copy_chirho
                    .flag_sentence_chirho(flag_chirho.code_chirho)
                    .to_string(),
                kind_chirho: flag_chirho.kind_chirho.clone().unwrap_or_default(),
                months_chirho: flag_chirho.months_chirho.join(", "),
            })
            .collect(),
    }
}

/// Minor units with thousands separators: `1,234.50`, or `-150.00`.
pub fn money_chirho(minor_chirho: i64) -> String {
    let sign_chirho = if minor_chirho < 0 { "-" } else { "" };
    let absolute_chirho = minor_chirho.unsigned_abs();
    let whole_chirho = (absolute_chirho / 100).to_string();
    let mut grouped_chirho = String::new();
    for (index_chirho, digit_chirho) in whole_chirho.chars().enumerate() {
        if index_chirho > 0 && (whole_chirho.len() - index_chirho).is_multiple_of(3) {
            grouped_chirho.push(',');
        }
        grouped_chirho.push(digit_chirho);
    }
    format!("{sign_chirho}{grouped_chirho}.{:02}", absolute_chirho % 100)
}
