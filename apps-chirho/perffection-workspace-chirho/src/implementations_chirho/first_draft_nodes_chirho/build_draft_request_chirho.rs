// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Brief-to-request builder of the first-draft DAG: her bounded brief becomes the one request
//! for the blessed `llm` card, or the Skipped stone when the brief cannot be sent (the rejection
//! renderer is then the live branch, judged by the same parser).

use crate::first_draft_chirho::{build_llm_request_chirho, parse_draft_brief_chirho};

pub fn build_draft_request_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match parse_draft_brief_chirho(request_bytes_chirho) {
        Ok(brief_chirho) => serde_json::to_vec(&build_llm_request_chirho(&brief_chirho))
            .unwrap_or_else(|_| cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO.to_vec()),
        Err(_error_chirho) => cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO.to_vec(),
    }
}
