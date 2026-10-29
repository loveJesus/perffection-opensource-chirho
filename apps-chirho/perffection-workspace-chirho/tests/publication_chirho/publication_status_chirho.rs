// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The Publishing page turns her intents and the platform's applied records into plain words,
//! resolves the live version through the applied intent, renders every refusal token as a
//! customer sentence, and — once the publish route is bound — hosts every approval form, each
//! carrying the address's approval base from the same read; nothing is offered for the live
//! record or while a version is on its way.

use perffection_ui_chirho::{UiLocaleChirho, WorkspaceCopyChirho};
use perffection_workspace_chirho::implementations_chirho::{
    build_publication_status_read_chirho::build_publication_status_read_chirho,
    render_publication_status_chirho::render_publication_status_chirho,
};
use perffection_workspace_chirho::page_render_chirho::render_publication_status_chirho as render_status_model_chirho;
use perffection_workspace_chirho::publication_intent_chirho::DARK_BASE_CHIRHO;
use perffection_workspace_chirho::publication_status_chirho::{
    decode_publication_status_chirho, PublicationOutcomeChirho, PublicationStatusV1Chirho,
    PUBLICATION_STATUS_REQUEST_SCHEMA_CHIRHO, PUBLICATION_STATUS_RESPONSE_SCHEMA_CHIRHO,
};
use perffection_workspace_chirho::workspace_page_chirho::{
    WORKSPACE_PREVIEW_HREF_CHIRHO, WORKSPACE_PUBLISH_ACTION_CHIRHO,
};
use serde_json::{json, Value};

#[path = "../support_chirho/mod.rs"]
mod support_chirho;

fn intent_chirho(
    seq_chirho: u64,
    digest_chirho: &str,
    kind_chirho: &str,
    version_chirho: u64,
) -> Value {
    json!({
        "seq_chirho": seq_chirho,
        "intent_digest_chirho": digest_chirho,
        "kind_chirho": kind_chirho,
        "params_chirho": format!("params-{version_chirho}"),
        "viewer_state_chirho": {"key_chirho": "page-document-chirho", "version_chirho": version_chirho, "receipt_digest_chirho": "r"},
        "at_ms_chirho": 1_000 + seq_chirho
    })
}

fn response_chirho(slugs_chirho: Vec<Value>) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "schema_chirho": PUBLICATION_STATUS_RESPONSE_SCHEMA_CHIRHO,
        "slugs_chirho": slugs_chirho,
    }))
    .unwrap()
}

fn response_with_head_chirho(slugs_chirho: Vec<Value>, head_version_chirho: u64) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "schema_chirho": PUBLICATION_STATUS_RESPONSE_SCHEMA_CHIRHO,
        "slugs_chirho": slugs_chirho,
        "page_head_chirho": {"key_chirho": "page-document-chirho", "version_chirho": head_version_chirho, "receipt_digest_chirho": "r".repeat(64)},
    }))
    .unwrap()
}

fn live_base_chirho() -> String {
    "c".repeat(64)
}

/// A site live at version 3 (applied d1), a refused republish (d2) and an older applied
/// record (d0, version 1) she may roll back to.
fn live_site_chirho(with_base_chirho: bool) -> Value {
    let mut site_chirho = json!({
        "slug_chirho": "good-earth-bakery",
        "address_chirho": "/app-chirho/good-earth-bakery/",
        "intents_chirho": [
            intent_chirho(1, "d0", "publish-chirho", 1),
            intent_chirho(2, "d1", "republish-chirho", 3),
            intent_chirho(3, "d2", "republish-chirho", 5),
        ],
        "applied_chirho": [
            {"intent_digest_chirho": "d0", "outcome_chirho": "applied-chirho", "params_chirho": "params-1", "at_ms_chirho": 2_000},
            {"intent_digest_chirho": "d1", "outcome_chirho": "applied-chirho", "params_chirho": "params-3", "at_ms_chirho": 2_001},
            {"intent_digest_chirho": "d2", "outcome_chirho": "refused-chirho", "refusal_code_chirho": "intent-invalid-chirho", "at_ms_chirho": 2_002},
        ],
        "live_chirho": {"params_chirho": "params-3", "applied_intent_digest_chirho": "d1", "params_record_seq_chirho": 7, "at_ms_chirho": 2_001}
    });
    if with_base_chirho {
        site_chirho["approval_base_chirho"] = json!(live_base_chirho());
    }
    site_chirho
}

