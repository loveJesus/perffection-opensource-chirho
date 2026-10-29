// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Bounded document accepted by the first Perffection page editor.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use serde::{Deserialize, Serialize};
use std::fmt;

pub const PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO: u16 = 2;
pub const PAGE_SERVICE_SLOT_COUNT_CHIRHO: usize = 3;
pub const PAGE_DOCUMENT_MAX_SERIALIZED_BYTES_CHIRHO: usize = 8 * 1024;

pub const BUSINESS_NAME_MAX_CHARACTERS_CHIRHO: usize = 80;
const BUSINESS_NAME_MAX_BYTES_CHIRHO: usize = 320;
pub const HEADLINE_MAX_CHARACTERS_CHIRHO: usize = 140;
const HEADLINE_MAX_BYTES_CHIRHO: usize = 560;
pub const SUMMARY_MAX_CHARACTERS_CHIRHO: usize = 600;
const SUMMARY_MAX_BYTES_CHIRHO: usize = 2_400;
pub const SERVICE_TITLE_MAX_CHARACTERS_CHIRHO: usize = 72;
const SERVICE_TITLE_MAX_BYTES_CHIRHO: usize = 288;
pub const SERVICE_BODY_MAX_CHARACTERS_CHIRHO: usize = 320;
const SERVICE_BODY_MAX_BYTES_CHIRHO: usize = 1_280;
pub const CONTACT_HEADING_MAX_CHARACTERS_CHIRHO: usize = 80;
const CONTACT_HEADING_MAX_BYTES_CHIRHO: usize = 320;
pub const CONTACT_PHONE_MAX_CHARACTERS_CHIRHO: usize = 40;
const CONTACT_PHONE_MAX_BYTES_CHIRHO: usize = 160;
pub const CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO: usize = 254;
const CONTACT_EMAIL_MAX_BYTES_CHIRHO: usize = 254;
pub const CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO: usize = 400;
const CONTACT_DETAILS_MAX_BYTES_CHIRHO: usize = 1_600;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct PageServiceV1Chirho {
    pub title_chirho: String,
    pub body_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PageDocumentV2Chirho {
    pub schema_version_chirho: u16,
    pub business_name_chirho: String,
    pub headline_chirho: String,
    pub summary_chirho: String,
    pub services_chirho: [PageServiceV1Chirho; PAGE_SERVICE_SLOT_COUNT_CHIRHO],
    pub contact_heading_chirho: String,
    pub contact_phone_chirho: String,
    pub contact_email_chirho: String,
    pub contact_details_chirho: String,
}

impl Default for PageDocumentV2Chirho {
    fn default() -> Self {
        Self {
            schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
            business_name_chirho: String::new(),
            headline_chirho: String::new(),
            summary_chirho: String::new(),
            services_chirho: std::array::from_fn(|_index_chirho| PageServiceV1Chirho::default()),
            contact_heading_chirho: String::new(),
            contact_phone_chirho: String::new(),
            contact_email_chirho: String::new(),
            contact_details_chirho: String::new(),
        }
    }
}

impl PageDocumentV2Chirho {
    pub fn validate_chirho(
        &self,
    ) -> Result<ValidatedPageDocumentV2Chirho, PageDocumentValidationErrorChirho> {
        let mut issues_chirho = Vec::new();

        if self.schema_version_chirho != PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO {
            issues_chirho.push(PageValidationIssueChirho::new_chirho(
                "schema_version_chirho",
                PageValidationCodeChirho::UnsupportedSchemaChirho,
            ));
        }

        let business_name_chirho = validate_text_chirho(
            &self.business_name_chirho,
            "business_name_chirho",
            BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
            BUSINESS_NAME_MAX_BYTES_CHIRHO,
            true,
            &mut issues_chirho,
        );
        let headline_chirho = validate_text_chirho(
            &self.headline_chirho,
            "headline_chirho",
            HEADLINE_MAX_CHARACTERS_CHIRHO,
            HEADLINE_MAX_BYTES_CHIRHO,
            true,
            &mut issues_chirho,
        );
        let summary_chirho = validate_text_chirho(
            &self.summary_chirho,
            "summary_chirho",
            SUMMARY_MAX_CHARACTERS_CHIRHO,
            SUMMARY_MAX_BYTES_CHIRHO,
            true,
            &mut issues_chirho,
        );
        let contact_heading_chirho = validate_text_chirho(
            &self.contact_heading_chirho,
            "contact_heading_chirho",
            CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
            CONTACT_HEADING_MAX_BYTES_CHIRHO,
            true,
            &mut issues_chirho,
        );
        let normalized_contact_phone_chirho = normalize_text_chirho(&self.contact_phone_chirho);
        let normalized_contact_email_chirho = normalize_text_chirho(&self.contact_email_chirho);
        if normalized_contact_phone_chirho.is_empty() && normalized_contact_email_chirho.is_empty()
        {
            issues_chirho.push(PageValidationIssueChirho::new_chirho(
                "contact_chirho",
                PageValidationCodeChirho::ContactMethodRequiredChirho,
            ));
        }
        let contact_phone_chirho =
            validate_phone_contact_chirho(normalized_contact_phone_chirho, &mut issues_chirho);
        let contact_email_chirho =
            validate_email_contact_chirho(normalized_contact_email_chirho, &mut issues_chirho);
        let contact_details_chirho = validate_text_chirho(
            &self.contact_details_chirho,
            "contact_details_chirho",
            CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
            CONTACT_DETAILS_MAX_BYTES_CHIRHO,
            false,
            &mut issues_chirho,
        );

        let mut services_chirho = Vec::with_capacity(PAGE_SERVICE_SLOT_COUNT_CHIRHO);
        for (service_index_chirho, service_chirho) in self.services_chirho.iter().enumerate() {
            let title_path_chirho = format!("services_chirho[{service_index_chirho}].title_chirho");
            let body_path_chirho = format!("services_chirho[{service_index_chirho}].body_chirho");
            let normalized_title_chirho = normalize_text_chirho(&service_chirho.title_chirho);
            let normalized_body_chirho = normalize_text_chirho(&service_chirho.body_chirho);

            if normalized_title_chirho.is_empty() && normalized_body_chirho.is_empty() {
                continue;
            }

            let title_chirho = validate_normalized_text_chirho(
                normalized_title_chirho,
                &title_path_chirho,
                SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
                SERVICE_TITLE_MAX_BYTES_CHIRHO,
                true,
                &mut issues_chirho,
            );
            let body_chirho = validate_normalized_text_chirho(
                normalized_body_chirho,
                &body_path_chirho,
                SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
                SERVICE_BODY_MAX_BYTES_CHIRHO,
                true,
                &mut issues_chirho,
            );
            services_chirho.push(ValidatedPageServiceV1Chirho {
                title_chirho,
                body_chirho,
            });
        }

        if services_chirho.is_empty() {
            issues_chirho.push(PageValidationIssueChirho::new_chirho(
                "services_chirho",
                PageValidationCodeChirho::RequiredChirho,
            ));
        }

        let validated_chirho = ValidatedPageDocumentV2Chirho {
            schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
            business_name_chirho,
            headline_chirho,
            summary_chirho,
            services_chirho,
            contact_heading_chirho,
            contact_phone_chirho,
            contact_email_chirho,
            contact_details_chirho,
        };

        let has_oversized_field_chirho = issues_chirho.iter().any(|issue_chirho| {
            matches!(
                issue_chirho.code_chirho,
                PageValidationCodeChirho::TooManyCharactersChirho
                    | PageValidationCodeChirho::TooManyBytesChirho
            )
        });
        if !has_oversized_field_chirho {
            let serialized_size_chirho = serde_json::to_vec(&validated_chirho.to_document_chirho())
                .expect("the validated page document is always JSON serializable")
                .len();
            if serialized_size_chirho > PAGE_DOCUMENT_MAX_SERIALIZED_BYTES_CHIRHO {
                issues_chirho.push(PageValidationIssueChirho::new_chirho(
                    "document_chirho",
                    PageValidationCodeChirho::DocumentTooLargeChirho,
                ));
            }
        }

        if issues_chirho.is_empty() {
            Ok(validated_chirho)
        } else {
            Err(PageDocumentValidationErrorChirho { issues_chirho })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedPageContactActionV2Chirho {
    pub(crate) display_chirho: String,
    pub(crate) href_chirho: String,
}

impl ValidatedPageContactActionV2Chirho {
    pub fn display_chirho(&self) -> &str {
        &self.display_chirho
    }

    pub fn href_chirho(&self) -> &str {
        &self.href_chirho
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedPageServiceV1Chirho {
    pub(crate) title_chirho: String,
    pub(crate) body_chirho: String,
}

impl ValidatedPageServiceV1Chirho {
    pub fn title_chirho(&self) -> &str {
        &self.title_chirho
    }

    pub fn body_chirho(&self) -> &str {
        &self.body_chirho
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidatedPageDocumentV2Chirho {
    pub(crate) schema_version_chirho: u16,
    pub(crate) business_name_chirho: String,
    pub(crate) headline_chirho: String,
    pub(crate) summary_chirho: String,
    pub(crate) services_chirho: Vec<ValidatedPageServiceV1Chirho>,
    pub(crate) contact_heading_chirho: String,
    pub(crate) contact_phone_chirho: Option<ValidatedPageContactActionV2Chirho>,
    pub(crate) contact_email_chirho: Option<ValidatedPageContactActionV2Chirho>,
    pub(crate) contact_details_chirho: String,
}

impl ValidatedPageDocumentV2Chirho {
    pub fn schema_version_chirho(&self) -> u16 {
        self.schema_version_chirho
    }

    pub fn business_name_chirho(&self) -> &str {
        &self.business_name_chirho
    }

    pub fn headline_chirho(&self) -> &str {
        &self.headline_chirho
    }

    pub fn summary_chirho(&self) -> &str {
        &self.summary_chirho
    }

    pub fn services_chirho(&self) -> &[ValidatedPageServiceV1Chirho] {
        &self.services_chirho
    }

    pub fn contact_heading_chirho(&self) -> &str {
        &self.contact_heading_chirho
    }

    pub fn contact_phone_chirho(&self) -> Option<&ValidatedPageContactActionV2Chirho> {
        self.contact_phone_chirho.as_ref()
    }

    pub fn contact_email_chirho(&self) -> Option<&ValidatedPageContactActionV2Chirho> {
        self.contact_email_chirho.as_ref()
    }

    pub fn contact_details_chirho(&self) -> &str {
        &self.contact_details_chirho
    }

    pub fn to_document_chirho(&self) -> PageDocumentV2Chirho {
        let mut services_chirho =
            std::array::from_fn(|_index_chirho| PageServiceV1Chirho::default());
        for (service_index_chirho, service_chirho) in self.services_chirho.iter().enumerate() {
            services_chirho[service_index_chirho] = PageServiceV1Chirho {
                title_chirho: service_chirho.title_chirho.clone(),
                body_chirho: service_chirho.body_chirho.clone(),
            };
        }

        PageDocumentV2Chirho {
            schema_version_chirho: self.schema_version_chirho,
            business_name_chirho: self.business_name_chirho.clone(),
            headline_chirho: self.headline_chirho.clone(),
            summary_chirho: self.summary_chirho.clone(),
            services_chirho,
            contact_heading_chirho: self.contact_heading_chirho.clone(),
            contact_phone_chirho: self
                .contact_phone_chirho
                .as_ref()
                .map(|contact_chirho| contact_chirho.display_chirho.clone())
                .unwrap_or_default(),
            contact_email_chirho: self
                .contact_email_chirho
                .as_ref()
                .map(|contact_chirho| contact_chirho.display_chirho.clone())
                .unwrap_or_default(),
            contact_details_chirho: self.contact_details_chirho.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageValidationCodeChirho {
    RequiredChirho,
    ContactMethodRequiredChirho,
    InvalidPhoneChirho,
    InvalidEmailChirho,
    TooManyCharactersChirho,
    TooManyBytesChirho,
    UnsupportedControlCharacterChirho,
    UnsupportedSchemaChirho,
    DocumentTooLargeChirho,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageValidationIssueChirho {
    pub field_path_chirho: String,
    pub code_chirho: PageValidationCodeChirho,
}

impl PageValidationIssueChirho {
    fn new_chirho(field_path_chirho: &str, code_chirho: PageValidationCodeChirho) -> Self {
        Self {
            field_path_chirho: field_path_chirho.to_string(),
            code_chirho,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageDocumentValidationErrorChirho {
    pub issues_chirho: Vec<PageValidationIssueChirho>,
}

impl fmt::Display for PageDocumentValidationErrorChirho {
    fn fmt(&self, formatter_chirho: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter_chirho,
            "page document has {} validation issue(s)",
            self.issues_chirho.len()
        )
    }
}

impl std::error::Error for PageDocumentValidationErrorChirho {}

fn validate_phone_contact_chirho(
    value_chirho: String,
    issues_chirho: &mut Vec<PageValidationIssueChirho>,
) -> Option<ValidatedPageContactActionV2Chirho> {
    let issue_count_before_chirho = issues_chirho.len();
    let display_chirho = validate_normalized_text_chirho(
        value_chirho,
        "contact_phone_chirho",
        CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
        CONTACT_PHONE_MAX_BYTES_CHIRHO,
        false,
        issues_chirho,
    );
    if display_chirho.is_empty() {
        return None;
    }

    let mut target_chirho = String::with_capacity(display_chirho.len());
    let mut digit_count_chirho = 0usize;
    let mut invalid_chirho = false;
    for (character_index_chirho, character_chirho) in display_chirho.chars().enumerate() {
        if character_chirho.is_ascii_digit() {
            target_chirho.push(character_chirho);
            digit_count_chirho += 1;
        } else if character_chirho == '+' && character_index_chirho == 0 {
            target_chirho.push(character_chirho);
        } else if !matches!(character_chirho, ' ' | '-' | '(' | ')' | '.') {
            invalid_chirho = true;
        }
    }
    if !(7..=15).contains(&digit_count_chirho) {
        invalid_chirho = true;
    }
    if invalid_chirho {
        issues_chirho.push(PageValidationIssueChirho::new_chirho(
            "contact_phone_chirho",
            PageValidationCodeChirho::InvalidPhoneChirho,
        ));
    }

    (issues_chirho.len() == issue_count_before_chirho).then(|| ValidatedPageContactActionV2Chirho {
        display_chirho,
        href_chirho: format!("tel:{target_chirho}"),
    })
}

fn validate_email_contact_chirho(
    value_chirho: String,
    issues_chirho: &mut Vec<PageValidationIssueChirho>,
) -> Option<ValidatedPageContactActionV2Chirho> {
    let issue_count_before_chirho = issues_chirho.len();
    let display_chirho = validate_normalized_text_chirho(
        value_chirho,
        "contact_email_chirho",
        CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
        CONTACT_EMAIL_MAX_BYTES_CHIRHO,
        false,
        issues_chirho,
    );
    if display_chirho.is_empty() {
        return None;
    }

    if !valid_email_address_chirho(&display_chirho) {
        issues_chirho.push(PageValidationIssueChirho::new_chirho(
            "contact_email_chirho",
            PageValidationCodeChirho::InvalidEmailChirho,
        ));
    }

    (issues_chirho.len() == issue_count_before_chirho).then(|| ValidatedPageContactActionV2Chirho {
        href_chirho: format!("mailto:{display_chirho}"),
        display_chirho,
    })
}

fn valid_email_address_chirho(value_chirho: &str) -> bool {
    if !value_chirho.is_ascii() {
        return false;
    }
    let Some((local_chirho, domain_chirho)) = value_chirho.split_once('@') else {
        return false;
    };
    if local_chirho.is_empty()
        || local_chirho.len() > 64
        || local_chirho.starts_with('.')
        || local_chirho.ends_with('.')
        || local_chirho.contains("..")
        || domain_chirho.contains('@')
        || !local_chirho.chars().all(|character_chirho| {
            character_chirho.is_ascii_alphanumeric()
                || matches!(character_chirho, '.' | '_' | '+' | '-')
        })
    {
        return false;
    }

    let labels_chirho = domain_chirho.split('.').collect::<Vec<_>>();
    labels_chirho.len() >= 2
        && labels_chirho.iter().all(|label_chirho| {
            !label_chirho.is_empty()
                && label_chirho.len() <= 63
                && label_chirho
                    .chars()
                    .next()
                    .is_some_and(|character_chirho| character_chirho.is_ascii_alphanumeric())
                && label_chirho
                    .chars()
                    .last()
                    .is_some_and(|character_chirho| character_chirho.is_ascii_alphanumeric())
                && label_chirho.chars().all(|character_chirho| {
                    character_chirho.is_ascii_alphanumeric() || character_chirho == '-'
                })
        })
}

fn validate_text_chirho(
    value_chirho: &str,
    field_path_chirho: &str,
    max_characters_chirho: usize,
    max_bytes_chirho: usize,
    required_chirho: bool,
    issues_chirho: &mut Vec<PageValidationIssueChirho>,
) -> String {
    validate_normalized_text_chirho(
        normalize_text_chirho(value_chirho),
        field_path_chirho,
        max_characters_chirho,
        max_bytes_chirho,
        required_chirho,
        issues_chirho,
    )
}

fn validate_normalized_text_chirho(
    value_chirho: String,
    field_path_chirho: &str,
    max_characters_chirho: usize,
    max_bytes_chirho: usize,
    required_chirho: bool,
    issues_chirho: &mut Vec<PageValidationIssueChirho>,
) -> String {
    if required_chirho && value_chirho.is_empty() {
        issues_chirho.push(PageValidationIssueChirho::new_chirho(
            field_path_chirho,
            PageValidationCodeChirho::RequiredChirho,
        ));
    }
    if value_chirho.chars().count() > max_characters_chirho {
        issues_chirho.push(PageValidationIssueChirho::new_chirho(
            field_path_chirho,
            PageValidationCodeChirho::TooManyCharactersChirho,
        ));
    }
    if value_chirho.len() > max_bytes_chirho {
        issues_chirho.push(PageValidationIssueChirho::new_chirho(
            field_path_chirho,
            PageValidationCodeChirho::TooManyBytesChirho,
        ));
    }
    if value_chirho
        .chars()
        .any(|character_chirho| character_chirho.is_control() && character_chirho != '\n')
    {
        issues_chirho.push(PageValidationIssueChirho::new_chirho(
            field_path_chirho,
            PageValidationCodeChirho::UnsupportedControlCharacterChirho,
        ));
    }
    value_chirho
}

fn normalize_text_chirho(value_chirho: &str) -> String {
    value_chirho
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .trim()
        .to_string()
}
