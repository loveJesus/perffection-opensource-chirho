// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her assistant's read of her figures: the report, the figures, the version and how to read them, as JSON.

use crate::figures_flow_chirho::agent_figures_chirho::AgentFiguresV1Chirho;

pub fn render_agent_figures_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    AgentFiguresV1Chirho::from_state_response_chirho(response_bytes_chirho).to_json_bytes_chirho()
}
