// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_workspace_chirho::viewer_state_contract_chirho::{
    build_page_read_request_chirho, build_page_restore_request_chirho,
    build_page_write_request_chirho, decode_page_state_response_chirho,
    ViewerStateContractErrorChirho, PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
    PAGE_STATE_HISTORY_MAX_VERSIONS_CHIRHO, VIEWER_STATE_REQUEST_SCHEMA_CHIRHO,
    VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
};
use serde_json::{json, Value};

fn valid_document_chirho() -> PageDocumentV2Chirho {
    PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "  North Street Repairs  ".to_string(),
        headline_chirho: "Clear help for the work that cannot wait.".to_string(),
        summary_chirho: "Local repair service with straightforward scheduling.".to_string(),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "Same-week repairs".to_string(),
                body_chirho: "Practical repair work for homes and small shops.".to_string(),
            },
            PageServiceV1Chirho::default(),
            PageServiceV1Chirho::default(),
        ],
        contact_heading_chirho: "Tell us what needs attention.".to_string(),
        contact_phone_chirho: "555-0124".to_string(),
        contact_email_chirho: String::new(),
        contact_details_chirho: "Available during business hours.".to_string(),
    }
}

#[test]
fn read_request_uses_one_fixed_key_and_cannot_select_scope_chirho() {
    let request_chirho: Value = serde_json::from_slice(&build_page_read_request_chirho().unwrap())
        .expect("the read request should be JSON");

    assert_eq!(
        request_chirho["schema_chirho"],
        VIEWER_STATE_REQUEST_SCHEMA_CHIRHO
    );
    assert_eq!(request_chirho["action_chirho"], "read-chirho");
    assert_eq!(
        request_chirho["key_chirho"],
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO
    );
    assert!(request_chirho
        .get("expected_current_version_chirho")
        .is_none());
    assert!(request_chirho.get("value_base64_chirho").is_none());
    assert!(request_chirho.get("restore_from_version_chirho").is_none());
    let request_text_chirho = request_chirho.to_string();
    assert!(!request_text_chirho.contains("principal_chirho"));
    assert!(!request_text_chirho.contains("viewer_chirho"));
    assert!(!request_text_chirho.contains("app_chirho"));
    assert!(!request_text_chirho.contains("workspace_chirho"));
}

#[test]
fn write_request_carries_validated_normalized_page_and_exact_expected_version_chirho() {
    let document_chirho = valid_document_chirho().validate_chirho().unwrap();
    let request_chirho: Value = serde_json::from_slice(
        &build_page_write_request_chirho(&document_chirho, Some(4)).unwrap(),
    )
    .unwrap();

    assert_eq!(request_chirho["action_chirho"], "write-chirho");
    assert_eq!(
        request_chirho["key_chirho"],
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO
    );
    assert_eq!(request_chirho["expected_current_version_chirho"], 4);
    assert!(request_chirho.get("restore_from_version_chirho").is_none());

    let page_bytes_chirho = BASE64_CHIRHO
        .decode(request_chirho["value_base64_chirho"].as_str().unwrap())
        .unwrap();
    let persisted_chirho: PageDocumentV2Chirho =
        serde_json::from_slice(&page_bytes_chirho).unwrap();
    assert_eq!(
        persisted_chirho.business_name_chirho,
        "North Street Repairs"
    );
    persisted_chirho
        .validate_chirho()
        .expect("the exact persisted bytes must revalidate");

    assert_eq!(
        build_page_write_request_chirho(&document_chirho, Some(0)).unwrap_err(),
        ViewerStateContractErrorChirho::InvalidExpectedVersionChirho
    );
}

#[test]
fn first_write_and_restore_requests_preserve_optimistic_semantics_chirho() {
    let document_chirho = valid_document_chirho().validate_chirho().unwrap();
    let create_chirho: Value =
        serde_json::from_slice(&build_page_write_request_chirho(&document_chirho, None).unwrap())
            .unwrap();
    assert!(create_chirho
        .get("expected_current_version_chirho")
        .is_none());

    let restore_chirho: Value =
        serde_json::from_slice(&build_page_restore_request_chirho(7, 3).unwrap()).unwrap();
    assert_eq!(restore_chirho["action_chirho"], "restore-chirho");
    assert_eq!(restore_chirho["expected_current_version_chirho"], 7);
    assert_eq!(restore_chirho["restore_from_version_chirho"], 3);
    assert!(restore_chirho.get("value_base64_chirho").is_none());

    assert_eq!(
        build_page_restore_request_chirho(0, 1).unwrap_err(),
        ViewerStateContractErrorChirho::InvalidExpectedVersionChirho
    );
    assert_eq!(
        build_page_restore_request_chirho(7, 7).unwrap_err(),
        ViewerStateContractErrorChirho::InvalidRestoreVersionChirho
    );
    assert_eq!(
        build_page_restore_request_chirho(7, 0).unwrap_err(),
        ViewerStateContractErrorChirho::InvalidRestoreVersionChirho
    );
}