fn bound_html_chirho(bytes_chirho: &[u8]) -> String {
    let snapshot_chirho = decode_publication_status_chirho(bytes_chirho).unwrap();
    let model_chirho = PublicationStatusV1Chirho::available_chirho(
        &snapshot_chirho,
        UiLocaleChirho::EnglishChirho,
    )
    .with_publish_action_chirho(WORKSPACE_PUBLISH_ACTION_CHIRHO);
    render_status_model_chirho(&model_chirho).unwrap()
}

fn hidden_chirho(name_chirho: &str, value_chirho: &str) -> String {
    format!("name=\"{name_chirho}\" value=\"{value_chirho}\"")
}

#[test]
fn the_read_request_is_fixed_and_names_nothing_of_hers_chirho() {
    let request_chirho: Value =
        serde_json::from_slice(&build_publication_status_read_chirho(b"ignored")).unwrap();
    assert_eq!(
        request_chirho,
        json!({"schema_chirho": PUBLICATION_STATUS_REQUEST_SCHEMA_CHIRHO, "page_key_chirho": "page-document-chirho"})
    );
}

#[test]
fn nothing_published_renders_an_honest_page_with_no_controls_chirho() {
    // the deployed renderer binds the publish door, so the operator fallback sentence is gone;
    // with no stated head there is still nothing to approve, so no form and no button
    let html_chirho =
        String::from_utf8(render_publication_status_chirho(&response_chirho(vec![]))).unwrap();
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.status_not_published_chirho));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.not_published_heading_chirho));
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.operator_publish_body_chirho));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.retention_note_chirho));
    assert!(!html_chirho.contains("<form"));
    assert!(!html_chirho.contains("<button"));
    assert!(html_chirho.contains("publication-status-chirho"));

    // an unbound model (the held state) still names the operator road
    let snapshot_chirho = decode_publication_status_chirho(&response_chirho(vec![])).unwrap();
    let held_chirho = PublicationStatusV1Chirho::available_chirho(
        &snapshot_chirho,
        UiLocaleChirho::EnglishChirho,
    );
    let held_html_chirho = render_status_model_chirho(&held_chirho).unwrap();
    assert!(held_html_chirho.contains(&copy_chirho.publication_chirho.operator_publish_body_chirho));
    assert!(!held_html_chirho.contains("<form"));
}

#[test]
fn live_pending_and_refused_events_become_plain_words_chirho() {
    let bytes_chirho = response_chirho(vec![json!({
        "slug_chirho": "good-earth-bakery",
        "address_chirho": "/app-chirho/good-earth-bakery/",
        "intents_chirho": [
            intent_chirho(1, "d1", "publish-chirho", 3),
            intent_chirho(2, "d2", "republish-chirho", 5),
            intent_chirho(3, "d3", "rollback-chirho", 3),
        ],
        "applied_chirho": [
            {"intent_digest_chirho": "d1", "kind_chirho": "publish-chirho", "outcome_chirho": "applied-chirho", "params_chirho": "params-3", "at_ms_chirho": 2_000},
            {"intent_digest_chirho": "d2", "kind_chirho": "republish-chirho", "outcome_chirho": "refused-chirho", "refusal_code_chirho": "slug-taken-chirho", "at_ms_chirho": 2_001},
        ],
        "live_chirho": {"params_chirho": "params-3", "applied_intent_digest_chirho": "d1", "params_record_seq_chirho": 7, "at_ms_chirho": 2_000}
    })]);
    let snapshot_chirho = decode_publication_status_chirho(&bytes_chirho).unwrap();
    let site_chirho = &snapshot_chirho.sites_chirho[0];
    assert_eq!(site_chirho.live_version_chirho, Some(3));
    assert!(site_chirho.is_live_chirho);
    assert_eq!(site_chirho.pending_chirho.len(), 1);
    assert_eq!(site_chirho.pending_chirho[0].kind_chirho, "rollback-chirho");
    assert_eq!(site_chirho.history_chirho.len(), 2);
    assert_eq!(
        site_chirho.history_chirho[0].outcome_chirho,
        PublicationOutcomeChirho::RefusedChirho("slug-taken-chirho".to_string())
    );
    assert_eq!(
        site_chirho.history_chirho[1].outcome_chirho,
        PublicationOutcomeChirho::AppliedChirho
    );
    assert_eq!(snapshot_chirho.head_version_chirho, None);

    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let html_chirho = String::from_utf8(render_publication_status_chirho(&bytes_chirho)).unwrap();
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.status_pending_chirho));
    assert!(html_chirho.contains("href=\"/app-chirho/good-earth-bakery/\""));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.refusal_slug_taken_chirho));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.kind_rollback_chirho));
    assert!(!html_chirho.contains("slug-taken-chirho"));
    // a version is on its way: the deployed renderer offers no approval form
    assert!(!html_chirho.contains("<form"));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.pending_wait_chirho));

    let model_chirho = PublicationStatusV1Chirho::available_chirho(
        &snapshot_chirho,
        UiLocaleChirho::EnglishChirho,
    );
    assert!(model_chirho.is_available_chirho());
}

