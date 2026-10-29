// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! After the write door in the agent save route: saved with the new version and the page as
//! stored, stale with the head version, or unverifiable - never "saved" on an answer that could
//! not be verified.

use crate::agent_save_chirho::agent_save_outcome_from_door_chirho;

pub fn render_agent_save_outcome_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    agent_save_outcome_from_door_chirho(response_bytes_chirho).to_json_bytes_chirho()
}
