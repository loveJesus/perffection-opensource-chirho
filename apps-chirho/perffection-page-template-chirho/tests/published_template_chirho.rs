// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use perffection_page_template_chirho::implementations_chirho::render_published_page_chirho::render_published_page_chirho;
use perffection_page_template_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_page_template_chirho::page_view_chirho::PAGE_STYLESHEET_HREF_CHIRHO;
use perffection_page_template_chirho::published_request_chirho::PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO;
use perffection_page_template_chirho::{
    page_render_chirho::render_page_chirho, page_view_chirho::PagePreviewV1Chirho,
};
use perffection_ui_chirho::{PageCopyChirho, UiLocaleChirho};

fn valid_document_chirho() -> PageDocumentV2Chirho {
    PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "Good Earth Bakery".to_string(),
        headline_chirho: "Bread made before the town wakes.".to_string(),
        summary_chirho: "Small-batch loaves for our neighbors.".to_string(),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "Daily bread".to_string(),
                body_chirho: "Naturally leavened and baked each morning.".to_string(),
            },
            PageServiceV1Chirho::default(),
            PageServiceV1Chirho::default(),
        ],
        contact_heading_chirho: "Save a loaf for tomorrow.".to_string(),
        contact_phone_chirho: "+1 (555) 010-0100".to_string(),
        contact_email_chirho: "hello@good-earth.example".to_string(),
        contact_details_chirho: "Open between 7am and 2pm.".to_string(),
    }
}

fn render_json_chirho(value_chirho: serde_json::Value) -> String {
    String::from_utf8(render_published_page_chirho(
        &serde_json::to_vec(&value_chirho).unwrap(),
    ))
    .unwrap()
}

#[test]
fn signed_app_params_shape_renders_one_actionless_public_page_chirho() {
    let html_chirho = render_json_chirho(serde_json::json!({
        "app_params_chirho": valid_document_chirho()
    }));

    assert!(html_chirho.contains("Good Earth Bakery"));
    assert!(html_chirho.contains("Bread made before the town wakes."));
    assert!(html_chirho.contains(PAGE_STYLESHEET_HREF_CHIRHO));
    assert!(html_chirho.contains("href=\"tel:+15550100100\""));
    assert!(html_chirho.contains("href=\"mailto:hello@good-earth.example\""));
    assert!(!html_chirho.contains("page-preview-rail-chirho"));
    assert_eq!(html_chirho.matches("<form").count(), 0);
    assert_eq!(html_chirho.matches("<button").count(), 0);
    assert_eq!(html_chirho.matches("<script").count(), 0);
    assert_eq!(html_chirho.matches("style=").count(), 0);
}

#[test]
fn direct_top_level_page_fields_do_not_impersonate_host_app_params_chirho() {
    let document_chirho = serde_json::to_value(valid_document_chirho()).unwrap();
    let html_chirho = render_json_chirho(document_chirho);

    assert!(html_chirho.contains("page-unavailable-chirho"));
    assert!(!html_chirho.contains("Good Earth Bakery"));
}

#[test]
fn unknown_document_or_service_fields_fail_customer_safe_chirho() {
    let mut unknown_document_chirho = serde_json::to_value(valid_document_chirho()).unwrap();
    unknown_document_chirho
        .as_object_mut()
        .unwrap()
        .insert("unreviewed_document_field_chirho".to_string(), true.into());
    let document_html_chirho = render_json_chirho(serde_json::json!({
        "app_params_chirho": unknown_document_chirho
    }));

    let mut unknown_service_chirho = serde_json::to_value(valid_document_chirho()).unwrap();
    unknown_service_chirho["services_chirho"][0]["unreviewed_service_field_chirho"] = true.into();
    let service_html_chirho = render_json_chirho(serde_json::json!({
        "app_params_chirho": unknown_service_chirho
    }));

    for html_chirho in [document_html_chirho, service_html_chirho] {
        assert!(html_chirho.contains("page-unavailable-chirho"));
        assert!(!html_chirho.contains("Good Earth Bakery"));
        assert_eq!(html_chirho.matches("<form").count(), 0);
    }
}

