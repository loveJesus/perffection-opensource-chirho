// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Product chrome belongs to the shared UI crate; this re-export keeps page imports concise.
pub use perffection_ui_chirho::{ChromeNavLinkChirho, SiteChromeChirho};

pub const HOME_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/";
pub const CRAFT_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/craft-chirho";
pub const TRUST_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/trust-chirho";
pub const HELP_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/help-chirho";

/// Where an invitation is asked for, and where invitations come from (L.J., 2026-10-02: "use aleluya"). A Mailu
/// mailbox; invitations go out through 2SMTP from it. The builder's separately approved HTTPS
/// attribution is not an invitation or account-action destination.
/// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/emailed-invitation-chirho.md
pub const INVITATION_ADDRESS_CHIRHO: &str = "aleluya@perffection.com";
pub const INVITATION_MAILTO_CHIRHO: &str = "mailto:aleluya@perffection.com";

/// One order and route map for the public app. Auth display belongs to the canonical service;
/// the issued-account deployment may redirect its GET to the older public display.
/// Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/public-signin-chirho.md.
pub fn public_navigation_chirho(
    current_path_chirho: &str,
    home_label_chirho: &str,
    craft_label_chirho: &str,
    trust_label_chirho: &str,
    help_label_chirho: &str,
    signin_label_chirho: &str,
) -> Vec<ChromeNavLinkChirho> {
    use super::login_page_v1_chirho::{LOGIN_ACTION_CHIRHO, SIGNIN_DISPLAY_PATH_CHIRHO};

    let current_path_chirho = if current_path_chirho == SIGNIN_DISPLAY_PATH_CHIRHO {
        LOGIN_ACTION_CHIRHO
    } else {
        current_path_chirho
    };
    [
        (home_label_chirho, HOME_DISPLAY_PATH_CHIRHO),
        (craft_label_chirho, CRAFT_DISPLAY_PATH_CHIRHO),
        (trust_label_chirho, TRUST_DISPLAY_PATH_CHIRHO),
        (help_label_chirho, HELP_DISPLAY_PATH_CHIRHO),
        (signin_label_chirho, LOGIN_ACTION_CHIRHO),
    ]
    .into_iter()
    .map(|(label_chirho, path_chirho)| {
        if path_chirho == current_path_chirho {
            ChromeNavLinkChirho::current_chirho(label_chirho, path_chirho)
        } else {
            ChromeNavLinkChirho::link_chirho(label_chirho, path_chirho)
        }
    })
    .collect()
}
