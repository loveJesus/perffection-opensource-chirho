// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

mod support_chirho;

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use perffection_workspace_chirho::implementations_chirho::{
    build_page_read_chirho::build_page_read_chirho,
    build_page_restore_chirho::build_page_restore_chirho,
    build_page_write_chirho::build_page_write_chirho,
    render_page_rejection_chirho::render_page_rejection_chirho,
    render_private_preview_chirho::render_private_preview_chirho,
    render_workspace_state_chirho::render_workspace_state_chirho,
};
use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_workspace_chirho::page_edit_request_chirho::{
    CARRIER_REFUSAL_TOKENS_CHIRHO, FORM_CALLER_KEY_FIELD_CHIRHO, FORM_CALLER_KEY_PLACEHOLDER_CHIRHO,
};
use perffection_workspace_chirho::viewer_state_contract_chirho::{
    PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO, VIEWER_STATE_REQUEST_SCHEMA_CHIRHO,
    VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
};
use perffection_workspace_chirho::workspace_page_chirho::{
    WORKSPACE_HOME_HREF_CHIRHO, WORKSPACE_RESTORE_ACTION_CHIRHO,
};
use serde_json::{json, Value};

fn valid_form_chirho(expected_version_chirho: u64) -> Vec<u8> {
    format!(
        "schema_version_chirho=2&expected_version_chirho={expected_version_chirho}&business_name_chirho=North+Street+Repairs&headline_chirho=Clear+help&summary_chirho=Straightforward+local+service&service_1_title_chirho=Repairs&service_1_body_chirho=Homes+and+small+shops&service_2_title_chirho=&service_2_body_chirho=&service_3_title_chirho=&service_3_body_chirho=&contact_heading_chirho=Tell+us+what+needs+attention&contact_phone_chirho=555-0124&contact_email_chirho=&contact_details_chirho=During+business+hours"
    )
    .into_bytes()
}

fn current_response_chirho(version_chirho: u64) -> Vec<u8> {
    named_response_chirho(version_chirho, "North Street Repairs")
}

fn named_response_chirho(version_chirho: u64, business_name_chirho: &str) -> Vec<u8> {
    let document_chirho = PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: business_name_chirho.to_string(),
        headline_chirho: "Clear help".to_string(),
        summary_chirho: "Straightforward local service".to_string(),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "Repairs".to_string(),
                body_chirho: "Homes and small shops".to_string(),
            },
            PageServiceV1Chirho::default(),
            PageServiceV1Chirho::default(),
        ],
        contact_heading_chirho: "Tell us what needs attention".to_string(),
        contact_phone_chirho: "555-0124".to_string(),
        contact_email_chirho: String::new(),
        contact_details_chirho: "During business hours".to_string(),
    };
    let current_chirho = json!({
        "version_chirho": version_chirho,
        "value_base64_chirho": BASE64_CHIRHO.encode(serde_json::to_vec(&document_chirho).unwrap()),
    });
    serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": current_chirho.clone(),
        "history_chirho": [current_chirho],
    }))
    .unwrap()
}

fn history_response_chirho() -> Vec<u8> {
    let response_chirho: Value = serde_json::from_slice(&current_response_chirho(3)).unwrap();
    let current_chirho = response_chirho["current_chirho"].clone();
    let value_base64_chirho = current_chirho["value_base64_chirho"].clone();
    serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": current_chirho,
        "history_chirho": [
            {"version_chirho": 3, "value_base64_chirho": value_base64_chirho.clone()},
            {"version_chirho": 2, "value_base64_chirho": value_base64_chirho.clone()},
            {"version_chirho": 1, "value_base64_chirho": value_base64_chirho},
        ],
    }))
    .unwrap()
}

#[test]
fn read_node_emits_only_the_fixed_viewer_state_request_chirho() {
    let request_chirho: Value =
        serde_json::from_slice(&build_page_read_chirho(b"ignored request bytes")).unwrap();

    assert_eq!(
        request_chirho["schema_chirho"],
        VIEWER_STATE_REQUEST_SCHEMA_CHIRHO
    );
    assert_eq!(request_chirho["action_chirho"], "read-chirho");
    assert_eq!(
        request_chirho["key_chirho"],
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO
    );
    let request_text_chirho = request_chirho.to_string();
    for forbidden_chirho in [
        "principal_chirho",
        "viewer_chirho",
        "app_chirho",
        "workspace_chirho",
        "published_chirho",
    ] {
        assert!(!request_text_chirho.contains(forbidden_chirho));
    }
}

