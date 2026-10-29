// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! One classification for every branch of the save route.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.
//!
//! The save route forks: a valid form becomes exactly one optimistic viewer-state write, an
//! invalid form becomes the editor re-rendered around the submitted draft with a notice per
//! field, and a body our form never produces becomes an actionless page. Both DAG branches call
//! this single function on the same bytes, so exactly one of them is live by construction.

use crate::page_document_chirho::PageDocumentV2Chirho;
use crate::page_edit_request_chirho::{
    parse_page_edit_form_outcome_chirho, PageEditFormOutcomeChirho,
};
use crate::viewer_state_contract_chirho::build_page_write_request_chirho;
use crate::workspace_page_chirho::{WorkspaceEditorBindingV1Chirho, WorkspacePageV1Chirho};

pub const UNREADABLE_FORM_STATUS_CHIRHO: &str =
    "We could not read that form. Open your page again and retry; nothing was changed.";
pub const UNPREPARABLE_WRITE_STATUS_CHIRHO: &str =
    "Your page could not be prepared for saving. Open your page again and retry; nothing was changed.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SavePageDecisionChirho {
    /// The exact viewer-state write request for the blessed write door.
    WriteChirho(Vec<u8>),
    /// The page to show instead of writing anything.
    RejectChirho(Box<WorkspacePageV1Chirho>),
}

pub fn classify_page_edit_form_chirho(request_bytes_chirho: &[u8]) -> SavePageDecisionChirho {
    match parse_page_edit_form_outcome_chirho(request_bytes_chirho) {
        PageEditFormOutcomeChirho::ValidChirho(request_chirho) => {
            match build_page_write_request_chirho(
                request_chirho.document_chirho(),
                request_chirho.expected_current_version_chirho(),
            ) {
                Ok(write_request_chirho) => {
                    SavePageDecisionChirho::WriteChirho(write_request_chirho)
                }
                Err(_error_chirho) => SavePageDecisionChirho::RejectChirho(Box::new(
                    WorkspacePageV1Chirho::unavailable_chirho(
                        request_chirho.document_chirho().to_document_chirho(),
                        request_chirho
                            .expected_current_version_chirho()
                            .unwrap_or(0),
                        UNPREPARABLE_WRITE_STATUS_CHIRHO,
                    ),
                )),
            }
        }
        PageEditFormOutcomeChirho::InvalidDocumentChirho {
            draft_chirho,
            expected_current_version_chirho,
            error_chirho,
        } => {
            SavePageDecisionChirho::RejectChirho(Box::new(WorkspacePageV1Chirho::rejected_chirho(
                draft_chirho,
                expected_current_version_chirho.unwrap_or(0),
                WorkspaceEditorBindingV1Chirho::route_contract_chirho(),
                &error_chirho,
            )))
        }
        PageEditFormOutcomeChirho::MalformedChirho(_error_chirho) => {
            SavePageDecisionChirho::RejectChirho(Box::new(
                WorkspacePageV1Chirho::unavailable_chirho(
                    PageDocumentV2Chirho::default(),
                    0,
                    UNREADABLE_FORM_STATUS_CHIRHO,
                ),
            ))
        }
    }
}