#[test]
fn a_live_site_with_no_pending_work_reports_its_version_chirho() {
    let bytes_chirho = response_chirho(vec![json!({
        "slug_chirho": "good-earth-bakery",
        "address_chirho": "/app-chirho/good-earth-bakery/",
        "intents_chirho": [intent_chirho(1, "d1", "publish-chirho", 4)],
        "applied_chirho": [{"intent_digest_chirho": "d1", "outcome_chirho": "applied-chirho", "at_ms_chirho": 2_000}],
        "live_chirho": {"params_chirho": "params-4", "applied_intent_digest_chirho": "d1", "at_ms_chirho": 2_000}
    })]);
    let snapshot_chirho = decode_publication_status_chirho(&bytes_chirho).unwrap();
    let model_chirho = PublicationStatusV1Chirho::available_chirho(
        &snapshot_chirho,
        UiLocaleChirho::EnglishChirho,
    );
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    assert_eq!(
        model_chirho.status_chirho(),
        copy_chirho.publication_chirho.live_status_chirho(4)
    );
}

#[test]
fn a_live_rollback_names_the_version_it_restored_chirho() {
    let rollback_chirho = json!({
        "seq_chirho": 3,
        "intent_digest_chirho": "d3",
        "kind_chirho": "rollback-chirho",
        "params_chirho": "params-1",
        "at_ms_chirho": 1_003
    });
    let bytes_chirho = response_chirho(vec![json!({
        "slug_chirho": "good-earth-bakery",
        "address_chirho": "/app-chirho/good-earth-bakery/",
        "intents_chirho": [
            intent_chirho(1, "d1", "publish-chirho", 1),
            intent_chirho(2, "d2", "republish-chirho", 2),
            rollback_chirho,
        ],
        "applied_chirho": [
            {"intent_digest_chirho": "d1", "outcome_chirho": "applied-chirho", "params_chirho": "params-1", "at_ms_chirho": 2_000},
            {"intent_digest_chirho": "d2", "outcome_chirho": "applied-chirho", "params_chirho": "params-2", "at_ms_chirho": 2_001},
            {"intent_digest_chirho": "d3", "outcome_chirho": "applied-chirho", "params_chirho": "params-1", "at_ms_chirho": 2_002},
        ],
        "live_chirho": {"params_chirho": "params-1", "applied_intent_digest_chirho": "d3", "at_ms_chirho": 2_002}
    })]);
    let snapshot_chirho = decode_publication_status_chirho(&bytes_chirho).unwrap();
    assert_eq!(snapshot_chirho.sites_chirho[0].live_version_chirho, Some(1));
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let model_chirho = PublicationStatusV1Chirho::available_chirho(
        &snapshot_chirho,
        UiLocaleChirho::EnglishChirho,
    );
    assert_eq!(
        model_chirho.status_chirho(),
        copy_chirho.publication_chirho.live_status_chirho(1)
    );
    // the rollback target now is version 2's record, never the live one
    let html_chirho = bound_html_chirho(&bytes_chirho);
    assert!(html_chirho.contains(&hidden_chirho("params_hash_chirho", "params-2")));
    assert!(!html_chirho.contains(&hidden_chirho("params_hash_chirho", "params-1")));
    // exactly one record is live now; the earlier applied records are history
    let live_chirho: Vec<bool> = snapshot_chirho.sites_chirho[0]
        .history_chirho
        .iter()
        .map(|event_chirho| event_chirho.is_live_chirho)
        .collect();
    assert_eq!(live_chirho, vec![true, false, false]);
    assert_eq!(
        html_chirho
            .matches(&copy_chirho.publication_chirho.outcome_live_chirho)
            .count(),
        1
    );
    assert_eq!(
        html_chirho
            .matches(&copy_chirho.publication_chirho.outcome_applied_chirho)
            .count(),
        2
    );
    assert_ne!(
        copy_chirho.publication_chirho.outcome_live_chirho,
        copy_chirho.publication_chirho.outcome_applied_chirho
    );
}

