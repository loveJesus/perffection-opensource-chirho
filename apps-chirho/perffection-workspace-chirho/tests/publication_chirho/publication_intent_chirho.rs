// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Her approval act becomes one door request carrying a version binding, the address's approval
//! base and never the document; the two branches of the publish route are live exactly one at a
//! time; the queued answer and the rejections read as plain words; the editor only points at the
//! Publishing page, and only in the plain saved state.

use perffection_ui_chirho::WorkspaceCopyChirho;
use perffection_workspace_chirho::implementations_chirho::{
    build_publication_intent_chirho::build_publication_intent_chirho,
    render_publication_form_rejection_chirho::render_publication_form_rejection_chirho,
    render_publication_intent_chirho::render_publication_intent_chirho,
};
use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_workspace_chirho::page_render_chirho::render_workspace_chirho;
use perffection_workspace_chirho::publication_intent_chirho::{
    decode_publication_intent_answer_chirho, decode_publication_intent_response_chirho,
    parse_publication_form_chirho, PublicationApprovalChirho, PublicationFormErrorChirho,
    PublicationIntentAnswerChirho, DARK_BASE_CHIRHO, PUBLICATION_INTENT_REFUSAL_SCHEMA_CHIRHO,
    PUBLICATION_INTENT_REQUEST_SCHEMA_CHIRHO, PUBLICATION_INTENT_RESPONSE_SCHEMA_CHIRHO,
    PUBLICATION_TEMPLATE_ID_CHIRHO,
};
use perffection_workspace_chirho::viewer_state_contract_chirho::PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO;
use perffection_workspace_chirho::workspace_page_chirho::{
    WorkspaceEditorBindingV1Chirho, WorkspacePageV1Chirho, WORKSPACE_PUBLICATION_HREF_CHIRHO,
};
use serde_json::{json, Value};

#[path = "../support_chirho/mod.rs"]
mod support_chirho;

const SKIPPED_STONE_CHIRHO: &[u8] = cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO;

/// The engine's short-id grammar (template_registry_chirho.rs `validate_short_id_chirho`):
/// 1..=128 bytes of ASCII letters, digits, dot, dash or underscore.
fn legal_short_id_chirho(value_chirho: &str) -> bool {
    !value_chirho.is_empty()
        && value_chirho.len() <= 128
        && value_chirho.chars().all(|character_chirho| {
            character_chirho.is_ascii_alphanumeric() || matches!(character_chirho, '-' | '_' | '.')
        })
}

fn live_base_chirho() -> String {
    "b".repeat(64)
}

fn document_chirho() -> PageDocumentV2Chirho {
    PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "Good Earth Bakery".to_string(),
        headline_chirho: "Bread".to_string(),
        summary_chirho: "Sourdough.".to_string(),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "Loaves".to_string(),
                body_chirho: "Daily".to_string(),
            },
            PageServiceV1Chirho::default(),
            PageServiceV1Chirho::default(),
        ],
        contact_heading_chirho: "Order ahead".to_string(),
        contact_phone_chirho: "555-0142".to_string(),
        contact_email_chirho: String::new(),
        contact_details_chirho: String::new(),
    }
}

#[test]
fn every_attempt_key_obeys_the_engine_grammar_and_is_stable_per_act_chirho() {
    let long_slug_chirho = "a".repeat(48);
    let receipt_chirho = "f".repeat(64);
    let hash_chirho = "0".repeat(64);
    let base_chirho = live_base_chirho();
    let forms_chirho = [
        format!("kind_chirho=publish-chirho&slug_chirho={long_slug_chirho}&version_chirho=18446744073709551615&receipt_digest_chirho={receipt_chirho}&base_publication_chirho={DARK_BASE_CHIRHO}"),
        format!("kind_chirho=republish-chirho&slug_chirho={long_slug_chirho}&version_chirho=7&receipt_digest_chirho=&base_publication_chirho={base_chirho}"),
        format!("kind_chirho=rollback-chirho&slug_chirho={long_slug_chirho}&params_hash_chirho={hash_chirho}&version_chirho=2&base_publication_chirho={base_chirho}"),
        // the same rollback approved again after the address moved is a new act
        format!("kind_chirho=rollback-chirho&slug_chirho={long_slug_chirho}&params_hash_chirho={hash_chirho}&version_chirho=2&base_publication_chirho={hash_chirho}"),
    ];
    let mut keys_chirho = Vec::new();
    for form_chirho in &forms_chirho {
        let approval_chirho = parse_publication_form_chirho(form_chirho.as_bytes()).unwrap();
        let key_chirho = approval_chirho.idem_key_chirho();
        assert!(legal_short_id_chirho(&key_chirho), "{key_chirho}");
        assert_eq!(key_chirho.len(), 64);
        assert_eq!(key_chirho, approval_chirho.idem_key_chirho());
        keys_chirho.push(key_chirho);
    }
    keys_chirho.sort();
    keys_chirho.dedup();
    assert_eq!(keys_chirho.len(), forms_chirho.len());
}

