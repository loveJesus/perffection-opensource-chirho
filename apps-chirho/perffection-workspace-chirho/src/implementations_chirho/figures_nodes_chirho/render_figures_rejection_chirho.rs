// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Rejection branch of the figures save: the review again with every reason, and no effect. For
//! figures that can be kept it returns the Skipped stone, so the write branch is the one live path.

use crate::figures_flow_chirho::figures_form_chirho::{
    classify_figures_save_chirho, FiguresSaveDecisionChirho,
};
use crate::figures_flow_chirho::figures_page_chirho::FiguresReviewV1Chirho;
use crate::page_render_chirho::render_figures_review_chirho;

pub fn render_figures_rejection_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match classify_figures_save_chirho(request_bytes_chirho) {
        FiguresSaveDecisionChirho::WriteChirho(_request_chirho) => {
            return cairn_node_sdk_chirho::skipped_bytes_chirho()
        }
        FiguresSaveDecisionChirho::UnreadableChirho => {
            FiguresReviewV1Chirho::unreadable_form_chirho()
        }
        FiguresSaveDecisionChirho::RefusedChirho(refused_chirho) => {
            FiguresReviewV1Chirho::from_import_chirho(&refused_chirho.0, &refused_chirho.1)
        }
    };
    render_figures_review_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