#[test]
fn the_approval_base_is_the_roads_word_or_the_live_intent_or_dark_chirho() {
    let stated_chirho =
        decode_publication_status_chirho(&response_chirho(vec![live_site_chirho(true)])).unwrap();
    assert_eq!(
        stated_chirho.sites_chirho[0].approval_base_chirho,
        live_base_chirho()
    );
    let implied_chirho =
        decode_publication_status_chirho(&response_chirho(vec![live_site_chirho(false)])).unwrap();
    assert_eq!(implied_chirho.sites_chirho[0].approval_base_chirho, "d1");
    let dark_chirho = decode_publication_status_chirho(&response_chirho(vec![json!({
        "slug_chirho": "good-earth-bakery",
        "address_chirho": "/app-chirho/good-earth-bakery/",
        "intents_chirho": [intent_chirho(1, "d1", "publish-chirho", 2)],
    })]))
    .unwrap();
    assert_eq!(
        dark_chirho.sites_chirho[0].approval_base_chirho,
        DARK_BASE_CHIRHO
    );
    assert_eq!(dark_chirho.sites_chirho[0].live_params_chirho, None);
    assert_eq!(
        stated_chirho.sites_chirho[0].live_params_chirho.as_deref(),
        Some("params-3")
    );
}

#[test]
fn bound_forms_carry_the_base_and_skip_the_live_record_and_pending_addresses_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let base_chirho = live_base_chirho();
    let html_chirho =
        bound_html_chirho(&response_with_head_chirho(vec![live_site_chirho(true)], 6));

    // the saved version is named and reviewable before any approval
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.saved_version_chirho(6)));
    assert!(html_chirho.contains(&format!(
        "href=\"{WORKSPACE_PREVIEW_HREF_CHIRHO}\">{}",
        copy_chirho.publication_chirho.review_link_label_chirho
    )));
    // republish at the live address: version 6, the live base, no document
    assert!(html_chirho.contains(&hidden_chirho("kind_chirho", "republish-chirho")));
    assert!(html_chirho.contains(&hidden_chirho("version_chirho", "6")));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.republish_button_chirho(6)));
    assert!(html_chirho.contains(
        &copy_chirho
            .publication_chirho
            .consent_chirho(6, "/app-chirho/good-earth-bakery/")
    ));
    // rollback only to the older applied record, never to the live one or a refusal
    assert!(html_chirho.contains(&hidden_chirho("params_hash_chirho", "params-1")));
    assert!(!html_chirho.contains(&hidden_chirho("params_hash_chirho", "params-3")));
    assert!(!html_chirho.contains(&hidden_chirho("params_hash_chirho", "params-5")));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.rollback_button_chirho(1)));
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.rollback_button_chirho(3)));
    // every form posts to the publish door and carries the same base
    assert_eq!(html_chirho.matches("<form").count(), 2);
    assert_eq!(
        html_chirho
            .matches(&format!("action=\"{WORKSPACE_PUBLISH_ACTION_CHIRHO}\""))
            .count(),
        2
    );
    assert_eq!(
        html_chirho
            .matches(&hidden_chirho("base_publication_chirho", &base_chirho))
            .count(),
        2
    );
    // she already has a live address: no new-address form
    assert!(!html_chirho.contains(&hidden_chirho("kind_chirho", "publish-chirho")));
    assert!(!html_chirho.contains(DARK_BASE_CHIRHO));
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.operator_publish_body_chirho));

    // a version on its way: no republish, no rollback, a plain wait sentence
    let mut pending_site_chirho = live_site_chirho(true);
    pending_site_chirho["intents_chirho"]
        .as_array_mut()
        .unwrap()
        .push(intent_chirho(4, "d3", "republish-chirho", 6));
    let html_chirho = bound_html_chirho(&response_with_head_chirho(vec![pending_site_chirho], 6));
    assert!(!html_chirho.contains("<form"));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.pending_wait_chirho));
}

