// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rejection branch of the publish route in the page-document lifecycle workflow: live exactly when
//! the approval form cannot be sent, so the door is never invoked for it.

use crate::page_render_chirho::render_publication_intent_chirho as render_template_chirho;
use crate::publication_intent_chirho::{parse_publication_form_chirho, PublicationIntentV1Chirho};
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_publication_form_rejection_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match parse_publication_form_chirho(request_bytes_chirho) {
        Ok(_approval_chirho) => cairn_node_sdk_chirho::skipped_bytes_chirho(),
        Err(error_chirho) => render_template_chirho(&PublicationIntentV1Chirho::rejected_chirho(
            error_chirho,
            UiLocaleChirho::default(),
        ))
        .map(String::into_bytes)
        .unwrap_or_default(),
    }
}
