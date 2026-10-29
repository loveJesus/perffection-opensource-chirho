// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her own assistant saves her figures (F6c, gpt #26625): the complete figures document it read,
//! bounded before parsing, strict, and either ONE optimistic private write through the same versioned
//! viewer-state wire her browser's Save uses, or a refusal that changes nothing.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. The carrier is the
//! document `read_figures_chirho` returns (one shape both ways), validated by the one figures validator,
//! so there is no second parser and no second backend. Both branches of the save DAG call
//! `classify_agent_figures_save_chirho` on the same bytes, so exactly one is live. The body is untrusted:
//! it is never echoed back, and its version is only an optimistic precondition, never authority over
//! whose figures are written: the engine keys the write to the key's own user, and K1 limits the key
//! to `business-figures-chirho` at the viewer-state door.

use crate::figures_flow_chirho::agent_figures_sample_chirho::agent_sample_figures_chirho;
use crate::figures_flow_chirho::figures_document_chirho::{
    FiguresDocumentV1Chirho, FiguresValidationErrorChirho,
};
use crate::figures_flow_chirho::figures_report_chirho::{
    figures_report_chirho, FiguresReportV1Chirho,
};
use crate::viewer_state_chirho::figures_contract_chirho::{
    build_figures_write_request_chirho, decode_figures_outcome_chirho, FiguresOutcomeChirho,
};
use serde::{Deserialize, Serialize};

pub const AGENT_FIGURES_SAVE_SCHEMA_CHIRHO: &str = "perffection-agent-figures-save-v1-chirho";
/// The document is at most 16 KiB when stored compactly; an assistant's JSON may be spaced out, so the
/// body may be four times that. Past it, the body is not parsed.
pub const AGENT_FIGURES_SAVE_MAX_BODY_BYTES_CHIRHO: usize = 64 * 1024;

/// Why a save was not confirmed; the code is what an assistant branches on. Every code except
/// `UnverifiableChirho` means THIS request stored nothing. `UnverifiableChirho` means the result is
/// unknown: read her figures again before saving again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentFiguresSaveRefusalChirho {
    OversizeChirho,
    UnreadableChirho,
    UnsupportedSchemaChirho,
    InvalidFiguresChirho,
    /// The figures are the illustrative sample of the start surface, unchanged (F6d): never hers.
    SampleNotHerFiguresChirho,
    StaleVersionChirho,
    UnverifiableChirho,
}

impl AgentFiguresSaveRefusalChirho {
    pub const ALL_CHIRHO: [Self; 7] = [
        Self::OversizeChirho,
        Self::UnreadableChirho,
        Self::UnsupportedSchemaChirho,
        Self::InvalidFiguresChirho,
        Self::SampleNotHerFiguresChirho,
        Self::StaleVersionChirho,
        Self::UnverifiableChirho,
    ];

    pub fn code_chirho(self) -> &'static str {
        match self {
            Self::OversizeChirho => "oversize-chirho",
            Self::UnreadableChirho => "unreadable-chirho",
            Self::UnsupportedSchemaChirho => "unsupported-schema-chirho",
            Self::InvalidFiguresChirho => "invalid-figures-chirho",
            Self::SampleNotHerFiguresChirho => "sample-not-her-figures-chirho",
            Self::StaleVersionChirho => "stale-version-chirho",
            Self::UnverifiableChirho => "unverifiable-chirho",
        }
    }

    /// What the code means, so an assistant need not guess.
    pub fn meaning_chirho(self) -> &'static str {
        match self {
            Self::OversizeChirho => "the request body is larger than 65536 bytes; it was not parsed",
            Self::UnreadableChirho => "the body is not JSON with exactly expected_version_chirho and figures_chirho, or the figures have a key this service does not know or lack one it needs",
            Self::UnsupportedSchemaChirho => "figures_chirho.schema_chirho is not the version this service reads",
            Self::InvalidFiguresChirho => "the figures are readable but not valid; issues_chirho names the problem",
            Self::SampleNotHerFiguresChirho => "figures_chirho is the illustrative sample shown while nothing is saved, not her figures; ask her for her own figures and save those",
            Self::StaleVersionChirho => "newer figures were saved since you read; current_version_chirho is the head - read them again, then save against that version",
            Self::UnverifiableChirho => "the save could not be prepared or its result could not be verified; assume nothing: read the figures again before saving again",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AgentFiguresSaveRequestV1Chirho {
    expected_version_chirho: u64,
    figures_chirho: FiguresDocumentV1Chirho,
}

/// What the save route answers. A refusal and a success share one shape so an assistant reads one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentFiguresSaveOutcomeV1Chirho {
    pub schema_chirho: &'static str,
    /// The write door answers an exact repeat of an earlier identical save AGAIN as saved, without a
    /// second copy, so this is not always a new version made by THIS call.
    pub saved_chirho: bool,
    pub refusal_chirho: Option<&'static str>,
    /// After a save: the version it produced. After a stale refusal: the head. Otherwise absent, which
    /// means unknown here: a refusal before the door never reads the head.
    pub current_version_chirho: Option<u64>,
    /// After a save: her figures exactly as stored, and the report of that version.
    pub figures_chirho: Option<FiguresDocumentV1Chirho>,
    pub report_chirho: Option<FiguresReportV1Chirho>,
    /// For an invalid document: the validator's code.
    pub issues_chirho: Vec<&'static str>,
}

