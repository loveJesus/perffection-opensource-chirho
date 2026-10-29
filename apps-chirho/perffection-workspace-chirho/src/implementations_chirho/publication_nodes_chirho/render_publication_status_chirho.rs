// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Publication status renderer in the page-document lifecycle workflow: the verified read of her
//! intents and the platform's applied records becomes plain words; an unverifiable read renders
//! an actionless page.

use crate::page_render_chirho::render_publication_status_chirho as render_template_chirho;
use crate::publication_status_chirho::{
    decode_publication_status_chirho, PublicationStatusV1Chirho,
};
use crate::workspace_page_chirho::WORKSPACE_PUBLISH_ACTION_CHIRHO;
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_publication_status_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match decode_publication_status_chirho(response_bytes_chirho) {
        Ok(snapshot_chirho) => {
            PublicationStatusV1Chirho::available_chirho(&snapshot_chirho, UiLocaleChirho::default())
                .with_publish_action_chirho(WORKSPACE_PUBLISH_ACTION_CHIRHO)
        }
        Err(_error_chirho) => {
            PublicationStatusV1Chirho::unavailable_chirho(UiLocaleChirho::default())
        }
    };
    render_template_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