#[test]
fn a_publish_approval_binds_the_version_and_its_base_and_carries_no_document_chirho() {
    let form_chirho = format!(
        "kind_chirho=publish-chirho&slug_chirho=good-earth-bakery&version_chirho=3&receipt_digest_chirho=abc123&base_publication_chirho={DARK_BASE_CHIRHO}"
    );
    let form_chirho = form_chirho.as_bytes();
    let approval_chirho = parse_publication_form_chirho(form_chirho).unwrap();
    assert_eq!(
        approval_chirho,
        PublicationApprovalChirho::VersionChirho {
            kind_chirho: "publish-chirho",
            slug_chirho: "good-earth-bakery".to_string(),
            version_chirho: 3,
            receipt_digest_chirho: Some("abc123".to_string()),
            base_publication_chirho: DARK_BASE_CHIRHO.to_string(),
        }
    );
    let request_chirho: Value =
        serde_json::from_slice(&build_publication_intent_chirho(form_chirho)).unwrap();
    assert_eq!(
        request_chirho["schema_chirho"],
        PUBLICATION_INTENT_REQUEST_SCHEMA_CHIRHO
    );
    assert_eq!(request_chirho["kind_chirho"], "publish-chirho");
    assert_eq!(request_chirho["slug_chirho"], "good-earth-bakery");
    assert_eq!(request_chirho["base_publication_chirho"], DARK_BASE_CHIRHO);
    assert_eq!(
        request_chirho["viewer_state_chirho"],
        json!({"key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO, "version_chirho": 3, "receipt_digest_chirho": "abc123"})
    );
    assert_eq!(
        request_chirho["template_id_chirho"],
        PUBLICATION_TEMPLATE_ID_CHIRHO
    );
    let text_chirho = request_chirho.to_string();
    for forbidden_chirho in [
        "params_chirho\"",
        "user_principal_chirho",
        "app_params_chirho",
        "business_name_chirho",
    ] {
        assert!(
            !text_chirho.contains(forbidden_chirho),
            "{forbidden_chirho}"
        );
    }
    assert!(legal_short_id_chirho(
        request_chirho["idem_key_chirho"].as_str().unwrap()
    ));
    // identical bytes are the same act (a retry replays); a new version is a new act
    assert_eq!(
        build_publication_intent_chirho(form_chirho),
        build_publication_intent_chirho(form_chirho)
    );
    let next_chirho: Value = serde_json::from_slice(&build_publication_intent_chirho(
        format!("kind_chirho=publish-chirho&slug_chirho=good-earth-bakery&version_chirho=4&receipt_digest_chirho=def456&base_publication_chirho={DARK_BASE_CHIRHO}").as_bytes(),
    ))
    .unwrap();
    assert_ne!(
        next_chirho["idem_key_chirho"],
        request_chirho["idem_key_chirho"]
    );
    // a republish carries the live base the status page showed her, verbatim
    let base_chirho = live_base_chirho();
    let republish_chirho: Value = serde_json::from_slice(&build_publication_intent_chirho(
        format!("kind_chirho=republish-chirho&slug_chirho=good-earth-bakery&version_chirho=3&receipt_digest_chirho=&base_publication_chirho={base_chirho}").as_bytes(),
    ))
    .unwrap();
    assert_eq!(republish_chirho["kind_chirho"], "republish-chirho");
    assert_eq!(republish_chirho["base_publication_chirho"], base_chirho);
    assert_ne!(
        republish_chirho["idem_key_chirho"],
        request_chirho["idem_key_chirho"]
    );
    // an absent receipt digest is omitted, not sent empty
    assert!(republish_chirho["viewer_state_chirho"]
        .get("receipt_digest_chirho")
        .is_none());
    assert_eq!(
        render_publication_form_rejection_chirho(form_chirho),
        SKIPPED_STONE_CHIRHO
    );
}

#[test]
fn a_rollback_names_one_prior_record_and_its_base_and_nothing_else_chirho() {
    let hash_chirho = "a".repeat(64);
    let base_chirho = live_base_chirho();
    let form_chirho = format!(
        "kind_chirho=rollback-chirho&slug_chirho=good-earth-bakery&params_hash_chirho={hash_chirho}&version_chirho=2&base_publication_chirho={base_chirho}"
    );
    let request_chirho: Value =
        serde_json::from_slice(&build_publication_intent_chirho(form_chirho.as_bytes())).unwrap();
    assert_eq!(request_chirho["kind_chirho"], "rollback-chirho");
    assert_eq!(request_chirho["params_hash_chirho"], hash_chirho);
    assert_eq!(request_chirho["base_publication_chirho"], base_chirho);
    assert!(request_chirho.get("viewer_state_chirho").is_none());
    assert!(request_chirho.get("params_chirho").is_none());

    let bad_chirho = format!(
        "kind_chirho=rollback-chirho&slug_chirho=good-earth-bakery&params_hash_chirho=nope&base_publication_chirho={base_chirho}"
    );
    assert_eq!(
        parse_publication_form_chirho(bad_chirho.as_bytes()),
        Err(PublicationFormErrorChirho::InvalidRollbackTargetChirho)
    );
}

#[test]
fn an_approval_without_a_readable_base_is_not_sent_chirho() {
    let hash_chirho = "a".repeat(64);
    let short_chirho = "b".repeat(63);
    let forms_chirho = [
        "kind_chirho=publish-chirho&slug_chirho=fine&version_chirho=3&receipt_digest_chirho=".to_string(),
        "kind_chirho=publish-chirho&slug_chirho=fine&version_chirho=3&base_publication_chirho=".to_string(),
        "kind_chirho=publish-chirho&slug_chirho=fine&version_chirho=3&base_publication_chirho=live".to_string(),
        format!("kind_chirho=republish-chirho&slug_chirho=fine&version_chirho=3&base_publication_chirho={short_chirho}"),
        format!("kind_chirho=rollback-chirho&slug_chirho=fine&params_hash_chirho={hash_chirho}"),
    ];
    for form_chirho in &forms_chirho {
        assert_eq!(
            parse_publication_form_chirho(form_chirho.as_bytes()),
            Err(PublicationFormErrorChirho::UnreadableChirho),
            "{form_chirho}"
        );
        assert_eq!(
            build_publication_intent_chirho(form_chirho.as_bytes()),
            SKIPPED_STONE_CHIRHO
        );
    }
}

#[test]
fn exactly_one_branch_is_live_and_rejections_read_as_words_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let cases_chirho: Vec<(&[u8], PublicationFormErrorChirho, &str)> = vec![
        (
            b"kind_chirho=publish-chirho&slug_chirho=Bad+Name&version_chirho=3&receipt_digest_chirho=&base_publication_chirho=dark-chirho",
            PublicationFormErrorChirho::InvalidSlugChirho,
            &copy_chirho.publication_chirho.invalid_slug_notice_chirho,
        ),
        (
            b"kind_chirho=publish-chirho&slug_chirho=fine&version_chirho=0&receipt_digest_chirho=&base_publication_chirho=dark-chirho",
            PublicationFormErrorChirho::NothingSavedChirho,
            &copy_chirho.publication_chirho.nothing_saved_notice_chirho,
        ),
        (
            b"kind_chirho=unpublish-chirho&slug_chirho=fine&version_chirho=3&base_publication_chirho=dark-chirho",
            PublicationFormErrorChirho::UnknownKindChirho,
            &copy_chirho.publication_chirho.unknown_request_notice_chirho,
        ),
        (
            b"not=a&form",
            PublicationFormErrorChirho::UnreadableChirho,
            &copy_chirho.publication_chirho.unknown_request_notice_chirho,
        ),
    ];
    for (body_chirho, error_chirho, sentence_chirho) in cases_chirho {
        assert_eq!(
            parse_publication_form_chirho(body_chirho),
            Err(error_chirho)
        );
        assert_eq!(
            build_publication_intent_chirho(body_chirho),
            SKIPPED_STONE_CHIRHO
        );
        let html_chirho =
            String::from_utf8(render_publication_form_rejection_chirho(body_chirho)).unwrap();
        assert!(html_chirho.contains(sentence_chirho));
        assert!(html_chirho.contains(&copy_chirho.publication_chirho.rejected_heading_chirho));
        assert!(!html_chirho.contains("<form"));
    }
    for body_chirho in [
        &b"kind_chirho=publish-chirho&slug_chirho=fine&version_chirho=3&receipt_digest_chirho=&base_publication_chirho=dark-chirho"[..],
        &b"kind_chirho=publish-chirho&slug_chirho=Bad+Name&version_chirho=3&base_publication_chirho=dark-chirho"[..],
        &b"kind_chirho=publish-chirho&slug_chirho=fine&version_chirho=3"[..],
        &b""[..],
    ] {
        let intent_skipped_chirho =
            build_publication_intent_chirho(body_chirho) == SKIPPED_STONE_CHIRHO;
        let rejection_skipped_chirho =
            render_publication_form_rejection_chirho(body_chirho) == SKIPPED_STONE_CHIRHO;
        assert_ne!(intent_skipped_chirho, rejection_skipped_chirho);
    }
}

#[test]
fn the_queued_answer_says_on_its_way_and_never_live_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let response_chirho = serde_json::to_vec(&json!({
        "schema_chirho": PUBLICATION_INTENT_RESPONSE_SCHEMA_CHIRHO,
        "intent_digest_chirho": "d1", "seq_chirho": 1, "kind_chirho": "publish-chirho",
        "slug_chirho": "good-earth-bakery", "app_chirho": "good-earth-bakery",
        "address_chirho": "/app-chirho/good-earth-bakery/", "params_chirho": "p1",
        "viewer_state_chirho": {"key_chirho": "page-document-chirho", "version_chirho": 3},
        "at_ms_chirho": 5, "applied_chirho": false
    }))
    .unwrap();
    let receipt_chirho = decode_publication_intent_response_chirho(&response_chirho).unwrap();
    assert_eq!(receipt_chirho.version_chirho, Some(3));
    let html_chirho =
        String::from_utf8(render_publication_intent_chirho(&response_chirho)).unwrap();
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.queued_heading_chirho));
    assert!(html_chirho.contains("/app-chirho/good-earth-bakery/"));
    assert!(html_chirho.contains("publication-status-chirho"));
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.outcome_applied_chirho));
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.outcome_live_chirho));

    let applied_claim_chirho = serde_json::to_vec(&json!({
        "schema_chirho": PUBLICATION_INTENT_RESPONSE_SCHEMA_CHIRHO,
        "intent_digest_chirho": "d1", "kind_chirho": "publish-chirho", "slug_chirho": "x",
        "applied_chirho": true
    }))
    .unwrap();
    assert!(decode_publication_intent_response_chirho(&applied_claim_chirho).is_err());
    let html_chirho =
        String::from_utf8(render_publication_intent_chirho(&applied_claim_chirho)).unwrap();
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.status_unavailable_chirho));
}