#[test]
fn missing_malformed_invalid_and_oversized_params_fail_customer_safe_chirho() {
    let missing_chirho = render_json_chirho(serde_json::json!({}));
    let malformed_chirho = String::from_utf8(render_published_page_chirho(b"not-json")).unwrap();
    let invalid_chirho = render_json_chirho(serde_json::json!({
        "app_params_chirho": PageDocumentV2Chirho::default()
    }));
    let oversized_chirho = String::from_utf8(render_published_page_chirho(&vec![
        b'x';
        PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO
            + 1
    ]))
    .unwrap();

    for html_chirho in [
        missing_chirho,
        malformed_chirho,
        invalid_chirho,
        oversized_chirho,
    ] {
        assert!(html_chirho.contains("page-unavailable-chirho"));
        assert!(!html_chirho.contains("could not be verified"));
        assert!(!html_chirho.contains("Perffection page"));
        assert!(!html_chirho.contains("555"));
        assert!(!html_chirho.contains("mailto:"));
        assert!(!html_chirho.contains("tel:"));
        assert_eq!(html_chirho.matches("<form").count(), 0);
        assert_eq!(html_chirho.matches("<script").count(), 0);
    }
}

#[test]
fn validated_contact_fields_alone_can_create_public_actions_chirho() {
    let mut phone_only_chirho = valid_document_chirho();
    phone_only_chirho.contact_email_chirho.clear();
    phone_only_chirho.contact_details_chirho =
        "Hours mention 7am, never a dial target.".to_string();
    let phone_html_chirho = render_json_chirho(serde_json::json!({
        "app_params_chirho": phone_only_chirho
    }));
    assert!(phone_html_chirho.contains("href=\"tel:+15550100100\""));
    assert!(!phone_html_chirho.contains("mailto:"));
    assert!(!phone_html_chirho.contains("href=\"tel:7"));

    let mut email_only_chirho = valid_document_chirho();
    email_only_chirho.contact_phone_chirho.clear();
    let email_html_chirho = render_json_chirho(serde_json::json!({
        "app_params_chirho": email_only_chirho
    }));
    assert!(email_html_chirho.contains("href=\"mailto:hello@good-earth.example\""));
    assert!(!email_html_chirho.contains("href=\"tel:"));
}

#[test]
fn missing_or_unsafe_contact_actions_fail_customer_safe_chirho() {
    let mut no_contact_chirho = valid_document_chirho();
    no_contact_chirho.contact_phone_chirho.clear();
    no_contact_chirho.contact_email_chirho.clear();

    let mut unsafe_phone_chirho = valid_document_chirho();
    unsafe_phone_chirho.contact_phone_chirho = "555-0100?body=redirect".to_string();

    let mut unsafe_email_chirho = valid_document_chirho();
    unsafe_email_chirho.contact_email_chirho = "hello@good-earth.example?body=redirect".to_string();

    for document_chirho in [no_contact_chirho, unsafe_phone_chirho, unsafe_email_chirho] {
        let html_chirho = render_json_chirho(serde_json::json!({
            "app_params_chirho": document_chirho
        }));
        assert!(html_chirho.contains("page-unavailable-chirho"));
        assert!(!html_chirho.contains("redirect"));
        assert!(!html_chirho.contains("mailto:"));
        assert!(!html_chirho.contains("tel:"));
    }
}

#[test]
fn customer_text_is_escaped_after_revalidation_chirho() {
    let mut document_chirho = valid_document_chirho();
    document_chirho.business_name_chirho = "Bright <script>alert(1)</script> & Sons".to_string();
    let html_chirho = render_json_chirho(serde_json::json!({
        "app_params_chirho": document_chirho
    }));

    assert!(!html_chirho.contains("<script"));
    assert!(!html_chirho.contains("Bright <script>alert(1)</script> & Sons"));
    assert!(html_chirho.contains("Bright "));
    assert!(html_chirho.contains(" Sons"));
}