#[test]
fn a_new_address_form_appears_only_with_a_saved_version_and_nothing_live_or_pending_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let html_chirho = bound_html_chirho(&response_with_head_chirho(vec![], 2));
    // the first-publish form carries the address field and its help text
    assert_eq!(
        support_chirho::assert_page_wiring_chirho("publishing", &html_chirho),
        1
    );
    // the first-publish form carries the engine's caller-key placeholder (the browser carrier)
    assert!(support_chirho::assert_caller_key_forms_chirho("publishing", &html_chirho) >= 1);
    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert!(html_chirho.contains(&hidden_chirho("kind_chirho", "publish-chirho")));
    assert!(html_chirho.contains(&hidden_chirho("version_chirho", "2")));
    assert!(html_chirho.contains(&hidden_chirho("base_publication_chirho", DARK_BASE_CHIRHO)));
    assert!(html_chirho.contains("name=\"slug_chirho\""));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.publish_button_chirho(2)));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.new_address_heading_chirho));
    assert!(!html_chirho.contains(&copy_chirho.publication_chirho.operator_publish_body_chirho));

    // a refused-only address (nothing live) still lets her choose another
    let refused_site_chirho = json!({
        "slug_chirho": "taken-name",
        "address_chirho": "/app-chirho/taken-name/",
        "intents_chirho": [intent_chirho(1, "d1", "publish-chirho", 2)],
        "applied_chirho": [{"intent_digest_chirho": "d1", "outcome_chirho": "refused-chirho", "refusal_code_chirho": "slug-taken-chirho", "at_ms_chirho": 2_000}],
    });
    let html_chirho = bound_html_chirho(&response_with_head_chirho(vec![refused_site_chirho], 2));
    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert!(html_chirho.contains(&hidden_chirho("kind_chirho", "publish-chirho")));
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.refusal_slug_taken_chirho));

    // nothing saved yet (the road omits the head, or states version 0): the honest sentence,
    // no form, no review link
    for bytes_chirho in [
        response_chirho(vec![]),
        response_with_head_chirho(vec![], 0),
    ] {
        let html_chirho = bound_html_chirho(&bytes_chirho);
        assert!(!html_chirho.contains("<form"));
        assert!(html_chirho.contains(&copy_chirho.publication_chirho.nothing_saved_notice_chirho));
        // the saved-version review link is not offered (the nav's Preview link is unrelated)
        assert!(!html_chirho.contains(&copy_chirho.publication_chirho.review_link_label_chirho));
    }

    // a head stated for some other key is not hers to publish: rollback alone, never a form
    // that names a version
    let other_key_chirho = serde_json::to_vec(&json!({
        "schema_chirho": PUBLICATION_STATUS_RESPONSE_SCHEMA_CHIRHO,
        "slugs_chirho": [live_site_chirho(true)],
        "page_head_chirho": {"key_chirho": "something-else-chirho", "version_chirho": 6},
    }))
    .unwrap();
    for bytes_chirho in [
        response_chirho(vec![live_site_chirho(true)]),
        other_key_chirho,
    ] {
        let html_chirho = bound_html_chirho(&bytes_chirho);
        assert_eq!(html_chirho.matches("<form").count(), 1);
        assert!(html_chirho.contains(&hidden_chirho("kind_chirho", "rollback-chirho")));
        assert!(!html_chirho.contains("name=\"version_chirho\" value=\"6\""));
        assert!(!html_chirho.contains(&hidden_chirho("kind_chirho", "republish-chirho")));
    }
}

#[test]
fn unknown_tokens_and_unverifiable_reads_fail_soft_and_closed_respectively_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    assert_eq!(
        copy_chirho
            .publication_chirho
            .refusal_sentence_chirho("some-new-token-chirho"),
        copy_chirho.publication_chirho.refusal_unknown_chirho
    );

    assert!(decode_publication_status_chirho(b"{\"schema_chirho\":\"x\"}").is_err());
    assert!(decode_publication_status_chirho(b"not json").is_err());
    let html_chirho = String::from_utf8(render_publication_status_chirho(
        b"{\"schema_chirho\":\"x\"}",
    ))
    .unwrap();
    assert!(html_chirho.contains(&copy_chirho.publication_chirho.unavailable_heading_chirho));
    assert!(!html_chirho.contains("<form"));

    let rtl_chirho = WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::PseudolocaleRtlChirho);
    assert!(rtl_chirho
        .publication_chirho
        .status_live_template_chirho
        .contains("{version_chirho}"));
    assert!(rtl_chirho
        .publication_chirho
        .republish_button_template_chirho
        .contains("{version_chirho}"));
}

