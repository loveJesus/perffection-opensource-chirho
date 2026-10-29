// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures form, read strictly, and the one save decision both branches of the save DAG share.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. The review and the
//! save post the same four fields: the pasted CSV, the currency, her note and the version she saw (0
//! when nothing was saved). Any other field, a repeat, a version that is not a whole number, or a
//! body past the bound makes the form unreadable. The save re-reads the exact reviewed CSV: the same
//! bytes give the same figures, so what she reviewed is what is stored.
//!
//! The review also reads her figures as an uploaded file (`figures_upload_chirho`): one strict grammar
//! turns the upload into these same four fields, so everything after the read is the paste's path.
//! The save reads only the pasted form: its Save button posts the reviewed fields, never a file.

use crate::figures_flow_chirho::figures_csv_chirho::{
    import_figures_csv_chirho, FiguresImportChirho,
};
use crate::figures_flow_chirho::figures_document_chirho::FiguresDocumentV1Chirho;
use crate::figures_flow_chirho::figures_upload_chirho::parse_figures_upload_chirho;
use crate::page_edit_request_chirho::parse_form_fields_chirho;
use crate::viewer_state_chirho::figures_contract_chirho::build_figures_write_request_chirho;

/// 64 KiB of CSV percent-encoded at worst three times over, plus the other fields. The same bound
/// holds for a whole upload.
pub const FIGURES_FORM_MAX_BYTES_CHIRHO: usize = 200 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiguresFormChirho {
    pub csv_chirho: String,
    pub currency_chirho: String,
    pub note_chirho: String,
    /// The version she saw when she reviewed; 0 when nothing was saved yet.
    pub expected_version_chirho: u64,
    /// The name of the file she chose, when her figures came as a file. Display only: never a path,
    /// never saved, never trusted for what the file holds.
    pub file_name_chirho: Option<String>,
}

/// Why a figures form gives no figures to review: each is her sentence, by code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiguresFormRefusalChirho {
    /// Framing, headers, or fields not as the form's grammar requires.
    UnreadableChirho,
    /// A chosen file and pasted figures together: she picks one, nothing is chosen for her.
    FileAndPasteChirho,
    /// A chosen file with nothing in it.
    EmptyFileChirho,
    /// A chosen file that is not UTF-8 text.
    FileNotTextChirho,
    /// A CSV source over 64 KiB, or a whole form over 200 KiB, refused before it is read.
    TooLargeChirho,
}

impl FiguresFormRefusalChirho {
    pub fn code_chirho(self) -> &'static str {
        match self {
            Self::UnreadableChirho => "unreadable-form-chirho",
            Self::FileAndPasteChirho => "file-and-paste-chirho",
            Self::EmptyFileChirho => "empty-file-chirho",
            Self::FileNotTextChirho => "file-not-text-chirho",
            Self::TooLargeChirho => "too-large-chirho",
        }
    }
}

pub fn parse_figures_form_chirho(body_chirho: &[u8]) -> Option<FiguresFormChirho> {
    let mut fields_chirho =
        parse_form_fields_chirho(body_chirho, FIGURES_FORM_MAX_BYTES_CHIRHO).ok()?;
    let mut take_chirho = |name_chirho: &str| fields_chirho.remove(name_chirho);
    let form_chirho = FiguresFormChirho {
        csv_chirho: take_chirho("csv_chirho")?,
        currency_chirho: take_chirho("currency_chirho")?.trim().to_string(),
        note_chirho: take_chirho("note_chirho")?.trim().to_string(),
        expected_version_chirho: expected_version_from_chirho(&take_chirho(
            "expected_version_chirho",
        )?)?,
        file_name_chirho: None,
    };
    fields_chirho.is_empty().then_some(form_chirho)
}

/// The version field, the same in both forms: 1 to 19 ASCII digits, nothing else.
pub fn expected_version_from_chirho(version_chirho: &str) -> Option<u64> {
    if version_chirho.is_empty()
        || version_chirho.len() > 19
        || !version_chirho
            .bytes()
            .all(|byte_chirho| byte_chirho.is_ascii_digit())
    {
        return None;
    }
    version_chirho.parse().ok()
}

