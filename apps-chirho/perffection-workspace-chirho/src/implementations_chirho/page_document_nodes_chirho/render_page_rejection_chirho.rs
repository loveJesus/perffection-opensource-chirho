// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Rejection branch of the save route in the page-document lifecycle workflow.
//!
//! Live only when the submitted form must not be written: it renders the editor around the
//! customer's own draft with a notice per field and performs no effect. For a valid form it
//! returns the engine's Skipped stone so the write branch is the one live path.

use crate::page_render_chirho::render_workspace_chirho;
use crate::save_page_decision_chirho::{classify_page_edit_form_chirho, SavePageDecisionChirho};

pub fn render_page_rejection_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match classify_page_edit_form_chirho(request_bytes_chirho) {
        SavePageDecisionChirho::WriteChirho(_request_chirho) => {
            cairn_node_sdk_chirho::skipped_bytes_chirho()
        }
        SavePageDecisionChirho::RejectChirho(model_chirho) => {
            render_workspace_chirho(&model_chirho)
                .map(String::into_bytes)
                .unwrap_or_default()
        }
    }
}
