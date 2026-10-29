// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Where her assistant starts when she has no figures saved yet (F6d, gpt #26662): the exact shape of
//! a figures document, shown with illustrative values that are plainly NOT hers, and how a first save
//! is made, with every bound and refusal taken from the validator and the save themselves.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. It appears in the
//! read answer only while nothing is saved. Nothing here is ever saved for her: a save of the sample
//! itself is refused (`sample-not-her-figures-chirho`), so an assistant cannot store the illustration
//! as her figures, and her real figures come only from her.

use crate::agent_api_chirho::agent_surface_chirho::AgentMeaningV1Chirho;
use crate::figures_flow_chirho::agent_figures_save_chirho::{
    AgentFiguresSaveRefusalChirho, AGENT_FIGURES_SAVE_MAX_BODY_BYTES_CHIRHO,
};
use crate::figures_flow_chirho::figures_document_chirho::{
    FiguresDocumentV1Chirho, FIGURES_CURRENCIES_CHIRHO, FIGURES_MAX_AMOUNT_MINOR_CHIRHO,
    FIGURES_MAX_JOBS_CHIRHO, FIGURES_MAX_KINDS_CHIRHO, FIGURES_MAX_KIND_CHARS_CHIRHO,
    FIGURES_MAX_MONTHS_CHIRHO, FIGURES_MAX_NOTE_CHARS_CHIRHO, FIGURES_MAX_STORED_BYTES_CHIRHO,
    FIGURES_SCHEMA_CHIRHO,
};
use serde::Serialize;

pub const AGENT_FIGURES_SAVE_PATH_CHIRHO: &str =
    "/app-chirho/perffection-workspace/agent-save-figures-chirho";

/// The illustration: two months and two kinds, one figure left unknown. Every value is made up and
/// labelled so; a test holds that the one figures validator accepts it.
pub fn agent_sample_figures_chirho() -> FiguresDocumentV1Chirho {
    FiguresDocumentV1Chirho {
        schema_chirho: FIGURES_SCHEMA_CHIRHO.to_string(),
        currency_chirho: "USD".to_string(),
        note_chirho: "Illustrative sample, not her figures: replace every value with hers"
            .to_string(),
        months_chirho: vec!["2026-01".to_string(), "2026-02".to_string()],
        kinds_chirho: vec!["Example work A".to_string(), "Example work B".to_string()],
        money_in_chirho: vec![vec![Some(120_000), Some(135_000)], vec![Some(45_000), None]],
        costs_chirho: vec![vec![Some(40_000), Some(42_000)], vec![None, None]],
        jobs_chirho: vec![vec![Some(12), Some(14)], vec![None, None]],
    }
}

/// The bounds a figures document keeps, as the validator enforces them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentFiguresBoundsV1Chirho {
    pub max_months_chirho: usize,
    pub max_kinds_chirho: usize,
    pub max_kind_characters_chirho: usize,
    pub max_note_characters_chirho: usize,
    pub max_amount_minor_chirho: i64,
    pub max_jobs_chirho: u32,
    pub max_stored_bytes_chirho: usize,
    pub currencies_chirho: Vec<&'static str>,
}

/// How her first save is made, shown only while nothing is saved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentFiguresStartV1Chirho {
    pub sample_is_chirho: &'static str,
    pub save_route_chirho: &'static str,
    pub first_save_expected_version_chirho: u64,
    /// The save's body has exactly these two keys.
    pub expected_version_key_chirho: &'static str,
    pub figures_key_chirho: &'static str,
    /// A save REPLACES the whole figures document with what is sent; it is not a patch.
    pub replaces_whole_document_chirho: bool,
    pub max_body_bytes_chirho: usize,
    pub bounds_chirho: AgentFiguresBoundsV1Chirho,
    pub refusals_chirho: Vec<AgentMeaningV1Chirho>,
    pub sample_figures_chirho: FiguresDocumentV1Chirho,
}

pub fn agent_figures_start_chirho() -> AgentFiguresStartV1Chirho {
    AgentFiguresStartV1Chirho {
        sample_is_chirho: "an illustration of the shape only, never her figures: ask her for her own figures, put them in this shape, and save those; a save of the sample itself is refused",
        save_route_chirho: AGENT_FIGURES_SAVE_PATH_CHIRHO,
        first_save_expected_version_chirho: 0,
        expected_version_key_chirho: "expected_version_chirho",
        figures_key_chirho: "figures_chirho",
        replaces_whole_document_chirho: true,
        max_body_bytes_chirho: AGENT_FIGURES_SAVE_MAX_BODY_BYTES_CHIRHO,
        bounds_chirho: AgentFiguresBoundsV1Chirho {
            max_months_chirho: FIGURES_MAX_MONTHS_CHIRHO,
            max_kinds_chirho: FIGURES_MAX_KINDS_CHIRHO,
            max_kind_characters_chirho: FIGURES_MAX_KIND_CHARS_CHIRHO,
            max_note_characters_chirho: FIGURES_MAX_NOTE_CHARS_CHIRHO,
            max_amount_minor_chirho: FIGURES_MAX_AMOUNT_MINOR_CHIRHO,
            max_jobs_chirho: FIGURES_MAX_JOBS_CHIRHO,
            max_stored_bytes_chirho: FIGURES_MAX_STORED_BYTES_CHIRHO,
            currencies_chirho: FIGURES_CURRENCIES_CHIRHO.to_vec(),
        },
        refusals_chirho: AgentFiguresSaveRefusalChirho::ALL_CHIRHO
            .iter()
            .map(|refusal_chirho| AgentMeaningV1Chirho {
                code_chirho: refusal_chirho.code_chirho().to_string(),
                meaning_chirho: refusal_chirho.meaning_chirho(),
                path_chirho: None,
            })
            .collect(),
        sample_figures_chirho: agent_sample_figures_chirho(),
    }
}