#[test]
fn response_decoder_revalidates_the_exact_private_page_bytes_chirho() {
    let normalized_chirho = valid_document_chirho()
        .validate_chirho()
        .unwrap()
        .to_document_chirho();
    let current_chirho = json!({
        "version_chirho": 9,
        "value_base64_chirho": BASE64_CHIRHO.encode(serde_json::to_vec(&normalized_chirho).unwrap()),
        "receipt_chirho": { "ignored_by_product_adapter_chirho": true }
    });
    let response_chirho = json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": current_chirho.clone(),
        "history_chirho": [current_chirho],
        "mutation_receipt_chirho": null
    });
    let snapshot_chirho =
        decode_page_state_response_chirho(&serde_json::to_vec(&response_chirho).unwrap())
            .unwrap()
            .expect("the current page should be present");

    assert_eq!(snapshot_chirho.current_version_chirho(), 9);
    assert_eq!(snapshot_chirho.history_chirho().len(), 1);
    assert_eq!(snapshot_chirho.history_chirho()[0].version_chirho(), 9);
    assert_eq!(
        snapshot_chirho.document_chirho().business_name_chirho(),
        "North Street Repairs"
    );

    let missing_chirho = json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": null,
        "history_chirho": []
    });
    assert_eq!(
        decode_page_state_response_chirho(&serde_json::to_vec(&missing_chirho).unwrap()).unwrap(),
        None
    );
}

#[test]
fn malformed_wrong_scope_and_invalid_document_responses_fail_closed_chirho() {
    assert_eq!(
        decode_page_state_response_chirho(b"not-json").unwrap_err(),
        ViewerStateContractErrorChirho::MalformedResponseChirho
    );

    for (response_chirho, expected_chirho) in [
        (
            json!({
                "schema_chirho": "viewer-state-response-v1-chirho",
                "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
                "current_chirho": null,
                "history_chirho": []
            }),
            ViewerStateContractErrorChirho::UnsupportedResponseSchemaChirho,
        ),
        (
            json!({
                "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
                "key_chirho": "another-key-chirho",
                "current_chirho": null,
                "history_chirho": []
            }),
            ViewerStateContractErrorChirho::WrongLogicalKeyChirho,
        ),
        (
            json!({
                "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
                "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
                "current_chirho": {"version_chirho": 0, "value_base64_chirho": "e30="},
                "history_chirho": [{"version_chirho": 0, "value_base64_chirho": "e30="}]
            }),
            ViewerStateContractErrorChirho::InvalidCurrentVersionChirho,
        ),
        (
            json!({
                "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
                "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
                "current_chirho": {"version_chirho": 1, "value_base64_chirho": "%%%"},
                "history_chirho": [{"version_chirho": 1, "value_base64_chirho": "%%%"}]
            }),
            ViewerStateContractErrorChirho::InvalidBase64ValueChirho,
        ),
    ] {
        assert_eq!(
            decode_page_state_response_chirho(&serde_json::to_vec(&response_chirho).unwrap())
                .unwrap_err(),
            expected_chirho
        );
    }

    let invalid_document_chirho = PageDocumentV2Chirho::default();
    let invalid_current_chirho = json!({
        "version_chirho": 1,
        "value_base64_chirho": BASE64_CHIRHO.encode(serde_json::to_vec(&invalid_document_chirho).unwrap())
    });
    let invalid_response_chirho = json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": invalid_current_chirho.clone(),
        "history_chirho": [invalid_current_chirho]
    });
    assert_eq!(
        decode_page_state_response_chirho(&serde_json::to_vec(&invalid_response_chirho).unwrap())
            .unwrap_err(),
        ViewerStateContractErrorChirho::InvalidPageDocumentChirho
    );
}

#[test]
fn history_must_be_bounded_current_first_and_contiguous_chirho() {
    let document_chirho = valid_document_chirho()
        .validate_chirho()
        .unwrap()
        .to_document_chirho();
    let value_base64_chirho = BASE64_CHIRHO.encode(serde_json::to_vec(&document_chirho).unwrap());
    let version_chirho = |version_chirho: u64| {
        json!({
            "version_chirho": version_chirho,
            "value_base64_chirho": value_base64_chirho,
        })
    };

    let inconsistent_chirho = json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": version_chirho(4),
        "history_chirho": [version_chirho(3), version_chirho(2)]
    });
    assert_eq!(
        decode_page_state_response_chirho(&serde_json::to_vec(&inconsistent_chirho).unwrap())
            .unwrap_err(),
        ViewerStateContractErrorChirho::InconsistentCurrentHistoryChirho
    );

    let non_contiguous_chirho = json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": version_chirho(4),
        "history_chirho": [version_chirho(4), version_chirho(2)]
    });
    assert_eq!(
        decode_page_state_response_chirho(&serde_json::to_vec(&non_contiguous_chirho).unwrap())
            .unwrap_err(),
        ViewerStateContractErrorChirho::NonContiguousHistoryChirho
    );

    let oversized_history_chirho = (1..=PAGE_STATE_HISTORY_MAX_VERSIONS_CHIRHO + 1)
        .rev()
        .map(|version_number_chirho| version_chirho(version_number_chirho as u64))
        .collect::<Vec<_>>();
    let oversized_chirho = json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": oversized_history_chirho[0].clone(),
        "history_chirho": oversized_history_chirho
    });
    assert_eq!(
        decode_page_state_response_chirho(&serde_json::to_vec(&oversized_chirho).unwrap())
            .unwrap_err(),
        ViewerStateContractErrorChirho::InvalidHistoryLengthChirho
    );
}
