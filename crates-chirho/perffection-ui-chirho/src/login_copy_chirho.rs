// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Public sign-in display copy. Form destinations and credential names remain app-owned contracts.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub help_label_chirho: String,
    pub signin_label_chirho: String,
    pub eyebrow_chirho: String,
    pub heading_chirho: String,
    pub intro_chirho: String,
    pub skip_to_form_chirho: String,
    pub principal_label_chirho: String,
    pub principal_help_chirho: String,
    pub credential_label_chirho: String,
    pub credential_help_chirho: String,
    pub submit_label_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
    pub help_link_chirho: String,
    pub retry_heading_chirho: String,
    pub retry_body_chirho: String,
    /// The help panel's pointer for an invitee who has a setup code but no password yet: the Set-up page.
    pub setup_heading_chirho: String,
    pub setup_body_chirho: String,
    pub setup_link_chirho: String,
    pub no_session_heading_chirho: String,
    pub no_session_body_chirho: String,
    pub exhausted_heading_chirho: String,
    pub exhausted_body_chirho: String,
    pub invalid_credential_heading_chirho: String,
    pub invalid_credential_body_chirho: String,
    pub rate_limited_heading_chirho: String,
    pub rate_limited_body_chirho: String,
    /// The calm notice after the recover road sets a password (setup or recovery): `#notice-password-set-chirho`.
    pub password_set_heading_chirho: String,
    pub password_set_body_chirho: String,
    /// The calm notice after the close road accepts a closure (R3): `#notice-account-closed-chirho`. It says she is closed
    /// and signed out, and claims nothing about erasing public records, replicas or backups.
    pub account_closed_heading_chirho: String,
    pub account_closed_body_chirho: String,
    pub dismiss_notice_chirho: String,
}

impl LoginCopyChirho {
    pub fn english_chirho() -> Self {
        Self::for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/account-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the embedded typed login catalog must remain valid")
    }
}
