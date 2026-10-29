// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The save route's two branches classify one submission identically: a valid form is exactly one
//! write request, an invalid form is the editor around the customer's own words with a notice per
//! field and no write, and a body our form never produces is an actionless page. Exactly one branch
//! is live for every body, which is what the builtin phi merge requires.

use perffection_workspace_chirho::implementations_chirho::{
    build_page_write_chirho::build_page_write_chirho,
    render_page_rejection_chirho::render_page_rejection_chirho,
};
use perffection_workspace_chirho::page_edit_request_chirho::{
    parse_page_edit_form_outcome_chirho, PageEditFormOutcomeChirho,
};
use perffection_workspace_chirho::page_render_chirho::render_workspace_chirho;
use perffection_workspace_chirho::save_page_decision_chirho::{
    classify_page_edit_form_chirho, SavePageDecisionChirho, UNREADABLE_FORM_STATUS_CHIRHO,
};
use perffection_workspace_chirho::viewer_state_contract_chirho::VIEWER_STATE_REQUEST_SCHEMA_CHIRHO;
use perffection_workspace_chirho::workspace_page_chirho::{
    WORKSPACE_EXPORT_HREF_CHIRHO, WORKSPACE_SAVE_ACTION_CHIRHO,
};
use serde_json::Value;

const SKIPPED_STONE_CHIRHO: &[u8] = cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO;

fn form_chirho(
    expected_version_chirho: u64,
    service_1_chirho: &str,
    phone_chirho: &str,
) -> Vec<u8> {
    format!(
        "schema_version_chirho=2&expected_version_chirho={expected_version_chirho}&business_name_chirho=North+Street+Repairs&headline_chirho=Clear+help&summary_chirho=Straightforward+local+service&service_1_title_chirho={service_1_chirho}&service_1_body_chirho={service_1_chirho}&service_2_title_chirho=&service_2_body_chirho=&service_3_title_chirho=&service_3_body_chirho=&contact_heading_chirho=Tell+us+what+needs+attention&contact_phone_chirho={phone_chirho}&contact_email_chirho=&contact_details_chirho=During+business+hours"
    )
    .into_bytes()
}

#[test]
fn a_valid_form_is_exactly_one_write_and_the_rejection_branch_stays_silent_chirho() {
    let form_chirho = form_chirho(4, "Repairs", "555-0124");
    let SavePageDecisionChirho::WriteChirho(request_chirho) =
        classify_page_edit_form_chirho(&form_chirho)
    else {
        panic!("a valid form must become a write request");
    };
    let request_json_chirho: Value = serde_json::from_slice(&request_chirho).unwrap();
    assert_eq!(
        request_json_chirho["schema_chirho"],
        VIEWER_STATE_REQUEST_SCHEMA_CHIRHO
    );
    assert_eq!(request_json_chirho["action_chirho"], "write-chirho");
    assert_eq!(request_json_chirho["expected_current_version_chirho"], 4);

    assert_eq!(build_page_write_chirho(&form_chirho), request_chirho);
    assert_eq!(
        render_page_rejection_chirho(&form_chirho),
        SKIPPED_STONE_CHIRHO
    );
}

#[test]
fn an_invalid_document_keeps_her_words_and_names_each_failing_field_without_writing_chirho() {
    let form_chirho = form_chirho(4, "", "");
    match parse_page_edit_form_outcome_chirho(&form_chirho) {
        PageEditFormOutcomeChirho::InvalidDocumentChirho {
            draft_chirho,
            expected_current_version_chirho,
            error_chirho,
        } => {
            assert_eq!(draft_chirho.business_name_chirho, "North Street Repairs");
            assert_eq!(expected_current_version_chirho, Some(4));
            let paths_chirho: Vec<&str> = error_chirho
                .issues_chirho
                .iter()
                .map(|issue_chirho| issue_chirho.field_path_chirho.as_str())
                .collect();
            assert!(
                paths_chirho.contains(&"services_chirho"),
                "{paths_chirho:?}"
            );
            assert!(paths_chirho.contains(&"contact_chirho"), "{paths_chirho:?}");
        }
        other_chirho => panic!("expected an invalid document, got {other_chirho:?}"),
    }

    let SavePageDecisionChirho::RejectChirho(model_chirho) =
        classify_page_edit_form_chirho(&form_chirho)
    else {
        panic!("an invalid document must be rejected");
    };
    assert!(!model_chirho.is_saved_state_chirho());
    let html_chirho = render_workspace_chirho(&model_chirho).unwrap();
    // Each failing group is named in the summary and linked to the control that can fix it.
    assert!(html_chirho.contains("href=\"#service-1-title-chirho\""));
    assert!(html_chirho.contains("href=\"#contact-phone-chirho\""));
    assert!(html_chirho.contains("value=\"North Street Repairs\""));
    assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"4\""));
    assert!(html_chirho.contains(WORKSPACE_SAVE_ACTION_CHIRHO));
    assert!(html_chirho.contains(WORKSPACE_EXPORT_HREF_CHIRHO));
    assert!(!html_chirho.contains("Your first saved version will appear here"));

    assert_eq!(
        render_page_rejection_chirho(&form_chirho),
        html_chirho.as_bytes()
    );
    assert_eq!(build_page_write_chirho(&form_chirho), SKIPPED_STONE_CHIRHO);
}