fn main_content_chirho(html_chirho: &str) -> &str {
    html_chirho
        .split_once("<main")
        .expect("one main landmark")
        .1
        .split_once("</main>")
        .expect("main closes")
        .0
}

#[test]
fn preview_and_public_share_customer_content_actions_and_locale_chirho() {
    let document_chirho = valid_document_chirho().validate_chirho().unwrap();
    for locale_chirho in [
        UiLocaleChirho::EnglishChirho,
        UiLocaleChirho::PseudolocaleRtlChirho,
    ] {
        let public_chirho = render_page_chirho(&PagePreviewV1Chirho::published_for_locale_chirho(
            document_chirho.clone(),
            locale_chirho,
        ))
        .unwrap();
        let preview_chirho =
            render_page_chirho(&PagePreviewV1Chirho::private_preview_for_locale_chirho(
                document_chirho.clone(),
                "/app-chirho/perffection-workspace/",
                locale_chirho,
            ))
            .unwrap();
        // Compare the two live outputs, not a golden document: the rail is the only page-body mode
        // difference, and catalogs may change without updating expected prose or screenshots.
        assert_eq!(
            main_content_chirho(&public_chirho),
            main_content_chirho(&preview_chirho)
        );
        assert!(preview_chirho.contains("page-preview-rail-chirho"));
        assert!(!public_chirho.contains("page-preview-rail-chirho"));
        for html_chirho in [&public_chirho, &preview_chirho] {
            assert!(html_chirho.contains(&format!("lang=\"{}\"", locale_chirho.lang_chirho())));
            assert!(html_chirho.contains(&format!("dir=\"{}\"", locale_chirho.dir_chirho())));
            assert!(html_chirho.contains("href=\"tel:+15550100100\""));
            assert!(html_chirho.contains("href=\"mailto:hello@good-earth.example\""));
            assert!(html_chirho.contains("Good Earth Bakery"));
            assert!(!html_chirho.contains("<script"));
            assert!(!html_chirho.contains("<form"));
        }
    }
}

#[test]
fn catalog_expansion_preserves_customer_words_and_named_title_slot_chirho() {
    let english_chirho = PageCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
    let expanded_chirho = PageCopyChirho::for_locale_chirho(UiLocaleChirho::PseudolocaleRtlChirho);
    let name_chirho = "A & B <Bakery>";
    assert!(
        expanded_chirho.preview_title_template_chirho.len()
            > english_chirho.preview_title_template_chirho.len()
    );
    assert!(expanded_chirho
        .preview_title_chirho(name_chirho)
        .contains(name_chirho));
    assert!(!expanded_chirho
        .preview_title_chirho(name_chirho)
        .contains("{business_name_chirho}"));

    let mut document_chirho = valid_document_chirho();
    document_chirho.business_name_chirho = name_chirho.to_string();
    document_chirho.services_chirho[0].body_chirho = "</p><script>bad()</script>".to_string();
    let html_chirho = render_page_chirho(&PagePreviewV1Chirho::published_for_locale_chirho(
        document_chirho.validate_chirho().unwrap(),
        UiLocaleChirho::PseudolocaleRtlChirho,
    ))
    .unwrap();
    assert!(!html_chirho.contains("<Bakery>"));
    assert!(!html_chirho.contains("<script>"));
    assert!(html_chirho.contains("Bakery"));
    assert!(html_chirho.contains("bad()"));
}

#[test]
fn browser_cannot_choose_a_preview_mode_or_catalog_through_public_params_chirho() {
    for field_chirho in ["locale_chirho", "preview_return_href_chirho"] {
        let mut input_chirho = serde_json::json!({ "app_params_chirho": valid_document_chirho() });
        input_chirho[field_chirho] = "untrusted-chirho".into();
        let html_chirho = render_json_chirho(input_chirho);
        assert!(html_chirho.contains("page-unavailable-chirho"));
        assert!(!html_chirho.contains("Good Earth Bakery"));
        assert!(!html_chirho.contains("untrusted-chirho"));
        assert!(!html_chirho.contains("mailto:"));
    }
}
