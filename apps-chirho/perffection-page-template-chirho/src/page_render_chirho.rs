// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Typed wrapper around the editor-owned customer-page template.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::page_view_chirho::PagePreviewV1Chirho;
use askama::Template;

#[derive(Template)]
#[template(path = "page_chirho.html", escape = "html")]
struct PageTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho PagePreviewV1Chirho,
}

pub fn render_page_chirho(model_chirho: &PagePreviewV1Chirho) -> Result<String, askama::Error> {
    PageTemplateChirho { model_chirho }.render()
}