#[test]
fn a_body_our_form_never_produces_is_an_actionless_page_chirho() {
    let SavePageDecisionChirho::RejectChirho(model_chirho) =
        classify_page_edit_form_chirho(b"not=a&page=form")
    else {
        panic!("a malformed body must be rejected");
    };
    let html_chirho = render_workspace_chirho(&model_chirho).unwrap();
    assert!(html_chirho.contains(UNREADABLE_FORM_STATUS_CHIRHO));
    assert!(!html_chirho.contains("<form"));
    assert!(!html_chirho.contains("Current draft"));
}

#[test]
fn exactly_one_branch_is_live_for_every_kind_of_body_chirho() {
    for body_chirho in [
        form_chirho(4, "Repairs", "555-0124"),
        form_chirho(4, "", ""),
        b"not=a&page=form".to_vec(),
        Vec::new(),
    ] {
        let write_skipped_chirho = build_page_write_chirho(&body_chirho) == SKIPPED_STONE_CHIRHO;
        let reject_skipped_chirho =
            render_page_rejection_chirho(&body_chirho) == SKIPPED_STONE_CHIRHO;
        assert_ne!(write_skipped_chirho, reject_skipped_chirho);
    }
}

/// A save from two windows: the stale optimistic write keeps her words (was stale_save_conflict_chirho.rs).
mod stale_save_conflict_chirho {
    // A stale save is answered with the editor at the head version, the attempted words kept, and no
    // unverified echo ever rendered (engine `ViewerStateConflictChirho`, cairn e3da4eb2).

    use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
    use perffection_workspace_chirho::implementations_chirho::render_workspace_state_chirho::render_workspace_state_chirho;
    use perffection_workspace_chirho::page_document_chirho::{
        PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
    };
    use perffection_workspace_chirho::viewer_state_contract_chirho::{
        decode_page_state_outcome_chirho, PageStateOutcomeChirho, PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        VIEWER_STATE_CONFLICT_SCHEMA_CHIRHO, VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
    };
    use serde_json::{json, Value};

    fn document_chirho(business_name_chirho: &str) -> PageDocumentV2Chirho {
        PageDocumentV2Chirho {
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
            contact_details_chirho: String::new(),
        }
    }

    fn encoded_chirho(document_chirho: &PageDocumentV2Chirho) -> String {
        BASE64_CHIRHO.encode(serde_json::to_vec(document_chirho).unwrap())
    }

    fn head_body_chirho(head_version_chirho: u64) -> Value {
        let head_chirho = document_chirho("Head Version Bakery");
        let versions_chirho: Vec<Value> = (1..=head_version_chirho)
            .rev()
            .map(|version_chirho| {
                json!({"version_chirho": version_chirho, "value_base64_chirho": encoded_chirho(&head_chirho)})
            })
            .collect();
        json!({
            "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
            "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
            "current_chirho": versions_chirho[0].clone(),
            "history_chirho": versions_chirho,
        })
    }

    /// The engine's conflict body: the expected version is absent for a first write that met a head,
    /// the head version is stated, extra fields (action, attempted restore target) are present.
    fn conflict_chirho(
        expected_chirho: Option<u64>,
        head_chirho: u64,
        attempted_chirho: Option<Value>,
    ) -> Vec<u8> {
        let mut body_chirho = json!({
            "schema_chirho": VIEWER_STATE_CONFLICT_SCHEMA_CHIRHO,
            "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
            "action_chirho": "write-chirho",
            "current_version_chirho": head_chirho,
            "current_chirho": head_body_chirho(head_chirho),
        });
        if let Some(expected_chirho) = expected_chirho {
            body_chirho["expected_current_version_chirho"] = json!(expected_chirho);
        }
        if let Some(attempted_chirho) = attempted_chirho {
            body_chirho["attempted_value_base64_chirho"] = attempted_chirho;
        }
        serde_json::to_vec(&body_chirho).unwrap()
    }

