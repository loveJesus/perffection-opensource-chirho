// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Public account set-up display model: the ONE form of an emailed invitation (R2 revision 3, Cairn #26033; PFE
//! #26029). She types her sign-in name, the setup code from the email and her first password, and the recover road
//! sets the password and spends the code in one act. The code is TYPED, never carried in a link, so it reaches no
//! access log (agy #25977). It is routed (`setup-chirho`) from R12, the engine release that serves Cairn's R2b recover
//! road; before that a form would post to a missing road. Success is the road's 303 to the sign-in page, whose
//! `#notice-password-set-chirho` says so; every refusal comes back here as `#refusal-<token>`.
//! Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/emailed-invitation-chirho.md

use crate::types_chirho::login_page_v1_chirho::PRINCIPAL_FIELD_CHIRHO;
use crate::types_chirho::site_chrome_chirho::{public_navigation_chirho, SiteChromeChirho};
use perffection_ui_chirho::NEW_PASSWORD_MIN_CHARACTERS_CHIRHO;
use perffection_ui_chirho::{PublicSiteCopyChirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

const PUBLIC_SITE_COPY_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/en-chirho.json");

/// R2's served recover road (revision 3): one road for recovery and setup, form or JSON.
pub const SETUP_ACTION_CHIRHO: &str = "/auth-chirho/recover-chirho";
/// Revision 3's field names (Cairn #26033, item 1).
pub const SETUP_CODE_FIELD_CHIRHO: &str = "recovery_code_chirho";
pub const SETUP_NEW_CREDENTIAL_FIELD_CHIRHO: &str = "new_credential_chirho";
/// Optional on the road; when present it must equal the new password (Cairn #26040, R2b), so a typo spends no code.
pub const SETUP_CONFIRM_CREDENTIAL_FIELD_CHIRHO: &str = "new_credential_confirm_chirho";
pub const SETUP_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/setup-chirho";
/// The page names itself as the target, so every refusal lands back here (revision 3, item 2).
pub const SETUP_REDIRECT_CHIRHO: &str = SETUP_DISPLAY_PATH_CHIRHO;
// The road's refusal tokens, each pre-rendered as `refusal-<token>`: fixed display copy, never request input.
pub const CREDENTIAL_TOO_WEAK_FRAGMENT_CHIRHO: &str = "refusal-credential-too-weak-chirho";
/// Checked before the floor: when the two differ, "they did not match" is the true message.
pub const CONFIRMATION_MISMATCH_FRAGMENT_CHIRHO: &str =
    "refusal-credential-confirmation-mismatch-chirho";
/// The uniform refusal: an unknown name and a missing, wrong, used or expired code read alike.
pub const RECOVERY_REFUSED_FRAGMENT_CHIRHO: &str = "refusal-recovery-refused-chirho";
/// Every rate refusal on this road, its own limiter and 4a-1's per-source gate alike (Cairn #26040).
pub const RECOVER_RATE_LIMITED_FRAGMENT_CHIRHO: &str = "refusal-rate-limited-chirho";
pub const REQUEST_INVALID_FRAGMENT_CHIRHO: &str = "refusal-request-invalid-chirho";
/// Before the road's commit point: the password was NOT set and the code still works (R13, Cairn #26146, PFE #26147).
/// Sessions may already have ended, so its words never say "nothing changed".
pub const CREDENTIAL_NOT_SET_FRAGMENT_CHIRHO: &str = "refusal-credential-not-set-chirho";
/// After the road's commit point: the password may already be set.
pub const RECOVERY_FAILED_FRAGMENT_CHIRHO: &str = "refusal-recovery-failed-chirho";

pub use perffection_ui_chirho::SetupCopyChirho;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupNoticeChirho {
    pub fragment_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupPageV1Chirho {
    pub chrome_chirho: SiteChromeChirho,
    pub copy_chirho: SetupCopyChirho,
    pub action_chirho: String,
    pub principal_field_chirho: String,
    pub code_field_chirho: String,
    pub new_credential_field_chirho: String,
    pub confirm_credential_field_chirho: String,
    pub new_password_min_characters_chirho: usize,
    pub redirect_chirho: String,
    pub display_path_chirho: String,
    pub notices_chirho: Vec<SetupNoticeChirho>,
}

impl SetupPageV1Chirho {
    pub fn default_public_chirho() -> Self {
        Self::from_copy_chirho(SetupCopyChirho::english_chirho())
    }

    /// A locale changes presentation, never the form destination, its fields or the redirect.
    pub fn from_copy_chirho(copy_chirho: SetupCopyChirho) -> Self {
        let site_chirho = PublicSiteCopyChirho::from_json_for_locale_chirho(
            PUBLIC_SITE_COPY_CATALOG_CHIRHO,
            UiLocaleChirho::EnglishChirho,
        );
        Self {
            chrome_chirho: SiteChromeChirho {
                page_title_chirho: copy_chirho.page_title_chirho.clone(),
                page_description_chirho: copy_chirho.page_description_chirho.clone(),
                // The sign-in page's layout and styles, shared rather than copied.
                stylesheet_name_chirho: "login-chirho.css".to_string(),
                preview_marker_chirho: site_chirho.preview_marker_chirho.clone(),
                brand_chirho: site_chirho.brand_chirho.clone(),
                brand_href_chirho: "/app-chirho/perffection/".to_string(),
                nav_label_chirho: site_chirho.nav_label_chirho.clone(),
                nav_links_chirho: public_navigation_chirho(
                    SETUP_DISPLAY_PATH_CHIRHO,
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
                    CONFIRMATION_MISMATCH_FRAGMENT_CHIRHO,
                    &copy_chirho.mismatch_heading_chirho,
                    &copy_chirho.mismatch_body_chirho,
                ),
                (
                    CREDENTIAL_TOO_WEAK_FRAGMENT_CHIRHO,
                    &copy_chirho.too_weak_heading_chirho,
                    &copy_chirho.too_weak_body_chirho,
                ),
                (
                    RECOVERY_REFUSED_FRAGMENT_CHIRHO,
                    &copy_chirho.refused_heading_chirho,
                    &copy_chirho.refused_body_chirho,
                ),
                (
                    RECOVER_RATE_LIMITED_FRAGMENT_CHIRHO,
                    &copy_chirho.rate_limited_heading_chirho,
                    &copy_chirho.rate_limited_body_chirho,
                ),
                (
                    REQUEST_INVALID_FRAGMENT_CHIRHO,
                    &copy_chirho.request_invalid_heading_chirho,
                    &copy_chirho.request_invalid_body_chirho,
                ),
                (
                    CREDENTIAL_NOT_SET_FRAGMENT_CHIRHO,
                    &copy_chirho.not_set_heading_chirho,
                    &copy_chirho.not_set_body_chirho,
                ),
                (
                    RECOVERY_FAILED_FRAGMENT_CHIRHO,
                    &copy_chirho.failed_heading_chirho,
                    &copy_chirho.failed_body_chirho,
                ),
            ]
            .into_iter()
            .map(
                |(fragment_chirho, heading_chirho, body_chirho)| SetupNoticeChirho {
                    fragment_chirho: fragment_chirho.to_string(),
                    heading_chirho: heading_chirho.clone(),
                    body_chirho: body_chirho.clone(),
                },
            )
            .collect(),
            copy_chirho,
            action_chirho: SETUP_ACTION_CHIRHO.to_string(),
            principal_field_chirho: PRINCIPAL_FIELD_CHIRHO.to_string(),
            code_field_chirho: SETUP_CODE_FIELD_CHIRHO.to_string(),
            new_credential_field_chirho: SETUP_NEW_CREDENTIAL_FIELD_CHIRHO.to_string(),
            confirm_credential_field_chirho: SETUP_CONFIRM_CREDENTIAL_FIELD_CHIRHO.to_string(),
            new_password_min_characters_chirho: NEW_PASSWORD_MIN_CHARACTERS_CHIRHO,
            redirect_chirho: SETUP_REDIRECT_CHIRHO.to_string(),
            display_path_chirho: SETUP_DISPLAY_PATH_CHIRHO.to_string(),
        }
    }
}
