// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use perffection_workspace_chirho::implementations_chirho::{
    render_page_document_import_chirho::render_page_document_import_chirho,
    review_page_document_chirho::review_page_document_chirho,
};
use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho,
};
use perffection_workspace_chirho::page_document_export_chirho::percent_encode_chirho;
use perffection_workspace_chirho::page_document_import_chirho::ImportNoticeChirho;
use perffection_workspace_chirho::page_import_request_chirho::{
    parse_page_import_chirho, PageImportOutcomeChirho, PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO,
    PAGE_IMPORT_FORM_MAX_BYTES_CHIRHO,
};
use perffection_workspace_chirho::viewer_state_contract_chirho::{
    PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO, VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
};
use perffection_workspace_chirho::workspace_page_chirho::WORKSPACE_SAVE_ACTION_CHIRHO;
use serde_json::json;

#[path = "../support_chirho/mod.rs"]
mod support_chirho;

fn document_chirho() -> PageDocumentV2Chirho {
    PageDocumentV2Chirho {
        business_name_chirho: "Recovery Bakery Chirho".into(),
        headline_chirho: "Bread for our neighbours".into(),
        summary_chirho: "Local bread, made each morning.".into(),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "Bread".into(),
                body_chirho: "Fresh sourdough.".into(),
            },
            PageServiceV1Chirho::default(),
            PageServiceV1Chirho::default(),
        ],
        contact_heading_chirho: "Ask about today's bread".into(),
        contact_email_chirho: "bakery-chirho@example.invalid".into(),
        ..PageDocumentV2Chirho::default()
    }
}

fn form_chirho(document_json_chirho: &str) -> Vec<u8> {
    format!(
        "expected_version_chirho=7&page_document_chirho={}",
        percent_encode_chirho(document_json_chirho)
    )
    .into_bytes()
}

#[test]
fn exported_json_is_reviewed_as_data_at_the_submitted_version_chirho() {
    let document_chirho = document_chirho();
    let bytes_chirho = form_chirho(&serde_json::to_string_pretty(&document_chirho).unwrap());
    match parse_page_import_chirho(&bytes_chirho) {
        PageImportOutcomeChirho::DraftChirho {
            document_chirho: parsed_chirho,
            expected_version_chirho,
        } => {
            assert_eq!(*parsed_chirho, document_chirho);
            assert_eq!(expected_version_chirho, 7);
        }
        _ => panic!("valid export must be reviewable"),
    }
    let html_chirho = String::from_utf8(review_page_document_chirho(&bytes_chirho)).unwrap();
    assert!(html_chirho.contains(&format!("action=\"{WORKSPACE_SAVE_ACTION_CHIRHO}\"")));
    assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"7\""));
    assert!(html_chirho.contains(&document_chirho.headline_chirho));
    assert!(!html_chirho.contains("name=\"restore_from_version_chirho\""));
    assert!(!html_chirho.contains("<script"));
    assert!(!html_chirho.contains("viewer-state-request-v1-chirho"));
}

#[test]
fn imported_invalid_fields_remain_editable_but_do_not_become_a_write_chirho() {
    let mut document_chirho = document_chirho();
    document_chirho.contact_email_chirho.clear();
    let bytes_chirho = form_chirho(&serde_json::to_string(&document_chirho).unwrap());
    let html_chirho = String::from_utf8(review_page_document_chirho(&bytes_chirho)).unwrap();
    assert!(html_chirho.contains("role=\"alert\""));
    assert!(html_chirho.contains(&document_chirho.business_name_chirho));
    assert!(html_chirho.contains(WORKSPACE_SAVE_ACTION_CHIRHO));
    assert!(!html_chirho.contains("viewer-state-request-v1-chirho"));
}

