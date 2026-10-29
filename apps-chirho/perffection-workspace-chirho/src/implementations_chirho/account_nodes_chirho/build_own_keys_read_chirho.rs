// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pure builder of the one fixed own-keys read (Cairn K3, #1712) in the agent-connection workflow
//! (`spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md`). It names no viewer, app or
//! key: the engine reads the signed-in browser session's own keys and no one else's.

use crate::connections_chirho::OWN_KEYS_READ_REQUEST_CHIRHO;

pub fn build_own_keys_read_chirho(_request_bytes_chirho: &[u8]) -> Vec<u8> {
    OWN_KEYS_READ_REQUEST_CHIRHO.as_bytes().to_vec()
}
