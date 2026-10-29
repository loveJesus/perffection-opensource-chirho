// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Write branch of the agent save route: the strict viewer-state write request for a complete,
//! valid page document, or the engine's Skipped stone so the blessed write door is never invoked
//! for a refused body. The refusal renderer classifies the same bytes and is live exactly when
//! this branch is skipped.

use crate::agent_save_chirho::{classify_agent_save_chirho, AgentSaveDecisionChirho};

pub fn build_agent_page_write_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match classify_agent_save_chirho(request_bytes_chirho) {
        AgentSaveDecisionChirho::WriteChirho(request_chirho) => request_chirho,
        AgentSaveDecisionChirho::RefuseChirho(_outcome_chirho) => {
            cairn_node_sdk_chirho::skipped_bytes_chirho()
        }
    }
}
