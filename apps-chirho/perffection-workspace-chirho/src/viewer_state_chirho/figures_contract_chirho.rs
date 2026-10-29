// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures under the viewer-state door: a second logical key beside her page, never inside it.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. The value is the
//! exact compact JSON the figures validator measured, so what is stored is what was checked. A
//! retained value is decoded and validated again before any report reads it.

use crate::figures_flow_chirho::figures_document_chirho::{
    FiguresDocumentV1Chirho, ValidatedFiguresChirho,
};
use crate::viewer_state_chirho::wire_chirho::{
    decode_outcome_chirho, decode_state_chirho, read_request_chirho, write_request_chirho,
    ViewerStateContractErrorChirho, WireOutcomeChirho, WireSnapshotChirho,
};

pub const FIGURES_LOGICAL_KEY_CHIRHO: &str = "business-figures-chirho";

/// Her saved figures at their head version, with the versions the door still keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiguresStateChirho {
    pub current_version_chirho: u64,
    pub figures_chirho: FiguresDocumentV1Chirho,
    pub kept_versions_chirho: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FiguresOutcomeChirho {
    StateChirho(Option<Box<FiguresStateChirho>>),
    ConflictChirho(Box<FiguresConflictChirho>),
}

/// Someone saved first: the head is unchanged and her attempted figures come back to her.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiguresConflictChirho {
    pub expected_current_version_chirho: Option<u64>,
    pub head_chirho: FiguresStateChirho,
    pub attempted_chirho: Option<FiguresDocumentV1Chirho>,
}

pub fn build_figures_read_request_chirho() -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    read_request_chirho(FIGURES_LOGICAL_KEY_CHIRHO)
}

pub fn build_figures_write_request_chirho(
    figures_chirho: &ValidatedFiguresChirho,
    expected_current_version_chirho: Option<u64>,
) -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    write_request_chirho(
        FIGURES_LOGICAL_KEY_CHIRHO,
        figures_chirho.stored_bytes_chirho(),
        expected_current_version_chirho,
    )
}

pub fn decode_figures_state_chirho(
    response_bytes_chirho: &[u8],
) -> Result<Option<FiguresStateChirho>, ViewerStateContractErrorChirho> {
    decode_state_chirho(
        FIGURES_LOGICAL_KEY_CHIRHO,
        response_bytes_chirho,
        &decode_figures_value_chirho,
    )
    .map(|state_chirho| state_chirho.map(figures_state_chirho))
}

pub fn decode_figures_outcome_chirho(
    response_bytes_chirho: &[u8],
) -> Result<FiguresOutcomeChirho, ViewerStateContractErrorChirho> {
    Ok(
        match decode_outcome_chirho(
            FIGURES_LOGICAL_KEY_CHIRHO,
            response_bytes_chirho,
            &decode_figures_value_chirho,
            &|bytes_chirho| decode_figures_value_chirho(bytes_chirho).ok(),
        )? {
            WireOutcomeChirho::StateChirho(state_chirho) => FiguresOutcomeChirho::StateChirho(
                state_chirho.map(|snapshot_chirho| Box::new(figures_state_chirho(snapshot_chirho))),
            ),
            WireOutcomeChirho::ConflictChirho(conflict_chirho) => {
                FiguresOutcomeChirho::ConflictChirho(Box::new(FiguresConflictChirho {
                    expected_current_version_chirho: conflict_chirho
                        .expected_current_version_chirho,
                    head_chirho: figures_state_chirho(conflict_chirho.head_chirho),
                    attempted_chirho: conflict_chirho.attempted_chirho,
                }))
            }
        },
    )
}

fn decode_figures_value_chirho(
    value_bytes_chirho: &[u8],
) -> Result<FiguresDocumentV1Chirho, ViewerStateContractErrorChirho> {
    let figures_chirho: FiguresDocumentV1Chirho = serde_json::from_slice(value_bytes_chirho)
        .map_err(|_| ViewerStateContractErrorChirho::MalformedFiguresChirho)?;
    figures_chirho
        .validate_chirho()
        .map(|validated_chirho| validated_chirho.document_chirho().clone())
        .map_err(|_| ViewerStateContractErrorChirho::InvalidFiguresChirho)
}

fn figures_state_chirho(
    snapshot_chirho: WireSnapshotChirho<FiguresDocumentV1Chirho>,
) -> FiguresStateChirho {
    let kept_versions_chirho = snapshot_chirho
        .history_chirho
        .iter()
        .map(|version_chirho| version_chirho.version_chirho)
        .collect();
    let head_chirho = snapshot_chirho
        .history_chirho
        .into_iter()
        .next()
        .expect("the wire verified a non-empty history");
    FiguresStateChirho {
        current_version_chirho: head_chirho.version_chirho,
        figures_chirho: head_chirho.value_chirho,
        kept_versions_chirho,
    }
}