/// A republish is offered only when it is not known to be redundant: the saved version and the
/// live version both stated and equal give the sentence instead of the form; an unstated live
/// version keeps the form (nothing is guessed); rollback is untouched either way.
#[test]
fn a_saved_version_known_to_be_live_is_not_offered_again_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let mut live_chirho = live_site_chirho(true);
    // the live version is named by the applied intent's version binding, as the view resolves it
    let applied_chirho = live_chirho["live_chirho"]["applied_intent_digest_chirho"]
        .as_str()
        .unwrap()
        .to_string();
    let applied_index_chirho = live_chirho["intents_chirho"]
        .as_array()
        .unwrap()
        .iter()
        .position(|intent_chirho| intent_chirho["intent_digest_chirho"] == applied_chirho)
        .unwrap();
    let live_version_chirho = live_chirho["intents_chirho"][applied_index_chirho]
        ["viewer_state_chirho"]["version_chirho"]
        .as_u64()
        .unwrap();
    let html_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![live_chirho.clone()],
        live_version_chirho,
    ));
    assert!(!html_chirho.contains(&hidden_chirho("kind_chirho", "republish-chirho")));
    assert!(html_chirho.contains(
        &copy_chirho
            .publication_chirho
            .live_is_saved_chirho(live_version_chirho)
    ));
    assert!(html_chirho.contains(&hidden_chirho("kind_chirho", "rollback-chirho")));
    // a newer saved version: the form returns, the sentence goes
    let html_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![live_chirho.clone()],
        live_version_chirho + 1,
    ));
    assert!(html_chirho.contains(&hidden_chirho("kind_chirho", "republish-chirho")));
    assert!(!html_chirho.contains("is what is live at this address"));
    // no version binding names the live record (the applied intent lost its binding and no other
    // intent published those params under one): never guessed equal, the form stays
    live_chirho["intents_chirho"][applied_index_chirho]["viewer_state_chirho"] = Value::Null;
    let html_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![live_chirho],
        live_version_chirho,
    ));
    assert!(html_chirho.contains(&hidden_chirho("kind_chirho", "republish-chirho")));
    assert!(!html_chirho.contains("is what is live at this address"));
}

/// One address she tried: a first publish of `version_chirho`, decided at `at_ms_chirho` with
/// the platform's refusal code.
fn refused_address_chirho(
    slug_chirho: &str,
    seq_chirho: u64,
    refusal_code_chirho: &str,
    at_ms_chirho: u64,
) -> Value {
    let digest_chirho = format!("refused-{slug_chirho}");
    json!({
        "slug_chirho": slug_chirho,
        "address_chirho": format!("/app-chirho/{slug_chirho}/"),
        "intents_chirho": [intent_chirho(seq_chirho, &digest_chirho, "publish-chirho", 2)],
        "applied_chirho": [{"intent_digest_chirho": digest_chirho, "outcome_chirho": "refused-chirho", "refusal_code_chirho": refusal_code_chirho, "at_ms_chirho": at_ms_chirho}],
    })
}

