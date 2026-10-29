// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Review renderer of the first-draft DAG: the helper's completion becomes a bounded page-document
//! candidate shown in the EXISTING editor's imported state at version 0 — nothing is saved until
//! she saves — or an honest "no draft this time" when the answer is not a page.

use crate::draft_page_chirho::DraftPageV1Chirho;
use crate::first_draft_chirho::{
    decode_llm_response_chirho, draft_candidate_from_completion_chirho, DraftMappingErrorChirho,
};
use crate::page_render_chirho::{render_draft_chirho, render_workspace_chirho};
use crate::workspace_page_chirho::WorkspacePageV1Chirho;

pub fn render_draft_review_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let rendered_chirho = match decode_llm_response_chirho(response_bytes_chirho)
        .and_then(|completion_chirho| draft_candidate_from_completion_chirho(&completion_chirho))
    {
        Ok(candidate_chirho) => render_workspace_chirho(&WorkspacePageV1Chirho::drafted_chirho(
            candidate_chirho.document_chirho,
            candidate_chirho.dropped_contact_chirho,
        )),
        Err(DraftMappingErrorChirho::NotAPageChirho) => {
            render_draft_chirho(&DraftPageV1Chirho::not_drafted_chirho())
        }
        Err(DraftMappingErrorChirho::RefusedChirho(code_chirho)) => {
            render_draft_chirho(&DraftPageV1Chirho::refused_chirho(&code_chirho))
        }
        Err(DraftMappingErrorChirho::UnverifiableAnswerChirho) => {
            render_draft_chirho(&DraftPageV1Chirho::unavailable_chirho())
        }
    };
    rendered_chirho.map(String::into_bytes).unwrap_or_default()
}