#[test]
fn malformed_or_scope_selecting_forms_are_actionless_chirho() {
    let valid_chirho = String::from_utf8(form_chirho(
        &serde_json::to_string(&document_chirho()).unwrap(),
    ))
    .unwrap();
    for body_chirho in [
        String::new(),
        valid_chirho.replace("expected_version_chirho=7", "expected_version_chirho=-1"),
        valid_chirho.replace(
            "expected_version_chirho=7",
            "expected_version_chirho=18446744073709551616",
        ),
        format!("{valid_chirho}&expected_version_chirho=8"),
        format!("{valid_chirho}&viewer_chirho=someone-else"),
        format!("{valid_chirho}&page_document_chirho=%zz"),
        "page_document_chirho=%FF&expected_version_chirho=7".into(),
    ] {
        assert!(matches!(
            parse_page_import_chirho(body_chirho.as_bytes()),
            PageImportOutcomeChirho::UnreadableFormChirho
        ));
        let html_chirho =
            String::from_utf8(review_page_document_chirho(body_chirho.as_bytes())).unwrap();
        assert!(!html_chirho.contains("<form"));
    }
}

#[test]
fn json_unknown_and_duplicate_fields_are_not_an_authority_channel_chirho() {
    let valid_chirho = serde_json::to_string(&document_chirho()).unwrap();
    for text_chirho in [
        valid_chirho.replacen('{', "{\"owner_chirho\":\"owner/other\",", 1),
        valid_chirho.replacen('{', "{\"schema_version_chirho\":2,", 1),
        format!("{valid_chirho} trailing"),
        "null".into(),
    ] {
        assert!(matches!(
            parse_page_import_chirho(&form_chirho(&text_chirho)),
            PageImportOutcomeChirho::RejectedChirho {
                notice_chirho: ImportNoticeChirho::InvalidJsonChirho,
                ..
            }
        ));
    }
    let mut future_chirho = document_chirho();
    future_chirho.schema_version_chirho = 99;
    assert!(matches!(
        parse_page_import_chirho(&form_chirho(
            &serde_json::to_string(&future_chirho).unwrap()
        )),
        PageImportOutcomeChirho::RejectedChirho {
            notice_chirho: ImportNoticeChirho::UnsupportedSchemaChirho,
            ..
        }
    ));
}

#[test]
fn over_limit_content_is_bounded_without_response_amplification_chirho() {
    let oversized_chirho = "x".repeat(PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO + 1);
    match parse_page_import_chirho(&form_chirho(&oversized_chirho)) {
        PageImportOutcomeChirho::RejectedChirho {
            document_json_chirho,
            notice_chirho: ImportNoticeChirho::DocumentTooLargeChirho,
            ..
        } => assert!(document_json_chirho.is_empty()),
        _ => panic!("oversized pasted input must not be echoed"),
    }
    assert!(matches!(
        parse_page_import_chirho(&vec![b'x'; PAGE_IMPORT_FORM_MAX_BYTES_CHIRHO + 1]),
        PageImportOutcomeChirho::UnreadableFormChirho
    ));
}

#[test]
fn imported_markup_and_invalid_raw_text_are_escaped_chirho() {
    let attack_chirho = "</textarea><script>alert(1)</script>";
    let html_chirho =
        String::from_utf8(review_page_document_chirho(&form_chirho(attack_chirho))).unwrap();
    assert!(!html_chirho.contains("<script>"));
    assert!(html_chirho.contains("alert(1)"));
    assert!(!html_chirho.contains(attack_chirho));
    let mut document_chirho = document_chirho();
    document_chirho.headline_chirho = attack_chirho.into();
    let html_chirho = String::from_utf8(review_page_document_chirho(&form_chirho(
        &serde_json::to_string(&document_chirho).unwrap(),
    )))
    .unwrap();
    assert!(!html_chirho.contains("<script>"));
    assert!(html_chirho.contains("alert(1)"));
    assert!(!html_chirho.contains(attack_chirho));
}

#[test]
fn unverified_current_state_cannot_masquerade_as_version_zero_chirho() {
    let empty_chirho = serde_json::to_vec(&json!({"schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO, "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO, "history_chirho": []})).unwrap();
    let html_chirho = String::from_utf8(render_page_document_import_chirho(&empty_chirho)).unwrap();
    assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"0\""));
    for bad_chirho in [
        b"garbage".as_slice(),
        b"{\"schema_chirho\":\"wrong\"}".as_slice(),
    ] {
        let html_chirho =
            String::from_utf8(render_page_document_import_chirho(bad_chirho)).unwrap();
        assert!(!html_chirho.contains("<form"));
    }
}