    #[test]
    fn a_stale_save_keeps_her_words_and_targets_the_head_version_chirho() {
        let attempted_chirho = document_chirho("Her Unsaved Bakery");
        let bytes_chirho =
            conflict_chirho(Some(2), 3, Some(json!(encoded_chirho(&attempted_chirho))));
        let PageStateOutcomeChirho::ConflictChirho(conflict_chirho) =
            decode_page_state_outcome_chirho(&bytes_chirho).unwrap()
        else {
            panic!("a conflict schema must decode as a conflict");
        };
        assert_eq!(conflict_chirho.expected_current_version_chirho(), Some(2));
        assert_eq!(conflict_chirho.head_chirho().current_version_chirho(), 3);
        assert_eq!(
            conflict_chirho.attempted_document_chirho(),
            Some(&attempted_chirho)
        );

        let html_chirho = String::from_utf8(render_workspace_state_chirho(&bytes_chirho)).unwrap();
        assert!(html_chirho.contains("Someone saved version 3 while you were editing"));
        assert!(html_chirho.contains("value=\"Her Unsaved Bakery\""));
        assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"3\""));
        assert!(html_chirho.contains("Restore version 1"));
        assert!(html_chirho.contains("<form"));
    }

    #[test]
    fn a_first_write_that_met_a_head_is_a_conflict_without_an_expected_version_chirho() {
        let bytes_chirho = conflict_chirho(
            None,
            1,
            Some(json!(encoded_chirho(&document_chirho("Her First Try")))),
        );
        let PageStateOutcomeChirho::ConflictChirho(conflict_chirho) =
            decode_page_state_outcome_chirho(&bytes_chirho).unwrap()
        else {
            panic!("a conflict schema must decode as a conflict");
        };
        assert_eq!(conflict_chirho.expected_current_version_chirho(), None);
        let html_chirho = String::from_utf8(render_workspace_state_chirho(&bytes_chirho)).unwrap();
        assert!(html_chirho.contains("Someone saved version 1 while you were editing"));
        assert!(html_chirho.contains("value=\"Her First Try\""));
        assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"1\""));
    }

    #[test]
    fn a_stale_restore_or_an_unverifiable_echo_shows_the_head_document_chirho() {
        let html_chirho = String::from_utf8(render_workspace_state_chirho(&conflict_chirho(
            Some(1),
            2,
            None,
        )))
        .unwrap();
        assert!(html_chirho.contains("Someone saved version 2 before your request"));
        assert!(html_chirho.contains("value=\"Head Version Bakery\""));
        assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"2\""));

        let garbage_chirho = BASE64_CHIRHO
            .encode(b"{\"schema_version_chirho\":2,\"business_name_chirho\":\"<script>\"}");
        let html_chirho = String::from_utf8(render_workspace_state_chirho(&conflict_chirho(
            Some(1),
            2,
            Some(json!(garbage_chirho)),
        )))
        .unwrap();
        assert!(html_chirho.contains("value=\"Head Version Bakery\""));
        assert!(!html_chirho.contains("&lt;script&gt;"));
        assert!(!html_chirho.contains("<script"));
    }

    #[test]
    fn a_conflict_that_names_the_head_as_expected_or_a_plain_state_are_told_apart_chirho() {
        assert!(decode_page_state_outcome_chirho(&conflict_chirho(Some(3), 3, None)).is_err());

        let mut mismatched_chirho: Value =
            serde_json::from_slice(&conflict_chirho(Some(1), 2, None)).unwrap();
        mismatched_chirho["current_version_chirho"] = json!(9);
        assert!(
            decode_page_state_outcome_chirho(&serde_json::to_vec(&mismatched_chirho).unwrap())
                .is_err()
        );

        let state_chirho = serde_json::to_vec(&head_body_chirho(1)).unwrap();
        let PageStateOutcomeChirho::StateChirho(Some(snapshot_chirho)) =
            decode_page_state_outcome_chirho(&state_chirho).unwrap()
        else {
            panic!("a v2 state must still decode as state");
        };
        assert_eq!(snapshot_chirho.current_version_chirho(), 1);
        let html_chirho = String::from_utf8(render_workspace_state_chirho(&state_chirho)).unwrap();
        assert!(!html_chirho.contains("Not saved"));
    }
}
