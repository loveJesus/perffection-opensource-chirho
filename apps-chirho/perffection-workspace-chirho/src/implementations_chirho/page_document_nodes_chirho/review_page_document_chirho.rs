// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pure import review. The returned editor posts to the existing save door only after customer action.
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::page_document_import_chirho::PageDocumentImportV1Chirho;
use crate::page_import_request_chirho::{parse_page_import_chirho, PageImportOutcomeChirho};
use crate::page_render_chirho::{render_page_document_import_chirho, render_workspace_chirho};
use crate::workspace_page_chirho::WorkspacePageV1Chirho;

pub fn review_page_document_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    let rendered_chirho = match parse_page_import_chirho(request_bytes_chirho) {
        PageImportOutcomeChirho::DraftChirho {
            document_chirho,
            expected_version_chirho,
        } => {
            let model_chirho =
                WorkspacePageV1Chirho::imported_chirho(*document_chirho, expected_version_chirho);
            render_workspace_chirho(&model_chirho)
        }
        PageImportOutcomeChirho::RejectedChirho {
            expected_version_chirho,
            document_json_chirho,
            notice_chirho,
        } => render_page_document_import_chirho(&PageDocumentImportV1Chirho::rejected_chirho(
            expected_version_chirho,
            document_json_chirho,
            notice_chirho,
        )),
        PageImportOutcomeChirho::UnreadableFormChirho => {
            render_page_document_import_chirho(&PageDocumentImportV1Chirho::unreadable_form_chirho())
        }
    };
    rendered_chirho.map(String::into_bytes).unwrap_or_default()
}
