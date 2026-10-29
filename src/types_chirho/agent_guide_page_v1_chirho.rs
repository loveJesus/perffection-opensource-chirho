// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Retained, unbound manual-guide renderer; no customer data, authority or independent schema.
//! Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/customer-help-chirho.md.

use super::login_page_v1_chirho::LOGIN_ACTION_CHIRHO;
use super::site_chrome_chirho::{
    public_navigation_chirho, SiteChromeChirho, HOME_DISPLAY_PATH_CHIRHO,
};
use perffection_page_template_chirho::page_document_guide_chirho::{
    own_agent_guide_chirho, DraftAnswerKeyChirho, DRAFT_ANSWER_KEYS_CHIRHO,
    DRAFT_ANSWER_SERVICE_COUNT_CHIRHO, OWN_AGENT_SAMPLE_ANSWER_CHIRHO,
    OWN_AGENT_SAMPLE_PREFACE_CHIRHO,
};
use perffection_ui_chirho::{AgentGuideCopyChirho, PublicSiteCopyChirho, UiLocaleChirho};

pub const AGENT_GUIDE_PATH_CHIRHO: &str = "/app-chirho/perffection/agent-guide-chirho";

#[derive(Debug, Clone)]
pub struct AgentGuidePageV1Chirho {
    pub chrome_chirho: SiteChromeChirho,
    pub copy_chirho: AgentGuideCopyChirho,
    pub instruction_chirho: String,
    pub sample_answer_chirho: &'static str,
    pub sample_preface_chirho: &'static str,
    pub keys_chirho: &'static [DraftAnswerKeyChirho],
    pub service_count_chirho: usize,
    pub signin_href_chirho: &'static str,
    pub signin_label_chirho: String,
}

impl AgentGuidePageV1Chirho {
    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let site_copy_chirho = PublicSiteCopyChirho::from_json_for_locale_chirho(
            include_str!("../../locales-chirho/site-chirho/en-chirho.json"),
            locale_chirho,
        );
        let copy_chirho = AgentGuideCopyChirho::for_locale_chirho(locale_chirho);
        Self {
            chrome_chirho: SiteChromeChirho {
                page_title_chirho: copy_chirho.page_title_chirho.clone(),
                page_description_chirho: copy_chirho.page_description_chirho.clone(),
                stylesheet_name_chirho: "agent-guide-chirho.css".into(),
                preview_marker_chirho: site_copy_chirho.preview_marker_chirho,
                brand_chirho: site_copy_chirho.brand_chirho,
                brand_href_chirho: HOME_DISPLAY_PATH_CHIRHO.into(),
                nav_label_chirho: site_copy_chirho.nav_label_chirho,
                nav_links_chirho: public_navigation_chirho(
                    AGENT_GUIDE_PATH_CHIRHO,
                    &site_copy_chirho.home_label_chirho,
                    &site_copy_chirho.craft_label_chirho,
                    &site_copy_chirho.trust_label_chirho,
                    &site_copy_chirho.help_label_chirho,
                    &site_copy_chirho.signin_label_chirho,
                ),
                footer_note_chirho: Some(site_copy_chirho.footer_note_chirho),
            },
            copy_chirho,
            instruction_chirho: own_agent_guide_chirho(),
            sample_answer_chirho: OWN_AGENT_SAMPLE_ANSWER_CHIRHO,
            sample_preface_chirho: OWN_AGENT_SAMPLE_PREFACE_CHIRHO,
            keys_chirho: &DRAFT_ANSWER_KEYS_CHIRHO,
            service_count_chirho: DRAFT_ANSWER_SERVICE_COUNT_CHIRHO,
            signin_href_chirho: LOGIN_ACTION_CHIRHO,
            signin_label_chirho: site_copy_chirho.signin_label_chirho,
        }
    }
}
