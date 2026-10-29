// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Public marketing catalog boundaries: typed loading, escaped HTML and immutable route identity.

use crate::page_render_chirho::render_home_template_chirho;
use crate::types_chirho::{
    craft_page_v1_chirho::CraftPageV1Chirho,
    home_page_v1_chirho::{HomePageV1Chirho, HomeSiteContentCatalogChirho},
    trust_page_v1_chirho::TrustPageV1Chirho,
};
use perffection_ui_chirho::{PublicSiteCopyChirho, UiLocaleChirho};

const PUBLIC_SITE_COPY_CATALOG_CHIRHO: &str =
    include_str!("../../locales-chirho/site-chirho/en-chirho.json");
const HOME_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/";
const CRAFT_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/craft-chirho";
const TRUST_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/trust-chirho";
const HELP_DISPLAY_PATH_CHIRHO: &str = "/app-chirho/perffection/help-chirho";
const SIGNIN_DISPLAY_PATH_CHIRHO: &str = "/auth-chirho/login-chirho";

#[test]
fn embedded_english_catalogs_build_all_public_pages_chirho() {
    for (lang_chirho, dir_chirho) in [
        {
            let model_chirho = HomePageV1Chirho::default_public_chirho();
            (model_chirho.lang_chirho, model_chirho.dir_chirho)
        },
        {
            let model_chirho = CraftPageV1Chirho::default_public_chirho();
            (model_chirho.lang_chirho, model_chirho.dir_chirho)
        },
        {
            let model_chirho = TrustPageV1Chirho::default_public_chirho();
            (model_chirho.lang_chirho, model_chirho.dir_chirho)
        },
    ] {
        assert_eq!(lang_chirho, UiLocaleChirho::EnglishChirho.lang_chirho());
        assert_eq!(dir_chirho, UiLocaleChirho::EnglishChirho.dir_chirho());
    }
}

#[test]
fn pseudolocale_rtl_metadata_and_escaping_reach_the_html_boundary_chirho() {
    let locale_chirho = UiLocaleChirho::PseudolocaleRtlChirho;
    let site_copy_chirho = PublicSiteCopyChirho::from_json_for_locale_chirho(
        PUBLIC_SITE_COPY_CATALOG_CHIRHO,
        locale_chirho,
    );
    let mut content_chirho = HomeSiteContentCatalogChirho::english_chirho();
    content_chirho.hero_chirho.mark_chirho =
        "catalog-sentinel-chirho <b>must be escaped</b>".to_string();
    content_chirho.platform_chirho.heading_chirho =
        "platform-heading-sentinel-chirho <img src=x onerror=alert(1)>".to_string();
    content_chirho.platform_chirho.body_chirho =
        "platform-body-sentinel-chirho <script>alert(1)</script>".to_string();
    content_chirho.platform_chirho.examples_chirho[0].title_chirho =
        "platform-example-sentinel-chirho <button>action</button>".to_string();
    content_chirho.platform_chirho.note_chirho =
        "platform-note-sentinel-chirho <a href=https://example.com>link</a>".to_string();
    let model_chirho = HomePageV1Chirho::from_catalog_chirho(site_copy_chirho, content_chirho);
    let html_chirho = render_home_template_chirho(&model_chirho).html_chirho;

    assert!(html_chirho.contains(&format!(
        "<html lang=\"{}\" dir=\"{}\">",
        locale_chirho.lang_chirho(),
        locale_chirho.dir_chirho()
    )));
    assert!(html_chirho.contains("catalog-sentinel-chirho"));
    assert!(!html_chirho.contains("<b>must be escaped</b>"));
    for (sentinel_chirho, markup_chirho) in [
        (
            "platform-heading-sentinel-chirho",
            "<img src=x onerror=alert(1)>",
        ),
        ("platform-body-sentinel-chirho", "<script>alert(1)</script>"),
        (
            "platform-example-sentinel-chirho",
            "<button>action</button>",
        ),
        (
            "platform-note-sentinel-chirho",
            "<a href=https://example.com>link</a>",
        ),
    ] {
        assert!(html_chirho.contains(sentinel_chirho));
        assert!(!html_chirho.contains(markup_chirho));
    }
}

#[test]
fn locale_copy_cannot_rewrite_public_route_identity_chirho() {
    let locale_chirho = UiLocaleChirho::PseudolocaleRtlChirho;
    let site_copy_chirho = PublicSiteCopyChirho::from_json_for_locale_chirho(
        PUBLIC_SITE_COPY_CATALOG_CHIRHO,
        locale_chirho,
    );
    let model_chirho = HomePageV1Chirho::from_catalog_chirho(
        site_copy_chirho,
        HomeSiteContentCatalogChirho::english_chirho(),
    );
    let hrefs_chirho = model_chirho
        .chrome_chirho
        .nav_links_chirho
        .iter()
        .map(|link_chirho| link_chirho.href_chirho.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        hrefs_chirho,
        [
            HOME_DISPLAY_PATH_CHIRHO,
            CRAFT_DISPLAY_PATH_CHIRHO,
            TRUST_DISPLAY_PATH_CHIRHO,
            HELP_DISPLAY_PATH_CHIRHO,
            SIGNIN_DISPLAY_PATH_CHIRHO,
        ]
    );
    assert_eq!(
        model_chirho
            .chrome_chirho
            .nav_links_chirho
            .iter()
            .filter(|link_chirho| link_chirho.is_current_chirho)
            .map(|link_chirho| link_chirho.href_chirho.as_str())
            .collect::<Vec<_>>(),
        [HOME_DISPLAY_PATH_CHIRHO]
    );
}

#[test]
fn shared_footer_has_one_fixed_builder_destination_chirho() {
    let html_chirho =
        render_home_template_chirho(&HomePageV1Chirho::default_public_chirho()).html_chirho;
    let footer_chirho = html_chirho
        .split_once("<footer class=\"site-footer-chirho\">")
        .expect("the product footer is rendered")
        .1
        .split_once("</footer>")
        .expect("the product footer closes")
        .0;
    assert_eq!(
        super::rendered_document_chirho::referenced_urls_chirho(footer_chirho),
        [perffection_ui_chirho::LOVEJESUS_SOFTWARE_URL_CHIRHO]
    );
}

#[test]
#[should_panic(expected = "unexpected off-site link")]
fn builder_attribution_does_not_allow_lookalike_domains_chirho() {
    super::rendered_document_chirho::assert_public_page_invariants_chirho(
        "<!doctype html><html><body><a href=\"https://loveJesus.software.attacker.example/\">Untrusted destination</a></body></html>",
        "off-site-control-chirho",
    );
}
