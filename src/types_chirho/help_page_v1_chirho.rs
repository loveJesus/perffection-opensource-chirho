// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Public, actionless help for the existing customer journey. Copy never chooses a destination.
//! Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/customer-help-chirho.md.

use super::login_page_v1_chirho::{LOGIN_ACTION_CHIRHO, WORKSPACE_REDIRECT_CHIRHO};
use super::site_chrome_chirho::{
    public_navigation_chirho, SiteChromeChirho, HELP_DISPLAY_PATH_CHIRHO, HOME_DISPLAY_PATH_CHIRHO,
    TRUST_DISPLAY_PATH_CHIRHO,
};
use perffection_ui_chirho::{
    HelpCopyChirho, HelpTopicCopyChirho, PublicSiteCopyChirho, UiLocaleChirho,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpTopicChirho {
    pub anchor_chirho: String,
    pub copy_chirho: HelpTopicCopyChirho,
    pub action_href_chirho: String,
    pub needs_signin_chirho: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpPageV1Chirho {
    pub chrome_chirho: SiteChromeChirho,
    pub copy_chirho: HelpCopyChirho,
    pub topics_chirho: Vec<HelpTopicChirho>,
    pub signin_href_chirho: String,
    pub signin_label_chirho: String,
    pub workspace_href_chirho: String,
}

impl HelpPageV1Chirho {
    pub fn default_public_chirho() -> Self {
        Self::for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    /// Explicit authoring locale, not a request input. The render guest ignores request content.
    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        Self::from_catalog_chirho(
            PublicSiteCopyChirho::from_json_for_locale_chirho(
                include_str!("../../locales-chirho/site-chirho/en-chirho.json"),
                locale_chirho,
            ),
            HelpCopyChirho::for_locale_chirho(locale_chirho),
        )
    }

    pub fn from_catalog_chirho(
        site_copy_chirho: PublicSiteCopyChirho,
        copy_chirho: HelpCopyChirho,
    ) -> Self {
        let topic_chirho = |anchor_chirho: &str,
                            topic_copy_chirho: &HelpTopicCopyChirho,
                            action_href_chirho: &str,
                            needs_signin_chirho: bool| HelpTopicChirho {
            anchor_chirho: anchor_chirho.to_owned(),
            copy_chirho: topic_copy_chirho.clone(),
            action_href_chirho: action_href_chirho.to_owned(),
            needs_signin_chirho,
        };
        let topics_chirho = vec![
            topic_chirho(
                "access-chirho",
                &copy_chirho.access_chirho,
                LOGIN_ACTION_CHIRHO,
                false,
            ),
            topic_chirho(
                "save-chirho",
                &copy_chirho.save_chirho,
                WORKSPACE_REDIRECT_CHIRHO,
                true,
            ),
            topic_chirho(
                "publish-chirho",
                &copy_chirho.publish_chirho,
                "/app-chirho/perffection-workspace/publication-status-chirho",
                true,
            ),
            topic_chirho(
                "history-chirho",
                &copy_chirho.history_chirho,
                WORKSPACE_REDIRECT_CHIRHO,
                true,
            ),
            topic_chirho(
                "export-chirho",
                &copy_chirho.export_chirho,
                "/app-chirho/perffection-workspace/page-document-chirho",
                true,
            ),
            topic_chirho(
                "privacy-chirho",
                &copy_chirho.privacy_chirho,
                TRUST_DISPLAY_PATH_CHIRHO,
                false,
            ),
        ];
        Self {
            chrome_chirho: SiteChromeChirho {
                page_title_chirho: copy_chirho.page_title_chirho.clone(),
                page_description_chirho: copy_chirho.page_description_chirho.clone(),
                stylesheet_name_chirho: "help-chirho.css".to_owned(),
                preview_marker_chirho: site_copy_chirho.preview_marker_chirho,
                brand_chirho: site_copy_chirho.brand_chirho,
                brand_href_chirho: HOME_DISPLAY_PATH_CHIRHO.to_owned(),
                nav_label_chirho: site_copy_chirho.nav_label_chirho,
                nav_links_chirho: public_navigation_chirho(
                    HELP_DISPLAY_PATH_CHIRHO,
                    &site_copy_chirho.home_label_chirho,
                    &site_copy_chirho.craft_label_chirho,
                    &site_copy_chirho.trust_label_chirho,
                    &site_copy_chirho.help_label_chirho,
                    &site_copy_chirho.signin_label_chirho,
                ),
                footer_note_chirho: Some(site_copy_chirho.footer_note_chirho),
            },
            copy_chirho,
            topics_chirho,
            signin_href_chirho: LOGIN_ACTION_CHIRHO.to_owned(),
            signin_label_chirho: site_copy_chirho.signin_label_chirho,
            workspace_href_chirho: WORKSPACE_REDIRECT_CHIRHO.to_owned(),
        }
    }
}
