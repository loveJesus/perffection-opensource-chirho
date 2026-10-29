// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The surface, described for an agent: routes, the page document's fields with the validator's
//! own limits, the rules a complete page meets, how a save answers, and one fictional sample.
//!
//! Every number here is the validator's constant, so the description cannot drift from what a save
//! accepts; the field paths are the very paths a refusal names.

use crate::agent_save_chirho::{AgentSaveRefusalChirho, AGENT_SAVE_MAX_BODY_BYTES_CHIRHO};
use crate::first_draft_chirho::draft_candidate_from_completion_chirho;
use crate::page_document_chirho::PageValidationCodeChirho;
use crate::page_document_chirho::{
    PageDocumentV2Chirho, BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
    CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO, CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
    CONTACT_HEADING_MAX_CHARACTERS_CHIRHO, CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
    HEADLINE_MAX_CHARACTERS_CHIRHO, PAGE_DOCUMENT_MAX_SERIALIZED_BYTES_CHIRHO,
    PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO, PAGE_SERVICE_SLOT_COUNT_CHIRHO,
    SERVICE_BODY_MAX_CHARACTERS_CHIRHO, SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
    SUMMARY_MAX_CHARACTERS_CHIRHO,
};
use crate::workspace_page_chirho::{
    WORKSPACE_PREVIEW_HREF_CHIRHO, WORKSPACE_PUBLICATION_HREF_CHIRHO,
};
use perffection_page_template_chirho::page_document_guide_chirho::OWN_AGENT_SAMPLE_ANSWER_CHIRHO;
use serde::Serialize;

pub const AGENT_SURFACE_SCHEMA_CHIRHO: &str = "perffection-agent-surface-v1-chirho";
pub const AGENT_PAGE_PATH_CHIRHO: &str = "/app-chirho/perffection-workspace/agent-page-chirho";
pub const AGENT_SAVE_PATH_CHIRHO: &str = "/app-chirho/perffection-workspace/agent-save-page-chirho";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentRouteV1Chirho {
    pub method_chirho: &'static str,
    pub path_chirho: &'static str,
    pub sends_chirho: &'static str,
    pub answers_chirho: &'static str,
    pub purpose_chirho: &'static str,
}

/// When a field must be filled for the page to be complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AgentFieldNeedChirho {
    /// Always required.
    #[serde(rename = "required-chirho")]
    RequiredChirho,
    /// A page needs at least one service; in any service that is used, the title and the body
    /// are both required. Unused slots stay empty.
    #[serde(rename = "one-service-required-chirho")]
    OneServiceRequiredChirho,
    /// At least one of the phone and the email is required.
    #[serde(rename = "phone-or-email-chirho")]
    PhoneOrEmailChirho,
    #[serde(rename = "optional-chirho")]
    OptionalChirho,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentFieldV1Chirho {
    /// The path a refusal names; `N` stands for a service slot, 0-based.
    pub field_path_chirho: &'static str,
    pub max_characters_chirho: usize,
    pub need_chirho: AgentFieldNeedChirho,
    /// True for the owner's own contact values: copy them exactly as she gave them, never invented,
    /// reworded or reformatted. False means the agent may word the field itself. Whatever is sent,
    /// a value that is not valid is refused by name, never dropped or rewritten.
    pub copy_exactly_from_owner_chirho: bool,
}

/// A code an agent can meet, with what it means. The description explains itself: a real agent,
/// given only the address and a key, had to guess these (2026-09-21).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentMeaningV1Chirho {
    pub code_chirho: String,
    pub meaning_chirho: &'static str,
    /// Where the owner goes instead, when the entry names something this API does not do.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_chirho: Option<&'static str>,
}

impl AgentFieldNeedChirho {
    pub const ALL_CHIRHO: [Self; 4] = [
        Self::RequiredChirho,
        Self::OneServiceRequiredChirho,
        Self::PhoneOrEmailChirho,
        Self::OptionalChirho,
    ];

    pub fn meaning_chirho(self) -> &'static str {
        match self {
            Self::RequiredChirho => "must not be empty",
            Self::OneServiceRequiredChirho => "at least one service must be filled in; in any service you use, both the title and the body are required; leave unused services with both empty",
            Self::PhoneOrEmailChirho => "at least one of the phone and the email must be given; both is fine",
            Self::OptionalChirho => "may be empty",
        }
    }
}

