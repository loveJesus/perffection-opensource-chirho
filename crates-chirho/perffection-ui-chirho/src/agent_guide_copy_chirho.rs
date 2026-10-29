// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Human-facing guide copy only. The machine instruction belongs to the document contract.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentGuideCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub eyebrow_chirho: String,
    pub heading_chirho: String,
    pub introduction_chirho: String,
    pub skip_label_chirho: String,
    pub steps_heading_chirho: String,
    pub steps_chirho: Vec<String>,
    pub instructions_heading_chirho: String,
    pub instructions_help_chirho: String,
    pub sample_heading_chirho: String,
    pub privacy_note_chirho: String,
    pub review_heading_chirho: String,
    pub review_body_chirho: String,
    pub review_label_chirho: String,
    pub signin_needed_chirho: String,
    pub cost_note_chirho: String,
    pub limits_heading_chirho: String,
    pub limits_body_chirho: String,
    pub key_label_chirho: String,
    pub limit_label_chirho: String,
    pub handling_label_chirho: String,
    pub copied_label_chirho: String,
    pub written_label_chirho: String,
    pub services_label_chirho: String,
}

impl AgentGuideCopyChirho {
    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/agent-guide-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the embedded typed agent guide catalog must remain valid")
    }
}
