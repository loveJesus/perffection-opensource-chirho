// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Account page: change her password, keep a recovery code, close the account.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/account-recovery-chirho.md`.
//!
//! The acts themselves are cairn auth roads (CAIRN #21830: R1 credential change first, then
//! retire + delete, recovery codes last) and land contract-first. Until a road is bound here the
//! page renders NO form for it and names the operator path instead; a bound form posts straight
//! to the engine road with her session, carries no secret of ours, and never echoes a password.

use crate::connections_chirho::{
    ConnectionsStateChirho, ConnectionsViewChirho, DISCONNECT_REFUSAL_TOKENS_CHIRHO,
};
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
    WORKSPACE_ACCOUNT_HREF_CHIRHO,
};
use perffection_ui_chirho::{SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho};

/// Cairn's self-service credential-change road (CAIRN #21925, R1): a browser form with
/// `credential_chirho`, `new_credential_chirho`, `redirect_chirho`; accepted → every session
/// revoked and a redirect to login; refused → a redirect back here with a fixed token.
pub const AUTH_CREDENTIAL_CHANGE_ACTION_CHIRHO: &str = "/auth-chirho/credential-change-chirho";

/// Minimum length the new-password field asks for in the browser, shared with the Set-up page; the engine's own floor
/// (`credential_meets_floor_chirho`) is the authority and refuses shorter ones.
pub use perffection_ui_chirho::NEW_PASSWORD_MIN_CHARACTERS_CHIRHO;

/// The road's refusal tokens that come back to this page (never `no-session-chirho`, which lands
/// on the login page). Each is pre-rendered as a hidden alert whose id is `refusal-<token>`, so a
/// redirect to `#refusal-<token>` reveals exactly one sentence with CSS `:target` — no script, no
/// query parsing, no engine prose (the served road today appends `?refusal_chirho=<token>`, which
/// a GET's DAG input does not carry; the fragment form is the product's ask to CAIRN).
pub const CHANGE_PASSWORD_REFUSAL_TOKENS_CHIRHO: [&str; 4] = [
    "credential-mismatch-chirho",
    "credential-too-weak-chirho",
    "request-invalid-chirho",
    "credential-change-failed-chirho",
];
/// The recovery-code road's refusals that return here (R2; the code itself never travels by
/// fragment). Shared tokens render once per page.
pub const RECOVERY_CODE_REFUSAL_TOKENS_CHIRHO: [&str; 3] = [
    "credential-mismatch-chirho",
    "request-invalid-chirho",
    "recovery-code-failed-chirho",
];
/// The close-account road's refusals that leave her session alive and return here (R3); an
/// accepted closure lands on the public login page instead.
pub const CLOSE_ACCOUNT_REFUSAL_TOKENS_CHIRHO: [&str; 5] = [
    "credential-mismatch-chirho",
    "request-invalid-chirho",
    "consent-missing-chirho",
    "publication-pending-chirho",
    "close-account-failed-chirho",
];
/// The typed consent a close-account form must carry.
pub const CLOSE_ACCOUNT_CONSENT_CHIRHO: &str = "close-account-chirho";

/// The engine roads an Account form may post to. Each is `None` until that road is on the pin;
/// a page renders exactly the forms whose road is bound.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AccountBindingV1Chirho {
    pub(crate) change_password_action_chirho: Option<String>,
    pub(crate) recovery_code_action_chirho: Option<String>,
    pub(crate) close_account_action_chirho: Option<String>,
    /// The form road that ends one of her AI connections (`connections_chirho`).
    pub(crate) disconnect_action_chirho: Option<String>,
    /// The entry to the engine's form that creates a connection. Bound only explicitly, never with the list alone.
    pub(crate) connect_href_chirho: Option<String>,
}

impl AccountBindingV1Chirho {
    pub fn held_chirho() -> Self {
        Self::default()
    }

    pub fn with_change_password_action_chirho(mut self, action_chirho: &str) -> Self {
        self.change_password_action_chirho = Some(action_chirho.to_string());
        self
    }

    pub fn with_recovery_code_action_chirho(mut self, action_chirho: &str) -> Self {
        self.recovery_code_action_chirho = Some(action_chirho.to_string());
        self
    }

    pub fn with_close_account_action_chirho(mut self, action_chirho: &str) -> Self {
        self.close_account_action_chirho = Some(action_chirho.to_string());
        self
    }
    pub fn with_disconnect_action_chirho(mut self, action_chirho: &str) -> Self {
        self.disconnect_action_chirho = Some(action_chirho.to_string());
        self
    }

    pub fn with_connect_entry_chirho(mut self, href_chirho: &str) -> Self {
        self.connect_href_chirho = Some(href_chirho.to_string());
        self
    }

    pub fn is_held_chirho(&self) -> bool {
        self.change_password_action_chirho.is_none()
            && self.recovery_code_action_chirho.is_none()
            && self.close_account_action_chirho.is_none()
            && self.disconnect_action_chirho.is_none()
            && self.connect_href_chirho.is_none()
    }
}

/// One pre-rendered refusal: the element id the road's redirect targets, and her sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountRefusalViewChirho {
    pub(crate) id_chirho: String,
    pub(crate) sentence_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountPageV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) status_chirho: String,
    pub(crate) binding_chirho: AccountBindingV1Chirho,
    /// Present only with the change-password road bound.
    pub(crate) refusals_chirho: Vec<AccountRefusalViewChirho>,
    /// Where an engine road sends her back after the act (the page itself).
    pub(crate) return_href_chirho: String,
    pub(crate) new_password_min_characters_chirho: usize,
    pub(crate) close_account_consent_chirho: &'static str,
    /// Her AI-assistant connections: held until the engine's list input is bound.
    pub(crate) connections_chirho: ConnectionsViewChirho,
    /// The connections section's number: next after the numbered sections shown above it.
    pub(crate) connections_number_chirho: String,
}

