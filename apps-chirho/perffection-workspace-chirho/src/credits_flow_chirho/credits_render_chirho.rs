// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Thin typed wrappers around the Credits pages' checked-in templates. They live beside the held flow and move to
//! `page_render_chirho` with it when the pages are routed.

use crate::credits_flow_chirho::credits_page_chirho::CreditsPageV1Chirho;
use crate::credits_flow_chirho::credits_return_chirho::CreditsReturnV1Chirho;
use askama::Template;

#[derive(Template)]
#[template(path = "credits_chirho.html", escape = "html")]
struct CreditsTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho CreditsPageV1Chirho,
}

/// The Credits page.
pub fn render_credits_chirho(model_chirho: &CreditsPageV1Chirho) -> Result<String, askama::Error> {
    CreditsTemplateChirho { model_chirho }.render()
}

#[derive(Template)]
#[template(path = "credits_return_chirho.html", escape = "html")]
struct CreditsReturnTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho CreditsReturnV1Chirho,
}

/// A return page after Checkout, paid or cancelled.
pub fn render_credits_return_chirho(
    model_chirho: &CreditsReturnV1Chirho,
) -> Result<String, askama::Error> {
    CreditsReturnTemplateChirho { model_chirho }.render()
}
