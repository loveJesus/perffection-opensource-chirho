// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The AI first draft: her guided brief becomes one bounded request for the metered writing
//! helper, and the helper's completion becomes a bounded page-document candidate that the
//! EXISTING review-then-save editor shows at version 0 — nothing is saved until she saves.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/first-draft-chirho.md`.
//!
//! The wire words to and from the blessed `llm` card are the product's proposal until CAIRN names
//! the served card's contract (cairn #22278); the mapping from a completion to a document is the
//! product's alone and trusts nothing the model says: every field is clamped to the document's
//! limits, markup is stripped, unknown keys are dropped, and the document's own validation runs
//! again in the review editor.

use crate::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
    CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO, CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
    CONTACT_HEADING_MAX_CHARACTERS_CHIRHO, CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
    HEADLINE_MAX_CHARACTERS_CHIRHO, PAGE_SERVICE_SLOT_COUNT_CHIRHO,
    SERVICE_BODY_MAX_CHARACTERS_CHIRHO, SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
    SUMMARY_MAX_CHARACTERS_CHIRHO,
};
use crate::page_edit_request_chirho::parse_form_fields_chirho;
use perffection_page_template_chirho::page_document_guide_chirho::draft_answer_instruction_chirho;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const LLM_REQUEST_SCHEMA_CHIRHO: &str = "llm-request-v1-chirho";
pub const LLM_RESPONSE_SCHEMA_CHIRHO: &str = "llm-response-v1-chirho";
/// The card's typed refusal (CAIRN #22285): the hold is released in full, nothing is charged, and
/// the code becomes a catalog sentence.
pub const LLM_REFUSAL_SCHEMA_CHIRHO: &str = "llm-refusal-v1-chirho";
pub const DRAFT_BRIEF_FORM_MAX_BYTES_CHIRHO: usize = 16 * 1024;
// The brief form's caller key is the shared carrier field (FORM_CALLER_KEY_FIELD_CHIRHO, a2b; the
// invocation contract of 2026-09-19: one effect and one charge per invocation). The shared form parser
// drops it, so it never reaches the request to the card (no nonce in the prompt) and is never copied
// into a form the product renders: each render gets its own from the engine.
pub const BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO: usize = 1_200;
pub const BRIEF_AUDIENCE_MAX_CHARACTERS_CHIRHO: usize = 300;
pub const BRIEF_SERVICE_MAX_CHARACTERS_CHIRHO: usize = SERVICE_TITLE_MAX_CHARACTERS_CHIRHO;
pub const BRIEF_SERVICE_COUNT_CHIRHO: usize = PAGE_SERVICE_SLOT_COUNT_CHIRHO;
/// The tones the brief offers; the wire word is the form value, the sentence is the catalog's.
pub const BRIEF_TONES_CHIRHO: [&str; 3] = ["warm-chirho", "plain-chirho", "confident-chirho"];
/// The helper's answer must be one JSON object; anything past this is not a page.
/// The mapper's own byte bound on a completion, independent of any provider's token count: past
/// it the answer is not a page (a visible refusal, never a truncation). It bounds the input the
/// review guest walks under its fuel cap; the fuel probe measured 32 KiB trapping the debug guest
/// and, at this bound, the five probed shapes using at most about a quarter of the cap.
pub const COMPLETION_MAX_BYTES_CHIRHO: usize = 8 * 1024;

/// Her brief, in her own words, already bounded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftBriefChirho {
    pub business_name_chirho: String,
    pub about_chirho: String,
    pub audience_chirho: String,
    pub tone_chirho: &'static str,
    pub services_chirho: Vec<String>,
    pub contact_phone_chirho: String,
    pub contact_email_chirho: String,
    pub contact_details_chirho: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftBriefErrorChirho {
    /// Not a readable form, too large, or an unknown field.
    UnreadableChirho,
    /// The name of the business is missing.
    NameMissingChirho,
    /// Nothing said about the business.
    AboutMissingChirho,
    /// No phone and no email to reach her by.
    ContactMissingChirho,
    /// Too long for the page's own limits (the browser caps it; a bare post may not).
    TooLongChirho,
}

/// A control of the brief form: where a problem with her brief belongs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftBriefFieldChirho {
    BusinessNameChirho,
    AboutChirho,
    AudienceChirho,
    /// One-based slot, as the form names it.
    ServiceChirho(usize),
    ContactPhoneChirho,
    ContactEmailChirho,
    ContactDetailsChirho,
}

