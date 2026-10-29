// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Public account set-up display copy: the ONE form of an emailed invitation (R2 revision 3: sign-in name, setup code
//! and a first password). Form destinations and field names remain app-owned contracts. The words are a draft for L.J.
//! to settle.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetupCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub eyebrow_chirho: String,
    pub heading_chirho: String,
    pub intro_chirho: String,
    pub skip_to_form_chirho: String,
    pub principal_label_chirho: String,
    pub principal_help_chirho: String,
    pub code_label_chirho: String,
    pub code_help_chirho: String,
    pub new_password_label_chirho: String,
    pub new_password_help_chirho: String,
    pub confirm_password_label_chirho: String,
    pub confirm_password_help_chirho: String,
    pub submit_label_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
    pub too_weak_heading_chirho: String,
    pub too_weak_body_chirho: String,
    pub mismatch_heading_chirho: String,
    pub mismatch_body_chirho: String,
    /// The uniform refusal: an unknown name and a missing, wrong, used or expired code read alike.
    pub refused_heading_chirho: String,
    pub refused_body_chirho: String,
    pub rate_limited_heading_chirho: String,
    pub rate_limited_body_chirho: String,
    pub request_invalid_heading_chirho: String,
    pub request_invalid_body_chirho: String,
    /// Before the commit point (R13): the password was not set, and the code still works.
    pub not_set_heading_chirho: String,
    pub not_set_body_chirho: String,
    /// After the commit point: the password may already be set.
    pub failed_heading_chirho: String,
    pub failed_body_chirho: String,
    pub dismiss_notice_chirho: String,
}

impl SetupCopyChirho {
    pub fn english_chirho() -> Self {
        Self::for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/setup-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the embedded typed set-up catalog must remain valid")
    }
}
