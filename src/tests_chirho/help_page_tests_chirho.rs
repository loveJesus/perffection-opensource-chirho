// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::implementations_chirho::render_help_chirho::render_help_chirho;
use crate::page_render_chirho::render_help_template_chirho;
use crate::tests_chirho::rendered_document_chirho::{
    declared_asset_names_chirho, element_ids_chirho, referenced_urls_chirho,
};
use crate::types_chirho::{
    craft_page_v1_chirho::CraftPageV1Chirho, help_page_v1_chirho::HelpPageV1Chirho,
    home_page_v1_chirho::HomePageV1Chirho, http_request_v1_chirho::HttpRequestV1Chirho,
    login_page_v1_chirho::LoginPageV1Chirho, site_chrome_chirho::HELP_DISPLAY_PATH_CHIRHO,
    trust_page_v1_chirho::TrustPageV1Chirho,
};
use perffection_ui_chirho::{HelpCopyChirho, PublicSiteCopyChirho, UiLocaleChirho};

#[test]
fn public_help_does_not_consume_visitor_data_or_take_actions_chirho() {
    let expected_chirho = render_help_chirho(&HttpRequestV1Chirho::default()).html_chirho;
    let request_chirho = HttpRequestV1Chirho {
        path_chirho: "https://untrusted.invalid/?credential_chirho=private-sentinel-chirho".into(),
        user_principal_chirho: Some("user/private-sentinel-chirho".into()),
    };
    assert_eq!(
        render_help_chirho(&request_chirho).html_chirho,
        expected_chirho
    );
    for forbidden_chirho in [
        "private-sentinel-chirho",
        "<form",
        "<input",
        "<script",
        "<style",
        "style=",
    ] {
        assert!(!expected_chirho.contains(forbidden_chirho));
    }
}

#[test]
fn guide_links_are_real_anchors_or_known_display_destinations_chirho() {
    let model_chirho = HelpPageV1Chirho::default_public_chirho();
    let html_chirho = render_help_template_chirho(&model_chirho).html_chirho;
    let ids_chirho = element_ids_chirho(&html_chirho);
    let mut unique_chirho = ids_chirho.clone();
    unique_chirho.sort();
    unique_chirho.dedup();
    assert_eq!(ids_chirho.len(), unique_chirho.len());

    let assets_chirho = declared_asset_names_chirho();
    let destinations_chirho = model_chirho
        .chrome_chirho
        .nav_links_chirho
        .iter()
        .map(|link_chirho| link_chirho.href_chirho.as_str())
        .chain(
            model_chirho
                .topics_chirho
                .iter()
                .map(|topic_chirho| topic_chirho.action_href_chirho.as_str()),
        )
        .chain([
            model_chirho.workspace_href_chirho.as_str(),
            perffection_ui_chirho::LOVEJESUS_SOFTWARE_URL_CHIRHO,
        ])
        .collect::<Vec<_>>();
    for url_chirho in referenced_urls_chirho(&html_chirho) {
        if let Some(anchor_chirho) = url_chirho.strip_prefix('#') {
            assert!(ids_chirho
                .iter()
                .any(|id_chirho| id_chirho == anchor_chirho));
        } else if let Some(asset_chirho) =
            url_chirho.strip_prefix("/app-assets-chirho/perffection/by-name-chirho/")
        {
            assert!(assets_chirho
                .iter()
                .any(|name_chirho| name_chirho == asset_chirho));
        } else {
            assert!(
                destinations_chirho.contains(&url_chirho.as_str()),
                "unbound link: {url_chirho}"
            );
            assert!(
                !url_chirho.starts_with("/auth-chirho/")
                    || url_chirho == crate::types_chirho::login_page_v1_chirho::LOGIN_ACTION_CHIRHO
            );
        }
    }
    assert!(model_chirho
        .topics_chirho
        .iter()
        .filter(|topic_chirho| topic_chirho
            .action_href_chirho
            .starts_with("/app-chirho/perffection-workspace/"))
        .all(|topic_chirho| topic_chirho.needs_signin_chirho));
}

#[test]
fn help_catalog_changes_presentation_not_destinations_or_markup_chirho() {
    let locale_chirho = UiLocaleChirho::PseudolocaleRtlChirho;
    let mut copy_chirho = HelpCopyChirho::for_locale_chirho(locale_chirho);
    copy_chirho.save_chirho.heading_chirho = "<script>help-catalog-sentinel-chirho</script>".into();
    copy_chirho.save_chirho.steps_chirho =
        vec!["<a href='https://untrusted.invalid'>help-step-sentinel-chirho</a>".into()];
    let site_copy_chirho = PublicSiteCopyChirho::from_json_for_locale_chirho(
        include_str!("../../locales-chirho/site-chirho/en-chirho.json"),
        locale_chirho,
    );
    let model_chirho = HelpPageV1Chirho::from_catalog_chirho(site_copy_chirho, copy_chirho);
    let html_chirho = render_help_template_chirho(&model_chirho).html_chirho;
    assert!(html_chirho.contains("lang=\"ar-XB\" dir=\"rtl\""));
    assert!(html_chirho.contains("help-catalog-sentinel-chirho"));
    assert!(html_chirho.contains("help-step-sentinel-chirho"));
    assert!(!html_chirho.contains("<script>"));
    assert!(!referenced_urls_chirho(&html_chirho)
        .iter()
        .any(|href_chirho| href_chirho.starts_with("https://untrusted.invalid")));
    let english_chirho = HelpPageV1Chirho::default_public_chirho();
    assert_eq!(
        model_chirho
            .topics_chirho
            .iter()
            .map(|topic_chirho| (
                &topic_chirho.anchor_chirho,
                &topic_chirho.action_href_chirho
            ))
            .collect::<Vec<_>>(),
        english_chirho
            .topics_chirho
            .iter()
            .map(|topic_chirho| (
                &topic_chirho.anchor_chirho,
                &topic_chirho.action_href_chirho
            ))
            .collect::<Vec<_>>()
    );
}

#[test]
fn every_public_page_has_help_and_marks_only_its_own_route_chirho() {
    let help_chirho = HelpPageV1Chirho::default_public_chirho();
    let expected_routes_chirho = help_chirho
        .chrome_chirho
        .nav_links_chirho
        .iter()
        .map(|link_chirho| link_chirho.href_chirho.clone())
        .collect::<Vec<_>>();
    for chrome_chirho in [
        HomePageV1Chirho::default_public_chirho().chrome_chirho,
        CraftPageV1Chirho::default_public_chirho().chrome_chirho,
        TrustPageV1Chirho::default_public_chirho().chrome_chirho,
        LoginPageV1Chirho::default_public_chirho().chrome_chirho,
        help_chirho.chrome_chirho,
    ] {
        assert_eq!(
            chrome_chirho
                .nav_links_chirho
                .iter()
                .map(|link_chirho| link_chirho.href_chirho.clone())
                .collect::<Vec<_>>(),
            expected_routes_chirho
        );
        assert_eq!(
            chrome_chirho
                .nav_links_chirho
                .iter()
                .filter(|link_chirho| link_chirho.is_current_chirho)
                .count(),
            1
        );
        assert!(chrome_chirho
            .nav_links_chirho
            .iter()
            .any(|link_chirho| link_chirho.href_chirho == HELP_DISPLAY_PATH_CHIRHO));
    }
}
