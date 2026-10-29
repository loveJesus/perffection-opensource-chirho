// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Customer auth display copy only. No form destinations or authority come from this catalog.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthNoticeCopyChirho {
    pub heading_chirho: String,
    pub body_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub preview_marker_chirho: String,
    pub page_description_chirho: String,
    pub signup_chirho: AuthNoticeCopyChirho,
    pub signin_chirho: AuthNoticeCopyChirho,
    pub success_chirho: AuthNoticeCopyChirho,
    pub unavailable_chirho: AuthNoticeCopyChirho,
    pub username_label_chirho: String,
    pub username_help_chirho: String,
    pub new_password_help_chirho: String,
    pub challenge_label_chirho: String,
    pub challenge_help_chirho: String,
    pub signup_submit_chirho: String,
    pub signin_submit_chirho: String,
    pub availability_prompt_chirho: String,
    pub availability_link_chirho: String,
    pub start_again_chirho: String,
    pub continue_chirho: String,
    pub skip_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
    pub refusals_chirho: BTreeMap<String, AuthNoticeCopyChirho>,
}

impl AuthCopyChirho {
    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/auth-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the embedded auth display catalog must remain valid")
    }

    /// Unknown future codes remain a fixed, non-refining refusal, never engine prose.
    pub fn refusal_chirho(&self, code_chirho: &str) -> &AuthNoticeCopyChirho {
        self.refusals_chirho
            .get(code_chirho)
            .unwrap_or(&self.unavailable_chirho)
    }
}
