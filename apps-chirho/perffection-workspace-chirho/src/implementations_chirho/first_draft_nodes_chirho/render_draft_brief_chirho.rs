// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Brief page renderer (GET): the brief form when nothing is saved yet, a pointer to the editor
//! when a version exists, an actionless page when her state cannot be verified. The draft door is
//! bound here the day the served `llm` card is on the pin.

use crate::draft_page_chirho::{
    deployed_brief_action_chirho, DraftPageV1Chirho, SERVED_CREDIT_VIEW_CHIRHO,
};
use crate::page_render_chirho::render_draft_chirho;
use crate::viewer_state_contract_chirho::decode_page_state_response_chirho;
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_draft_brief_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match decode_page_state_response_chirho(response_bytes_chirho) {
        Ok(Some(snapshot_chirho)) if snapshot_chirho.current_version_chirho() > 0 => {
            DraftPageV1Chirho::has_page_chirho(snapshot_chirho.current_version_chirho())
        }
        Ok(_) => DraftPageV1Chirho::brief_chirho(deployed_brief_action_chirho())
            .with_credits_chirho(&SERVED_CREDIT_VIEW_CHIRHO, UiLocaleChirho::EnglishChirho),
        Err(_error_chirho) => DraftPageV1Chirho::unavailable_chirho(),
    };
    render_draft_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
