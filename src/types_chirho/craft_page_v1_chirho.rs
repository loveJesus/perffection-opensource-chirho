// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Typed copy for `/craft-chirho`, "What's in it": everything an owner gets, line by line, like the
//! price list on a shop counter where every line reads "Included".
//!
//! Rewritten 2026-10-01 at L.J.'s word on the old page: "why would i want to know about licenced
//! fonts... i want to know its secure, it is profitable and easy". The handover stack, the licensed
//! faces and the counts were answers to an engineer's questions; they are gone, and the few checks
//! a developer might want sit in the page's folded "For your web developer" note.
//!
//! Two rules for this catalog. A line says "Included" only for what an owner gets today, and anything
//! built but not open to owners says so ("Coming next"). And the owner decides while the details are
//! taken care of: no line casts her as the one doing the chores (the principle in
//! `docs-chirho/product-design-chirho/product-overview-chirho.md`).

use crate::types_chirho::login_page_v1_chirho::LOGIN_ACTION_CHIRHO;
use crate::types_chirho::public_page_parts_chirho::{
    DeveloperFoldChirho, PageCloseChirho, PageHeroChirho,
};
use crate::types_chirho::site_chrome_chirho::{
    public_navigation_chirho, SiteChromeChirho, CRAFT_DISPLAY_PATH_CHIRHO, HOME_DISPLAY_PATH_CHIRHO,
};
use perffection_ui_chirho::{PublicSiteCopyChirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

/// One line of the list: what it is, one plain sentence, and where it stands ("Included", or
/// "Coming next" for what is built but not yet open to owners).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryItemChirho {
    pub name_chirho: String,
    pub note_chirho: String,
    pub status_chirho: String,
    /// Built but not yet open to owners. Marks the line in its own colour, so "coming" never reads
    /// like "included" at a glance; the template keys on this flag, never on the words.
    #[serde(default)]
    pub coming_next_chirho: bool,
}

/// A part of the list: what visitors see, how the owner works, what is taken care of.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryGroupChirho {
    pub heading_chirho: String,
    pub intro_chirho: String,
    pub items_chirho: Vec<InventoryItemChirho>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub note_chirho: String,
    pub groups_chirho: Vec<InventoryGroupChirho>,
}

const CRAFT_STYLESHEET_NAME_CHIRHO: &str = "craft-chirho.css";
const PUBLIC_SITE_COPY_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/en-chirho.json");
const CRAFT_SITE_CONTENT_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/craft-en-chirho.json");

/// Editable public copy plus display-only asset and navigation configuration.
///
/// Public links and asset names in this catalog select already-reviewed presentation. They grant no
/// action, account or effect authority and must remain byte-stable across translations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CraftSiteContentCatalogChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub hero_chirho: PageHeroChirho,
    pub inventory_chirho: InventoryChirho,
    pub fold_chirho: DeveloperFoldChirho,
    pub close_chirho: PageCloseChirho,
}

impl CraftSiteContentCatalogChirho {
    pub fn from_json_chirho(catalog_json_chirho: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(catalog_json_chirho)
    }

    pub fn english_chirho() -> Self {
        Self::from_json_chirho(CRAFT_SITE_CONTENT_CATALOG_CHIRHO)
            .expect("the embedded typed craft-page catalog must remain valid")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CraftPageV1Chirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub chrome_chirho: SiteChromeChirho,
    /// The canonical sign-in route, for the closing band's sign-in.
    pub signin_href_chirho: String,
    pub hero_chirho: PageHeroChirho,
    pub inventory_chirho: InventoryChirho,
    pub fold_chirho: DeveloperFoldChirho,
    pub close_chirho: PageCloseChirho,
}

impl CraftPageV1Chirho {
    pub fn default_public_chirho() -> Self {
        Self::from_catalog_chirho(
            PublicSiteCopyChirho::from_json_for_locale_chirho(
                PUBLIC_SITE_COPY_CATALOG_CHIRHO,
                UiLocaleChirho::EnglishChirho,
            ),
            CraftSiteContentCatalogChirho::english_chirho(),
        )
    }

    pub fn from_catalog_chirho(
        public_site_copy_chirho: PublicSiteCopyChirho,
        content_catalog_chirho: CraftSiteContentCatalogChirho,
    ) -> Self {
        let chrome_chirho = SiteChromeChirho {
            page_title_chirho: content_catalog_chirho.page_title_chirho,
            page_description_chirho: content_catalog_chirho.page_description_chirho,
            stylesheet_name_chirho: CRAFT_STYLESHEET_NAME_CHIRHO.to_string(),
            preview_marker_chirho: public_site_copy_chirho.preview_marker_chirho.clone(),
            brand_chirho: public_site_copy_chirho.brand_chirho.clone(),
            brand_href_chirho: HOME_DISPLAY_PATH_CHIRHO.to_string(),
            nav_label_chirho: public_site_copy_chirho.nav_label_chirho.clone(),
            nav_links_chirho: public_navigation_chirho(
                CRAFT_DISPLAY_PATH_CHIRHO,
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
            inventory_chirho: content_catalog_chirho.inventory_chirho,
            fold_chirho: content_catalog_chirho.fold_chirho,
            close_chirho: content_catalog_chirho.close_chirho,
        }
    }
}