#[test]
fn translated_import_copy_cannot_change_action_or_document_bytes_chirho() {
    use perffection_ui_chirho::UiLocaleChirho;
    use perffection_workspace_chirho::page_document_import_chirho::{
        PageDocumentImportV1Chirho, WORKSPACE_IMPORT_REVIEW_ACTION_CHIRHO,
    };
    use perffection_workspace_chirho::page_render_chirho::render_page_document_import_chirho as render_import_chirho;
    let raw_chirho = "synthetic-original-json-chirho";
    let model_chirho = PageDocumentImportV1Chirho::rejected_for_locale_chirho(
        9,
        raw_chirho.into(),
        ImportNoticeChirho::InvalidJsonChirho,
        UiLocaleChirho::PseudolocaleRtlChirho,
    );
    let html_chirho = render_import_chirho(&model_chirho).unwrap();
    assert!(html_chirho.contains("lang=\"ar-XB\" dir=\"rtl\""));
    assert!(html_chirho.contains(&format!(
        "action=\"{WORKSPACE_IMPORT_REVIEW_ACTION_CHIRHO}\""
    )));
    assert!(html_chirho.contains(&format!(">{raw_chirho}</textarea>")));
    assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"9\""));
    assert!(html_chirho.contains("⟦!!"));
}

#[test]
fn a_refused_paste_stays_in_the_field_with_the_reason_beside_it_chirho() {
    use perffection_workspace_chirho::page_document_import_chirho::PageDocumentImportV1Chirho;
    use perffection_workspace_chirho::page_render_chirho::render_page_document_import_chirho as render_import_chirho;
    let pasted_chirho = "{ not json at all";
    let html_chirho = render_import_chirho(&PageDocumentImportV1Chirho::rejected_chirho(
        4,
        pasted_chirho.into(),
        ImportNoticeChirho::InvalidJsonChirho,
    ))
    .unwrap();
    // her paste is still there to fix, the field is marked and described by help and reason
    assert!(html_chirho.contains(&format!(">{pasted_chirho}</textarea>")));
    let field_chirho =
        support_chirho::opening_tag_chirho(&html_chirho, "page-document-import-chirho");
    assert!(
        field_chirho.contains(" aria-invalid=\"true\""),
        "{field_chirho}"
    );
    assert_eq!(
        support_chirho::attribute_tokens_chirho(field_chirho, "aria-describedby"),
        vec![
            "page-document-import-help-chirho",
            "page-document-import-error-chirho"
        ]
    );
    // said at the top as a link to the field, and once beside it
    assert_eq!(
        html_chirho
            .matches("href=\"#page-document-import-chirho\"")
            .count(),
        1
    );
    assert_eq!(
        html_chirho
            .matches(" id=\"page-document-import-error-chirho\"")
            .count(),
        1
    );
    assert_eq!(
        support_chirho::assert_page_wiring_chirho("refused import", &html_chirho),
        1
    );
    // the review form it hands back carries the engine's caller-key placeholder once
    assert_eq!(
        support_chirho::assert_caller_key_forms_chirho("refused import", &html_chirho),
        1
    );

    // a clean form carries no invalid state
    let clean_chirho = render_import_chirho(&PageDocumentImportV1Chirho::ready_chirho(4)).unwrap();
    // the carrier's four dormant notices are hidden until targeted; everything else must be clean
    let clean_chirho = support_chirho::without_carrier_notices_chirho(&clean_chirho);
    assert!(!clean_chirho.contains("aria-invalid"));
    assert!(!clean_chirho.contains("page-document-import-error-chirho"));
    assert!(!clean_chirho.contains("role=\"alert\""));

    // a paste too large to hand back is refused visibly, never truncated: nothing of it returns
    let oversized_chirho = "x".repeat(
        perffection_workspace_chirho::page_document_import_chirho::PAGE_DOCUMENT_IMPORT_MAX_BYTES_CHIRHO + 1,
    );
    let html_chirho = render_import_chirho(&PageDocumentImportV1Chirho::rejected_chirho(
        4,
        oversized_chirho,
        ImportNoticeChirho::InvalidJsonChirho,
    ))
    .unwrap();
    assert!(html_chirho.contains("></textarea>"));
    assert!(
        support_chirho::opening_tag_chirho(&html_chirho, "page-document-import-chirho")
            .contains(" aria-invalid=\"true\"")
    );
}
