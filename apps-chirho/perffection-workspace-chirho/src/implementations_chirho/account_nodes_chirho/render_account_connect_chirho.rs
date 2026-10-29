// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Account page bound for creating connections too (Cairn K4; gpt #26824, #26825), in the agent-connection
//! workflow (`spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md`). It is the list-bound page plus
//! one entry, a link to the engine's own form for this app with her way back here, shown only beside a listed
//! connection list. The product never creates, receives or renders a key: the engine's form does the asking and
//! shows the key once. A route runs this page only by its own explicit binding, never by binding the list.

use super::render_account_keys_chirho::{list_bound_binding_chirho, render_bound_account_chirho};
use crate::connections_chirho::connect_entry_href_chirho;

pub fn render_account_connect_chirho(own_keys_answer_bytes_chirho: &[u8]) -> Vec<u8> {
    render_bound_account_chirho(
        list_bound_binding_chirho().with_connect_entry_chirho(&connect_entry_href_chirho()),
        own_keys_answer_bytes_chirho,
    )
}
