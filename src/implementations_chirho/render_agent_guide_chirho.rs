// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/customer-help-chirho.md.
//! This public document never consumes a visitor's request or private workspace content.

use crate::page_render_chirho::render_agent_guide_template_chirho;
use crate::types_chirho::{
    agent_guide_page_v1_chirho::AgentGuidePageV1Chirho, http_request_v1_chirho::HttpRequestV1Chirho,
};
use cairn_node_templating_chirho::HtmlPartialV1Chirho;
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_agent_guide_chirho(_request_chirho: &HttpRequestV1Chirho) -> HtmlPartialV1Chirho {
    render_agent_guide_template_chirho(&AgentGuidePageV1Chirho::for_locale_chirho(
        UiLocaleChirho::EnglishChirho,
    ))
}