/// One thing wrong with a readable brief. Unlike `DraftBriefErrorChirho` there may be several.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftBriefIssueChirho {
    NameMissingChirho,
    AboutMissingChirho,
    /// No phone and no email: a rule over the pair, not over either control.
    ContactMissingChirho,
    TooLongChirho {
        field_chirho: DraftBriefFieldChirho,
        max_characters_chirho: usize,
    },
}

/// Her brief exactly as she typed it (trimmed), so a refusal can hand her words back to her.
/// Bounded by the form's own byte cap; never logged and never sent to the card from here.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DraftBriefWordsChirho {
    pub business_name_chirho: String,
    pub about_chirho: String,
    pub audience_chirho: String,
    /// The tone the form offers that she chose; the first tone when none matched.
    pub tone_chirho: &'static str,
    /// One entry per service slot, empty slots included.
    pub services_chirho: Vec<String>,
    pub contact_phone_chirho: String,
    pub contact_email_chirho: String,
    pub contact_details_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftBriefOutcomeChirho {
    ReadyChirho(DraftBriefChirho),
    /// A brief our form produced but the page cannot use yet: her words and every problem.
    RefusedChirho {
        words_chirho: DraftBriefWordsChirho,
        issues_chirho: Vec<DraftBriefIssueChirho>,
    },
    /// Not a readable form, too large, or a field our form never sends: nothing to hand back.
    UnreadableChirho,
}

/// The one reader of a brief submission. The request builder and the rejection renderer both go
/// through it, so exactly one of them is live for any bytes.
pub fn read_draft_brief_chirho(body_chirho: &[u8]) -> DraftBriefOutcomeChirho {
    let Ok(mut fields_chirho) =
        parse_form_fields_chirho(body_chirho, DRAFT_BRIEF_FORM_MAX_BYTES_CHIRHO)
    else {
        return DraftBriefOutcomeChirho::UnreadableChirho;
    };
    let mut issues_chirho = Vec::new();
    let mut take_chirho = |name_chirho: &str,
                           field_chirho: Option<DraftBriefFieldChirho>,
                           max_characters_chirho: usize|
     -> String {
        let value_chirho = fields_chirho
            .remove(name_chirho)
            .unwrap_or_default()
            .trim()
            .to_string();
        if let Some(field_chirho) = field_chirho {
            if value_chirho.chars().count() > max_characters_chirho {
                issues_chirho.push(DraftBriefIssueChirho::TooLongChirho {
                    field_chirho,
                    max_characters_chirho,
                });
            }
        }
        value_chirho
    };
    use DraftBriefFieldChirho as FieldChirho;
    let business_name_chirho = take_chirho(
        "business_name_chirho",
        Some(FieldChirho::BusinessNameChirho),
        BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
    );
    let about_chirho = take_chirho(
        "about_chirho",
        Some(FieldChirho::AboutChirho),
        BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO,
    );
    let audience_chirho = take_chirho(
        "audience_chirho",
        Some(FieldChirho::AudienceChirho),
        BRIEF_AUDIENCE_MAX_CHARACTERS_CHIRHO,
    );
    // The tone is one of the form's own words; anything else falls back to the first tone.
    let tone_value_chirho = take_chirho("tone_chirho", None, usize::MAX);
    let services_by_slot_chirho: Vec<String> = (1..=BRIEF_SERVICE_COUNT_CHIRHO)
        .map(|slot_chirho| {
            take_chirho(
                &format!("service_{slot_chirho}_chirho"),
                Some(FieldChirho::ServiceChirho(slot_chirho)),
                BRIEF_SERVICE_MAX_CHARACTERS_CHIRHO,
            )
        })
        .collect();
    let contact_phone_chirho = take_chirho(
        "contact_phone_chirho",
        Some(FieldChirho::ContactPhoneChirho),
        CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
    );
    let contact_email_chirho = take_chirho(
        "contact_email_chirho",
        Some(FieldChirho::ContactEmailChirho),
        CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
    );
    let contact_details_chirho = take_chirho(
        "contact_details_chirho",
        Some(FieldChirho::ContactDetailsChirho),
        CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
    );
    // The engine's own hidden field was dropped by the shared parser. Any OTHER field our form never
    // sends still makes the body unreadable.
    if !fields_chirho.is_empty() {
        return DraftBriefOutcomeChirho::UnreadableChirho;
    }
    let tone_chirho = BRIEF_TONES_CHIRHO
        .iter()
        .copied()
        .find(|tone_chirho| *tone_chirho == tone_value_chirho)
        .unwrap_or(BRIEF_TONES_CHIRHO[0]);
    if business_name_chirho.is_empty() {
        issues_chirho.push(DraftBriefIssueChirho::NameMissingChirho);
    }
    if about_chirho.is_empty() {
        issues_chirho.push(DraftBriefIssueChirho::AboutMissingChirho);
    }
    if contact_phone_chirho.is_empty() && contact_email_chirho.is_empty() {
        issues_chirho.push(DraftBriefIssueChirho::ContactMissingChirho);
    }
    if issues_chirho.is_empty() {
        return DraftBriefOutcomeChirho::ReadyChirho(DraftBriefChirho {
            business_name_chirho,
            about_chirho,
            audience_chirho,
            tone_chirho,
            services_chirho: services_by_slot_chirho
                .into_iter()
                .filter(|service_chirho| !service_chirho.is_empty())
                .collect(),
            contact_phone_chirho,
            contact_email_chirho,
            contact_details_chirho,
        });
    }
    DraftBriefOutcomeChirho::RefusedChirho {
        words_chirho: DraftBriefWordsChirho {
            business_name_chirho,
            about_chirho,
            audience_chirho,
            tone_chirho,
            services_chirho: services_by_slot_chirho,
            contact_phone_chirho,
            contact_email_chirho,
            contact_details_chirho,
        },
        issues_chirho,
    }
}

