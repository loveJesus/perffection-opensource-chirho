// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use perffection_ui_chirho::UiLocaleChirho;
use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PageValidationCodeChirho,
    PAGE_DOCUMENT_MAX_SERIALIZED_BYTES_CHIRHO, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_workspace_chirho::page_render_chirho::{
    render_page_chirho, render_workspace_chirho,
};
use perffection_workspace_chirho::workspace_page_chirho::{
    PagePreviewV1Chirho, WorkspaceEditorBindingV1Chirho, WorkspacePageV1Chirho,
    AUTH_LOGOUT_HREF_CHIRHO, WORKSPACE_HELP_HREF_CHIRHO, WORKSPACE_PREVIEW_HREF_CHIRHO,
    WORKSPACE_SAVE_ACTION_CHIRHO,
};

fn valid_document_chirho() -> PageDocumentV2Chirho {
    PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "  Good Earth Bakery  ".to_string(),
        headline_chirho: "Bread made before the town wakes.".to_string(),
        summary_chirho: "Small-batch loaves and morning pastries for our neighbors.".to_string(),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "Daily bread".to_string(),
                body_chirho: "Naturally leavened loaves, baked each morning.".to_string(),
            },
            PageServiceV1Chirho {
                title_chirho: "Celebration cakes".to_string(),
                body_chirho: "Made to order for the days worth remembering.".to_string(),
            },
            PageServiceV1Chirho::default(),
        ],
        contact_heading_chirho: "Save a loaf for tomorrow.".to_string(),
        contact_phone_chirho: "+1 (555) 010-0100".to_string(),
        contact_email_chirho: "hello@good-earth.example".to_string(),
        contact_details_chirho: "Open between 7am and 2pm.\r\nServing North Market.".to_string(),
    }
}

#[test]
fn valid_document_is_normalized_and_bounded_chirho() {
    let validated_chirho = valid_document_chirho()
        .validate_chirho()
        .expect("the representative document should validate");

    assert_eq!(validated_chirho.business_name_chirho(), "Good Earth Bakery");
    assert_eq!(validated_chirho.services_chirho().len(), 2);
    assert_eq!(
        validated_chirho.contact_details_chirho(),
        "Open between 7am and 2pm.\nServing North Market."
    );
    assert_eq!(
        validated_chirho
            .contact_phone_chirho()
            .expect("the phone action should be present")
            .href_chirho(),
        "tel:+15550100100"
    );
    assert_eq!(
        validated_chirho
            .contact_email_chirho()
            .expect("the email action should be present")
            .href_chirho(),
        "mailto:hello@good-earth.example"
    );
    assert!(
        serde_json::to_vec(&validated_chirho).unwrap().len()
            <= PAGE_DOCUMENT_MAX_SERIALIZED_BYTES_CHIRHO
    );

    let normalized_document_chirho = validated_chirho.to_document_chirho();
    assert_eq!(normalized_document_chirho.services_chirho.len(), 3);
    assert!(normalized_document_chirho.services_chirho[2]
        .title_chirho
        .is_empty());
    normalized_document_chirho
        .validate_chirho()
        .expect("normalized persisted bytes must revalidate");
}

#[test]
fn missing_partial_oversize_and_control_values_are_refused_chirho() {
    let document_chirho = PageDocumentV2Chirho {
        schema_version_chirho: 99,
        business_name_chirho: "x".repeat(81),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "A title without its description".to_string(),
                body_chirho: String::new(),
            },
            PageServiceV1Chirho::default(),
            PageServiceV1Chirho::default(),
        ],
        contact_phone_chirho: "555-0100".to_string(),
        contact_details_chirho: "public\u{0007}details".to_string(),
        ..PageDocumentV2Chirho::default()
    };

    let error_chirho = document_chirho
        .validate_chirho()
        .expect_err("the malformed document must be refused");
    let codes_chirho: Vec<PageValidationCodeChirho> = error_chirho
        .issues_chirho
        .iter()
        .map(|issue_chirho| issue_chirho.code_chirho)
        .collect();

    assert!(codes_chirho.contains(&PageValidationCodeChirho::UnsupportedSchemaChirho));
    assert!(codes_chirho.contains(&PageValidationCodeChirho::TooManyCharactersChirho));
    assert!(codes_chirho.contains(&PageValidationCodeChirho::RequiredChirho));
    assert!(codes_chirho.contains(&PageValidationCodeChirho::UnsupportedControlCharacterChirho));
}

#[test]
fn aggregate_serialized_size_has_its_own_bound_chirho() {
    let four_byte_character_chirho = "🪨";
    let mut document_chirho = valid_document_chirho();
    document_chirho.business_name_chirho = four_byte_character_chirho.repeat(80);
    document_chirho.headline_chirho = four_byte_character_chirho.repeat(140);
    document_chirho.summary_chirho = four_byte_character_chirho.repeat(600);
    document_chirho.contact_heading_chirho = four_byte_character_chirho.repeat(80);
    document_chirho.contact_details_chirho = four_byte_character_chirho.repeat(400);
    for service_chirho in &mut document_chirho.services_chirho {
        service_chirho.title_chirho = four_byte_character_chirho.repeat(72);
        service_chirho.body_chirho = four_byte_character_chirho.repeat(320);
    }

    let error_chirho = document_chirho
        .validate_chirho()
        .expect_err("aggregate bytes must be bounded even when every field is within its limit");
    assert!(error_chirho.issues_chirho.iter().any(|issue_chirho| {
        issue_chirho.code_chirho == PageValidationCodeChirho::DocumentTooLargeChirho
    }));
}