#[test]
fn the_doors_typed_refusal_reads_as_words_with_the_way_back_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let cases_chirho = [
        (
            "base-moved-chirho",
            &copy_chirho
                .publication_chirho
                .door_refusal_base_moved_chirho,
        ),
        (
            "version-stale",
            &copy_chirho
                .publication_chirho
                .door_refusal_version_stale_chirho,
        ),
        (
            "no-saved-version-chirho",
            &copy_chirho
                .publication_chirho
                .door_refusal_no_saved_version_chirho,
        ),
        (
            "saved-version-not-a-document-chirho",
            &copy_chirho
                .publication_chirho
                .door_refusal_not_a_document_chirho,
        ),
        (
            "request-invalid-chirho",
            &copy_chirho
                .publication_chirho
                .door_refusal_request_invalid_chirho,
        ),
        (
            "some-future-code-chirho",
            &copy_chirho.publication_chirho.refusal_unknown_chirho,
        ),
    ];
    for (code_chirho, sentence_chirho) in cases_chirho {
        let body_chirho = serde_json::to_vec(&json!({
            "schema_chirho": PUBLICATION_INTENT_REFUSAL_SCHEMA_CHIRHO,
            "refusal_code_chirho": code_chirho,
            "message_chirho": "engine sentence never shown",
            "live_base_chirho": "f".repeat(64),
            "head_version_chirho": 7
        }))
        .unwrap();
        assert!(matches!(
            decode_publication_intent_answer_chirho(&body_chirho),
            Ok(PublicationIntentAnswerChirho::RefusedChirho(_))
        ));
        let html_chirho =
            String::from_utf8(render_publication_intent_chirho(&body_chirho)).unwrap();
        assert!(html_chirho.contains(sentence_chirho), "{code_chirho}");
        assert!(html_chirho.contains(&copy_chirho.publication_chirho.door_refused_heading_chirho));
        assert!(html_chirho.contains("publication-status-chirho"));
        assert!(!html_chirho.contains(&copy_chirho.publication_chirho.queued_heading_chirho));
        assert!(!html_chirho.contains(code_chirho));
        assert!(!html_chirho.contains("engine sentence never shown"));
        assert!(!html_chirho.contains("<form"));
    }
    // the applier's new token reaches the status view as a sentence too
    assert_eq!(
        copy_chirho
            .publication_chirho
            .refusal_sentence_chirho("template-not-deployable-chirho"),
        copy_chirho
            .publication_chirho
            .refusal_template_not_deployable_chirho
    );
}