/// The request builder's view of the same reading: a brief ready for the card, or the one reason
/// it is not (the first, in the order the form asks).
pub fn parse_draft_brief_chirho(
    body_chirho: &[u8],
) -> Result<DraftBriefChirho, DraftBriefErrorChirho> {
    match read_draft_brief_chirho(body_chirho) {
        DraftBriefOutcomeChirho::ReadyChirho(brief_chirho) => Ok(brief_chirho),
        DraftBriefOutcomeChirho::UnreadableChirho => Err(DraftBriefErrorChirho::UnreadableChirho),
        DraftBriefOutcomeChirho::RefusedChirho { issues_chirho, .. } => {
            Err(match issues_chirho.first() {
                Some(DraftBriefIssueChirho::NameMissingChirho) => {
                    DraftBriefErrorChirho::NameMissingChirho
                }
                Some(DraftBriefIssueChirho::AboutMissingChirho) => {
                    DraftBriefErrorChirho::AboutMissingChirho
                }
                Some(DraftBriefIssueChirho::ContactMissingChirho) => {
                    DraftBriefErrorChirho::ContactMissingChirho
                }
                Some(DraftBriefIssueChirho::TooLongChirho { .. }) | None => {
                    DraftBriefErrorChirho::TooLongChirho
                }
            })
        }
    }
}

/// The one request to the blessed `llm` card (wire proposal; cairn #22278). Never a principal,
/// never an app name: the reviewed card carries the scope, caller and ceilings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmRequestV1Chirho {
    pub schema_chirho: String,
    pub system_chirho: String,
    pub user_chirho: String,
    pub response_format_chirho: String,
}

fn tone_words_chirho(tone_chirho: &str) -> &'static str {
    match tone_chirho {
        "plain-chirho" => "plain and direct, no sales talk",
        "confident-chirho" => "confident and professional",
        _ => "warm and neighbourly",
    }
}

/// The instruction is fixed text plus her brief; the document's limits are stated so the model
/// aims inside them, and the mapper enforces them regardless.
pub fn build_llm_request_chirho(brief_chirho: &DraftBriefChirho) -> LlmRequestV1Chirho {
    let system_chirho =
        draft_answer_instruction_chirho(Some(tone_words_chirho(brief_chirho.tone_chirho)));
    let mut user_chirho = format!(
        "Business name: {}\nWhat we do: {}\n",
        brief_chirho.business_name_chirho, brief_chirho.about_chirho
    );
    if !brief_chirho.audience_chirho.is_empty() {
        user_chirho.push_str(&format!(
            "Who we serve and where: {}\n",
            brief_chirho.audience_chirho
        ));
    }
    if !brief_chirho.services_chirho.is_empty() {
        user_chirho.push_str(&format!(
            "Services to list: {}\n",
            brief_chirho.services_chirho.join("; ")
        ));
    }
    user_chirho.push_str(&format!(
        "Phone (copy exactly): {}\nEmail (copy exactly): {}\nHours or how to reach us: {}\n",
        brief_chirho.contact_phone_chirho,
        brief_chirho.contact_email_chirho,
        brief_chirho.contact_details_chirho
    ));
    LlmRequestV1Chirho {
        schema_chirho: LLM_REQUEST_SCHEMA_CHIRHO.to_string(),
        system_chirho,
        user_chirho,
        response_format_chirho: "json-object".to_string(),
    }
}