#[test]
fn write_node_emits_one_optimistic_private_write_and_refuses_bad_forms_chirho() {
    let request_chirho: Value =
        serde_json::from_slice(&build_page_write_chirho(&valid_form_chirho(7))).unwrap();

    assert_eq!(request_chirho["action_chirho"], "write-chirho");
    assert_eq!(request_chirho["expected_current_version_chirho"], 7);
    assert_eq!(
        request_chirho["key_chirho"],
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO
    );
    assert!(request_chirho.get("principal_chirho").is_none());
    assert!(request_chirho.get("viewer_chirho").is_none());
    assert!(request_chirho.get("app_chirho").is_none());

    let private_page_bytes_chirho = BASE64_CHIRHO
        .decode(request_chirho["value_base64_chirho"].as_str().unwrap())
        .unwrap();
    let private_page_chirho: PageDocumentV2Chirho =
        serde_json::from_slice(&private_page_bytes_chirho).unwrap();
    assert_eq!(
        private_page_chirho.business_name_chirho,
        "North Street Repairs"
    );

    let bad_form_chirho: &[u8] = b"schema_version_chirho=2&private_secret_chirho=never-echo-this";
    assert_eq!(
        build_page_write_chirho(bad_form_chirho),
        cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO
    );
    let rejection_chirho =
        String::from_utf8(render_page_rejection_chirho(bad_form_chirho)).unwrap();
    assert!(rejection_chirho.contains("<html"));
    assert!(!rejection_chirho.contains("never-echo-this"));
}

#[test]
fn restore_node_emits_only_one_append_restore_coordinate_chirho() {
    let request_chirho: Value = serde_json::from_slice(&build_page_restore_chirho(
        b"expected_version_chirho=7&restore_from_version_chirho=3",
    ))
    .unwrap();

    assert_eq!(request_chirho["action_chirho"], "restore-chirho");
    assert_eq!(request_chirho["expected_current_version_chirho"], 7);
    assert_eq!(request_chirho["restore_from_version_chirho"], 3);
    assert_eq!(
        request_chirho["key_chirho"],
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO
    );
    assert!(request_chirho.get("value_base64_chirho").is_none());
    assert!(request_chirho.get("principal_chirho").is_none());
    assert!(request_chirho.get("viewer_chirho").is_none());
    assert!(request_chirho.get("app_chirho").is_none());

    let refused_chirho = build_page_restore_chirho(
        b"expected_version_chirho=7&restore_from_version_chirho=7&private_secret_chirho=never-echo-this",
    );
    assert_eq!(
        refused_chirho,
        br#"{"schema_chirho":"invalid-page-restore-request-v1-chirho"}"#
    );
    assert!(!String::from_utf8(refused_chirho)
        .unwrap()
        .contains("never-echo-this"));
}

#[test]
fn workspace_renderer_distinguishes_missing_current_and_malformed_state_chirho() {
    let missing_chirho = serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": null,
        "history_chirho": [],
    }))
    .unwrap();
    let missing_html_chirho = String::from_utf8(render_workspace_state_chirho(&missing_chirho))
        .expect("the missing-state workspace should be UTF-8");
    assert_eq!(missing_html_chirho.matches("<form").count(), 1);
    assert!(missing_html_chirho.contains("name=\"expected_version_chirho\" value=\"0\""));

    let malformed_html_chirho =
        String::from_utf8(render_workspace_state_chirho(b"unverified private bytes"))
            .expect("the unavailable workspace should be UTF-8");
    assert_eq!(malformed_html_chirho.matches("<form").count(), 0);
    assert!(malformed_html_chirho.contains("could not be verified"));
    assert!(!malformed_html_chirho.contains("unverified private bytes"));
}

#[test]
fn preview_renderer_exposes_only_a_verified_private_current_page_chirho() {
    let preview_html_chirho =
        String::from_utf8(render_private_preview_chirho(&current_response_chirho(3)))
            .expect("the private preview should be UTF-8");
    assert_eq!(preview_html_chirho.matches("<form").count(), 0);
    assert_eq!(
        preview_html_chirho
            .matches("page-preview-rail-chirho")
            .count(),
        1
    );
    assert!(preview_html_chirho.contains("North Street Repairs"));
    assert!(preview_html_chirho.contains(&format!("href=\"{WORKSPACE_HOME_HREF_CHIRHO}\"")));

    let malformed_html_chirho =
        String::from_utf8(render_private_preview_chirho(b"unverified preview bytes"))
            .expect("the unavailable preview should be UTF-8");
    assert_eq!(malformed_html_chirho.matches("<form").count(), 0);
    assert!(!malformed_html_chirho.contains("unverified preview bytes"));
    assert!(malformed_html_chirho.contains("Nothing was published"));
}