#[test]
fn the_editor_points_at_publishing_only_in_the_plain_saved_state_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let bound_chirho = || {
        WorkspaceEditorBindingV1Chirho::route_contract_chirho()
            .with_publication_href_chirho(WORKSPACE_PUBLICATION_HREF_CHIRHO)
    };
    // the editor cannot see what is live, so its pointer is neutral: it never offers to publish
    let pointer_chirho = copy_chirho
        .publication_chirho
        .editor_publishing_link_chirho
        .clone();
    let old_offer_chirho = copy_chirho.publication_chirho.publish_button_chirho(3);

    let held_chirho = WorkspacePageV1Chirho::editable_chirho(
        document_chirho(),
        3,
        WorkspaceEditorBindingV1Chirho::route_contract_chirho(),
        &[],
        None,
    );
    let html_chirho = render_workspace_chirho(&held_chirho).unwrap();
    assert!(!html_chirho.contains(WORKSPACE_PUBLICATION_HREF_CHIRHO));
    assert!(!html_chirho.contains(&pointer_chirho));

    let saved_chirho =
        WorkspacePageV1Chirho::editable_chirho(document_chirho(), 3, bound_chirho(), &[], None);
    assert!(saved_chirho.is_saved_state_chirho());
    let html_chirho = render_workspace_chirho(&saved_chirho).unwrap();
    assert!(html_chirho.contains(&format!("href=\"{WORKSPACE_PUBLICATION_HREF_CHIRHO}\"")));
    assert!(html_chirho.contains(&pointer_chirho));
    assert!(!html_chirho.contains(&old_offer_chirho));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.editor_pointer_chirho(3)));
    // a pointer, never a form: the only form on the page is the save form
    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert!(!html_chirho.contains("name=\"kind_chirho\""));
    assert!(!html_chirho.contains("base_publication_chirho"));

    let unsaved_chirho =
        WorkspacePageV1Chirho::editable_chirho(document_chirho(), 0, bound_chirho(), &[], None);
    let html_chirho = render_workspace_chirho(&unsaved_chirho).unwrap();
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.form_heading_chirho));

    // rejected and import-review renders echo unsaved words beside version 3: no pointer there
    let mut rejected_document_chirho = document_chirho();
    rejected_document_chirho.business_name_chirho = String::new();
    let validation_error_chirho = rejected_document_chirho
        .validate_chirho()
        .expect_err("an empty business name is refused");
    let rejected_chirho = WorkspacePageV1Chirho::rejected_chirho(
        rejected_document_chirho.clone(),
        3,
        bound_chirho(),
        &validation_error_chirho,
    );
    assert!(!rejected_chirho.is_saved_state_chirho());
    let html_chirho = render_workspace_chirho(&rejected_chirho).unwrap();
    assert!(html_chirho.contains(&copy_chirho.editor_chirho.status_rejected_chirho));
    assert!(!html_chirho.contains(&pointer_chirho));
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.form_heading_chirho));

    let imported_chirho = WorkspacePageV1Chirho::imported_chirho(document_chirho(), 3);
    let html_chirho = render_workspace_chirho(&imported_chirho).unwrap();
    assert!(html_chirho.contains(&copy_chirho.editor_chirho.status_imported_chirho));
    assert!(!html_chirho.contains(&pointer_chirho));

    // the Publishing nav link appears only with the binding
    let nav_label_chirho = format!(">{}<", copy_chirho.chrome_chirho.publish_label_chirho);
    assert!(render_workspace_chirho(&saved_chirho)
        .unwrap()
        .contains(&nav_label_chirho));
    assert!(!render_workspace_chirho(&held_chirho)
        .unwrap()
        .contains(&nav_label_chirho));
}

