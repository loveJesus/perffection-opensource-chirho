// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Exact current-version export renderer in the page-document lifecycle workflow.

use crate::page_document_export_chirho::PageDocumentExportV1Chirho;
use crate::page_render_chirho::render_page_document_export_chirho as render_export_template_chirho;
use crate::viewer_state_contract_chirho::decode_page_state_response_chirho;
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_page_document_export_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match decode_page_state_response_chirho(response_bytes_chirho) {
        Ok(Some(snapshot_chirho)) => PageDocumentExportV1Chirho::available_chirho(&snapshot_chirho)
            .unwrap_or_else(|_error_chirho| {
                PageDocumentExportV1Chirho::unverifiable_chirho(UiLocaleChirho::EnglishChirho)
            }),
        Ok(None) => PageDocumentExportV1Chirho::nothing_saved_chirho(UiLocaleChirho::EnglishChirho),
        Err(_error_chirho) => {
            PageDocumentExportV1Chirho::unverifiable_chirho(UiLocaleChirho::EnglishChirho)
        }
    };
    render_export_template_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
