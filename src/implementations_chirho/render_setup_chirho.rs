// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Public set-up form display only (an emailed invitation's step 1). Ignores request content, so a code placed in a
//! URL is never rendered back; Cairn's R2 redemption door owns what is submitted. HELD: no route until that door lands.
//! Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/emailed-invitation-chirho.md

use crate::page_render_chirho::render_setup_template_chirho;
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;
use crate::types_chirho::setup_page_v1_chirho::SetupPageV1Chirho;
use cairn_node_templating_chirho::HtmlPartialV1Chirho;

pub fn render_setup_chirho(_request_chirho: &HttpRequestV1Chirho) -> HtmlPartialV1Chirho {
    render_setup_template_chirho(&SetupPageV1Chirho::default_public_chirho())
}