/// The card's answer as the product reads it (wire proposal; cairn #22278): the completion is a
/// sealed viewer-subject value like a viewer-state read; counts and the model label are evidence.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct LlmResponseV1Chirho {
    schema_chirho: String,
    completion_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftMappingErrorChirho {
    /// The bytes are not the card's answer, or not the schema this product reads.
    UnverifiableAnswerChirho,
    /// The completion holds no JSON object that reads as a page.
    NotAPageChirho,
    /// The card refused with a fixed token (over ceiling, provider unavailable, timeout,
    /// unparseable, insufficient credit); nothing was charged.
    RefusedChirho(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct LlmRefusalV1Chirho {
    schema_chirho: String,
    refusal_code_chirho: String,
}

pub fn decode_llm_response_chirho(bytes_chirho: &[u8]) -> Result<String, DraftMappingErrorChirho> {
    if bytes_chirho.len() > COMPLETION_MAX_BYTES_CHIRHO * 2 {
        return Err(DraftMappingErrorChirho::UnverifiableAnswerChirho);
    }
    if let Ok(refusal_chirho) = serde_json::from_slice::<LlmRefusalV1Chirho>(bytes_chirho) {
        if refusal_chirho.schema_chirho == LLM_REFUSAL_SCHEMA_CHIRHO {
            return Err(DraftMappingErrorChirho::RefusedChirho(
                refusal_chirho.refusal_code_chirho,
            ));
        }
    }
    let response_chirho: LlmResponseV1Chirho = serde_json::from_slice(bytes_chirho)
        .map_err(|_| DraftMappingErrorChirho::UnverifiableAnswerChirho)?;
    if response_chirho.schema_chirho != LLM_RESPONSE_SCHEMA_CHIRHO {
        return Err(DraftMappingErrorChirho::UnverifiableAnswerChirho);
    }
    if response_chirho.completion_chirho.len() > COMPLETION_MAX_BYTES_CHIRHO {
        return Err(DraftMappingErrorChirho::NotAPageChirho);
    }
    Ok(response_chirho.completion_chirho)
}

/// Strip anything markup-shaped and fold whitespace; the model was told plain text, the mapper
/// does not rely on it. One pass: the OUTPUT stops at the field's limit, while markup and
/// whitespace may still be scanned across the whole (ceiling-bounded) input without adding to it;
/// the review guest's fuel cap covers that bounded walk (measured by the fuel probe).
fn plain_text_chirho(value_chirho: &str, max_chirho: usize) -> String {
    let mut out_chirho = String::with_capacity(max_chirho.min(value_chirho.len()) * 4);
    let mut kept_chirho = 0usize;
    let mut inside_tag_chirho = false;
    let mut pending_space_chirho = false;
    for character_chirho in value_chirho.chars() {
        if kept_chirho >= max_chirho {
            break;
        }
        match character_chirho {
            '<' => inside_tag_chirho = true,
            '>' if inside_tag_chirho => inside_tag_chirho = false,
            _ if inside_tag_chirho => {}
            c_chirho if c_chirho.is_whitespace() || c_chirho.is_control() => {
                pending_space_chirho = !out_chirho.is_empty();
            }
            c_chirho => {
                if pending_space_chirho {
                    out_chirho.push(' ');
                    kept_chirho += 1;
                    pending_space_chirho = false;
                    if kept_chirho >= max_chirho {
                        break;
                    }
                }
                out_chirho.push(c_chirho);
                kept_chirho += 1;
            }
        }
    }
    out_chirho
}

/// A contact value is kept exactly or dropped, never shortened or repaired: a phone or email the
/// helper mangled must not reach her page as something else (the editor then asks her for it).
fn contact_field_chirho(
    object_chirho: &serde_json::Map<String, Value>,
    key_chirho: &str,
    max_chirho: usize,
) -> String {
    match object_chirho.get(key_chirho) {
        Some(Value::String(text_chirho)) => {
            let trimmed_chirho = text_chirho.trim();
            // After the trim: more than four bytes per allowed character cannot fit; skip the counts.
            if trimmed_chirho.len() > max_chirho * 4
                || trimmed_chirho.chars().count() > max_chirho
                || trimmed_chirho.contains(['<', '>'])
                || trimmed_chirho.chars().any(char::is_control)
            {
                String::new()
            } else {
                trimmed_chirho.to_string()
            }
        }
        _ => String::new(),
    }
}

fn text_field_chirho(
    object_chirho: &serde_json::Map<String, Value>,
    key_chirho: &str,
    max_chirho: usize,
) -> String {
    match object_chirho.get(key_chirho) {
        Some(Value::String(text_chirho)) => plain_text_chirho(text_chirho, max_chirho),
        Some(Value::Number(number_chirho)) => {
            plain_text_chirho(&number_chirho.to_string(), max_chirho)
        }
        _ => String::new(),
    }
}

/// The first JSON object in the completion, tolerating a code fence or prose around it.
fn first_json_object_chirho(completion_chirho: &str) -> Option<serde_json::Map<String, Value>> {
    let start_chirho = completion_chirho.find('{')?;
    let end_chirho = completion_chirho.rfind('}')?;
    if end_chirho <= start_chirho {
        return None;
    }
    let candidate_chirho = &completion_chirho[start_chirho..=end_chirho];
    match serde_json::from_str::<Value>(candidate_chirho) {
        Ok(Value::Object(object_chirho)) => Some(object_chirho),
        _ => None,
    }
}

/// A drafted document plus what the mapper refused to carry: a helper-supplied contact value
/// that was not usable is dropped, and the drop is shown to her rather than lost silently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftCandidateChirho {
    pub document_chirho: PageDocumentV2Chirho,
    pub dropped_contact_chirho: bool,
}

/// The completion becomes a page-document candidate; the review editor validates it again and
/// shows every notice beside its field. Only a completion with no object at all is "not a page".
pub fn draft_document_from_completion_chirho(
    completion_chirho: &str,
) -> Result<PageDocumentV2Chirho, DraftMappingErrorChirho> {
    draft_candidate_from_completion_chirho(completion_chirho)
        .map(|candidate_chirho| candidate_chirho.document_chirho)
}

fn contact_was_dropped_chirho(
    object_chirho: &serde_json::Map<String, Value>,
    key_chirho: &str,
    kept_chirho: &str,
) -> bool {
    matches!(object_chirho.get(key_chirho), Some(Value::String(text_chirho)) if !text_chirho.trim().is_empty() && kept_chirho.is_empty())
}

pub fn draft_candidate_from_completion_chirho(
    completion_chirho: &str,
) -> Result<DraftCandidateChirho, DraftMappingErrorChirho> {
    let object_chirho = first_json_object_chirho(completion_chirho)
        .ok_or(DraftMappingErrorChirho::NotAPageChirho)?;
    let mut services_chirho: [PageServiceV1Chirho; PAGE_SERVICE_SLOT_COUNT_CHIRHO] =
        std::array::from_fn(|_index_chirho| PageServiceV1Chirho::default());
    if let Some(Value::Array(items_chirho)) = object_chirho.get("services") {
        for (slot_chirho, item_chirho) in items_chirho
            .iter()
            .take(PAGE_SERVICE_SLOT_COUNT_CHIRHO)
            .enumerate()
        {
            if let Value::Object(service_chirho) = item_chirho {
                services_chirho[slot_chirho] = PageServiceV1Chirho {
                    title_chirho: text_field_chirho(
                        service_chirho,
                        "title",
                        SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
                    ),
                    body_chirho: text_field_chirho(
                        service_chirho,
                        "body",
                        SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
                    ),
                };
            }
        }
    }
    let document_chirho = PageDocumentV2Chirho {
        business_name_chirho: text_field_chirho(
            &object_chirho,
            "business_name",
            BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
        ),
        headline_chirho: text_field_chirho(
            &object_chirho,
            "headline",
            HEADLINE_MAX_CHARACTERS_CHIRHO,
        ),
        summary_chirho: text_field_chirho(&object_chirho, "summary", SUMMARY_MAX_CHARACTERS_CHIRHO),
        services_chirho,
        contact_heading_chirho: text_field_chirho(
            &object_chirho,
            "contact_heading",
            CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
        ),
        contact_phone_chirho: contact_field_chirho(
            &object_chirho,
            "contact_phone",
            CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
        ),
        contact_email_chirho: contact_field_chirho(
            &object_chirho,
            "contact_email",
            CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
        ),
        contact_details_chirho: text_field_chirho(
            &object_chirho,
            "contact_details",
            CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
        ),
        ..PageDocumentV2Chirho::default()
    };
    if document_chirho.business_name_chirho.is_empty()
        && document_chirho.headline_chirho.is_empty()
        && document_chirho.summary_chirho.is_empty()
    {
        return Err(DraftMappingErrorChirho::NotAPageChirho);
    }
    let dropped_contact_chirho = contact_was_dropped_chirho(
        &object_chirho,
        "contact_phone",
        &document_chirho.contact_phone_chirho,
    ) || contact_was_dropped_chirho(
        &object_chirho,
        "contact_email",
        &document_chirho.contact_email_chirho,
    );
    Ok(DraftCandidateChirho {
        document_chirho,
        dropped_contact_chirho,
    })
}
