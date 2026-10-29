// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Render the Perffection front door through its compile-time HTML template.
//!
//! This wrapper owns NO markup. It selects a reviewed typed model and hands it to the render
//! function compiled from `templates_chirho/pages_chirho/home_chirho.html`; the page's structure
//! lives in that template and its copy lives in the model. The render is pure and ignores the
//! request, so it is cache-stable — the bench's trade and mood choices are browser-local and never
//! enter the DAG input at all.
//!
//! Pins for this page are in `src/tests_chirho/home_page_tests_chirho.rs`.

use crate::page_render_chirho::render_home_template_chirho;
use crate::types_chirho::home_page_v1_chirho::HomePageV1Chirho;
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;
use cairn_node_templating_chirho::HtmlPartialV1Chirho;

/// in request: HttpRequestV1Chirho (ignored) -> out page: HtmlPartialV1Chirho (the full front door)
/// Workflow: `spec-chirho/workflows-chirho/first-light-authoring-chirho.md`.
pub fn render_home_chirho(_request_chirho: &HttpRequestV1Chirho) -> HtmlPartialV1Chirho {
    let model_chirho = HomePageV1Chirho::default_public_chirho();
    render_home_template_chirho(&model_chirho)
}