#[test]
fn a_refused_address_meets_her_at_the_address_field_with_the_name_she_typed_chirho() {
    let html_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![refused_address_chirho(
            "good-earth-bakery",
            1,
            "slug-taken-chirho",
            2_000,
        )],
        2,
    ));
    let field_chirho = support_chirho::opening_tag_chirho(&html_chirho, "publish-slug-chirho");
    assert!(
        field_chirho.contains(" value=\"good-earth-bakery\""),
        "{field_chirho}"
    );
    assert!(
        field_chirho.contains(" aria-invalid=\"true\""),
        "{field_chirho}"
    );
    let described_chirho =
        support_chirho::attribute_tokens_chirho(field_chirho, "aria-describedby");
    assert_eq!(
        described_chirho,
        vec!["publish-slug-help-chirho", "publish-slug-error-chirho"]
    );
    // said once at the top with a link to the field, once beside it; the platform's token never
    assert_eq!(
        html_chirho.matches("href=\"#publish-slug-chirho\"").count(),
        1
    );
    assert_eq!(
        html_chirho
            .matches(" id=\"publish-slug-error-chirho\"")
            .count(),
        1
    );
    assert!(!html_chirho.contains("slug-taken-chirho"));
    assert_eq!(
        support_chirho::assert_page_wiring_chirho("refused address", &html_chirho),
        1
    );
    // the form still publishes her saved version from the dark base: only the name is hers to fix
    assert!(html_chirho.contains(&hidden_chirho("version_chirho", "2")));
    assert!(html_chirho.contains(&hidden_chirho("base_publication_chirho", DARK_BASE_CHIRHO)));
}

#[test]
fn only_the_last_thing_that_happened_and_only_a_refusal_about_the_address_reaches_the_field_chirho()
{
    let field_of_chirho = |slugs_chirho: Vec<Value>| {
        let html_chirho = bound_html_chirho(&response_with_head_chirho(slugs_chirho, 2));
        (
            support_chirho::opening_tag_chirho(&html_chirho, "publish-slug-chirho").to_string(),
            html_chirho.matches("href=\"#publish-slug-chirho\"").count(),
        )
    };
    // two names tried: the later refusal is the one she is looking at
    let (field_chirho, links_chirho) = field_of_chirho(vec![
        refused_address_chirho("first-try", 1, "slug-taken-chirho", 2_000),
        refused_address_chirho("perffection", 2, "slug-reserved-chirho", 3_000),
    ]);
    assert!(
        field_chirho.contains(" value=\"perffection\""),
        "{field_chirho}"
    );
    assert_eq!(links_chirho, 1);
    // the kind of request does not matter: a republish refused because the name is another
    // account's, on an address that never went live, sends her to the same field
    let mut republish_chirho =
        refused_address_chirho("borrowed-name", 1, "not-your-app-chirho", 2_000);
    republish_chirho["intents_chirho"][0]["kind_chirho"] = json!("republish-chirho");
    let (field_chirho, links_chirho) = field_of_chirho(vec![republish_chirho]);
    assert!(
        field_chirho.contains(" value=\"borrowed-name\""),
        "{field_chirho}"
    );
    assert_eq!(links_chirho, 1);
    // a refusal that is not about the name leaves the field alone
    let (field_chirho, links_chirho) = field_of_chirho(vec![refused_address_chirho(
        "good-earth-bakery",
        1,
        "unknown-template-chirho",
        2_000,
    )]);
    assert!(field_chirho.contains(" value=\"\""), "{field_chirho}");
    assert!(!field_chirho.contains("aria-invalid"), "{field_chirho}");
    assert_eq!(links_chirho, 0);
    // an address refusal that is no longer the last thing that happened does not linger
    let (field_chirho, links_chirho) = field_of_chirho(vec![
        refused_address_chirho("perffection", 1, "slug-reserved-chirho", 2_000),
        refused_address_chirho("good-earth-bakery", 2, "unknown-template-chirho", 3_000),
    ]);
    assert!(!field_chirho.contains("aria-invalid"), "{field_chirho}");
    assert_eq!(links_chirho, 0);
}

#[test]
fn no_link_points_at_an_address_field_the_page_does_not_offer_chirho() {
    // something of hers is live: no new-address form, so no summary and no dangling link
    let html_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![
            refused_address_chirho("perffection", 9, "slug-reserved-chirho", 9_000),
            live_site_chirho(true),
        ],
        6,
    ));
    assert!(!html_chirho.contains("publish-slug-chirho"));
    assert!(!html_chirho.contains("publication-address-refused-heading-chirho"));
    // the publish door is not bound: the page offers no form at all
    let snapshot_chirho = decode_publication_status_chirho(&response_with_head_chirho(
        vec![refused_address_chirho(
            "perffection",
            1,
            "slug-reserved-chirho",
            2_000,
        )],
        2,
    ))
    .unwrap();
    let unbound_chirho = render_status_model_chirho(&PublicationStatusV1Chirho::available_chirho(
        &snapshot_chirho,
        UiLocaleChirho::EnglishChirho,
    ))
    .unwrap();
    assert!(!unbound_chirho.contains("href=\"#publish-slug-chirho\""));
    support_chirho::assert_page_wiring_chirho("unbound publishing", &unbound_chirho);
}

