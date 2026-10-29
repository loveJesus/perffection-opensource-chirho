// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Write branch of the figures save: the strict viewer-state write request for figures that can be
//! kept, or the engine's Skipped stone so the write door is never reached for anything else. The
//! rejection renderer classifies the same bytes and is live exactly when this branch is skipped.

use crate::figures_flow_chirho::figures_form_chirho::{
    classify_figures_save_chirho, FiguresSaveDecisionChirho,
};

pub fn build_figures_write_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match classify_figures_save_chirho(request_bytes_chirho) {
        FiguresSaveDecisionChirho::WriteChirho(request_chirho) => request_chirho,
        _ => cairn_node_sdk_chirho::skipped_bytes_chirho(),
    }
}