#[test]
fn page_template_escapes_every_customer_value_chirho() {
    let mut document_chirho = valid_document_chirho();
    document_chirho.business_name_chirho = "Bright <script>alert(1)</script> & Sons".to_string();
    document_chirho.headline_chirho = "\" autofocus onfocus=\"alert(1)".to_string();
    let validated_chirho = document_chirho
        .validate_chirho()
        .expect("markup-like customer text is safe after template escaping");
    let model_chirho = PagePreviewV1Chirho::private_preview_chirho(
        validated_chirho,
        "/app-chirho/perffection-workspace/",
    );
    let html_chirho = render_page_chirho(&model_chirho).expect("the page template should render");

    assert!(html_chirho.starts_with("<!doctype html>"));
    assert!(!html_chirho.contains("<script"));
    assert!(!html_chirho.contains("Bright <script>alert(1)</script> & Sons"));
    assert!(html_chirho.contains("Bright "));
    assert!(html_chirho.contains(" Sons"));
    assert!(!html_chirho.contains("autofocus onfocus=\"alert(1)\""));
    assert_eq!(html_chirho.matches("<h1").count(), 1);
    assert_eq!(html_chirho.matches("<form").count(), 0);
    assert_eq!(html_chirho.matches("<button").count(), 0);
    assert_eq!(html_chirho.matches("<style").count(), 0);
    assert_eq!(html_chirho.matches("style=").count(), 0);
}

#[test]
fn private_preview_and_published_page_are_distinct_chirho() {
    let validated_chirho = valid_document_chirho().validate_chirho().unwrap();
    let private_html_chirho = render_page_chirho(&PagePreviewV1Chirho::private_preview_chirho(
        validated_chirho.clone(),
        "/app-chirho/perffection-workspace/",
    ))
    .unwrap();
    let published_html_chirho =
        render_page_chirho(&PagePreviewV1Chirho::published_chirho(validated_chirho)).unwrap();

    assert_eq!(
        private_html_chirho
            .matches("page-preview-rail-chirho")
            .count(),
        1
    );
    assert_eq!(
        published_html_chirho
            .matches("page-preview-rail-chirho")
            .count(),
        0
    );
    assert!(private_html_chirho.contains("/app-chirho/perffection-workspace/"));
    assert!(!published_html_chirho.contains("/app-chirho/perffection-workspace/"));
}

#[test]
fn editable_workspace_has_one_real_form_contract_chirho() {
    let model_chirho = WorkspacePageV1Chirho::editable_chirho(
        valid_document_chirho(),
        7,
        WorkspaceEditorBindingV1Chirho::route_contract_chirho(),
        &[],
        None,
    );
    let html_chirho =
        render_workspace_chirho(&model_chirho).expect("the workspace template should render");

    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert_eq!(html_chirho.matches("<button").count(), 1);
    assert!(html_chirho.contains(&format!("action=\"{WORKSPACE_SAVE_ACTION_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!("href=\"{WORKSPACE_PREVIEW_HREF_CHIRHO}\"")));
    assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"7\""));
    assert_eq!(html_chirho.matches("name=\"service_").count(), 6);
    assert!(html_chirho.contains("name=\"contact_phone_chirho\" type=\"tel\""));
    assert!(html_chirho.contains("name=\"contact_email_chirho\" type=\"email\""));
    assert!(html_chirho.contains("name=\"contact_details_chirho\""));
    assert_eq!(html_chirho.matches("<script").count(), 0);
    assert_eq!(html_chirho.matches("<style").count(), 0);
    assert_eq!(html_chirho.matches("style=").count(), 0);
}

#[test]
fn unavailable_workspace_exposes_no_action_chirho() {
    let model_chirho = WorkspacePageV1Chirho::unavailable_chirho(
        valid_document_chirho(),
        7,
        "The private state service is unavailable.",
    );
    let html_chirho = render_workspace_chirho(&model_chirho).unwrap();

    assert_eq!(html_chirho.matches("<form").count(), 0);
    assert_eq!(html_chirho.matches("<button").count(), 0);
    assert!(!html_chirho.contains(WORKSPACE_SAVE_ACTION_CHIRHO));
    assert!(!html_chirho.contains(WORKSPACE_PREVIEW_HREF_CHIRHO));
    assert!(html_chirho.contains("workspace-unavailable-chirho"));
}

#[test]
fn editor_pseudolocale_sets_rtl_without_changing_save_or_logout_authority_chirho() {
    let model_chirho = WorkspacePageV1Chirho::editable_for_locale_chirho(
        valid_document_chirho(),
        7,
        WorkspaceEditorBindingV1Chirho::route_contract_chirho(),
        &[],
        None,
        UiLocaleChirho::PseudolocaleRtlChirho,
    );
    let html_chirho = render_workspace_chirho(&model_chirho).unwrap();

    assert!(html_chirho.contains("lang=\"ar-XB\" dir=\"rtl\""));
    assert!(html_chirho.contains(&format!("action=\"{WORKSPACE_SAVE_ACTION_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!("href=\"{AUTH_LOGOUT_HREF_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!("href=\"{WORKSPACE_HELP_HREF_CHIRHO}\"")));
    assert!(html_chirho.contains("⟦!!"));
}

#[test]
fn hand_written_render_wrapper_owns_no_page_markup_chirho() {
    let wrapper_chirho = include_str!("../src/page_render_chirho.rs");
    for element_chirho in [
        "html", "head", "body", "main", "section", "form", "div", "h1",
    ] {
        assert!(!wrapper_chirho.contains(&format!("<{element_chirho}")));
    }
    assert!(!wrapper_chirho.contains("push_str"));
    assert!(!wrapper_chirho.contains("format!"));
}
