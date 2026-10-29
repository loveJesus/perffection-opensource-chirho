// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Customer guidance is presentation data. It carries no route, credential or effect authority.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpTopicCopyChirho {
    pub heading_chirho: String,
    pub introduction_chirho: String,
    pub steps_chirho: Vec<String>,
    pub question_chirho: String,
    pub answer_chirho: String,
    pub action_label_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub eyebrow_chirho: String,
    pub heading_chirho: String,
    pub introduction_chirho: String,
    pub skip_label_chirho: String,
    pub topics_label_chirho: String,
    pub start_heading_chirho: String,
    pub start_body_chirho: String,
    pub workspace_label_chirho: String,
    pub signin_needed_chirho: String,
    pub back_to_topics_chirho: String,
    pub access_chirho: HelpTopicCopyChirho,
    pub save_chirho: HelpTopicCopyChirho,
    pub publish_chirho: HelpTopicCopyChirho,
    pub history_chirho: HelpTopicCopyChirho,
    pub export_chirho: HelpTopicCopyChirho,
    pub own_agent_chirho: HelpTopicCopyChirho,
    pub privacy_chirho: HelpTopicCopyChirho,
}

impl HelpCopyChirho {
    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/help-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the embedded typed help catalog must remain valid")
    }
}
