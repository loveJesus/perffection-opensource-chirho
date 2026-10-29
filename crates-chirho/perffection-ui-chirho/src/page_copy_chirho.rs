// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Presentation-only copy for the shared published-page and private-preview renderer.
//! A catalog never chooses customer content, contacts, hrefs or publication authority.
//! Workflow: `spec-chirho/workflows-chirho/product-delivery-chirho/customer-page-rendering-chirho.md`.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub skip_label_chirho: String,
    pub preview_label_chirho: String,
    pub preview_note_chirho: String,
    pub preview_return_label_chirho: String,
    pub preview_title_template_chirho: String,
    pub nav_label_chirho: String,
    pub services_label_chirho: String,
    pub contact_label_chirho: String,
    pub welcome_label_chirho: String,
    pub contact_action_label_chirho: String,
    pub services_eyebrow_chirho: String,
    pub services_heading_chirho: String,
    pub contact_eyebrow_chirho: String,
    pub call_label_chirho: String,
    pub email_label_chirho: String,
    pub footer_note_chirho: String,
    pub unavailable_title_chirho: String,
    pub unavailable_heading_chirho: String,
}

impl PageCopyChirho {
    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/page-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the bundled customer-page catalog must match its typed model")
    }

    pub fn preview_title_chirho(&self, business_name_chirho: &str) -> String {
        self.preview_title_template_chirho
            .replace("{business_name_chirho}", business_name_chirho)
    }
}
