// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The figures read: one fixed logical key, and no app or viewer scope the caller could choose.

use crate::viewer_state_chirho::figures_contract_chirho::build_figures_read_request_chirho;

const INVALID_VIEWER_STATE_REQUEST_CHIRHO: &[u8] =
    br#"{"schema_chirho":"invalid-viewer-state-request-v1-chirho"}"#;

pub fn build_figures_read_chirho(_request_bytes_chirho: &[u8]) -> Vec<u8> {
    build_figures_read_request_chirho()
        .unwrap_or_else(|_error_chirho| INVALID_VIEWER_STATE_REQUEST_CHIRHO.to_vec())
}
