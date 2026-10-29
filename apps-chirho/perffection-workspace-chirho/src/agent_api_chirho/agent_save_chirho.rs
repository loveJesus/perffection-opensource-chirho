// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The agent's save: bounded before parsing, strict, and either ONE optimistic private write
//! through the existing door or a refusal that changes nothing.
//!
//! Both branches of the save DAG call `classify_agent_save_chirho` on the same bytes, so exactly
//! one is live by construction (the shape of the browser's save route). The body is untrusted
//! input from outside the platform: it is never echoed back, and the version it names is only an
//! optimistic-concurrency precondition, never authority over whose page is written - the engine
//! keys the write to the authenticated caller.

use crate::agent_surface_chirho::agent_field_limit_chirho;
use crate::page_document_chirho::{
    PageDocumentV2Chirho, PageValidationCodeChirho, PageValidationIssueChirho,
    PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use crate::page_import_request_chirho::PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO;
use crate::viewer_state_contract_chirho::{
    build_page_write_request_chirho, decode_page_state_outcome_chirho, PageStateOutcomeChirho,
};
use serde::{Deserialize, Serialize};

pub const AGENT_SAVE_OUTCOME_SCHEMA_CHIRHO: &str = "perffection-agent-save-v1-chirho";
/// The page document bound the import road uses, plus room for the small envelope around it.
pub const AGENT_SAVE_MAX_BODY_BYTES_CHIRHO: usize = PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO + 256;

/// Why a save was not confirmed; the code is what an agent branches on. Every code except
/// `UnverifiableChirho` means THIS request stored nothing. `UnverifiableChirho` after the write
/// door means the result is unknown - a write may have happened - so the caller must read again
/// (gpt #24436: never instruct a caller to assume an uncertain write did not happen).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentSaveRefusalChirho {
    /// The body is larger than the bound; it was not parsed.
    OversizeChirho,
    /// Not JSON, not the two expected keys, or a key the page document does not have.
    UnreadableChirho,
    /// A page document of a schema version this product does not read.
    UnsupportedSchemaChirho,
    /// The page is readable but not complete and valid; `issues_chirho` names every field.
    InvalidPageChirho,
    /// Someone saved a newer version first; `current_version_chirho` is the head to re-read.
    StaleVersionChirho,
    /// The write could not be prepared or its answer could not be verified. Nothing is assumed
    /// saved: read the page again before retrying.
    UnverifiableChirho,
}

impl AgentSaveRefusalChirho {
    pub const ALL_CHIRHO: [Self; 6] = [
        Self::OversizeChirho,
        Self::UnreadableChirho,
        Self::UnsupportedSchemaChirho,
        Self::InvalidPageChirho,
        Self::StaleVersionChirho,
        Self::UnverifiableChirho,
    ];

    /// What the code means, for the surface description: an agent should not have to guess.
    pub fn meaning_chirho(self) -> &'static str {
        match self {
            Self::OversizeChirho => "the request body is larger than max_body_bytes_chirho; it was not parsed",
            Self::UnreadableChirho => "the body is not JSON with exactly expected_version_chirho and page_chirho, or the page has a key this service does not know or lacks one it needs",
            Self::UnsupportedSchemaChirho => "page_chirho.schema_version_chirho is not the version this service reads",
            Self::InvalidPageChirho => "the page is readable but not complete and valid; issues_chirho names every problem",
            Self::StaleVersionChirho => "a newer version was saved since you read; current_version_chirho is the head - read the page again, then save against that version",
            Self::UnverifiableChirho => "the save could not be prepared or its result could not be verified; assume nothing: read the page again before retrying",
        }
    }

    pub fn code_chirho(self) -> &'static str {
        match self {
            Self::OversizeChirho => "oversize-chirho",
            Self::UnreadableChirho => "unreadable-chirho",
            Self::UnsupportedSchemaChirho => "unsupported-schema-chirho",
            Self::InvalidPageChirho => "invalid-page-chirho",
            Self::StaleVersionChirho => "stale-version-chirho",
            Self::UnverifiableChirho => "unverifiable-chirho",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentSaveRequestV1Chirho {
    expected_version_chirho: u64,
    page_chirho: PageDocumentV2Chirho,
}

/// One problem, as the agent reads it: the validator's own path and code, and for a length problem
/// the limit it ran past, so the agent can fix it without looking the field up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentIssueV1Chirho {
    pub field_path_chirho: String,
    pub code_chirho: PageValidationCodeChirho,
    pub max_characters_chirho: Option<usize>,
}

impl From<PageValidationIssueChirho> for AgentIssueV1Chirho {
    fn from(issue_chirho: PageValidationIssueChirho) -> Self {
        let max_characters_chirho = matches!(
            issue_chirho.code_chirho,
            PageValidationCodeChirho::TooManyCharactersChirho
        )
        .then(|| agent_field_limit_chirho(&issue_chirho.field_path_chirho))
        .flatten();
        Self {
            field_path_chirho: issue_chirho.field_path_chirho,
            code_chirho: issue_chirho.code_chirho,
            max_characters_chirho,
        }
    }
}

