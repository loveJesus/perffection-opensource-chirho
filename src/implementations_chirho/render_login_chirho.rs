// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Public form display only. Ignores request content; canonical Cairn auth owns submitted credentials.
//! Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/public-signin-chirho.md

use crate::page_render_chirho::render_login_template_chirho;
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;
use crate::types_chirho::login_page_v1_chirho::LoginPageV1Chirho;
use cairn_node_templating_chirho::HtmlPartialV1Chirho;

pub fn render_login_chirho(_request_chirho: &HttpRequestV1Chirho) -> HtmlPartialV1Chirho {
    render_login_template_chirho(&LoginPageV1Chirho::default_public_chirho())
}
