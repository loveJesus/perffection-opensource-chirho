// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared source model for public and private Perffection product chrome.
//!
//! Askama includes read this at the single path `model_chirho.chrome_chirho`. All fields retain the
//! names and meanings of the original public-site contract so existing page models can migrate
//! without a second adapter shape.

use serde::{Deserialize, Serialize};

/// L.J.'s requested product-builder attribution (2026-10-07), not a customer app destination.
/// Workflow: spec-chirho/workflows-chirho/first-light-authoring-chirho.md.
pub const LOVEJESUS_SOFTWARE_URL_CHIRHO: &str = "https://loveJesus.software/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChromeNavLinkChirho {
    pub label_chirho: String,
    pub href_chirho: String,
    pub is_cta_chirho: bool,
    pub is_current_chirho: bool,
}

impl ChromeNavLinkChirho {
    pub fn link_chirho(label_chirho: &str, href_chirho: &str) -> Self {
        Self {
            label_chirho: label_chirho.to_string(),
            href_chirho: href_chirho.to_string(),
            is_cta_chirho: false,
            is_current_chirho: false,
        }
    }

    pub fn current_chirho(label_chirho: &str, href_chirho: &str) -> Self {
        Self {
            label_chirho: label_chirho.to_string(),
            href_chirho: href_chirho.to_string(),
            is_cta_chirho: false,
            is_current_chirho: true,
        }
    }

    pub fn cta_chirho(label_chirho: &str, href_chirho: &str) -> Self {
        Self {
            label_chirho: label_chirho.to_string(),
            href_chirho: href_chirho.to_string(),
            is_cta_chirho: true,
            is_current_chirho: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SiteChromeChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub stylesheet_name_chirho: String,
    pub preview_marker_chirho: String,
    pub brand_chirho: String,
    pub brand_href_chirho: String,
    pub nav_label_chirho: String,
    pub nav_links_chirho: Vec<ChromeNavLinkChirho>,
    pub footer_note_chirho: Option<String>,
}

impl SiteChromeChirho {
    pub fn builder_url_chirho(&self) -> &'static str {
        LOVEJESUS_SOFTWARE_URL_CHIRHO
    }
}
