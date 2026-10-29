// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Typed copy for `/trust-chirho`: safe for the owner, safe for her visitors, in plain words, with
//! the proof folded away for anyone who wants to check it.
//!
//! Rewritten 2026-10-01 for owners rather than engineers (L.J.: "i want to know its secure"). The
//! old page's receipts did not disappear: the checks a developer can run now sit in the page's
//! "For your web developer" fold, and the limits stay on the page itself.
//!
//! This page is held to a harder standard than the rest of the site. Every claim must be checkable
//! or plainly marked as not yet true; a trust page that overstates spends the credibility it was
//! written to build. Corrections that shaped it, recorded so the next author does not undo them:
//!   * Recovery is scoped to the owner's PAGE: putting back an earlier version cannot take back what
//!     a visitor already saw, and there is no account-wide restore.
//!   * "Nothing runs" is false (stylesheets and typefaces do their work). The true claim is narrower:
//!     no script runs, and the browser is told to refuse one.
//!   * "Nothing is logged" is false. The true claim: the server notes only the time, the page and how
//!     it went, with no addresses (Cairn checked every line of real traffic, 2026-10-01).

use crate::types_chirho::login_page_v1_chirho::LOGIN_ACTION_CHIRHO;
use crate::types_chirho::public_page_parts_chirho::{
    DeveloperFoldChirho, PageCloseChirho, PageHeroChirho, PointChirho,
};
use crate::types_chirho::site_chrome_chirho::{
    public_navigation_chirho, SiteChromeChirho, HOME_DISPLAY_PATH_CHIRHO, TRUST_DISPLAY_PATH_CHIRHO,
};
use perffection_ui_chirho::{PublicSiteCopyChirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

/// One band of the page: a label, a heading, a sentence of context and its points.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustBandChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub note_chirho: String,
    pub points_chirho: Vec<PointChirho>,
}

const TRUST_STYLESHEET_NAME_CHIRHO: &str = "trust-chirho.css";
const PUBLIC_SITE_COPY_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/en-chirho.json");
const TRUST_SITE_CONTENT_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/trust-en-chirho.json");

/// Editable public copy plus display-only asset and navigation configuration.
///
/// Public links and asset names in this catalog select already-reviewed presentation. They grant no
/// action, account or effect authority and must remain byte-stable across translations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustSiteContentCatalogChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub hero_chirho: PageHeroChirho,
    /// What protects the owner's page.
    pub protects_chirho: TrustBandChirho,
    /// What the page never does to the people who visit it.
    pub visitors_chirho: TrustBandChirho,
    /// What we will not pretend. Never empty: a trust page without its limits reads as a stronger
    /// promise than the one that can be kept.
    pub limits_chirho: TrustBandChirho,
    pub fold_chirho: DeveloperFoldChirho,
    pub close_chirho: PageCloseChirho,
}

impl TrustSiteContentCatalogChirho {
    pub fn from_json_chirho(catalog_json_chirho: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(catalog_json_chirho)
    }

    pub fn english_chirho() -> Self {
        Self::from_json_chirho(TRUST_SITE_CONTENT_CATALOG_CHIRHO)
            .expect("the embedded typed trust-page catalog must remain valid")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustPageV1Chirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub chrome_chirho: SiteChromeChirho,
    /// The canonical sign-in route, for the closing band's sign-in.
    pub signin_href_chirho: String,
    pub hero_chirho: PageHeroChirho,
    pub protects_chirho: TrustBandChirho,
    pub visitors_chirho: TrustBandChirho,
    pub limits_chirho: TrustBandChirho,
    pub fold_chirho: DeveloperFoldChirho,
    pub close_chirho: PageCloseChirho,
}

impl TrustPageV1Chirho {
    pub fn default_public_chirho() -> Self {
        Self::from_catalog_chirho(
            PublicSiteCopyChirho::from_json_for_locale_chirho(
                PUBLIC_SITE_COPY_CATALOG_CHIRHO,
                UiLocaleChirho::EnglishChirho,
            ),
            TrustSiteContentCatalogChirho::english_chirho(),
        )
    }

    pub fn from_catalog_chirho(
        public_site_copy_chirho: PublicSiteCopyChirho,
        content_catalog_chirho: TrustSiteContentCatalogChirho,
    ) -> Self {
        let chrome_chirho = SiteChromeChirho {
            page_title_chirho: content_catalog_chirho.page_title_chirho,
            page_description_chirho: content_catalog_chirho.page_description_chirho,
            stylesheet_name_chirho: TRUST_STYLESHEET_NAME_CHIRHO.to_string(),
            preview_marker_chirho: public_site_copy_chirho.preview_marker_chirho.clone(),
            brand_chirho: public_site_copy_chirho.brand_chirho.clone(),
            brand_href_chirho: HOME_DISPLAY_PATH_CHIRHO.to_string(),
            nav_label_chirho: public_site_copy_chirho.nav_label_chirho.clone(),
            nav_links_chirho: public_navigation_chirho(
                TRUST_DISPLAY_PATH_CHIRHO,
                &public_site_copy_chirho.home_label_chirho,
                &public_site_copy_chirho.craft_label_chirho,
                &public_site_copy_chirho.trust_label_chirho,
                &public_site_copy_chirho.help_label_chirho,
                &public_site_copy_chirho.signin_label_chirho,
            ),
            footer_note_chirho: Some(public_site_copy_chirho.footer_note_chirho.clone()),
        };

        Self {
            lang_chirho: public_site_copy_chirho.lang_chirho,
            dir_chirho: public_site_copy_chirho.dir_chirho,
            chrome_chirho,
            signin_href_chirho: LOGIN_ACTION_CHIRHO.to_string(),
            hero_chirho: content_catalog_chirho.hero_chirho,
            protects_chirho: content_catalog_chirho.protects_chirho,
            visitors_chirho: content_catalog_chirho.visitors_chirho,
            limits_chirho: content_catalog_chirho.limits_chirho,
            fold_chirho: content_catalog_chirho.fold_chirho,
            close_chirho: content_catalog_chirho.close_chirho,
        }
    }
}