impl AccountPageV1Chirho {
    pub fn held_chirho() -> Self {
        Self::for_locale_chirho(
            AccountBindingV1Chirho::held_chirho(),
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn bound_chirho(binding_chirho: AccountBindingV1Chirho) -> Self {
        Self::for_locale_chirho(binding_chirho, UiLocaleChirho::EnglishChirho)
    }

    pub fn for_locale_chirho(
        binding_chirho: AccountBindingV1Chirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        Self::with_connections_chirho(
            binding_chirho,
            ConnectionsStateChirho::HeldChirho,
            locale_chirho,
        )
    }
    /// The page with what the engine gave of her connections this time.
    pub fn with_connections_chirho(
        binding_chirho: AccountBindingV1Chirho,
        connections_state_chirho: ConnectionsStateChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        // The password section (its form or its held notice) is always 01; recovery and closing are
        // numbered only when bound.
        let numbered_above_chirho = 1
            + usize::from(binding_chirho.recovery_code_action_chirho.is_some())
            + usize::from(binding_chirho.close_account_action_chirho.is_some());
        let connections_number_chirho = format!("{:02}", numbered_above_chirho + 1);
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let connections_chirho = ConnectionsViewChirho::new_chirho(
            connections_state_chirho,
            binding_chirho.disconnect_action_chirho.clone(),
            binding_chirho.connect_href_chirho.clone(),
            &copy_chirho.connections_chirho,
        );
        let status_chirho = if binding_chirho.is_held_chirho() {
            copy_chirho.account_chirho.status_held_chirho.clone()
        } else {
            copy_chirho.account_chirho.status_ready_chirho.clone()
        };
        let chrome_chirho = account_chrome_chirho(&copy_chirho);
        // Each bound road pre-renders the refusals that return here; a token shared by two roads
        // is one alert on the page.
        let mut tokens_chirho: Vec<&'static str> = Vec::new();
        let roads_chirho: [(bool, &[&'static str]); 4] = [
            (
                binding_chirho.change_password_action_chirho.is_some(),
                &CHANGE_PASSWORD_REFUSAL_TOKENS_CHIRHO,
            ),
            (
                binding_chirho.recovery_code_action_chirho.is_some(),
                &RECOVERY_CODE_REFUSAL_TOKENS_CHIRHO,
            ),
            (
                binding_chirho.close_account_action_chirho.is_some(),
                &CLOSE_ACCOUNT_REFUSAL_TOKENS_CHIRHO,
            ),
            // only with a list to disconnect from: a held or unavailable list offers no form
            (
                connections_chirho.disconnect_bound_chirho(),
                &DISCONNECT_REFUSAL_TOKENS_CHIRHO,
            ),
        ];
        for (bound_chirho, road_tokens_chirho) in roads_chirho {
            if bound_chirho {
                for token_chirho in road_tokens_chirho {
                    if !tokens_chirho.contains(token_chirho) {
                        tokens_chirho.push(token_chirho);
                    }
                }
            }
        }
        let refusals_chirho = tokens_chirho
            .iter()
            .map(|token_chirho| AccountRefusalViewChirho {
                id_chirho: format!("refusal-{token_chirho}"),
                sentence_chirho: refusal_sentence_chirho(&copy_chirho, token_chirho).to_string(),
            })
            .collect();
        Self {
            chrome_chirho,
            copy_chirho,
            status_chirho,
            binding_chirho,
            refusals_chirho,
            return_href_chirho: WORKSPACE_ACCOUNT_HREF_CHIRHO.to_string(),
            new_password_min_characters_chirho: NEW_PASSWORD_MIN_CHARACTERS_CHIRHO,
            close_account_consent_chirho: CLOSE_ACCOUNT_CONSENT_CHIRHO,
            connections_chirho,
            connections_number_chirho,
        }
    }

    pub fn is_held_chirho(&self) -> bool {
        self.binding_chirho.is_held_chirho()
    }
}

/// A disconnect's own refusals read from the connections copy; every other token is the Account's.
fn refusal_sentence_chirho<'copy_chirho>(
    copy_chirho: &'copy_chirho WorkspaceCopyChirho,
    token_chirho: &str,
) -> &'copy_chirho str {
    match token_chirho {
        "no-such-key-chirho" => &copy_chirho.connections_chirho.refusal_no_such_key_chirho,
        "keys-failed-chirho" => &copy_chirho.connections_chirho.refusal_failed_chirho,
        _ => copy_chirho
            .account_chirho
            .refusal_sentence_chirho(token_chirho),
    }
}

fn account_chrome_chirho(copy_chirho: &WorkspaceCopyChirho) -> SiteChromeChirho {
    workspace_chrome_chirho(
        copy_chirho,
        &copy_chirho.account_chirho.page_title_chirho,
        &copy_chirho.account_chirho.page_description_chirho,
        WorkspaceNavPageChirho::AccountChirho,
        WorkspaceNavRoadsChirho {
            account_chirho: true,
            ..WorkspaceNavRoadsChirho::deployed_chirho()
        },
    )
}
