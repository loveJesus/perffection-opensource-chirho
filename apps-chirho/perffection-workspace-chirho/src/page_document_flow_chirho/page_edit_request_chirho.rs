// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Strict browser-form boundary for one private page edit or append-only restore.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.
//!
//! The authenticated Cairn route supplies only raw private request bytes. This parser accepts the
//! exact product forms, rejects duplicate or unknown fields, validates page writes, and exposes no
//! caller-selected app, viewer, workspace, state key, page bytes on restore, or publication target.

use crate::page_document_chirho::{
    PageDocumentV2Chirho, PageDocumentValidationErrorChirho, PageServiceV1Chirho,
    ValidatedPageDocumentV2Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use std::collections::BTreeMap;
use std::fmt;

pub const PAGE_EDIT_FORM_MAX_BYTES_CHIRHO: usize = 32 * 1024;
pub const PAGE_RESTORE_FORM_MAX_BYTES_CHIRHO: usize = 4 * 1024;

/// The hidden field a browser form carries the engine's caller key in, so a no-script POST can be
/// admitted on a geo-elected Cairn root, where a write needs exactly one caller idempotency key and a
/// browser cannot send Cairn's header. Workflow and contract:
/// `spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md`.
/// The field is the engine's, not hers and not ours. Every form reader drops it before reading her
/// fields, and a second copy is a duplicate field like any other. The product never reads, stores,
/// logs or re-renders its value.
pub const FORM_CALLER_KEY_FIELD_CHIRHO: &str = "idempotency_key_chirho";
/// The exact empty placeholder each app-route POST form renders once (the
/// `partials_chirho/form_caller_key_chirho.html` partial). On an elected root the engine fills each
/// occurrence with its own fresh key in the answer it sends. On a local root it arrives empty.
pub const FORM_CALLER_KEY_PLACEHOLDER_CHIRHO: &str =
    r#"<input type="hidden" name="idempotency_key_chirho" value="">"#;
/// The carrier's four refusal tokens, final and product-owned (#28751). The engine answers a refused form with a 303
/// to `#refusal-<token>`. Every page with an app-route write form pre-renders one hidden notice per token
/// (`partials_chirho/carrier_refusals_chirho.html`).
pub const CARRIER_REFUSAL_TOKENS_CHIRHO: [&str; 4] = [
    "caller-key-invalid-chirho",
    "caller-key-conflict-chirho",
    "write-unavailable-chirho",
    "write-outcome-unknown-chirho",
];

const EMPTY_BODY_CODE_CHIRHO: &str = "empty-body-chirho";
const BODY_TOO_LARGE_CODE_CHIRHO: &str = "body-too-large-chirho";
const MALFORMED_FORM_ENCODING_CODE_CHIRHO: &str = "malformed-form-encoding-chirho";
const DUPLICATE_FIELD_CODE_CHIRHO: &str = "duplicate-field-chirho";
const UNEXPECTED_FIELD_CODE_CHIRHO: &str = "unexpected-field-chirho";
const MISSING_FIELD_CODE_CHIRHO: &str = "missing-field-chirho";
const INVALID_SCHEMA_VERSION_CODE_CHIRHO: &str = "invalid-schema-version-chirho";
const INVALID_EXPECTED_VERSION_CODE_CHIRHO: &str = "invalid-expected-version-chirho";
const INVALID_RESTORE_VERSION_CODE_CHIRHO: &str = "invalid-restore-version-chirho";
const INVALID_PAGE_DOCUMENT_CODE_CHIRHO: &str = "invalid-page-document-chirho";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageEditRequestErrorChirho {
    code_chirho: &'static str,
}

impl PageEditRequestErrorChirho {
    const fn new_chirho(code_chirho: &'static str) -> Self {
        Self { code_chirho }
    }

    pub fn code_chirho(self) -> &'static str {
        self.code_chirho
    }
}

impl fmt::Display for PageEditRequestErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter_chirho.write_str(self.code_chirho)
    }
}