/// The door's ownership-aware refusal (CAIRN #24153): the name is another account's, no intent is
/// queued, and the door echoes only the caller's own validated name and her verified head.
fn taken_refusal_chirho(slug_chirho: Option<&str>, head_version_chirho: Option<u64>) -> Vec<u8> {
    let mut body_chirho = json!({
        "schema_chirho": PUBLICATION_INTENT_REFUSAL_SCHEMA_CHIRHO,
        "refusal_code_chirho": "slug-taken-chirho",
        "message_chirho": "engine sentence never shown",
        // an engine that still leaked the other owner's base must not reach her page through us
        "live_base_chirho": "f".repeat(64),
    });
    if let Some(slug_chirho) = slug_chirho {
        body_chirho["slug_chirho"] = json!(slug_chirho);
    }
    if let Some(head_version_chirho) = head_version_chirho {
        body_chirho["head_version_chirho"] = json!(head_version_chirho);
    }
    serde_json::to_vec(&body_chirho).unwrap()
}

#[test]
fn a_name_that_is_another_accounts_comes_back_in_the_address_form_with_nothing_of_theirs_chirho() {
    let html_chirho = String::from_utf8(render_publication_intent_chirho(&taken_refusal_chirho(
        Some("good-earth-bakery"),
        Some(5),
    )))
    .unwrap();
    let field_chirho = support_chirho::opening_tag_chirho(&html_chirho, "publish-slug-chirho");
    assert!(
        field_chirho.contains(" value=\"good-earth-bakery\""),
        "{field_chirho}"
    );
    assert!(
        field_chirho.contains(" aria-invalid=\"true\""),
        "{field_chirho}"
    );
    assert_eq!(
        support_chirho::attribute_tokens_chirho(field_chirho, "aria-describedby"),
        vec!["publish-slug-help-chirho", "publish-slug-error-chirho"]
    );
    assert_eq!(
        html_chirho.matches("href=\"#publish-slug-chirho\"").count(),
        1
    );
    // it publishes HER verified saved version under a NEW name: her head, the dark base
    assert!(html_chirho.contains("name=\"kind_chirho\" value=\"publish-chirho\""));
    assert!(html_chirho.contains("name=\"version_chirho\" value=\"5\""));
    assert!(html_chirho.contains(&format!(
        "name=\"base_publication_chirho\" value=\"{DARK_BASE_CHIRHO}\""
    )));
    assert_eq!(html_chirho.matches("<form").count(), 1);
    // nothing of the other owner's, no token, no engine sentence
    assert!(!html_chirho.contains(&"f".repeat(64)));
    assert!(!html_chirho.contains("/app-chirho/good-earth-bakery/"));
    assert!(!html_chirho.contains("slug-taken-chirho"));
    assert!(!html_chirho.contains("engine sentence never shown"));
    support_chirho::assert_page_wiring_chirho("taken name at the door", &html_chirho);
}

