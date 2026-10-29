// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Private form-to-viewer-state builder in the page-document lifecycle workflow.
//!
//! Write branch of the save route: the strict viewer-state write request for a valid form, or the
//! engine's Skipped stone so the blessed write door is never invoked for a rejected one. The
//! rejection renderer classifies the same bytes and is live exactly when this branch is skipped.

use crate::save_page_decision_chirho::{classify_page_edit_form_chirho, SavePageDecisionChirho};

pub fn build_page_write_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match classify_page_edit_form_chirho(request_bytes_chirho) {
        SavePageDecisionChirho::WriteChirho(request_chirho) => request_chirho,
        SavePageDecisionChirho::RejectChirho(_model_chirho) => {
            cairn_node_sdk_chirho::skipped_bytes_chirho()
        }
    }
}