impl std::error::Error for PageEditRequestErrorChirho {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedPageEditRequestV1Chirho {
    document_chirho: ValidatedPageDocumentV2Chirho,
    expected_current_version_chirho: Option<u64>,
}

/// Everything one browser submission of the page form can turn out to be.
///
/// `InvalidDocumentChirho` keeps the exact submitted draft so the editor can be re-rendered with
/// the customer's own words and a notice beside each field; nothing in it is authoritative state.
/// `MalformedChirho` is a body our own form never produces (duplicate, unknown, missing, oversized
/// or undecodable fields); it carries no draft because none can be trusted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageEditFormOutcomeChirho {
    ValidChirho(ValidatedPageEditRequestV1Chirho),
    InvalidDocumentChirho {
        draft_chirho: PageDocumentV2Chirho,
        expected_current_version_chirho: Option<u64>,
        error_chirho: PageDocumentValidationErrorChirho,
    },
    MalformedChirho(PageEditRequestErrorChirho),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedPageRestoreRequestV1Chirho {
    expected_current_version_chirho: u64,
    restore_from_version_chirho: u64,
}

impl ValidatedPageRestoreRequestV1Chirho {
    pub fn expected_current_version_chirho(self) -> u64 {
        self.expected_current_version_chirho
    }

    pub fn restore_from_version_chirho(self) -> u64 {
        self.restore_from_version_chirho
    }
}

impl ValidatedPageEditRequestV1Chirho {
    pub fn document_chirho(&self) -> &ValidatedPageDocumentV2Chirho {
        &self.document_chirho
    }

