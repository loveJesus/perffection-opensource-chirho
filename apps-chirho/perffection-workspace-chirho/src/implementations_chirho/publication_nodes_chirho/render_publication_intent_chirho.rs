// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Answer renderer of the publish route in the page-document lifecycle workflow: the queued
//! receipt, the door's typed refusal (base moved, head moved, nothing saved…), or an
//! unverifiable answer — each as plain words with the way back to the Publishing page.

use crate::page_render_chirho::render_publication_intent_chirho as render_template_chirho;
use crate::publication_intent_chirho::{
    decode_publication_intent_answer_chirho, PublicationIntentAnswerChirho,
    PublicationIntentV1Chirho,
};
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_publication_intent_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let locale_chirho = UiLocaleChirho::default();
    let model_chirho = match decode_publication_intent_answer_chirho(response_bytes_chirho) {
        Ok(PublicationIntentAnswerChirho::QueuedChirho(receipt_chirho)) => {
            PublicationIntentV1Chirho::queued_chirho(&receipt_chirho, locale_chirho)
        }
        Ok(PublicationIntentAnswerChirho::RefusedChirho(refusal_chirho)) => {
            PublicationIntentV1Chirho::refused_chirho(&refusal_chirho, locale_chirho)
        }
        Err(_error_chirho) => PublicationIntentV1Chirho::unverifiable_chirho(locale_chirho),
    };
    render_template_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
