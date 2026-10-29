// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Exact current-version private preview renderer in the page-document lifecycle workflow.

use crate::page_document_chirho::PageDocumentV2Chirho;
use crate::page_render_chirho::{render_page_chirho, render_workspace_chirho};
use crate::viewer_state_contract_chirho::decode_page_state_response_chirho;
use crate::workspace_page_chirho::{
    PagePreviewV1Chirho, WorkspacePageV1Chirho, WORKSPACE_HOME_HREF_CHIRHO,
};

pub fn render_private_preview_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    match decode_page_state_response_chirho(response_bytes_chirho) {
        Ok(Some(snapshot_chirho)) => {
            render_page_chirho(&PagePreviewV1Chirho::private_preview_chirho(
                snapshot_chirho.document_chirho().clone(),
                WORKSPACE_HOME_HREF_CHIRHO,
            ))
            .map(String::into_bytes)
            .unwrap_or_default()
        }
        Ok(None) => {
            render_unavailable_preview_chirho("Save a private draft before opening its preview.")
        }
        Err(_error_chirho) => render_unavailable_preview_chirho(
            "The private preview could not be verified. Nothing was published.",
        ),
    }
}

fn render_unavailable_preview_chirho(status_chirho: &str) -> Vec<u8> {
    let model_chirho = WorkspacePageV1Chirho::unavailable_chirho(
        PageDocumentV2Chirho::default(),
        0,
        status_chirho,
    );
    render_workspace_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
