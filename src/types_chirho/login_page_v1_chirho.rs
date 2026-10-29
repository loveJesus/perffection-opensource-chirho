// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Public issued-account display model. Copy is a locale catalog; destinations are engine contracts.
//! Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/public-signin-chirho.md

use crate::types_chirho::setup_page_v1_chirho::SETUP_DISPLAY_PATH_CHIRHO;
use crate::types_chirho::site_chrome_chirho::{
    public_navigation_chirho, ChromeNavLinkChirho, SiteChromeChirho,
};
use perffection_ui_chirho::{PublicSiteCopyChirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

/// The rail and the footer say what every other public page says. A separate account catalog once
/// gave this page its own ("perffection", "Private beta", "Home", "Craft"), so a visitor moving from
/// the front door to sign-in saw a different site (2026-10-01).
const PUBLIC_SITE_COPY_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/en-chirho.json");

pub const LOGIN_ACTION_CHIRHO: &str = "/auth-chirho/login-chirho";
pub const PRINCIPAL_FIELD_CHIRHO: &str = "principal_chirho";
pub const CREDENTIAL_FIELD_CHIRHO: &str = "credential_chirho";
pub const WORKSPACE_REDIRECT_CHIRHO: &str = "/app-chirho/perffection-workspace/";
pub const SIGNIN_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/login-chirho";
// Fixed auth fragments are display-only; they grant no authority and are never request inputs.
pub const NO_SESSION_FRAGMENT_CHIRHO: &str = "refusal-no-session-chirho";
pub const EXHAUSTED_FRAGMENT_CHIRHO: &str = "refusal-credential-guess-exhausted-chirho";
pub const INVALID_CREDENTIAL_FRAGMENT_CHIRHO: &str = "refusal-invalid-credential-chirho";
pub const RATE_LIMITED_FRAGMENT_CHIRHO: &str = "refusal-auth-rate-limited-chirho";
/// The recover road's success (R2 revision 3): a 303 here after it sets a password, for setup and recovery alike.
pub const PASSWORD_SET_NOTICE_FRAGMENT_CHIRHO: &str = "notice-password-set-chirho";
/// The close road's accepted closure (R3): it revokes every session, clears the cookie and 303s here, to a public page
/// the dead session can still open (account-recovery-chirho.md).
pub const ACCOUNT_CLOSED_NOTICE_FRAGMENT_CHIRHO: &str = "notice-account-closed-chirho";

pub use perffection_ui_chirho::LoginCopyChirho;

/// Existing public pages share the account entry label and route without duplicating its copy.
pub fn signin_navigation_link_chirho() -> ChromeNavLinkChirho {
    ChromeNavLinkChirho::link_chirho(
        &LoginCopyChirho::english_chirho().signin_label_chirho,
        LOGIN_ACTION_CHIRHO,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginNoticeChirho {
    pub fragment_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginPageV1Chirho {
    pub chrome_chirho: SiteChromeChirho,
    pub copy_chirho: LoginCopyChirho,
    pub action_chirho: String,
    pub principal_field_chirho: String,
    pub credential_field_chirho: String,
    pub redirect_chirho: String,
    pub display_path_chirho: String,
    /// Where an invitee with a setup code chooses a first password (R12's Set-up page), linked from the help panel.
    pub setup_path_chirho: String,
    pub notices_chirho: Vec<LoginNoticeChirho>,
    /// The calm notices (password set, account closed): each a status, not an alert, with no "try again", since nothing
    /// went wrong.
    pub status_notices_chirho: Vec<LoginNoticeChirho>,
}

impl LoginPageV1Chirho {
    pub fn default_public_chirho() -> Self {
        Self::from_copy_chirho(LoginCopyChirho::english_chirho())
    }

    /// A locale changes presentation, never the form destination or workspace binding.
    pub fn from_copy_chirho(copy_chirho: LoginCopyChirho) -> Self {
        let site_chirho = PublicSiteCopyChirho::from_json_for_locale_chirho(
            PUBLIC_SITE_COPY_CATALOG_CHIRHO,
            UiLocaleChirho::EnglishChirho,
        );
        Self {
            chrome_chirho: SiteChromeChirho {
                page_title_chirho: copy_chirho.page_title_chirho.clone(),
                page_description_chirho: copy_chirho.page_description_chirho.clone(),
                stylesheet_name_chirho: "login-chirho.css".to_string(),
                preview_marker_chirho: site_chirho.preview_marker_chirho.clone(),
                brand_chirho: site_chirho.brand_chirho.clone(),
                brand_href_chirho: "/app-chirho/perffection/".to_string(),
                nav_label_chirho: site_chirho.nav_label_chirho.clone(),
                nav_links_chirho: public_navigation_chirho(
                    SIGNIN_DISPLAY_PATH_CHIRHO,
                    &site_chirho.home_label_chirho,
                    &site_chirho.craft_label_chirho,
                    &site_chirho.trust_label_chirho,
                    &site_chirho.help_label_chirho,
                    &site_chirho.signin_label_chirho,
                ),
                footer_note_chirho: Some(site_chirho.footer_note_chirho.clone()),
            },
            notices_chirho: [
                (
                    NO_SESSION_FRAGMENT_CHIRHO,
                    &copy_chirho.no_session_heading_chirho,
                    &copy_chirho.no_session_body_chirho,
                ),
                (
                    EXHAUSTED_FRAGMENT_CHIRHO,
                    &copy_chirho.exhausted_heading_chirho,
                    &copy_chirho.exhausted_body_chirho,
                ),
                (
                    INVALID_CREDENTIAL_FRAGMENT_CHIRHO,
                    &copy_chirho.invalid_credential_heading_chirho,
                    &copy_chirho.invalid_credential_body_chirho,
                ),
                (
                    RATE_LIMITED_FRAGMENT_CHIRHO,
                    &copy_chirho.rate_limited_heading_chirho,
                    &copy_chirho.rate_limited_body_chirho,
                ),
            ]
            .into_iter()
            .map(
                |(fragment_chirho, heading_chirho, body_chirho)| LoginNoticeChirho {
                    fragment_chirho: fragment_chirho.to_string(),
                    heading_chirho: heading_chirho.clone(),
                    body_chirho: body_chirho.clone(),
                },
            )
            .collect(),
            status_notices_chirho: [
                (
                    PASSWORD_SET_NOTICE_FRAGMENT_CHIRHO,
                    &copy_chirho.password_set_heading_chirho,
                    &copy_chirho.password_set_body_chirho,
                ),
                (
                    ACCOUNT_CLOSED_NOTICE_FRAGMENT_CHIRHO,
                    &copy_chirho.account_closed_heading_chirho,
                    &copy_chirho.account_closed_body_chirho,
                ),
            ]
            .into_iter()
            .map(
                |(fragment_chirho, heading_chirho, body_chirho)| LoginNoticeChirho {
                    fragment_chirho: fragment_chirho.to_string(),
                    heading_chirho: heading_chirho.clone(),
                    body_chirho: body_chirho.clone(),
                },
            )
            .collect(),
            copy_chirho,
            action_chirho: LOGIN_ACTION_CHIRHO.to_string(),
            principal_field_chirho: PRINCIPAL_FIELD_CHIRHO.to_string(),
            credential_field_chirho: CREDENTIAL_FIELD_CHIRHO.to_string(),
            redirect_chirho: WORKSPACE_REDIRECT_CHIRHO.to_string(),
            display_path_chirho: SIGNIN_DISPLAY_PATH_CHIRHO.to_string(),
            setup_path_chirho: SETUP_DISPLAY_PATH_CHIRHO.to_string(),
        }
    }
}
