// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Private viewer-state-to-editor renderer in the page-document lifecycle workflow.
//!
//! Renders the door's answer to a read, a write or a restore: the verified state as the editor,
//! a typed stale-version conflict as the editor at the head with the attempted words kept, and
//! anything unverifiable as an actionless page.

use crate::page_document_chirho::PageDocumentV2Chirho;
use crate::page_render_chirho::render_workspace_chirho;
use crate::viewer_state_contract_chirho::{
    decode_page_state_outcome_chirho, PageStateOutcomeChirho,
};
use crate::workspace_page_chirho::{WorkspaceEditorBindingV1Chirho, WorkspacePageV1Chirho};
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_workspace_state_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match decode_page_state_outcome_chirho(response_bytes_chirho) {
        Ok(PageStateOutcomeChirho::StateChirho(Some(snapshot_chirho))) => {
            WorkspacePageV1Chirho::editable_chirho(
                snapshot_chirho.document_chirho().to_document_chirho(),
                snapshot_chirho.current_version_chirho(),
                WorkspaceEditorBindingV1Chirho::published_contract_chirho(),
                snapshot_chirho.history_chirho(),
                None,
            )
        }
        Ok(PageStateOutcomeChirho::StateChirho(None)) => WorkspacePageV1Chirho::editable_chirho(
            PageDocumentV2Chirho::default(),
            0,
            WorkspaceEditorBindingV1Chirho::published_contract_chirho(),
            &[],
            None,
        ),
        Ok(PageStateOutcomeChirho::ConflictChirho(conflict_chirho)) => {
            WorkspacePageV1Chirho::conflict_chirho(
                &conflict_chirho,
                WorkspaceEditorBindingV1Chirho::published_contract_chirho(),
            )
        }
        Err(_error_chirho) => {
            WorkspacePageV1Chirho::unverifiable_chirho(UiLocaleChirho::EnglishChirho)
        }
    };
    render_workspace_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