/// Every issue code the validator can name, in one place. The match is exhaustive, so a new
/// validator code cannot be added without giving it a meaning here.
pub const AGENT_ISSUE_CODES_CHIRHO: [PageValidationCodeChirho; 9] = [
    PageValidationCodeChirho::RequiredChirho,
    PageValidationCodeChirho::ContactMethodRequiredChirho,
    PageValidationCodeChirho::InvalidPhoneChirho,
    PageValidationCodeChirho::InvalidEmailChirho,
    PageValidationCodeChirho::TooManyCharactersChirho,
    PageValidationCodeChirho::TooManyBytesChirho,
    PageValidationCodeChirho::UnsupportedControlCharacterChirho,
    PageValidationCodeChirho::UnsupportedSchemaChirho,
    PageValidationCodeChirho::DocumentTooLargeChirho,
];

pub fn agent_issue_meaning_chirho(code_chirho: PageValidationCodeChirho) -> &'static str {
    match code_chirho {
        PageValidationCodeChirho::RequiredChirho => "the field is empty and must not be",
        PageValidationCodeChirho::ContactMethodRequiredChirho => "neither a phone nor an email was given (field path contact_chirho)",
        PageValidationCodeChirho::InvalidPhoneChirho => "not a phone number: digits with optional spaces, dashes, dots, parentheses and one leading +",
        PageValidationCodeChirho::InvalidEmailChirho => "not an email address",
        PageValidationCodeChirho::TooManyCharactersChirho => "longer than the field's max_characters_chirho (the issue carries that limit)",
        PageValidationCodeChirho::TooManyBytesChirho => "too many bytes as UTF-8 even though the character count fits",
        PageValidationCodeChirho::UnsupportedControlCharacterChirho => "contains a control character; only line breaks are allowed, and only where text may span lines",
        PageValidationCodeChirho::UnsupportedSchemaChirho => "schema_version_chirho is not the version this service reads",
        PageValidationCodeChirho::DocumentTooLargeChirho => "the whole page is larger than max_serialized_bytes_chirho (field path document_chirho)",
    }
}