#[test]
fn the_address_form_is_offered_only_when_every_part_of_it_is_hers_and_verified_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    for (case_chirho, body_chirho) in [
        // no verified head from the door: a version is never presented as current on a guess
        (
            "no head",
            taken_refusal_chirho(Some("good-earth-bakery"), None),
        ),
        (
            "head zero",
            taken_refusal_chirho(Some("good-earth-bakery"), Some(0)),
        ),
        // no name echoed: nothing to hand back
        ("no name", taken_refusal_chirho(None, Some(5))),
        // an echo that fails OUR slug rule is never trusted into a form value
        (
            "markup",
            taken_refusal_chirho(Some("\"><script>alert(1)</script>"), Some(5)),
        ),
        (
            "uppercase",
            taken_refusal_chirho(Some("Good-Earth"), Some(5)),
        ),
    ] {
        let html_chirho =
            String::from_utf8(render_publication_intent_chirho(&body_chirho)).unwrap();
        assert!(!html_chirho.contains("<form"), "{case_chirho}");
        assert!(!html_chirho.contains("<script>"), "{case_chirho}");
        assert!(
            !html_chirho.contains("href=\"#publish-slug-chirho\""),
            "{case_chirho}"
        );
        // she still gets her words for it and the way back to the Publishing page
        assert!(
            html_chirho.contains(&copy_chirho.publication_chirho.refusal_slug_taken_chirho),
            "{case_chirho}"
        );
        assert!(
            html_chirho.contains("publication-status-chirho"),
            "{case_chirho}"
        );
        support_chirho::assert_page_wiring_chirho(case_chirho, &html_chirho);
    }
    // a different refusal never grows a form, even if an engine echoed a name with it
    let mut other_chirho: Value =
        serde_json::from_slice(&taken_refusal_chirho(Some("good-earth-bakery"), Some(5))).unwrap();
    other_chirho["refusal_code_chirho"] = json!("base-moved-chirho");
    let html_chirho = String::from_utf8(render_publication_intent_chirho(
        &serde_json::to_vec(&other_chirho).unwrap(),
    ))
    .unwrap();
    assert!(!html_chirho.contains("<form"));
    assert!(!html_chirho.contains("good-earth-bakery"));
}

