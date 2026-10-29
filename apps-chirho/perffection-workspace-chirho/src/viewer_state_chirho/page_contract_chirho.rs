// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Product adapter for Cairn's audited viewer-state wire contract: her page document.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.
//!
//! The wire itself (requests, verified answers, conflicts) is `wire_chirho`, shared with her figures.
//! This module brings the page's one logical key and its value decoder, and keeps the page types
//! and their accessors that the editor, history and preview render from. Reached at its old path,
//! `crate::viewer_state_contract_chirho`, through a re-export.

use crate::page_document_chirho::{PageDocumentV2Chirho, ValidatedPageDocumentV2Chirho};
use crate::viewer_state_chirho::wire_chirho::{
    decode_outcome_chirho, decode_state_chirho, read_request_chirho, restore_request_chirho,
    write_request_chirho, WireOutcomeChirho, WireSnapshotChirho,
    VIEWER_STATE_HISTORY_MAX_VERSIONS_CHIRHO,
};
pub use crate::viewer_state_chirho::wire_chirho::{
    ViewerStateContractErrorChirho, VIEWER_STATE_CONFLICT_SCHEMA_CHIRHO,
    VIEWER_STATE_REQUEST_SCHEMA_CHIRHO, VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
};

pub const PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO: &str = "page-document-chirho";
pub const PAGE_STATE_HISTORY_MAX_VERSIONS_CHIRHO: usize = VIEWER_STATE_HISTORY_MAX_VERSIONS_CHIRHO;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageStateConflictV1Chirho {
    expected_current_version_chirho: Option<u64>,
    head_chirho: PageStateSnapshotV1Chirho,
    attempted_document_chirho: Option<PageDocumentV2Chirho>,
}

impl PageStateConflictV1Chirho {
    /// The version the viewer believed was current; `None` when she believed nothing was saved yet.
    pub fn expected_current_version_chirho(&self) -> Option<u64> {
        self.expected_current_version_chirho
    }

    pub fn head_chirho(&self) -> &PageStateSnapshotV1Chirho {
        &self.head_chirho
    }

    /// The document the viewer tried to save, only when the echo decoded and validated; a restore
    /// conflict carries none.
    pub fn attempted_document_chirho(&self) -> Option<&PageDocumentV2Chirho> {
        self.attempted_document_chirho.as_ref()
    }
}

/// Everything the viewer-state door can answer a mutation with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageStateOutcomeChirho {
    StateChirho(Option<Box<PageStateSnapshotV1Chirho>>),
    ConflictChirho(Box<PageStateConflictV1Chirho>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageStateSnapshotV1Chirho {
    current_version_chirho: u64,
    document_chirho: ValidatedPageDocumentV2Chirho,
    history_chirho: Vec<PageStateHistoryVersionV1Chirho>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageStateHistoryVersionV1Chirho {
    version_chirho: u64,
    document_chirho: ValidatedPageDocumentV2Chirho,
}

impl PageStateHistoryVersionV1Chirho {
    pub fn version_chirho(&self) -> u64 {
        self.version_chirho
    }

    pub fn document_chirho(&self) -> &ValidatedPageDocumentV2Chirho {
        &self.document_chirho
    }
}

impl PageStateSnapshotV1Chirho {
    pub fn current_version_chirho(&self) -> u64 {
        self.current_version_chirho
    }

    pub fn document_chirho(&self) -> &ValidatedPageDocumentV2Chirho {
        &self.document_chirho
    }

    pub fn history_chirho(&self) -> &[PageStateHistoryVersionV1Chirho] {
        &self.history_chirho
    }
}

pub fn build_page_read_request_chirho() -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    read_request_chirho(PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO)
}

pub fn build_page_write_request_chirho(
    document_chirho: &ValidatedPageDocumentV2Chirho,
    expected_current_version_chirho: Option<u64>,
) -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    if expected_current_version_chirho == Some(0) {
        return Err(ViewerStateContractErrorChirho::InvalidExpectedVersionChirho);
    }
    let value_chirho = serde_json::to_vec(&document_chirho.to_document_chirho())
        .map_err(|_| ViewerStateContractErrorChirho::SerializeRequestChirho)?;
    write_request_chirho(
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        &value_chirho,
        expected_current_version_chirho,
    )
}