#[test]
fn workspace_history_offers_append_restore_only_for_retained_older_versions_chirho() {
    let history_html_chirho =
        String::from_utf8(render_workspace_state_chirho(&history_response_chirho()))
            .expect("the private history should be UTF-8");

    assert_eq!(history_html_chirho.matches("<form").count(), 3);
    assert_eq!(
        history_html_chirho
            .matches(&format!("action=\"{WORKSPACE_RESTORE_ACTION_CHIRHO}\""))
            .count(),
        2
    );
    assert!(history_html_chirho.contains("name=\"restore_from_version_chirho\" value=\"2\""));
    assert!(history_html_chirho.contains("name=\"restore_from_version_chirho\" value=\"1\""));
    assert!(!history_html_chirho.contains("name=\"restore_from_version_chirho\" value=\"3\""));
    assert_eq!(
        history_html_chirho
            .matches("name=\"expected_version_chirho\" value=\"3\"")
            .count(),
        3
    );
}

/// The browser caller-key carrier (the contract:
/// spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md):
/// the save form and every restore form each carry the engine's empty placeholder once, and nothing
/// else on the page does. Her own words can't forge one, even when they are the placeholder's text.
#[test]
fn every_editor_write_form_carries_one_caller_key_placeholder_her_words_cannot_forge_chirho() {
    let history_html_chirho =
        String::from_utf8(render_workspace_state_chirho(&history_response_chirho())).unwrap();
    assert_eq!(
        support_chirho::assert_caller_key_forms_chirho("history", &history_html_chirho),
        3
    );
    let forged_html_chirho = String::from_utf8(render_workspace_state_chirho(
        &named_response_chirho(3, FORM_CALLER_KEY_PLACEHOLDER_CHIRHO),
    ))
    .unwrap();
    // Control: her name, which is the placeholder's own text, is on the page. It is escaped, but the
    // field's name still shows there beside the one real placeholder, so the page names it at least twice.
    assert!(
        forged_html_chirho
            .matches(FORM_CALLER_KEY_FIELD_CHIRHO)
            .count()
            >= 2
    );
    assert_eq!(
        support_chirho::assert_caller_key_forms_chirho(
            "placeholder as her name",
            &forged_html_chirho
        ),
        1
    );
}

