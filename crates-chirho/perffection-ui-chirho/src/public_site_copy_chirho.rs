// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared public-site chrome copy without page content, stylesheet choice or route authority.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicSiteCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub brand_chirho: String,
    pub preview_marker_chirho: String,
    pub nav_label_chirho: String,
    pub home_label_chirho: String,
    pub craft_label_chirho: String,
    pub trust_label_chirho: String,
    pub help_label_chirho: String,
    pub signin_label_chirho: String,
    pub footer_note_chirho: String,
}

impl PublicSiteCopyChirho {
    pub fn from_json_for_locale_chirho(
        catalog_json_chirho: &str,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        load_catalog_chirho(catalog_json_chirho, locale_chirho)
            .expect("the embedded typed public-site catalog must remain valid")
    }
}
