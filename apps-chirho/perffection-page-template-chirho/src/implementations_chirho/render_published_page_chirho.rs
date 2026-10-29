// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Public page renderer for host-injected, signed app params.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::page_render_chirho::render_page_chirho;
use crate::page_view_chirho::PagePreviewV1Chirho;
use crate::published_request_chirho::{
    decode_published_page_request_chirho, PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO,
};

pub fn render_published_page_chirho(input_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = if input_chirho.len() > PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO {
        PagePreviewV1Chirho::public_unavailable_chirho()
    } else {
        decode_published_page_request_chirho(input_chirho)
            .ok()
            .and_then(|request_chirho| request_chirho.app_params_chirho)
            .and_then(|document_chirho| document_chirho.validate_chirho().ok())
            .map(PagePreviewV1Chirho::published_chirho)
            .unwrap_or_else(PagePreviewV1Chirho::public_unavailable_chirho)
    };

    render_page_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
