// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The agent's first read in the agent API workflow: the surface, her current private page and
//! the version to save against, as JSON. Unverifiable state carries no page and no version.

use crate::agent_page_chirho::AgentPageV1Chirho;

pub fn render_agent_page_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    AgentPageV1Chirho::from_state_response_chirho(response_bytes_chirho).to_json_bytes_chirho()
}