/// The carrier's four refusal tokens (#28751) each have their own sentence in every locale, and no other
/// token borrows one. The one whose outcome is unknown never says "Nothing changed": it has its own
/// heading. Behaviour, not wording: only presence, distinctness and the heading split are pinned.
#[test]
fn each_carrier_refusal_token_has_its_own_sentence_and_the_unknown_one_its_own_heading_chirho() {
    for locale_chirho in [
        perffection_ui_chirho::UiLocaleChirho::EnglishChirho,
        perffection_ui_chirho::UiLocaleChirho::PseudolocaleRtlChirho,
    ] {
        let carrier_chirho =
            perffection_ui_chirho::WorkspaceCopyChirho::for_locale_chirho(locale_chirho)
                .carrier_chirho;
        let sentences_chirho: Vec<&str> = CARRIER_REFUSAL_TOKENS_CHIRHO
            .iter()
            .map(|token_chirho| {
                carrier_chirho
                    .sentence_chirho(token_chirho)
                    .unwrap_or_else(|| panic!("{token_chirho} has no sentence"))
            })
            .collect();
        assert!(sentences_chirho
            .iter()
            .all(|sentence_chirho| !sentence_chirho.trim().is_empty()));
        let distinct_chirho: std::collections::BTreeSet<&str> =
            sentences_chirho.iter().copied().collect();
        assert_eq!(distinct_chirho.len(), CARRIER_REFUSAL_TOKENS_CHIRHO.len());
        assert_eq!(
            carrier_chirho.sentence_chirho("request-invalid-chirho"),
            None
        );
        assert!(!carrier_chirho.unknown_heading_chirho.trim().is_empty());
        assert_ne!(
            carrier_chirho.unknown_heading_chirho,
            carrier_chirho.not_run_heading_chirho
        );
    }
    // each notice the page renders says its own token's sentence, as the catalog maps it
    let html_chirho =
        String::from_utf8(render_workspace_state_chirho(&history_response_chirho())).unwrap();
    let copy_chirho = perffection_ui_chirho::WorkspaceCopyChirho::english_chirho();
    for token_chirho in CARRIER_REFUSAL_TOKENS_CHIRHO {
        let start_chirho = html_chirho
            .find(&format!(r#"<section id="refusal-{token_chirho}""#))
            .unwrap_or_else(|| panic!("no {token_chirho} notice"));
        let notice_chirho = &html_chirho[start_chirho..];
        let notice_chirho = &notice_chirho[..notice_chirho.find("</section>").unwrap()];
        assert!(
            notice_chirho.contains(
                copy_chirho
                    .carrier_chirho
                    .sentence_chirho(token_chirho)
                    .unwrap()
            ),
            "{token_chirho} shows another sentence"
        );
    }
    // the page puts the unknown heading on the unknown notice, and only there
    let unknown_chirho = support_chirho::opening_tag_chirho(
        &html_chirho,
        "refusal-write-outcome-unknown-chirho-heading-chirho",
    );
    let start_chirho = html_chirho.find(unknown_chirho).unwrap();
    assert!(html_chirho[start_chirho..]
        .split("</h2>")
        .next()
        .unwrap()
        .ends_with(copy_chirho.carrier_chirho.unknown_heading_chirho.as_str()));
    assert_eq!(
        html_chirho
            .matches(copy_chirho.carrier_chirho.unknown_heading_chirho.as_str())
            .count(),
        1
    );
}

/// The first minute (L.J. 2026-09-20, the shape of his Genesis intake): ONE form, five guided
/// steps over the same mounted fields, an example in every box, a progress line per step. Pinned
/// as STRUCTURE - one form, every control inside it, every step and anchor present, the save
/// button after the last step - never as prose.
#[test]
fn the_editor_is_one_form_with_five_guided_steps_over_the_same_fields_chirho() {
    let copy_chirho = perffection_ui_chirho::WorkspaceCopyChirho::english_chirho();
    for html_chirho in [
        String::from_utf8(render_workspace_state_chirho(&current_response_chirho(3))).unwrap(),
        String::from_utf8(render_workspace_state_chirho(
            &serde_json::to_vec(&json!({
                "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
                "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
                "current_chirho": null,
                "history_chirho": [],
            }))
            .unwrap(),
        ))
        .unwrap(),
    ] {
        // one form, and every one of the page's controls is inside it
        assert_eq!(
            html_chirho
                .matches("<form class=\"workspace-form-chirho\"")
                .count(),
            1
        );
        let form_start_chirho = html_chirho
            .find("<form class=\"workspace-form-chirho\"")
            .unwrap();
        let form_end_chirho =
            html_chirho[form_start_chirho..].find("</form>").unwrap() + form_start_chirho;
        let form_chirho = &html_chirho[form_start_chirho..form_end_chirho];
        let controls_chirho = [
            "business-name-chirho",
            "headline-chirho",
            "summary-chirho",
            "service-1-title-chirho",
            "service-1-body-chirho",
            "service-2-title-chirho",
            "service-2-body-chirho",
            "service-3-title-chirho",
            "service-3-body-chirho",
            "contact-heading-chirho",
            "contact-phone-chirho",
            "contact-email-chirho",
            "contact-details-chirho",
        ];
        for control_chirho in controls_chirho {
            let tag_chirho = support_chirho::opening_tag_chirho(form_chirho, control_chirho);
            assert!(
                tag_chirho.contains(" name=\""),
                "{control_chirho} is a named control in the form"
            );
            // an example in every box
            assert!(
                tag_chirho.contains(" placeholder=\"") && !tag_chirho.contains(" placeholder=\"\""),
                "{control_chirho} carries an example: {tag_chirho}"
            );
        }
        // every step, its progress line from the catalog, and the chooser's two roads
        for step_chirho in 1..=perffection_ui_chirho::WORKSPACE_GUIDED_STEP_COUNT_CHIRHO {
            assert!(form_chirho.contains(&format!(" id=\"step-{step_chirho}-chirho\"")));
            assert!(form_chirho.contains(&copy_chirho.guided_chirho.progress_chirho(step_chirho)));
        }
        assert!(form_chirho.contains(" id=\"all-fields-chirho\""));
        assert!(form_chirho.contains("href=\"#step-1-chirho\""));
        assert!(form_chirho.contains("href=\"#all-fields-chirho\""));
        // the save button comes after the last step's fields, so "one thing at a time" ends at Save
        assert!(
            form_chirho.rfind("type=\"submit\"").unwrap()
                > form_chirho.find(" id=\"step-5-chirho\"").unwrap()
        );
        // the last step carries the note that earlier answers are still missing (the stylesheet
        // shows it only then), so she is told where Save will take her
        let still_needed_chirho = copy_chirho.guided_chirho.still_needed_chirho.as_str();
        assert_eq!(form_chirho.matches(still_needed_chirho).count(), 1);
        let step_5_start_chirho = form_chirho.find(" id=\"step-5-chirho\"").unwrap();
        assert!(form_chirho[step_5_start_chirho..].contains(still_needed_chirho));
        // no dangling anchor, label or description anywhere on the page
        support_chirho::assert_page_wiring_chirho("guided editor", &html_chirho);
    }
}