/// What the save route answers. A refusal and a success share one shape so an agent reads one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentSaveOutcomeV1Chirho {
    pub schema_chirho: &'static str,
    /// A successful save outcome. The write door answers an exact repeat of an earlier identical
    /// save AGAIN as saved, without storing a second copy, so this is not always a new version
    /// made by THIS call.
    pub saved_chirho: bool,
    pub refusal_chirho: Option<&'static str>,
    /// After a save: the version that save produced (for a repeated identical save, the version it
    /// produced the first time - the head may be newer). After a stale refusal: the head.
    /// Otherwise absent, which means UNKNOWN HERE: a validation refusal never reads the head, and
    /// someone else may have saved meanwhile.
    pub current_version_chirho: Option<u64>,
    /// After a save: the page exactly as stored. Text is normalized the way her browser's save
    /// normalizes it (the surface description lists how), so this is the proof of what was kept.
    pub page_chirho: Option<PageDocumentV2Chirho>,
    pub issues_chirho: Vec<AgentIssueV1Chirho>,
}

impl AgentSaveOutcomeV1Chirho {
    pub fn refused_chirho(refusal_chirho: AgentSaveRefusalChirho) -> Self {
        Self {
            schema_chirho: AGENT_SAVE_OUTCOME_SCHEMA_CHIRHO,
            saved_chirho: false,
            refusal_chirho: Some(refusal_chirho.code_chirho()),
            current_version_chirho: None,
            page_chirho: None,
            issues_chirho: Vec::new(),
        }
    }

    fn invalid_chirho(issues_chirho: Vec<PageValidationIssueChirho>) -> Self {
        Self {
            issues_chirho: issues_chirho.into_iter().map(Into::into).collect(),
            ..Self::refused_chirho(AgentSaveRefusalChirho::InvalidPageChirho)
        }
    }

    fn stale_chirho(head_version_chirho: u64) -> Self {
        Self {
            current_version_chirho: Some(head_version_chirho),
            ..Self::refused_chirho(AgentSaveRefusalChirho::StaleVersionChirho)
        }
    }

    fn saved_chirho(version_chirho: u64, page_chirho: PageDocumentV2Chirho) -> Self {
        Self {
            schema_chirho: AGENT_SAVE_OUTCOME_SCHEMA_CHIRHO,
            saved_chirho: true,
            refusal_chirho: None,
            current_version_chirho: Some(version_chirho),
            page_chirho: Some(page_chirho),
            issues_chirho: Vec::new(),
        }
    }

    pub fn to_json_bytes_chirho(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentSaveDecisionChirho {
    /// The exact viewer-state write request for the blessed write door.
    WriteChirho(Vec<u8>),
    /// The answer to give instead of writing anything.
    RefuseChirho(Box<AgentSaveOutcomeV1Chirho>),
}

pub fn classify_agent_save_chirho(body_chirho: &[u8]) -> AgentSaveDecisionChirho {
    use AgentSaveRefusalChirho::{
        OversizeChirho, UnreadableChirho, UnsupportedSchemaChirho, UnverifiableChirho,
    };
    let refuse_chirho = |refusal_chirho| {
        AgentSaveDecisionChirho::RefuseChirho(Box::new(AgentSaveOutcomeV1Chirho::refused_chirho(
            refusal_chirho,
        )))
    };
    if body_chirho.len() > AGENT_SAVE_MAX_BODY_BYTES_CHIRHO {
        return refuse_chirho(OversizeChirho);
    }
    let Ok(request_chirho) = serde_json::from_slice::<AgentSaveRequestV1Chirho>(body_chirho) else {
        return refuse_chirho(UnreadableChirho);
    };
    if request_chirho.page_chirho.schema_version_chirho != PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO {
        return refuse_chirho(UnsupportedSchemaChirho);
    }
    let document_chirho = match request_chirho.page_chirho.validate_chirho() {
        Ok(document_chirho) => document_chirho,
        Err(error_chirho) => {
            return AgentSaveDecisionChirho::RefuseChirho(Box::new(
                AgentSaveOutcomeV1Chirho::invalid_chirho(error_chirho.issues_chirho),
            ));
        }
    };
    // 0 is "nothing saved yet", the same convention as the editor's hidden field.
    let expected_chirho = (request_chirho.expected_version_chirho != 0)
        .then_some(request_chirho.expected_version_chirho);
    match build_page_write_request_chirho(&document_chirho, expected_chirho) {
        Ok(write_request_chirho) => AgentSaveDecisionChirho::WriteChirho(write_request_chirho),
        Err(_error_chirho) => refuse_chirho(UnverifiableChirho),
    }
}

/// The door's typed answer, as the agent reads it.
pub fn agent_save_outcome_from_door_chirho(
    response_bytes_chirho: &[u8],
) -> AgentSaveOutcomeV1Chirho {
    match decode_page_state_outcome_chirho(response_bytes_chirho) {
        Ok(PageStateOutcomeChirho::StateChirho(Some(snapshot_chirho))) => {
            AgentSaveOutcomeV1Chirho::saved_chirho(
                snapshot_chirho.current_version_chirho(),
                snapshot_chirho.document_chirho().to_document_chirho(),
            )
        }
        Ok(PageStateOutcomeChirho::ConflictChirho(conflict_chirho)) => {
            AgentSaveOutcomeV1Chirho::stale_chirho(
                conflict_chirho.head_chirho().current_version_chirho(),
            )
        }
        Ok(PageStateOutcomeChirho::StateChirho(None)) | Err(_) => {
            AgentSaveOutcomeV1Chirho::refused_chirho(AgentSaveRefusalChirho::UnverifiableChirho)
        }
    }
}
