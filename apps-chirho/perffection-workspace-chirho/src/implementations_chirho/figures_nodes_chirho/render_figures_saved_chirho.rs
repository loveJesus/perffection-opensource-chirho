// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The figures save's answer: the report of the new version; or, when someone saved first, the head's
//! report with her attempt back in the paste box; or, when the answer cannot be verified, a page that
//! says the result is unknown and shows nothing it cannot verify. Never a retry.

use crate::figures_flow_chirho::figures_page_chirho::{FiguresNoticeChirho, FiguresPageV1Chirho};
use crate::page_render_chirho::render_figures_chirho;
use crate::viewer_state_chirho::figures_contract_chirho::{
    decode_figures_outcome_chirho, FiguresOutcomeChirho,
};

pub fn render_figures_saved_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match decode_figures_outcome_chirho(response_bytes_chirho) {
        Ok(FiguresOutcomeChirho::StateChirho(Some(state_chirho))) => {
            FiguresPageV1Chirho::from_state_chirho(
                Ok(Some(&state_chirho)),
                Some(FiguresNoticeChirho::SavedChirho),
            )
        }
        Ok(FiguresOutcomeChirho::ConflictChirho(conflict_chirho)) => {
            FiguresPageV1Chirho::conflict_chirho(
                &conflict_chirho.head_chirho,
                conflict_chirho.attempted_chirho.as_ref(),
            )
        }
        Ok(FiguresOutcomeChirho::StateChirho(None)) | Err(_) => {
            FiguresPageV1Chirho::from_state_chirho(
                Err(()),
                Some(FiguresNoticeChirho::UnknownChirho),
            )
        }
    };
    render_figures_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