/// The receiver is a second fence for the door's no-leak rule: on a taken-name refusal the other
/// owner's live base is dropped at DECODE, not merely left unrendered; on a base-moved refusal the
/// base is hers and stays. (CAIRN claude2 #24595, 2026-09-22.)
#[test]
fn a_taken_name_refusal_drops_the_other_owners_base_at_decode_chirho() {
    let taken_chirho = decode_publication_intent_answer_chirho(&taken_refusal_chirho(
        Some("good-earth-bakery"),
        Some(5),
    ))
    .unwrap();
    let PublicationIntentAnswerChirho::RefusedChirho(refusal_chirho) = taken_chirho else {
        panic!("a refusal body decodes as a refusal");
    };
    assert_eq!(refusal_chirho.live_base_chirho, None);
    assert_eq!(refusal_chirho.head_version_chirho, Some(5));
    assert_eq!(
        refusal_chirho.slug_chirho.as_deref(),
        Some("good-earth-bakery")
    );

    let moved_chirho = serde_json::to_vec(&json!({
        "schema_chirho": PUBLICATION_INTENT_REFUSAL_SCHEMA_CHIRHO,
        "refusal_code_chirho": "base-moved-chirho",
        "live_base_chirho": "a".repeat(64),
        "head_version_chirho": 2
    }))
    .unwrap();
    let PublicationIntentAnswerChirho::RefusedChirho(hers_chirho) =
        decode_publication_intent_answer_chirho(&moved_chirho).unwrap()
    else {
        panic!("a refusal body decodes as a refusal");
    };
    assert_eq!(
        hers_chirho.live_base_chirho.as_deref(),
        Some("a".repeat(64).as_str())
    );
}
