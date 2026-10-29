// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::implementations_chirho::render_agent_guide_chirho::render_agent_guide_chirho;
use crate::page_render_chirho::{render_agent_guide_template_chirho, render_help_template_chirho};
use crate::tests_chirho::rendered_document_chirho::{
    declared_asset_names_chirho, element_ids_chirho, referenced_urls_chirho,
};
use crate::types_chirho::{
    agent_guide_page_v1_chirho::{AgentGuidePageV1Chirho, AGENT_GUIDE_PATH_CHIRHO},
    help_page_v1_chirho::HelpPageV1Chirho,
    http_request_v1_chirho::HttpRequestV1Chirho,
    login_page_v1_chirho::LOGIN_ACTION_CHIRHO,
};
use perffection_page_template_chirho::page_document_guide_chirho::{
    own_agent_guide_chirho, DRAFT_ANSWER_KEYS_CHIRHO, DRAFT_ANSWER_SERVICE_COUNT_CHIRHO,
    OWN_AGENT_SAMPLE_ANSWER_CHIRHO, OWN_AGENT_SAMPLE_PREFACE_CHIRHO,
};
use perffection_ui_chirho::UiLocaleChirho;

#[test]
fn public_agent_guide_uses_the_shared_answer_contract_and_ignores_visitors_chirho() {
    let english_chirho = AgentGuidePageV1Chirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
    let rtl_chirho =
        AgentGuidePageV1Chirho::for_locale_chirho(UiLocaleChirho::PseudolocaleRtlChirho);
    for model_chirho in [&english_chirho, &rtl_chirho] {
        assert_eq!(model_chirho.instruction_chirho, own_agent_guide_chirho());
        assert_eq!(
            model_chirho.sample_answer_chirho,
            OWN_AGENT_SAMPLE_ANSWER_CHIRHO
        );
        assert_eq!(
            model_chirho.sample_preface_chirho,
            OWN_AGENT_SAMPLE_PREFACE_CHIRHO
        );
        assert_eq!(model_chirho.keys_chirho, DRAFT_ANSWER_KEYS_CHIRHO);
        assert_eq!(
            model_chirho.service_count_chirho,
            DRAFT_ANSWER_SERVICE_COUNT_CHIRHO
        );
        assert_eq!(model_chirho.signin_href_chirho, LOGIN_ACTION_CHIRHO);
    }
    let expected_chirho = render_agent_guide_chirho(&HttpRequestV1Chirho::default()).html_chirho;
    let visitor_chirho = HttpRequestV1Chirho {
        path_chirho: "/?agent-guide-private-sentinel-chirho".into(),
        user_principal_chirho: Some("agent-guide-private-sentinel-chirho".into()),
    };
    assert_eq!(
        render_agent_guide_chirho(&visitor_chirho).html_chirho,
        expected_chirho
    );
    for forbidden_chirho in [
        "agent-guide-private-sentinel-chirho",
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
fn guide_copy_and_instruction_are_escaped_without_changing_destinations_chirho() {
    let mut model_chirho =
        AgentGuidePageV1Chirho::for_locale_chirho(UiLocaleChirho::PseudolocaleRtlChirho);
    model_chirho.copy_chirho.heading_chirho =
        "<img src=https://untrusted.invalid/copy-chirho>".into();
    model_chirho.instruction_chirho = "</pre><script>instruction-sentinel-chirho</script>".into();
    model_chirho.sample_answer_chirho = "</pre><img src=https://untrusted.invalid/sample-chirho>";
    let html_chirho = render_agent_guide_template_chirho(&model_chirho).html_chirho;
    assert!(html_chirho.contains("lang=\"ar-XB\" dir=\"rtl\""));
    assert!(html_chirho.contains("copy-chirho"));
    assert!(html_chirho.contains("instruction-sentinel-chirho"));
    assert!(html_chirho.contains("sample-chirho"));
    assert!(!html_chirho.contains(&model_chirho.copy_chirho.heading_chirho));
    assert!(!html_chirho.contains(&model_chirho.instruction_chirho));
    assert!(!html_chirho.contains(model_chirho.sample_answer_chirho));
    assert!(!html_chirho.contains("<script>"));
    let urls_chirho = referenced_urls_chirho(&html_chirho);
    assert!(!urls_chirho
        .iter()
        .any(|url_chirho| url_chirho.contains("untrusted.invalid")));
    assert!(!urls_chirho
        .iter()
        .any(|url_chirho| url_chirho.contains("/own-ai-chirho")));
    assert!(!urls_chirho
        .iter()
        .any(|url_chirho| url_chirho.contains("page-document-import")));
}

#[test]
fn retained_unbound_guide_has_no_retired_handoff_or_help_entry_chirho() {
    let help_chirho = HelpPageV1Chirho::default_public_chirho();
    let help_html_chirho = render_help_template_chirho(&help_chirho).html_chirho;
    assert!(!referenced_urls_chirho(&help_html_chirho)
        .iter()
        .any(|url_chirho| url_chirho == AGENT_GUIDE_PATH_CHIRHO
            || url_chirho.contains("/own-ai-chirho")));
    let model_chirho = AgentGuidePageV1Chirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
    let html_chirho = render_agent_guide_template_chirho(&model_chirho).html_chirho;
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
        .chain([
            LOGIN_ACTION_CHIRHO,
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
                "unknown guide destination: {url_chirho}"
            );
        }
    }
}
