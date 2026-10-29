// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rejection renderer of the first-draft DAG: live exactly when the request builder skipped; the
//! same reader decides, so exactly one branch answers. A readable brief comes back as the form
//! around her own words, every problem beside its control; her words go nowhere else (the edge is
//! private at the sink, and nothing here logs). An unreadable body gets a sentence and nothing
//! of hers.

use crate::draft_page_chirho::{
    deployed_brief_action_chirho, DraftPageV1Chirho, SERVED_CREDIT_VIEW_CHIRHO,
};
use crate::first_draft_chirho::{
    read_draft_brief_chirho, DraftBriefErrorChirho, DraftBriefOutcomeChirho,
};
use crate::page_render_chirho::render_draft_chirho;
use perffection_ui_chirho::UiLocaleChirho;

pub fn render_draft_brief_rejection_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match read_draft_brief_chirho(request_bytes_chirho) {
        DraftBriefOutcomeChirho::ReadyChirho(_brief_chirho) => {
            return cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO.to_vec();
        }
        DraftBriefOutcomeChirho::RefusedChirho {
            words_chirho,
            issues_chirho,
        } => DraftPageV1Chirho::refused_brief_chirho(
            words_chirho,
            &issues_chirho,
            deployed_brief_action_chirho(),
        )
        .with_credits_chirho(&SERVED_CREDIT_VIEW_CHIRHO, UiLocaleChirho::EnglishChirho),
        DraftBriefOutcomeChirho::UnreadableChirho => {
            DraftPageV1Chirho::rejected_chirho(DraftBriefErrorChirho::UnreadableChirho)
        }
    };
    render_draft_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