pub fn agent_issue_code_text_chirho(code_chirho: PageValidationCodeChirho) -> String {
    serde_json::to_value(code_chirho)
        .ok()
        .and_then(|value_chirho| value_chirho.as_str().map(str::to_string))
        .unwrap_or_default()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentPageDocumentShapeV1Chirho {
    pub schema_version_chirho: u16,
    pub max_serialized_bytes_chirho: usize,
    /// `services_chirho` is an array of EXACTLY this many objects; unused ones keep both fields empty.
    pub service_slots_chirho: usize,
    /// Send every key the sample page has, `schema_version_chirho` included, even when empty.
    pub every_key_required_chirho: bool,
    /// What `max_characters_chirho` counts.
    pub characters_are_chirho: &'static str,
    pub plain_text_only_chirho: bool,
    /// What a save does to text before storing it - the same as her browser's save, stated so
    /// nothing is changed unannounced. The save's answer carries the page exactly as stored.
    pub normalization_chirho: Vec<&'static str>,
    pub fields_chirho: Vec<AgentFieldV1Chirho>,
    pub needs_chirho: Vec<AgentMeaningV1Chirho>,
    pub issue_codes_chirho: Vec<AgentMeaningV1Chirho>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSaveShapeV1Chirho {
    pub max_body_bytes_chirho: usize,
    /// 0 when nothing is saved yet; otherwise the `current_version_chirho` the page was read at.
    pub expected_version_key_chirho: &'static str,
    pub page_key_chirho: &'static str,
    /// A save REPLACES the whole page with what is sent; it is not a patch.
    pub replaces_whole_page_chirho: bool,
    pub first_save_expected_version_chirho: u64,
    /// An answer of this shape always arrives as this status, saved or refused: read `saved_chirho`.
    pub http_status_of_this_answer_chirho: u16,
    /// Any other status carries NO answer of this shape (gpt #24436: key acceptance alone does not
    /// guarantee 200 - admission, body-limit and engine errors answer otherwise).
    pub other_http_statuses_chirho: &'static str,
    pub answer_keys_chirho: Vec<AgentMeaningV1Chirho>,
    pub refusals_chirho: Vec<AgentMeaningV1Chirho>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSurfaceV1Chirho {
    pub schema_chirho: &'static str,
    pub authentication_chirho: &'static str,
    pub routes_chirho: Vec<AgentRouteV1Chirho>,
    pub page_document_chirho: AgentPageDocumentShapeV1Chirho,
    pub save_chirho: AgentSaveShapeV1Chirho,
    /// A fictional business, complete and valid, in the exact shape a save accepts.
    pub sample_page_chirho: PageDocumentV2Chirho,
    /// What this API does not do, and what happens instead.
    pub not_offered_chirho: Vec<AgentMeaningV1Chirho>,
}

/// A saved page is PRIVATE. Tell the owner so: nothing here makes it public.
const PUBLISHING_INSTEAD_CHIRHO: &str =
    "a saved page is private; the owner previews and publishes it herself, signed in to her workspace, at path_chirho";

fn meaning_chirho(code_chirho: &str, meaning_chirho: &'static str) -> AgentMeaningV1Chirho {
    AgentMeaningV1Chirho {
        code_chirho: code_chirho.to_string(),
        meaning_chirho,
        path_chirho: None,
    }
}

const fn field_chirho(
    field_path_chirho: &'static str,
    max_characters_chirho: usize,
    need_chirho: AgentFieldNeedChirho,
    copy_exactly_from_owner_chirho: bool,
) -> AgentFieldV1Chirho {
    AgentFieldV1Chirho {
        field_path_chirho,
        max_characters_chirho,
        need_chirho,
        copy_exactly_from_owner_chirho,
    }
}

/// The published sample, through the one mapper, so the sample cannot drift from the guide's.
pub fn agent_sample_page_chirho() -> PageDocumentV2Chirho {
    draft_candidate_from_completion_chirho(OWN_AGENT_SAMPLE_ANSWER_CHIRHO)
        .map(|candidate_chirho| candidate_chirho.document_chirho)
        .unwrap_or_default()
}

/// The described limit for a validator field path (`services_chirho[1].title_chirho` reads the
/// `services_chirho[N].title_chirho` row), or None for a path the table does not list.
pub fn agent_field_limit_chirho(field_path_chirho: &str) -> Option<usize> {
    let mut generic_chirho = String::with_capacity(field_path_chirho.len());
    let mut in_index_chirho = false;
    for character_chirho in field_path_chirho.chars() {
        match character_chirho {
            '[' => {
                in_index_chirho = true;
                generic_chirho.push_str("[N");
            }
            ']' => {
                in_index_chirho = false;
                generic_chirho.push(']');
            }
            _ if in_index_chirho => {}
            other_chirho => generic_chirho.push(other_chirho),
        }
    }
    agent_surface_chirho()
        .page_document_chirho
        .fields_chirho
        .into_iter()
        .find(|field_chirho| field_chirho.field_path_chirho == generic_chirho)
        .map(|field_chirho| field_chirho.max_characters_chirho)
}

pub fn agent_surface_chirho() -> AgentSurfaceV1Chirho {
    use AgentFieldNeedChirho::{
        OneServiceRequiredChirho, OptionalChirho, PhoneOrEmailChirho, RequiredChirho,
    };
    AgentSurfaceV1Chirho {
        schema_chirho: AGENT_SURFACE_SCHEMA_CHIRHO,
        authentication_chirho: "Authorization: Bearer <key>",
        routes_chirho: vec![
            AgentRouteV1Chirho {
                method_chirho: "GET",
                path_chirho: AGENT_PAGE_PATH_CHIRHO,
                sends_chirho: "nothing",
                answers_chirho: "application/json",
                purpose_chirho: "this description, her current private page and its version",
            },
            AgentRouteV1Chirho {
                method_chirho: "POST",
                path_chirho: AGENT_SAVE_PATH_CHIRHO,
                sends_chirho: "application/json: expected_version_chirho and page_chirho",
                answers_chirho: "application/json",
                purpose_chirho: "save one new private version, or refuse and change nothing",
            },
            AgentRouteV1Chirho {
                method_chirho: "GET",
                path_chirho: WORKSPACE_PREVIEW_HREF_CHIRHO,
                sends_chirho: "nothing",
                answers_chirho: "text/html",
                purpose_chirho: "her saved private page as a visitor would see it",
            },
        ],
        page_document_chirho: AgentPageDocumentShapeV1Chirho {
            schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
            max_serialized_bytes_chirho: PAGE_DOCUMENT_MAX_SERIALIZED_BYTES_CHIRHO,
            service_slots_chirho: PAGE_SERVICE_SLOT_COUNT_CHIRHO,
            every_key_required_chirho: true,
            characters_are_chirho: "unicode-code-points-chirho",
            plain_text_only_chirho: true,
            normalization_chirho: vec![
                "surrounding-whitespace-trimmed-chirho",
                "crlf-and-cr-become-lf-chirho",
                "services-stored-without-gaps-chirho",
            ],
            fields_chirho: vec![
                field_chirho(
                    "business_name_chirho",
                    BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
                    RequiredChirho,
                    false,
                ),
                field_chirho(
                    "headline_chirho",
                    HEADLINE_MAX_CHARACTERS_CHIRHO,
                    RequiredChirho,
                    false,
                ),
                field_chirho(
                    "summary_chirho",
                    SUMMARY_MAX_CHARACTERS_CHIRHO,
                    RequiredChirho,
                    false,
                ),
                field_chirho(
                    "services_chirho[N].title_chirho",
                    SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
                    OneServiceRequiredChirho,
                    false,
                ),
                field_chirho(
                    "services_chirho[N].body_chirho",
                    SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
                    OneServiceRequiredChirho,
                    false,
                ),
                field_chirho(
                    "contact_heading_chirho",
                    CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
                    RequiredChirho,
                    false,
                ),
                field_chirho(
                    "contact_phone_chirho",
                    CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
                    PhoneOrEmailChirho,
                    true,
                ),
                field_chirho(
                    "contact_email_chirho",
                    CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
                    PhoneOrEmailChirho,
                    true,
                ),
                field_chirho(
                    "contact_details_chirho",
                    CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
                    OptionalChirho,
                    false,
                ),
            ],
            needs_chirho: AgentFieldNeedChirho::ALL_CHIRHO
                .iter()
                .map(|need_chirho| AgentMeaningV1Chirho {
                    code_chirho: serde_json::to_value(need_chirho)
                        .ok()
                        .and_then(|value_chirho| value_chirho.as_str().map(str::to_string))
                        .unwrap_or_default(),
                    meaning_chirho: need_chirho.meaning_chirho(),
                    path_chirho: None,
                })
                .collect(),
            issue_codes_chirho: AGENT_ISSUE_CODES_CHIRHO
                .iter()
                .map(|code_chirho| AgentMeaningV1Chirho {
                    code_chirho: agent_issue_code_text_chirho(*code_chirho),
                    meaning_chirho: agent_issue_meaning_chirho(*code_chirho),
                    path_chirho: None,
                })
                .collect(),
        },
        save_chirho: AgentSaveShapeV1Chirho {
            max_body_bytes_chirho: AGENT_SAVE_MAX_BODY_BYTES_CHIRHO,
            expected_version_key_chirho: "expected_version_chirho",
            page_key_chirho: "page_chirho",
            replaces_whole_page_chirho: true,
            first_save_expected_version_chirho: 0,
            http_status_of_this_answer_chirho: 200,
            other_http_statuses_chirho: "any other status (for example 400, 401, 403, 413, 429) carries no answer of this shape: the request was stopped before or outside the page logic; read the page again before assuming anything about a save",
            answer_keys_chirho: vec![
                meaning_chirho("saved_chirho", "true when this save succeeded - including an exact repeat of an earlier identical save, which is answered as saved again without storing a second copy"),
                meaning_chirho("refusal_chirho", "null when saved; otherwise one of refusals_chirho. Every refusal except unverifiable-chirho means this request stored nothing; after unverifiable-chirho the result is unknown - read the page again"),
                meaning_chirho("current_version_chirho", "after a save, the version that save produced (for a repeated identical save, the version it produced the first time - the head may be newer); after stale-version-chirho, the head version; otherwise null, which means unknown here - read the page again to learn it"),
                meaning_chirho("page_chirho", "after a save, the page exactly as stored; otherwise null"),
                meaning_chirho("issues_chirho", "after invalid-page-chirho, EVERY problem: field_path_chirho, code_chirho and, for a length problem, max_characters_chirho"),
            ],
            refusals_chirho: AgentSaveRefusalChirho::ALL_CHIRHO
                .iter()
                .map(|refusal_chirho| meaning_chirho(refusal_chirho.code_chirho(), refusal_chirho.meaning_chirho()))
                .collect(),
        },
        sample_page_chirho: agent_sample_page_chirho(),
        not_offered_chirho: vec![
            AgentMeaningV1Chirho {
                path_chirho: Some(WORKSPACE_PUBLICATION_HREF_CHIRHO),
                ..meaning_chirho("publishing", PUBLISHING_INSTEAD_CHIRHO)
            },
            meaning_chirho("account-and-password", "she manages her account herself, signed in to her workspace"),
        ],
    }
}
