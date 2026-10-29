// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Import GET: a verified read supplies the optimistic version; corruption never becomes empty state.
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::page_document_import_chirho::PageDocumentImportV1Chirho;
use crate::page_render_chirho::render_page_document_import_chirho as render_import_template_chirho;
use crate::viewer_state_contract_chirho::decode_page_state_response_chirho;

pub fn render_page_document_import_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match decode_page_state_response_chirho(response_bytes_chirho) {
        Ok(Some(snapshot_chirho)) => {
            PageDocumentImportV1Chirho::ready_chirho(snapshot_chirho.current_version_chirho())
        }
        Ok(None) => PageDocumentImportV1Chirho::ready_chirho(0),
        Err(_) => PageDocumentImportV1Chirho::unavailable_chirho(),
    };
    render_import_template_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
