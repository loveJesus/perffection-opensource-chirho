// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Refusal branch of the agent save route: the JSON refusal for a body that must not be written,
//! or the Skipped stone when the write branch is the live one. It reaches no effect and never
//! echoes the body.

use crate::agent_save_chirho::{classify_agent_save_chirho, AgentSaveDecisionChirho};

pub fn render_agent_save_refusal_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match classify_agent_save_chirho(request_bytes_chirho) {
        AgentSaveDecisionChirho::RefuseChirho(outcome_chirho) => {
            outcome_chirho.to_json_bytes_chirho()
        }
        AgentSaveDecisionChirho::WriteChirho(_request_chirho) => {
            cairn_node_sdk_chirho::skipped_bytes_chirho()
        }
    }
}
