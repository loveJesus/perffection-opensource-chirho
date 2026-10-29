// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Account page renderer in the account-recovery workflow: a pure page off the authenticated
//! request. It reads nothing of hers and binds no road until cairn's credential roads are on
//! the pin (then the bindings are set here, the day the routes land).

use crate::account_page_chirho::{
    AccountBindingV1Chirho, AccountPageV1Chirho, AUTH_CREDENTIAL_CHANGE_ACTION_CHIRHO,
};
use crate::page_render_chirho::render_account_chirho as render_template_chirho;

/// The roads on this pin: R1 credential change; recovery codes and deletion still held.
pub(super) fn deployed_binding_chirho() -> AccountBindingV1Chirho {
    AccountBindingV1Chirho::held_chirho()
        .with_change_password_action_chirho(AUTH_CREDENTIAL_CHANGE_ACTION_CHIRHO)
}

pub fn render_account_chirho(_request_bytes_chirho: &[u8]) -> Vec<u8> {
    render_template_chirho(&AccountPageV1Chirho::bound_chirho(deployed_binding_chirho()))
        .map(String::into_bytes)
        .unwrap_or_default()
}