pub fn build_page_restore_request_chirho(
    expected_current_version_chirho: u64,
    restore_from_version_chirho: u64,
) -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    restore_request_chirho(
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        expected_current_version_chirho,
        restore_from_version_chirho,
    )
}

/// Decode a mutation answer: the v2 state on success, or the typed conflict when the door refused
/// a stale optimistic version without touching the head.
pub fn decode_page_state_outcome_chirho(
    response_bytes_chirho: &[u8],
) -> Result<PageStateOutcomeChirho, ViewerStateContractErrorChirho> {
    match decode_outcome_chirho(
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        response_bytes_chirho,
        &decode_page_value_chirho,
        &decode_attempted_document_chirho,
    )? {
        WireOutcomeChirho::StateChirho(state_chirho) => Ok(PageStateOutcomeChirho::StateChirho(
            state_chirho.map(|snapshot_chirho| Box::new(page_snapshot_chirho(snapshot_chirho))),
        )),
        WireOutcomeChirho::ConflictChirho(conflict_chirho) => Ok(
            PageStateOutcomeChirho::ConflictChirho(Box::new(PageStateConflictV1Chirho {
                expected_current_version_chirho: conflict_chirho.expected_current_version_chirho,
                head_chirho: page_snapshot_chirho(conflict_chirho.head_chirho),
                attempted_document_chirho: conflict_chirho.attempted_chirho,
            })),
        ),
    }
}

pub fn decode_page_state_response_chirho(
    response_bytes_chirho: &[u8],
) -> Result<Option<PageStateSnapshotV1Chirho>, ViewerStateContractErrorChirho> {
    decode_state_chirho(
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        response_bytes_chirho,
        &decode_page_value_chirho,
    )
    .map(|state_chirho| state_chirho.map(page_snapshot_chirho))
}

/// A retained page value: exact JSON of a document that validates.
fn decode_page_value_chirho(
    value_bytes_chirho: &[u8],
) -> Result<ValidatedPageDocumentV2Chirho, ViewerStateContractErrorChirho> {
    let document_chirho: PageDocumentV2Chirho = serde_json::from_slice(value_bytes_chirho)
        .map_err(|_| ViewerStateContractErrorChirho::MalformedPageDocumentChirho)?;
    document_chirho
        .validate_chirho()
        .map_err(|_| ViewerStateContractErrorChirho::InvalidPageDocumentChirho)
}

/// An echoed attempt renders only when it decodes and validates exactly; anything else is dropped
/// rather than shown.
fn decode_attempted_document_chirho(value_bytes_chirho: &[u8]) -> Option<PageDocumentV2Chirho> {
    let document_chirho: PageDocumentV2Chirho = serde_json::from_slice(value_bytes_chirho).ok()?;
    document_chirho
        .validate_chirho()
        .ok()
        .map(|validated_chirho| validated_chirho.to_document_chirho())
}

/// The wire verified the history non-empty, newest first and contiguous; the head is its first.
fn page_snapshot_chirho(
    snapshot_chirho: WireSnapshotChirho<ValidatedPageDocumentV2Chirho>,
) -> PageStateSnapshotV1Chirho {
    let history_chirho: Vec<PageStateHistoryVersionV1Chirho> = snapshot_chirho
        .history_chirho
        .into_iter()
        .map(|version_chirho| PageStateHistoryVersionV1Chirho {
            version_chirho: version_chirho.version_chirho,
            document_chirho: version_chirho.value_chirho,
        })
        .collect();
    PageStateSnapshotV1Chirho {
        current_version_chirho: history_chirho[0].version_chirho,
        document_chirho: history_chirho[0].document_chirho.clone(),
        history_chirho,
    }
}
