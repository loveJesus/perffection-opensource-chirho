// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Typed copy and showcase data for the Perffection front door, authored on the FIRST LIGHT
//! system: the page is one night becoming morning. The business-platform destination comes first;
//! Secure (the night), Abundant (the dawn), Easy (the morning) explain its foundations and first step. The
//! template owns structure; the typed locale catalog owns every public claim.
//!
//! Three rules the copy in this file must keep.
//! 1. It speaks to the person building a business, not to an engine. Cairn is named in plain words:
//!    why its foundations matter to her work, not an implementation inventory.
//! 2. Keep the benefit, delete the promise. No invented numbers, no measured-sounding durations,
//!    no guarantee nobody can give. The showcase businesses are invented and say so.
//! 3. The owner decides; the details are taken care of (L.J., 2026-10-01). Copy never casts the
//!    owner as the one doing the detail work. Automation is described only as far as it is true
//!    today, and a door that is built but not yet open to owners is called "coming", not offered.
//!    The principle itself: docs-chirho/product-design-chirho/product-overview-chirho.md.

use crate::types_chirho::login_page_v1_chirho::LOGIN_ACTION_CHIRHO;
use crate::types_chirho::public_page_parts_chirho::PointChirho;
use crate::types_chirho::site_chrome_chirho::{
    public_navigation_chirho, SiteChromeChirho, HOME_DISPLAY_PATH_CHIRHO, INVITATION_MAILTO_CHIRHO,
};
use perffection_ui_chirho::{PublicSiteCopyChirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

/// An in-page link: the hero's three pillars jump to their own bands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PillarLinkChirho {
    pub label_chirho: String,
    pub href_chirho: String,
}

/// The opening night band: the floating town before it wakes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeroChirho {
    /// The first focusable thing on the page: a keyboard visitor's way past the navigation.
    pub skip_label_chirho: String,
    pub mark_chirho: String,
    /// One entry per rendered line. Where a headline this large breaks is a typographic decision.
    pub headline_lines_chirho: Vec<String>,
    pub lead_chirho: String,
    pub primary_label_chirho: String,
    pub primary_href_chirho: String,
    pub secondary_label_chirho: String,
    pub secondary_href_chirho: String,
    pub pillars_label_chirho: String,
    pub pillars_chirho: Vec<PillarLinkChirho>,
    pub quiet_note_chirho: String,
    pub signin_label_chirho: String,
}

/// Secure: the crystal seal turning in the night.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecureChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
    pub points_chirho: Vec<PointChirho>,
}

/// The business destination, clearly distinguished from the first website release.
/// Workflow: spec-chirho/workflows-chirho/first-light-authoring-chirho.md.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
    pub examples_chirho: Vec<PointChirho>,
    pub note_chirho: String,
}

/// One stop on the path from a stranger to a customer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathStepChirho {
    pub step_chirho: String,
    pub title_chirho: String,
    pub body_chirho: String,
}

/// Abundant: the dawn, and the path of light to the door. L.J. chose the word on 2026-10-01
/// ("Profitable isn't what I want to say"): more good work, not a promise of earnings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbundantChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
    /// One third-party finding that shows the mechanism, with its source named in the sentence.
    /// Evidence for why a light page matters, never a promise of what a customer will earn.
    pub proof_chirho: String,
    pub proof_source_chirho: String,
    pub path_chirho: Vec<PathStepChirho>,
}

/// Easy: the morning, the real editor and the page it makes, and the split between what the owner
/// decides and what is taken care of for her.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EasyChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
    pub steps_label_chirho: String,
    pub steps_chirho: Vec<String>,
    /// What stays in the owner's hands.
    pub decide_label_chirho: String,
    pub decide_chirho: Vec<PointChirho>,
    /// What the platform does for her. Each line must be true of the product today, or say plainly
    /// that it is coming.
    pub handled_label_chirho: String,
    pub handled_chirho: Vec<PointChirho>,
    pub question_image_chirho: String,
    pub question_alt_chirho: String,
    pub question_caption_chirho: String,
    pub result_image_chirho: String,
    pub result_alt_chirho: String,
    pub result_caption_chirho: String,
    pub phone_image_chirho: String,
    pub phone_alt_chirho: String,
}

/// One plain question a shop owner asks before trusting anyone with her website.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionChirho {
    pub question_chirho: String,
    pub answer_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionsChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub items_chirho: Vec<QuestionChirho>,
}

/// The closing band, in full morning.
///
/// An invitation is asked for here. L.J. confirmed where a request goes (2026-10-02: "use aleluya"), so the band
/// carries the one reviewed mailto, and the public-page link invariant was widened deliberately to exactly that
/// address (`INVITATION_MAILTO_CHIRHO`). Invitees still sign in beside it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClosingChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
    pub signin_label_chirho: String,
    pub invitation_label_chirho: String,
    /// A line of Scripture under the ask (L.J.: "invitation use something more Biblical").
    pub invitation_note_chirho: String,
    /// Why the work is done this way, in plain words; the whole site refuses to overclaim because
    /// of it, and a reader is owed the reason rather than a footer badge.
    pub because_chirho: String,
    pub verse_chirho: String,
    pub verse_source_chirho: String,
}

/// A line on a showcase shop's price list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShopItemChirho {
    pub name_chirho: String,
    pub note_chirho: String,
    pub price_chirho: String,
}

/// One named design direction for a trade. Studios present concepts by name because a name is what
/// a client remembers and argues about; "option 2" is not a thing anyone falls in love with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignChirho {
    pub key_chirho: String,
    pub name_chirho: String,
    /// Three or four words naming the moves that make this direction what it is.
    pub blurb_chirho: String,
}