    pub fn expected_current_version_chirho(&self) -> Option<u64> {
        self.expected_current_version_chirho
    }
}

/// The strict `Result` view of one page-form submission: any invalid document collapses to the
/// `invalid-page-document-chirho` code. Prefer `parse_page_edit_form_outcome_chirho` where the
/// submitted draft and its field notices must survive.
pub fn parse_page_edit_form_chirho(
    body_chirho: &[u8],
) -> Result<ValidatedPageEditRequestV1Chirho, PageEditRequestErrorChirho> {
    match parse_page_edit_form_outcome_chirho(body_chirho) {
        PageEditFormOutcomeChirho::ValidChirho(request_chirho) => Ok(request_chirho),
        PageEditFormOutcomeChirho::InvalidDocumentChirho { .. } => Err(
            PageEditRequestErrorChirho::new_chirho(INVALID_PAGE_DOCUMENT_CODE_CHIRHO),
        ),
        PageEditFormOutcomeChirho::MalformedChirho(error_chirho) => Err(error_chirho),
    }
}

/// Classify one page-form submission without losing the submitted draft.
///
/// Part of `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.
pub fn parse_page_edit_form_outcome_chirho(body_chirho: &[u8]) -> PageEditFormOutcomeChirho {
    let (document_chirho, expected_current_version_chirho) =
        match parse_page_edit_form_fields_chirho(body_chirho) {
            Ok(parsed_chirho) => parsed_chirho,
            Err(error_chirho) => return PageEditFormOutcomeChirho::MalformedChirho(error_chirho),
        };
    match document_chirho.validate_chirho() {
        Ok(validated_chirho) => {
            PageEditFormOutcomeChirho::ValidChirho(ValidatedPageEditRequestV1Chirho {
                document_chirho: validated_chirho,
                expected_current_version_chirho,
            })
        }
        Err(error_chirho) => PageEditFormOutcomeChirho::InvalidDocumentChirho {
            draft_chirho: document_chirho,
            expected_current_version_chirho,
            error_chirho,
        },
    }
}

fn parse_page_edit_form_fields_chirho(
    body_chirho: &[u8],
) -> Result<(PageDocumentV2Chirho, Option<u64>), PageEditRequestErrorChirho> {
    let mut fields_chirho = parse_form_fields_chirho(body_chirho, PAGE_EDIT_FORM_MAX_BYTES_CHIRHO)?;

    let schema_version_chirho =
        take_required_field_chirho(&mut fields_chirho, "schema_version_chirho")?
            .parse::<u16>()
            .map_err(|_error_chirho| {
                PageEditRequestErrorChirho::new_chirho(INVALID_SCHEMA_VERSION_CODE_CHIRHO)
            })?;
    if schema_version_chirho != PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO {
        return Err(PageEditRequestErrorChirho::new_chirho(
            INVALID_SCHEMA_VERSION_CODE_CHIRHO,
        ));
    }

    let expected_version_chirho =
        take_required_field_chirho(&mut fields_chirho, "expected_version_chirho")?
            .parse::<u64>()
            .map_err(|_error_chirho| {
                PageEditRequestErrorChirho::new_chirho(INVALID_EXPECTED_VERSION_CODE_CHIRHO)
            })?;
    let expected_current_version_chirho =
        (expected_version_chirho != 0).then_some(expected_version_chirho);

    let document_chirho = PageDocumentV2Chirho {
        schema_version_chirho,
        business_name_chirho: take_required_field_chirho(
            &mut fields_chirho,
            "business_name_chirho",
        )?,
        headline_chirho: take_required_field_chirho(&mut fields_chirho, "headline_chirho")?,
        summary_chirho: take_required_field_chirho(&mut fields_chirho, "summary_chirho")?,
        services_chirho: [
            take_service_chirho(&mut fields_chirho, 1)?,
            take_service_chirho(&mut fields_chirho, 2)?,
            take_service_chirho(&mut fields_chirho, 3)?,
        ],
        contact_heading_chirho: take_required_field_chirho(
            &mut fields_chirho,
            "contact_heading_chirho",
        )?,
        contact_phone_chirho: take_required_field_chirho(
            &mut fields_chirho,
            "contact_phone_chirho",
        )?,
        contact_email_chirho: take_required_field_chirho(
            &mut fields_chirho,
            "contact_email_chirho",
        )?,
        contact_details_chirho: take_required_field_chirho(
            &mut fields_chirho,
            "contact_details_chirho",
        )?,
    };

    if !fields_chirho.is_empty() {
        return Err(PageEditRequestErrorChirho::new_chirho(
            UNEXPECTED_FIELD_CODE_CHIRHO,
        ));
    }
    Ok((document_chirho, expected_current_version_chirho))
}

pub fn parse_page_restore_form_chirho(
    body_chirho: &[u8],
) -> Result<ValidatedPageRestoreRequestV1Chirho, PageEditRequestErrorChirho> {
    let mut fields_chirho =
        parse_form_fields_chirho(body_chirho, PAGE_RESTORE_FORM_MAX_BYTES_CHIRHO)?;
    let expected_current_version_chirho =
        take_required_field_chirho(&mut fields_chirho, "expected_version_chirho")?
            .parse::<u64>()
            .map_err(|_error_chirho| {
                PageEditRequestErrorChirho::new_chirho(INVALID_EXPECTED_VERSION_CODE_CHIRHO)
            })?;
    if expected_current_version_chirho == 0 {
        return Err(PageEditRequestErrorChirho::new_chirho(
            INVALID_EXPECTED_VERSION_CODE_CHIRHO,
        ));
    }

    let restore_from_version_chirho =
        take_required_field_chirho(&mut fields_chirho, "restore_from_version_chirho")?
            .parse::<u64>()
            .map_err(|_error_chirho| {
                PageEditRequestErrorChirho::new_chirho(INVALID_RESTORE_VERSION_CODE_CHIRHO)
            })?;
    if restore_from_version_chirho == 0
        || restore_from_version_chirho >= expected_current_version_chirho
    {
        return Err(PageEditRequestErrorChirho::new_chirho(
            INVALID_RESTORE_VERSION_CODE_CHIRHO,
        ));
    }
    if !fields_chirho.is_empty() {
        return Err(PageEditRequestErrorChirho::new_chirho(
            UNEXPECTED_FIELD_CODE_CHIRHO,
        ));
    }

    Ok(ValidatedPageRestoreRequestV1Chirho {
        expected_current_version_chirho,
        restore_from_version_chirho,
    })
}

fn take_service_chirho(
    fields_chirho: &mut BTreeMap<String, String>,
    service_number_chirho: usize,
) -> Result<PageServiceV1Chirho, PageEditRequestErrorChirho> {
    Ok(PageServiceV1Chirho {
        title_chirho: take_required_field_chirho(
            fields_chirho,
            &format!("service_{service_number_chirho}_title_chirho"),
        )?,
        body_chirho: take_required_field_chirho(
            fields_chirho,
            &format!("service_{service_number_chirho}_body_chirho"),
        )?,
    })
}

fn take_required_field_chirho(
    fields_chirho: &mut BTreeMap<String, String>,
    field_name_chirho: &str,
) -> Result<String, PageEditRequestErrorChirho> {
    fields_chirho
        .remove(field_name_chirho)
        .ok_or_else(|| PageEditRequestErrorChirho::new_chirho(MISSING_FIELD_CODE_CHIRHO))
}

pub(crate) fn parse_form_fields_chirho(
    body_chirho: &[u8],
    max_bytes_chirho: usize,
) -> Result<BTreeMap<String, String>, PageEditRequestErrorChirho> {
    if body_chirho.is_empty() {
        return Err(PageEditRequestErrorChirho::new_chirho(
            EMPTY_BODY_CODE_CHIRHO,
        ));
    }
    if body_chirho.len() > max_bytes_chirho {
        return Err(PageEditRequestErrorChirho::new_chirho(
            BODY_TOO_LARGE_CODE_CHIRHO,
        ));
    }
    let body_text_chirho = std::str::from_utf8(body_chirho).map_err(|_error_chirho| {
        PageEditRequestErrorChirho::new_chirho(MALFORMED_FORM_ENCODING_CODE_CHIRHO)
    })?;
    let mut fields_chirho = BTreeMap::new();
    for pair_chirho in body_text_chirho.split('&') {
        let (raw_key_chirho, raw_value_chirho) = pair_chirho.split_once('=').ok_or_else(|| {
            PageEditRequestErrorChirho::new_chirho(MALFORMED_FORM_ENCODING_CODE_CHIRHO)
        })?;
        let key_chirho = decode_form_component_chirho(raw_key_chirho)?;
        let value_chirho = decode_form_component_chirho(raw_value_chirho)?;
        if fields_chirho.insert(key_chirho, value_chirho).is_some() {
            return Err(PageEditRequestErrorChirho::new_chirho(
                DUPLICATE_FIELD_CODE_CHIRHO,
            ));
        }
    }
    // The engine's caller key (FORM_CALLER_KEY_FIELD_CHIRHO): at most once, by the duplicate check
    // above, then dropped unread, so every form reader sees only her fields. Workflow:
    // `spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md`.
    fields_chirho.remove(FORM_CALLER_KEY_FIELD_CHIRHO);
    Ok(fields_chirho)
}

fn decode_form_component_chirho(raw_chirho: &str) -> Result<String, PageEditRequestErrorChirho> {
    let raw_bytes_chirho = raw_chirho.as_bytes();
    let mut decoded_chirho = Vec::with_capacity(raw_bytes_chirho.len());
    let mut index_chirho = 0usize;
    while index_chirho < raw_bytes_chirho.len() {
        match raw_bytes_chirho[index_chirho] {
            b'+' => {
                decoded_chirho.push(b' ');
                index_chirho += 1;
            }
            b'%' => {
                let high_chirho = raw_bytes_chirho
                    .get(index_chirho + 1)
                    .and_then(|byte_chirho| decode_hex_chirho(*byte_chirho));
                let low_chirho = raw_bytes_chirho
                    .get(index_chirho + 2)
                    .and_then(|byte_chirho| decode_hex_chirho(*byte_chirho));
                let (Some(high_chirho), Some(low_chirho)) = (high_chirho, low_chirho) else {
                    return Err(PageEditRequestErrorChirho::new_chirho(
                        MALFORMED_FORM_ENCODING_CODE_CHIRHO,
                    ));
                };
                decoded_chirho.push((high_chirho << 4) | low_chirho);
                index_chirho += 3;
            }
            byte_chirho => {
                decoded_chirho.push(byte_chirho);
                index_chirho += 1;
            }
        }
    }
    String::from_utf8(decoded_chirho).map_err(|_error_chirho| {
        PageEditRequestErrorChirho::new_chirho(MALFORMED_FORM_ENCODING_CODE_CHIRHO)
    })
}

fn decode_hex_chirho(byte_chirho: u8) -> Option<u8> {
    match byte_chirho {
        b'0'..=b'9' => Some(byte_chirho - b'0'),
        b'a'..=b'f' => Some(byte_chirho - b'a' + 10),
        b'A'..=b'F' => Some(byte_chirho - b'A' + 10),
        _ => None,
    }
}