impl AgentFiguresSaveOutcomeV1Chirho {
    pub fn refused_chirho(refusal_chirho: AgentFiguresSaveRefusalChirho) -> Self {
        Self {
            schema_chirho: AGENT_FIGURES_SAVE_SCHEMA_CHIRHO,
            saved_chirho: false,
            refusal_chirho: Some(refusal_chirho.code_chirho()),
            current_version_chirho: None,
            figures_chirho: None,
            report_chirho: None,
            issues_chirho: Vec::new(),
        }
    }

    pub fn to_json_bytes_chirho(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentFiguresSaveDecisionChirho {
    /// The exact viewer-state write request for the blessed write door.
    WriteChirho(Vec<u8>),
    /// The answer to give instead of writing anything.
    RefuseChirho(Box<AgentFiguresSaveOutcomeV1Chirho>),
}

pub fn classify_agent_figures_save_chirho(body_chirho: &[u8]) -> AgentFiguresSaveDecisionChirho {
    use AgentFiguresSaveRefusalChirho::{
        InvalidFiguresChirho, OversizeChirho, SampleNotHerFiguresChirho, UnreadableChirho,
        UnsupportedSchemaChirho, UnverifiableChirho,
    };
    let refuse_chirho = |outcome_chirho: AgentFiguresSaveOutcomeV1Chirho| {
        AgentFiguresSaveDecisionChirho::RefuseChirho(Box::new(outcome_chirho))
    };
    if body_chirho.len() > AGENT_FIGURES_SAVE_MAX_BODY_BYTES_CHIRHO {
        return refuse_chirho(AgentFiguresSaveOutcomeV1Chirho::refused_chirho(
            OversizeChirho,
        ));
    }
    let Ok(request_chirho) = serde_json::from_slice::<AgentFiguresSaveRequestV1Chirho>(body_chirho)
    else {
        return refuse_chirho(AgentFiguresSaveOutcomeV1Chirho::refused_chirho(
            UnreadableChirho,
        ));
    };
    let validated_chirho = match request_chirho.figures_chirho.validate_chirho() {
        Ok(validated_chirho) => validated_chirho,
        Err(FiguresValidationErrorChirho::UnsupportedSchemaChirho) => {
            return refuse_chirho(AgentFiguresSaveOutcomeV1Chirho::refused_chirho(
                UnsupportedSchemaChirho,
            ));
        }
        Err(error_chirho) => {
            return refuse_chirho(AgentFiguresSaveOutcomeV1Chirho {
                issues_chirho: vec![error_chirho.code_chirho()],
                ..AgentFiguresSaveOutcomeV1Chirho::refused_chirho(InvalidFiguresChirho)
            });
        }
    };
    // The illustration is never saved as her figures, whatever version it is sent against.
    if validated_chirho.document_chirho() == &agent_sample_figures_chirho() {
        return refuse_chirho(AgentFiguresSaveOutcomeV1Chirho::refused_chirho(
            SampleNotHerFiguresChirho,
        ));
    }
    // 0 is "nothing saved yet", the same convention as her browser's hidden field.
    let expected_chirho = (request_chirho.expected_version_chirho != 0)
        .then_some(request_chirho.expected_version_chirho);
    match build_figures_write_request_chirho(&validated_chirho, expected_chirho) {
        Ok(write_request_chirho) => {
            AgentFiguresSaveDecisionChirho::WriteChirho(write_request_chirho)
        }
        Err(_error_chirho) => refuse_chirho(AgentFiguresSaveOutcomeV1Chirho::refused_chirho(
            UnverifiableChirho,
        )),
    }
}

/// The door's typed answer, as the assistant reads it: saved with her figures as stored and their
/// report; stale with the head's version only; anything unverifiable as unknown.
pub fn agent_figures_save_outcome_from_door_chirho(
    response_bytes_chirho: &[u8],
) -> AgentFiguresSaveOutcomeV1Chirho {
    match decode_figures_outcome_chirho(response_bytes_chirho) {
        Ok(FiguresOutcomeChirho::StateChirho(Some(state_chirho))) => {
            AgentFiguresSaveOutcomeV1Chirho {
                schema_chirho: AGENT_FIGURES_SAVE_SCHEMA_CHIRHO,
                saved_chirho: true,
                refusal_chirho: None,
                current_version_chirho: Some(state_chirho.current_version_chirho),
                report_chirho: Some(figures_report_chirho(
                    state_chirho.current_version_chirho,
                    &state_chirho.figures_chirho,
                )),
                figures_chirho: Some(state_chirho.figures_chirho),
                issues_chirho: Vec::new(),
            }
        }
        Ok(FiguresOutcomeChirho::ConflictChirho(conflict_chirho)) => {
            AgentFiguresSaveOutcomeV1Chirho {
                current_version_chirho: Some(conflict_chirho.head_chirho.current_version_chirho),
                ..AgentFiguresSaveOutcomeV1Chirho::refused_chirho(
                    AgentFiguresSaveRefusalChirho::StaleVersionChirho,
                )
            }
        }
        Ok(FiguresOutcomeChirho::StateChirho(None)) | Err(_) => {
            AgentFiguresSaveOutcomeV1Chirho::refused_chirho(
                AgentFiguresSaveRefusalChirho::UnverifiableChirho,
            )
        }
    }
}