/// What the review reads: an upload (its body opens with a multipart delimiter, `--`) through the one
/// upload grammar, and anything else as the pasted form, exactly as before. A pasted form never opens
/// with `--`: it opens with one of the form's field names.
pub fn read_figures_review_form_chirho(
    body_chirho: &[u8],
) -> Result<FiguresFormChirho, FiguresFormRefusalChirho> {
    if body_chirho.starts_with(b"--") {
        parse_figures_upload_chirho(body_chirho)
    } else {
        parse_figures_form_chirho(body_chirho).ok_or(FiguresFormRefusalChirho::UnreadableChirho)
    }
}

impl FiguresFormChirho {
    pub fn import_chirho(&self) -> FiguresImportChirho {
        import_figures_csv_chirho(&self.csv_chirho, &self.currency_chirho, &self.note_chirho)
    }
}

/// The save's one decision: the strict write request, or why nothing will be written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiguresSaveDecisionChirho {
    WriteChirho(Vec<u8>),
    /// The form could not be read at all.
    UnreadableChirho,
    /// The form was read, but nothing can be kept from it: the review again, with every reason.
    RefusedChirho(Box<(FiguresFormChirho, FiguresImportChirho)>),
}

pub fn classify_figures_save_chirho(body_chirho: &[u8]) -> FiguresSaveDecisionChirho {
    let Some(form_chirho) = parse_figures_form_chirho(body_chirho) else {
        return FiguresSaveDecisionChirho::UnreadableChirho;
    };
    let import_chirho = form_chirho.import_chirho();
    let expected_chirho =
        (form_chirho.expected_version_chirho > 0).then_some(form_chirho.expected_version_chirho);
    match import_chirho
        .kept_chirho
        .as_ref()
        .ok()
        .map(|kept_chirho| build_figures_write_request_chirho(kept_chirho, expected_chirho))
    {
        Some(Ok(request_chirho)) => FiguresSaveDecisionChirho::WriteChirho(request_chirho),
        _ => FiguresSaveDecisionChirho::RefusedChirho(Box::new((form_chirho, import_chirho))),
    }
}

/// Her figures back as the CSV she would paste: the header, then one line per figure she gave, months
/// in order and kinds in her order. A kind with a comma or a quote is quoted. Reading it back gives
/// the same figures.
pub fn figures_to_csv_chirho(document_chirho: &FiguresDocumentV1Chirho) -> String {
    let mut csv_chirho = String::from("month,kind,money_in,costs,jobs\n");
    for (month_index_chirho, month_chirho) in document_chirho.months_chirho.iter().enumerate() {
        for (kind_index_chirho, kind_chirho) in document_chirho.kinds_chirho.iter().enumerate() {
            let Some(money_in_chirho) =
                document_chirho.money_in_chirho[kind_index_chirho][month_index_chirho]
            else {
                continue;
            };
            let kind_field_chirho = if kind_chirho.contains([',', '"']) {
                format!("\"{}\"", kind_chirho.replace('"', "\"\""))
            } else {
                kind_chirho.clone()
            };
            let costs_chirho = document_chirho.costs_chirho[kind_index_chirho][month_index_chirho]
                .map(plain_amount_chirho)
                .unwrap_or_default();
            let jobs_chirho = document_chirho.jobs_chirho[kind_index_chirho][month_index_chirho]
                .map(|jobs_chirho| jobs_chirho.to_string())
                .unwrap_or_default();
            csv_chirho.push_str(&format!(
                "{month_chirho},{kind_field_chirho},{},{costs_chirho},{jobs_chirho}\n",
                plain_amount_chirho(money_in_chirho)
            ));
        }
    }
    csv_chirho
}

/// Minor units as the plain decimal the importer reads: `1200.50`.
pub fn plain_amount_chirho(minor_chirho: i64) -> String {
    format!("{}.{:02}", minor_chirho / 100, minor_chirho % 100)
}
