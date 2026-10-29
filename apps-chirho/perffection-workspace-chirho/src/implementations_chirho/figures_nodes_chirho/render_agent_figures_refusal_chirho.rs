// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Refusal branch of her assistant's figures Save: the JSON refusal for a body that must not be written,
//! or the Skipped stone when the write branch is the live one. It reaches no effect and never echoes
//! the body.

use crate::figures_flow_chirho::agent_figures_save_chirho::{
    classify_agent_figures_save_chirho, AgentFiguresSaveDecisionChirho,
};

pub fn render_agent_figures_refusal_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match classify_agent_figures_save_chirho(request_bytes_chirho) {
        AgentFiguresSaveDecisionChirho::RefuseChirho(outcome_chirho) => {
            outcome_chirho.to_json_bytes_chirho()
        }
        AgentFiguresSaveDecisionChirho::WriteChirho(_request_chirho) => {
            cairn_node_sdk_chirho::skipped_bytes_chirho()
        }
    }
}