#[test]
fn a_name_she_only_asked_for_is_never_shown_as_her_address_chirho() {
    // refused for the name: that address may be someone else's
    let html_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![refused_address_chirho(
            "good-earth-bakery",
            1,
            "slug-taken-chirho",
            2_000,
        )],
        2,
    ));
    assert!(!html_chirho.contains("/app-chirho/good-earth-bakery/"));
    // her record of having asked for it stays: the name and one decided event
    assert!(html_chirho.contains(">good-earth-bakery</h2>"));
    assert_eq!(
        html_chirho.matches("workspace-history-item-chirho").count(),
        1
    );
    // an address she holds keeps its path, linked while it is live
    let live_chirho =
        bound_html_chirho(&response_with_head_chirho(vec![live_site_chirho(true)], 6));
    assert!(live_chirho.contains("<a href=\"/app-chirho/good-earth-bakery/\">"));
    // a request still on its way names the address it asked for
    let pending_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![json!({
            "slug_chirho": "good-earth-bakery",
            "address_chirho": "/app-chirho/good-earth-bakery/",
            "intents_chirho": [intent_chirho(1, "d0", "publish-chirho", 2)],
            "applied_chirho": [],
        })],
        2,
    ));
    assert!(pending_chirho.contains("/app-chirho/good-earth-bakery/"));
}

/// The header line says what the panel below says (claude2 2026-09-24: with version 3 live, the
/// header still said version 3 was "ready to publish"). Pinned as WHICH catalog sentence the
/// header carries in each state, never the prose.
#[test]
fn the_header_line_agrees_with_the_panel_about_the_saved_version_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let publication_chirho = &copy_chirho.publication_chirho;
    let header_chirho = |html_chirho: &str| -> String {
        let start_chirho = html_chirho
            .find("class=\"workspace-saved-version-chirho\">")
            .expect("the saved-version line is always rendered")
            + "class=\"workspace-saved-version-chirho\">".len();
        let rest_chirho = &html_chirho[start_chirho..];
        rest_chirho[..rest_chirho.find('<').unwrap()]
            .trim()
            .to_string()
    };

    // saved version 3, live version 3, nothing pending: "is what is live", not "ready to publish"
    let live_chirho =
        bound_html_chirho(&response_with_head_chirho(vec![live_site_chirho(true)], 3));
    assert_eq!(
        header_chirho(&live_chirho),
        publication_chirho.saved_version_live_chirho(3)
    );
    assert!(!live_chirho.contains(&publication_chirho.saved_version_chirho(3)));

    // saved version 6 while version 3 is live: ready to publish (the republish form's case)
    let newer_chirho =
        bound_html_chirho(&response_with_head_chirho(vec![live_site_chirho(true)], 6));
    assert_eq!(
        header_chirho(&newer_chirho),
        publication_chirho.saved_version_chirho(6)
    );

    // saved version 5 with an approval of version 5 queued: "on its way"
    let pending_chirho = bound_html_chirho(&response_with_head_chirho(
        vec![json!({
            "slug_chirho": "good-earth-bakery",
            "address_chirho": "/app-chirho/good-earth-bakery/",
            "intents_chirho": [
                intent_chirho(1, "d1", "publish-chirho", 3),
                intent_chirho(2, "d2", "republish-chirho", 5),
            ],
            "applied_chirho": [
                {"intent_digest_chirho": "d1", "outcome_chirho": "applied-chirho", "params_chirho": "params-3", "at_ms_chirho": 2_000},
            ],
            "live_chirho": {"params_chirho": "params-3", "applied_intent_digest_chirho": "d1", "params_record_seq_chirho": 7, "at_ms_chirho": 2_000},
            "approval_base_chirho": live_base_chirho(),
        })],
        5,
    ));
    assert_eq!(
        header_chirho(&pending_chirho),
        publication_chirho.saved_version_pending_chirho(5)
    );

    // nothing saved: the notice, as before
    let nothing_chirho = bound_html_chirho(&response_chirho(vec![]));
    assert_eq!(
        header_chirho(&nothing_chirho),
        publication_chirho.nothing_saved_notice_chirho
    );
}
