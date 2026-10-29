// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The bound Account pages in the agent-connection workflow
//! (`spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md`): the deployed page's roads, plus her
//! connections from the engine's own-keys read, each live one with a Disconnect form posting to the engine's revoke
//! road. An answer that is not a verified list is unavailable, never "none". This list-only page offers no way to
//! create a connection; `render_account_connect_chirho` is the separately bound page that does (gpt #26825).

use super::render_account_chirho::deployed_binding_chirho;
use crate::account_page_chirho::{AccountBindingV1Chirho, AccountPageV1Chirho};
use crate::connections_chirho::{connections_state_from_read_chirho, KEY_REVOKE_ACTION_CHIRHO};
use crate::page_render_chirho::render_account_chirho as render_template_chirho;
use perffection_ui_chirho::UiLocaleChirho;

/// Her Account page with her connections read, under the given roads.
pub(super) fn render_bound_account_chirho(
    binding_chirho: AccountBindingV1Chirho,
    own_keys_answer_bytes_chirho: &[u8],
) -> Vec<u8> {
    let page_chirho = AccountPageV1Chirho::with_connections_chirho(
        binding_chirho,
        connections_state_from_read_chirho(own_keys_answer_bytes_chirho),
        UiLocaleChirho::EnglishChirho,
    );
    render_template_chirho(&page_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}

/// The list-bound roads: her password and Disconnect.
pub(super) fn list_bound_binding_chirho() -> AccountBindingV1Chirho {
    deployed_binding_chirho().with_disconnect_action_chirho(KEY_REVOKE_ACTION_CHIRHO)
}

pub fn render_account_keys_chirho(own_keys_answer_bytes_chirho: &[u8]) -> Vec<u8> {
    render_bound_account_chirho(list_bound_binding_chirho(), own_keys_answer_bytes_chirho)
}