/// One invented business, rendered in full inside the bench preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeChirho {
    pub key_chirho: String,
    pub tab_label_chirho: String,
    pub tab_note_chirho: String,
    pub url_chirho: String,
    pub shop_name_chirho: String,
    pub shop_meta_chirho: String,
    pub eyebrow_chirho: String,
    pub headline_chirho: String,
    pub body_chirho: String,
    pub cta_chirho: String,
    pub image_chirho: String,
    pub image_alt_chirho: String,
    pub items_chirho: Vec<ShopItemChirho>,
    pub foot_left_chirho: String,
    pub foot_right_chirho: String,
    /// Accessible name for the preview frame, and the spoken note that follows it. The frame draws
    /// a shop that does not exist, so assistive technology is told so before it reads the contents
    /// — the visual label ("Invented for this page") must not be the only place that is said.
    pub designs_chirho: Vec<DesignChirho>,
    pub preview_label_chirho: String,
    pub preview_note_chirho: String,
    /// The one ornamental mark each design owns: a newspaper dateline, a gallery wall label, a
    /// botanical plate number. Deliberately used differently by each preview rather than shared.
    pub edition_mark_chirho: String,
}

/// The interactive bench band.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabChirho {
    pub mark_chirho: String,
    /// Marginal leaf mark, set vertically in the outer gutter. Decorative and hidden from assistive
    /// technology; it exists to give a wide page the feel of a ruled manuscript leaf.
    pub folio_chirho: String,
    pub heading_chirho: String,
    pub note_chirho: String,
    pub trade_legend_chirho: String,
    pub gallery_work_label_chirho: String,
    pub trades_chirho: Vec<TradeChirho>,
    pub aside_chirho: String,
    pub stage_note_chirho: String,
}

const HOME_STYLESHEET_NAME_CHIRHO: &str = "home-chirho.css";
const PUBLIC_SITE_COPY_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/en-chirho.json");
const HOME_SITE_CONTENT_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/home-en-chirho.json");

/// Editable public copy plus display-only asset, preview and navigation configuration.
///
/// Fragments, showcase keys and asset names in this catalog select already-reviewed public
/// presentation. They grant no effect authority and must remain byte-stable across translations.
/// The sign-in destination is not in the catalog at all: it is the canonical route constant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomeSiteContentCatalogChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub hero_chirho: HeroChirho,
    pub platform_chirho: PlatformChirho,
    pub secure_chirho: SecureChirho,
    pub abundant_chirho: AbundantChirho,
    pub easy_chirho: EasyChirho,
    pub lab_chirho: LabChirho,
    pub questions_chirho: QuestionsChirho,
    pub closing_chirho: ClosingChirho,
}

impl HomeSiteContentCatalogChirho {
    pub fn from_json_chirho(catalog_json_chirho: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(catalog_json_chirho)
    }

    pub fn english_chirho() -> Self {
        Self::from_json_chirho(HOME_SITE_CONTENT_CATALOG_CHIRHO)
            .expect("the embedded typed home-page catalog must remain valid")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomePageV1Chirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub chrome_chirho: SiteChromeChirho,
    /// The canonical sign-in route, for the hero's and the closing band's sign-in links.
    pub signin_href_chirho: String,
    /// The closing band's "Ask for an invitation": the one reviewed mailto.
    pub invitation_href_chirho: String,
    pub hero_chirho: HeroChirho,
    pub platform_chirho: PlatformChirho,
    pub secure_chirho: SecureChirho,
    pub abundant_chirho: AbundantChirho,
    pub easy_chirho: EasyChirho,
    pub lab_chirho: LabChirho,
    pub questions_chirho: QuestionsChirho,
    pub closing_chirho: ClosingChirho,
}

impl HomePageV1Chirho {
    /// English is the only reviewed release catalog. Alternate catalogs enter through
    /// `from_catalog_chirho`; no request-selected locale transport is implemented here.
    pub fn default_public_chirho() -> Self {
        Self::from_catalog_chirho(
            PublicSiteCopyChirho::from_json_for_locale_chirho(
                PUBLIC_SITE_COPY_CATALOG_CHIRHO,
                UiLocaleChirho::EnglishChirho,
            ),
            HomeSiteContentCatalogChirho::english_chirho(),
        )
    }

    pub fn from_catalog_chirho(
        public_site_copy_chirho: PublicSiteCopyChirho,
        content_catalog_chirho: HomeSiteContentCatalogChirho,
    ) -> Self {
        let chrome_chirho = SiteChromeChirho {
            page_title_chirho: content_catalog_chirho.page_title_chirho,
            page_description_chirho: content_catalog_chirho.page_description_chirho,
            stylesheet_name_chirho: HOME_STYLESHEET_NAME_CHIRHO.to_string(),
            preview_marker_chirho: public_site_copy_chirho.preview_marker_chirho.clone(),
            brand_chirho: public_site_copy_chirho.brand_chirho.clone(),
            brand_href_chirho: HOME_DISPLAY_PATH_CHIRHO.to_string(),
            nav_label_chirho: public_site_copy_chirho.nav_label_chirho.clone(),
            nav_links_chirho: public_navigation_chirho(
                HOME_DISPLAY_PATH_CHIRHO,
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
            invitation_href_chirho: INVITATION_MAILTO_CHIRHO.to_string(),
            hero_chirho: content_catalog_chirho.hero_chirho,
            platform_chirho: content_catalog_chirho.platform_chirho,
            secure_chirho: content_catalog_chirho.secure_chirho,
            abundant_chirho: content_catalog_chirho.abundant_chirho,
            easy_chirho: content_catalog_chirho.easy_chirho,
            lab_chirho: content_catalog_chirho.lab_chirho,
            questions_chirho: content_catalog_chirho.questions_chirho,
            closing_chirho: content_catalog_chirho.closing_chirho,
        }
    }
}
