// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/customer-help-chirho.md.
//! Public guidance never consumes visitor identity, query strings or request body content.

use crate::page_render_chirho::render_help_template_chirho;
use crate::types_chirho::{
    help_page_v1_chirho::HelpPageV1Chirho, http_request_v1_chirho::HttpRequestV1Chirho,
};
use cairn_node_templating_chirho::HtmlPartialV1Chirho;

pub fn render_help_chirho(_request_chirho: &HttpRequestV1Chirho) -> HtmlPartialV1Chirho {
    render_help_template_chirho(&HelpPageV1Chirho::default_public_chirho())
}
