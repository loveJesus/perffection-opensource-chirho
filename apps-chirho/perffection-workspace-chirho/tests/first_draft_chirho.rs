// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The AI first draft, product half: her brief becomes one bounded request and nothing else; the
//! two branches of the draft route are live exactly one at a time; the helper's completion is
//! trusted with nothing and becomes a reviewable draft at version 0; refusals are sentences.

use perffection_ui_chirho::{UiLocaleChirho, WorkspaceCopyChirho};
use perffection_workspace_chirho::draft_page_chirho::{
    DraftPageKindChirho, DraftPageV1Chirho, WORKSPACE_DRAFT_ACTION_CHIRHO,
    WORKSPACE_DRAFT_HREF_CHIRHO,
};
use perffection_workspace_chirho::first_draft_chirho::{
    build_llm_request_chirho, decode_llm_response_chirho, draft_document_from_completion_chirho,
    parse_draft_brief_chirho, DraftBriefErrorChirho, DraftMappingErrorChirho,
    LLM_REFUSAL_SCHEMA_CHIRHO, LLM_REQUEST_SCHEMA_CHIRHO, LLM_RESPONSE_SCHEMA_CHIRHO,
};
use perffection_workspace_chirho::implementations_chirho::{
    build_draft_request_chirho::build_draft_request_chirho,
    render_draft_brief_chirho::render_draft_brief_chirho,
    render_draft_brief_rejection_chirho::render_draft_brief_rejection_chirho,
    render_draft_review_chirho::render_draft_review_chirho,
};
use perffection_workspace_chirho::page_document_chirho::{
    BUSINESS_NAME_MAX_CHARACTERS_CHIRHO, SUMMARY_MAX_CHARACTERS_CHIRHO,
};
use perffection_workspace_chirho::page_render_chirho::render_draft_chirho;
use perffection_workspace_chirho::viewer_state_contract_chirho::{
    PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO, VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
};
use serde_json::{json, Value};

mod support_chirho;

const SKIPPED_STONE_CHIRHO: &[u8] = cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO;

const GOOD_BRIEF_CHIRHO: &[u8] = b"business_name_chirho=North+Street+Repairs&about_chirho=We+fix+what+breaks+in+homes+and+small+shops.&audience_chirho=Homes+around+Mesa&tone_chirho=plain-chirho&service_1_chirho=Repairs&service_2_chirho=&service_3_chirho=&contact_phone_chirho=555-0124&contact_email_chirho=&contact_details_chirho=Weekdays+8+to+5";

fn response_chirho(completion_chirho: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "schema_chirho": LLM_RESPONSE_SCHEMA_CHIRHO,
        "completion_chirho": completion_chirho,
        "tokens_in_chirho": 300, "tokens_out_chirho": 200, "model_chirho": "stub",
        "evidence_chirho": {"prompt_blob_chirho": "p", "completion_blob_chirho": "c"}
    }))
    .unwrap()
}

fn page_json_chirho() -> String {
    json!({
        "business_name": "North Street Repairs",
        "headline": "Fixed right the first time",
        "summary": "Repairs for homes and small shops around Mesa, weekdays 8 to 5.",
        "services": [{"title": "Repairs", "body": "Doors, locks, fixtures and the small jobs nobody else wants."}],
        "contact_heading": "Tell us what needs attention",
        "contact_phone": "555-0124",
        "contact_email": "",
        "contact_details": "Weekdays 8 to 5"
    })
    .to_string()
}

#[test]
fn the_brief_becomes_one_bounded_request_and_never_a_principal_chirho() {
    let brief_chirho = parse_draft_brief_chirho(GOOD_BRIEF_CHIRHO).unwrap();
    assert_eq!(brief_chirho.tone_chirho, "plain-chirho");
    assert_eq!(brief_chirho.services_chirho, vec!["Repairs".to_string()]);
    let request_chirho = build_llm_request_chirho(&brief_chirho);
    assert_eq!(request_chirho.schema_chirho, LLM_REQUEST_SCHEMA_CHIRHO);
    assert_eq!(request_chirho.response_format_chirho, "json-object");
    assert!(request_chirho
        .system_chirho
        .contains(&BUSINESS_NAME_MAX_CHARACTERS_CHIRHO.to_string()));
    assert!(request_chirho
        .system_chirho
        .contains(&SUMMARY_MAX_CHARACTERS_CHIRHO.to_string()));
    assert!(request_chirho.system_chirho.contains("no HTML"));
    assert!(request_chirho.user_chirho.contains("North Street Repairs"));
    assert!(request_chirho.user_chirho.contains("555-0124"));
    let wire_chirho: Value =
        serde_json::from_slice(&build_draft_request_chirho(GOOD_BRIEF_CHIRHO)).unwrap();
    let text_chirho = wire_chirho.to_string();
    for forbidden_chirho in [
        "principal",
        "app_chirho",
        "caller_chirho",
        "scope_chirho",
        "max_price_per_token",
        "max_tokens_chirho",
    ] {
        assert!(
            !text_chirho.contains(forbidden_chirho),
            "{forbidden_chirho}"
        );
    }
    // the same bytes build the same request: a retry is the same act for the card's binding
    assert_eq!(
        build_draft_request_chirho(GOOD_BRIEF_CHIRHO),
        build_draft_request_chirho(GOOD_BRIEF_CHIRHO)
    );
    // an unknown tone falls to the first one; a foreign field makes the brief unreadable
    let odd_tone_chirho = parse_draft_brief_chirho(
        b"business_name_chirho=A&about_chirho=B&tone_chirho=shouty&contact_phone_chirho=1",
    )
    .unwrap();
    assert_eq!(odd_tone_chirho.tone_chirho, "warm-chirho");
    assert_eq!(
        parse_draft_brief_chirho(
            b"business_name_chirho=A&about_chirho=B&contact_phone_chirho=1&principal_chirho=x"
        ),
        Err(DraftBriefErrorChirho::UnreadableChirho)
    );
}

/// The draft door is unbound on main, so there is no form to hand back: a refusal is a sentence
/// and nothing of hers. The bound door's behavior lives in `draft_brief_form_chirho.rs`.
#[test]
fn exactly_one_branch_is_live_and_an_unbound_door_never_echoes_her_words_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let cases_chirho: Vec<(&[u8], DraftBriefErrorChirho, &str)> = vec![
        (
            b"about_chirho=We+fix&contact_phone_chirho=1",
            DraftBriefErrorChirho::NameMissingChirho,
            &copy_chirho.draft_chirho.notice_name_missing_chirho,
        ),
        (
            b"business_name_chirho=Secret+Name+Never+Shown&contact_phone_chirho=1",
            DraftBriefErrorChirho::AboutMissingChirho,
            &copy_chirho.draft_chirho.notice_about_missing_chirho,
        ),
        (
            b"business_name_chirho=A&about_chirho=B",
            DraftBriefErrorChirho::ContactMissingChirho,
            &copy_chirho.draft_chirho.notice_contact_missing_chirho,
        ),
        (
            b"not=a&form",
            DraftBriefErrorChirho::UnreadableChirho,
            &copy_chirho.draft_chirho.notice_unreadable_chirho,
        ),
    ];
    for (body_chirho, error_chirho, sentence_chirho) in cases_chirho {
        assert_eq!(parse_draft_brief_chirho(body_chirho), Err(error_chirho));
        assert_eq!(
            build_draft_request_chirho(body_chirho),
            SKIPPED_STONE_CHIRHO
        );
        let html_chirho =
            String::from_utf8(render_draft_brief_rejection_chirho(body_chirho)).unwrap();
        assert!(html_chirho.contains(sentence_chirho));
        assert!(html_chirho.contains(&copy_chirho.draft_chirho.rejected_heading_chirho));
        assert!(!html_chirho.contains("Secret Name Never Shown"));
        assert!(!html_chirho.contains("<form"));
    }
    let long_about_chirho = format!(
        "business_name_chirho=A&about_chirho={}&contact_phone_chirho=1",
        "x".repeat(1_300)
    );
    assert_eq!(
        parse_draft_brief_chirho(long_about_chirho.as_bytes()),
        Err(DraftBriefErrorChirho::TooLongChirho)
    );
    for body_chirho in [GOOD_BRIEF_CHIRHO, &b"not=a&form"[..], &b""[..]] {
        let request_skipped_chirho =
            build_draft_request_chirho(body_chirho) == SKIPPED_STONE_CHIRHO;
        let rejection_skipped_chirho =
            render_draft_brief_rejection_chirho(body_chirho) == SKIPPED_STONE_CHIRHO;
        assert_ne!(request_skipped_chirho, rejection_skipped_chirho);
    }
}

#[test]
fn the_completion_is_trusted_with_nothing_and_becomes_a_reviewable_draft_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    // a clean answer, and the same answer wrapped in a fence and prose
    for completion_chirho in [
        page_json_chirho(),
        format!(
            "Here is the page:\n```json\n{}\n```\nLet me know!",
            page_json_chirho()
        ),
    ] {
        let document_chirho = draft_document_from_completion_chirho(&completion_chirho).unwrap();
        assert_eq!(document_chirho.business_name_chirho, "North Street Repairs");
        assert_eq!(document_chirho.contact_phone_chirho, "555-0124");
        assert_eq!(document_chirho.services_chirho[0].title_chirho, "Repairs");
        assert!(document_chirho.validate_chirho().is_ok());
        let html_chirho = String::from_utf8(render_draft_review_chirho(&response_chirho(
            &completion_chirho,
        )))
        .unwrap();
        assert!(html_chirho.contains(&copy_chirho.editor_chirho.status_imported_chirho));
        assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"0\""));
        assert!(html_chirho.contains("North Street Repairs"));
        assert!(!html_chirho.contains("```"));
    }
    // markup stripped, oversized clamped, unknown keys dropped, numbers tolerated, extra services ignored
    let hostile_chirho = json!({
        "business_name": "<script>alert(1)</script>Good <b>Earth</b> Bakery",
        "headline": "x".repeat(5_000),
        "summary": 42,
        "services": [
            {"title": "Loaves", "body": "Daily"}, {"title": "Cakes", "body": "Orders"},
            {"title": "Pies", "body": "Fridays"}, {"title": "Fourth", "body": "Dropped"}
        ],
        "contact_phone": "555-0142<img src=x onerror=alert(1)>",
        "injected_chirho": "ignored",
        "contact_details": "line one\nline\ttwo"
    })
    .to_string();
    let document_chirho = draft_document_from_completion_chirho(&hostile_chirho).unwrap();
    assert_eq!(
        document_chirho.business_name_chirho,
        "alert(1)Good Earth Bakery"
    );
    assert!(!document_chirho.business_name_chirho.contains('<'));
    assert_eq!(document_chirho.headline_chirho.chars().count(), 140);
    assert_eq!(document_chirho.summary_chirho, "42");
    assert_eq!(document_chirho.services_chirho[2].title_chirho, "Pies");
    // a contact value is kept exactly or dropped, never repaired into a different number
    assert_eq!(document_chirho.contact_phone_chirho, "");
    assert_eq!(document_chirho.contact_details_chirho, "line one line two");
    let long_phone_chirho = json!({"business_name": "A", "headline": "B", "summary": "C", "contact_phone": "5".repeat(41), "contact_email": "a@b.example"}).to_string();
    let long_phone_document_chirho =
        draft_document_from_completion_chirho(&long_phone_chirho).unwrap();
    assert_eq!(long_phone_document_chirho.contact_phone_chirho, "");
    assert_eq!(
        long_phone_document_chirho.contact_email_chirho,
        "a@b.example"
    );
    // the review editor shows the document's own validation notices beside the fields — here the
    // missing contact method, so she types it herself rather than trusting an altered one
    let html_chirho = String::from_utf8(render_draft_review_chirho(&response_chirho(
        &hostile_chirho,
    )))
    .unwrap();
    assert!(html_chirho.contains(&copy_chirho.editor_chirho.status_imported_chirho));
    assert!(html_chirho.contains(&copy_chirho.editor_chirho.contact_method_label_chirho));
    assert!(!html_chirho.contains("<script>"));
    assert!(!html_chirho.contains("onerror"));
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.review_notice_chirho));
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.dropped_contact_notice_chirho));
    // a mangled phone beside a usable email: the drop is still visible, never silent
    let html_chirho = String::from_utf8(render_draft_review_chirho(&response_chirho(
        &long_phone_chirho,
    )))
    .unwrap();
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.dropped_contact_notice_chirho));
    assert!(html_chirho.contains("a@b.example"));
    // a clean draft carries the draft notice and no drop sentence
    let html_chirho = String::from_utf8(render_draft_review_chirho(&response_chirho(
        &page_json_chirho(),
    )))
    .unwrap();
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.review_notice_chirho));
    assert!(!html_chirho.contains(&copy_chirho.draft_chirho.dropped_contact_notice_chirho));
    // not a page at all
    for completion_chirho in [
        "I cannot help with that.",
        "{}",
        "[1,2,3]",
        "{\"business_name\": \"\", \"headline\": \"\", \"summary\": \"\"}",
    ] {
        assert_eq!(
            draft_document_from_completion_chirho(completion_chirho),
            Err(DraftMappingErrorChirho::NotAPageChirho)
        );
        let html_chirho = String::from_utf8(render_draft_review_chirho(&response_chirho(
            completion_chirho,
        )))
        .unwrap();
        assert!(html_chirho.contains(&copy_chirho.draft_chirho.not_drafted_heading_chirho));
        assert!(html_chirho.contains(WORKSPACE_DRAFT_HREF_CHIRHO));
        assert!(!html_chirho.contains("<form"));
    }
}

#[test]
fn the_cards_refusals_and_an_unverifiable_answer_read_as_words_and_save_nothing_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    for (code_chirho, sentence_chirho) in [
        (
            "over-ceiling-chirho",
            &copy_chirho.draft_chirho.refusal_over_ceiling_chirho,
        ),
        (
            "provider-unavailable-chirho",
            &copy_chirho.draft_chirho.refusal_provider_unavailable_chirho,
        ),
        (
            "timeout-chirho",
            &copy_chirho.draft_chirho.refusal_timeout_chirho,
        ),
        (
            "response-unparseable-chirho",
            &copy_chirho.draft_chirho.refusal_unparseable_chirho,
        ),
        (
            "insufficient-credit-chirho",
            &copy_chirho.draft_chirho.refusal_insufficient_credit_chirho,
        ),
        (
            "some-future-code-chirho",
            &copy_chirho.draft_chirho.refusal_unknown_chirho,
        ),
    ] {
        let body_chirho = serde_json::to_vec(&json!({
            "schema_chirho": LLM_REFUSAL_SCHEMA_CHIRHO,
            "refusal_code_chirho": code_chirho,
            "message_chirho": "engine literal never shown"
        }))
        .unwrap();
        assert_eq!(
            decode_llm_response_chirho(&body_chirho),
            Err(DraftMappingErrorChirho::RefusedChirho(
                code_chirho.to_string()
            ))
        );
        let html_chirho = String::from_utf8(render_draft_review_chirho(&body_chirho)).unwrap();
        assert!(html_chirho.contains(sentence_chirho), "{code_chirho}");
        assert!(!html_chirho.contains("engine literal never shown"));
        assert!(!html_chirho.contains(code_chirho));
        assert!(!html_chirho.contains("<form"));
    }
    for bytes_chirho in [
        &b"not json"[..],
        br#"{"schema_chirho":"something-else-chirho","completion_chirho":"x"}"#,
    ] {
        assert_eq!(
            decode_llm_response_chirho(bytes_chirho),
            Err(DraftMappingErrorChirho::UnverifiableAnswerChirho)
        );
        let html_chirho = String::from_utf8(render_draft_review_chirho(bytes_chirho)).unwrap();
        assert!(html_chirho.contains(&copy_chirho.draft_chirho.unavailable_heading_chirho));
        assert!(!html_chirho.contains("<form"));
    }
}

#[test]
fn the_brief_page_is_offered_only_before_the_first_saved_version_and_only_when_bound_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let empty_state_chirho = serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": null,
        "history_chirho": []
    }))
    .unwrap();
    // held: the deployed renderer binds no door yet, so no form, the way to the editor instead
    let html_chirho = String::from_utf8(render_draft_brief_chirho(&empty_state_chirho)).unwrap();
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.held_heading_chirho));
    assert!(!html_chirho.contains("<form"));
    assert!(html_chirho.contains(WORKSPACE_DRAFT_HREF_CHIRHO));
    // bound: the brief form with every field, posting to the draft door
    let bound_chirho = DraftPageV1Chirho::brief_chirho(Some(WORKSPACE_DRAFT_ACTION_CHIRHO));
    assert!(bound_chirho.is_brief_chirho());
    let html_chirho = render_draft_chirho(&bound_chirho).unwrap();
    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert!(html_chirho.contains(&format!("action=\"{WORKSPACE_DRAFT_ACTION_CHIRHO}\"")));
    for field_chirho in [
        "business_name_chirho",
        "about_chirho",
        "audience_chirho",
        "tone_chirho",
        "service_1_chirho",
        "service_3_chirho",
        "contact_phone_chirho",
        "contact_email_chirho",
        "contact_details_chirho",
    ] {
        assert!(
            html_chirho.contains(&format!("name=\"{field_chirho}\"")),
            "{field_chirho}"
        );
    }
    assert_eq!(html_chirho.matches("<option").count(), 3);
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.boundary_chirho));
    // a saved version: no brief, a pointer to the editor with the version named
    let saved_chirho = serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": {"version_chirho": 4, "value_base64_chirho": "e30="},
        "history_chirho": []
    }))
    .unwrap();
    let html_chirho = String::from_utf8(render_draft_brief_chirho(&saved_chirho)).unwrap();
    assert!(
        html_chirho.contains(&copy_chirho.draft_chirho.has_page_heading_chirho)
            || html_chirho.contains(&copy_chirho.draft_chirho.unavailable_heading_chirho)
    );
    assert!(!html_chirho.contains("<form"));
    let html_chirho = render_draft_chirho(&DraftPageV1Chirho::has_page_chirho(4)).unwrap();
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.has_page_heading_chirho));
    assert!(html_chirho.contains("<strong>4</strong>"));
    assert!(!html_chirho.contains("<form"));
    // unverifiable state: actionless
    let html_chirho = String::from_utf8(render_draft_brief_chirho(b"unverified bytes")).unwrap();
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.unavailable_heading_chirho));
    assert!(!html_chirho.contains("<form"));
    assert!(!html_chirho.contains("unverified bytes"));
    // every locale carries the page
    let rtl_chirho = DraftPageV1Chirho::for_locale_chirho(
        DraftPageKindChirho::NotDraftedChirho,
        None,
        UiLocaleChirho::PseudolocaleRtlChirho,
    );
    assert!(render_draft_chirho(&rtl_chirho)
        .unwrap()
        .contains("dir=\"rtl\""));
}

/// What the deterministic stub is asked to answer (cairn #22327 reply): the document shape, with
/// the brief's name, phone, email, hours and services copied exactly from the request's user
/// lines and the rest seeded. Synthesized here from the request itself, so the contract is pinned
/// from the product's end before the two-root proof.
fn stub_shaped_completion_chirho(user_chirho: &str) -> String {
    let line_chirho = |prefix_chirho: &str| {
        user_chirho
            .lines()
            .find_map(|line_chirho| line_chirho.strip_prefix(prefix_chirho))
            .unwrap_or("")
            .trim()
            .to_string()
    };
    let services_chirho: Vec<Value> = line_chirho("Services to list: ")
        .split(';')
        .map(str::trim)
        .filter(|name_chirho| !name_chirho.is_empty())
        .map(|name_chirho| json!({"title": name_chirho, "body": format!("{name_chirho}, done properly.")}))
        .collect();
    json!({
        "business_name": line_chirho("Business name: "),
        "headline": "Draft 0a1b2c3d",
        "summary": "A starting point derived from your brief (deterministic).",
        "services": services_chirho,
        "contact_heading": "Contact",
        "contact_phone": line_chirho("Phone (copy exactly): "),
        "contact_email": line_chirho("Email (copy exactly): "),
        "contact_details": line_chirho("Hours or how to reach us: ")
    })
    .to_string()
}

#[test]
fn the_stub_shaped_completion_maps_to_a_clean_draft_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let brief_chirho = parse_draft_brief_chirho(GOOD_BRIEF_CHIRHO).unwrap();
    let request_chirho = build_llm_request_chirho(&brief_chirho);
    let completion_chirho = stub_shaped_completion_chirho(&request_chirho.user_chirho);
    let document_chirho = draft_document_from_completion_chirho(&completion_chirho).unwrap();
    assert_eq!(
        document_chirho.business_name_chirho,
        brief_chirho.business_name_chirho
    );
    assert_eq!(
        document_chirho.contact_phone_chirho,
        brief_chirho.contact_phone_chirho
    );
    assert_eq!(
        document_chirho.contact_details_chirho,
        brief_chirho.contact_details_chirho
    );
    assert_eq!(document_chirho.services_chirho[0].title_chirho, "Repairs");
    assert!(document_chirho.validate_chirho().is_ok());
    let html_chirho = String::from_utf8(render_draft_review_chirho(&response_chirho(
        &completion_chirho,
    )))
    .unwrap();
    assert!(html_chirho.contains(&copy_chirho.editor_chirho.status_imported_chirho));
    assert!(html_chirho.contains("North Street Repairs"));
    assert!(html_chirho.contains("555-0124"));
    assert!(!html_chirho.contains(&copy_chirho.draft_chirho.dropped_contact_notice_chirho));
    // a distinct brief gives a distinct draft
    let other_chirho = parse_draft_brief_chirho(
        b"business_name_chirho=Good+Earth+Bakery&about_chirho=Bread&contact_phone_chirho=555-0142",
    )
    .unwrap();
    let other_completion_chirho =
        stub_shaped_completion_chirho(&build_llm_request_chirho(&other_chirho).user_chirho);
    let other_document_chirho =
        draft_document_from_completion_chirho(&other_completion_chirho).unwrap();
    assert_ne!(
        other_document_chirho.business_name_chirho,
        document_chirho.business_name_chirho
    );
    assert_eq!(other_document_chirho.contact_phone_chirho, "555-0142");
}

/// The deterministic stub's ACTUAL completion for GOOD_BRIEF, captured from cairn branch
/// claude-llm-served-stub-chirho @ bcd1894d (StubLlmProviderChirho::complete_chirho on the exact
/// user text build_llm_request_chirho writes). A golden from the other end of the contract.
const REAL_STUB_COMPLETION_CHIRHO: &str = r#"{"business_name":"North Street Repairs","contact_details":"Weekdays 8 to 5","contact_email":"","contact_heading":"Get in touch","contact_phone":"555-0124","headline":"Welcome to North Street Repairs","services":[{"body":"Repairs at North Street Repairs.","title":"Repairs"}],"summary":"North Street Repairs — a starting draft from your brief. Reference 5bc2268f."}"#;

#[test]
fn the_real_stubs_completion_maps_to_a_clean_draft_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let brief_chirho = parse_draft_brief_chirho(GOOD_BRIEF_CHIRHO).unwrap();
    let document_chirho =
        draft_document_from_completion_chirho(REAL_STUB_COMPLETION_CHIRHO).unwrap();
    assert_eq!(
        document_chirho.business_name_chirho,
        brief_chirho.business_name_chirho
    );
    assert_eq!(
        document_chirho.contact_phone_chirho,
        brief_chirho.contact_phone_chirho
    );
    assert_eq!(document_chirho.contact_email_chirho, "");
    assert_eq!(
        document_chirho.contact_details_chirho,
        brief_chirho.contact_details_chirho
    );
    assert_eq!(document_chirho.services_chirho[0].title_chirho, "Repairs");
    assert_eq!(
        document_chirho.headline_chirho,
        "Welcome to North Street Repairs"
    );
    assert!(document_chirho.validate_chirho().is_ok());
    let html_chirho = String::from_utf8(render_draft_review_chirho(&response_chirho(
        REAL_STUB_COMPLETION_CHIRHO,
    )))
    .unwrap();
    assert!(html_chirho.contains(&copy_chirho.editor_chirho.status_imported_chirho));
    assert!(html_chirho.contains(&copy_chirho.draft_chirho.review_notice_chirho));
    assert!(!html_chirho.contains(&copy_chirho.draft_chirho.dropped_contact_notice_chirho));
    assert!(html_chirho.contains("name=\"expected_version_chirho\" value=\"0\""));
    assert!(html_chirho.contains("555-0124"));
}

/// The completion ceiling is the card's reach, not an arbitrary size: exactly the ceiling is read,
/// one byte more is not a page; a field at the ceiling folds to its own limit and no further.
#[test]
fn the_completion_ceiling_and_the_bounded_fold_chirho() {
    use perffection_workspace_chirho::first_draft_chirho::{
        decode_llm_response_chirho, draft_document_from_completion_chirho, DraftMappingErrorChirho,
        COMPLETION_MAX_BYTES_CHIRHO,
    };
    let wrap_chirho = |completion_chirho: &str| {
        serde_json::json!({"schema_chirho": "llm-response-v1-chirho", "completion_chirho": completion_chirho})
            .to_string()
    };
    let at_ceiling_chirho = "x".repeat(COMPLETION_MAX_BYTES_CHIRHO);
    assert_eq!(
        decode_llm_response_chirho(wrap_chirho(&at_ceiling_chirho).as_bytes())
            .map(|c_chirho| c_chirho.len()),
        Ok(COMPLETION_MAX_BYTES_CHIRHO)
    );
    let past_ceiling_chirho = "x".repeat(COMPLETION_MAX_BYTES_CHIRHO + 1);
    assert_eq!(
        decode_llm_response_chirho(wrap_chirho(&past_ceiling_chirho).as_bytes()),
        Err(DraftMappingErrorChirho::NotAPageChirho)
    );
    // a summary padded to the ceiling, with markup and runs of whitespace, folds to the limit
    let mut document_chirho: serde_json::Value =
        serde_json::from_str(REAL_STUB_COMPLETION_CHIRHO).unwrap();
    let base_len_chirho = document_chirho.to_string().len();
    let noisy_chirho =
        "<em>word</em>  \t\n ".repeat((COMPLETION_MAX_BYTES_CHIRHO - base_len_chirho) / 20);
    document_chirho["summary"] = serde_json::Value::String(noisy_chirho);
    let completion_chirho = document_chirho.to_string();
    assert!(completion_chirho.len() <= COMPLETION_MAX_BYTES_CHIRHO);
    let drafted_chirho = draft_document_from_completion_chirho(&completion_chirho).unwrap();
    assert_eq!(drafted_chirho.summary_chirho.chars().count(), 600);
    assert!(drafted_chirho.summary_chirho.starts_with("word word word"));
    assert!(!drafted_chirho.summary_chirho.contains('<'));
    assert!(!drafted_chirho.summary_chirho.contains("  "));
}

/// Multibyte text at every boundary the mapper keeps: the fold counts characters and never
/// splits one, a contact value of multibyte characters at its limit is kept exactly and one more
/// is dropped with the notice, and a multibyte completion at exactly the ceiling reads while one
/// byte more is not a page.
#[test]
fn multibyte_text_holds_its_boundaries_chirho() {
    use perffection_workspace_chirho::first_draft_chirho::{
        decode_llm_response_chirho, draft_candidate_from_completion_chirho,
        DraftMappingErrorChirho, COMPLETION_MAX_BYTES_CHIRHO,
    };
    use perffection_workspace_chirho::page_document_chirho::{
        BUSINESS_NAME_MAX_CHARACTERS_CHIRHO, CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
        SUMMARY_MAX_CHARACTERS_CHIRHO,
    };
    let kept_phone_chirho = "\u{ff15}".repeat(CONTACT_PHONE_MAX_CHARACTERS_CHIRHO); // fullwidth 5, 3 bytes each
    let completion_chirho = serde_json::json!({
        "business_name": "\u{e9}".repeat(BUSINESS_NAME_MAX_CHARACTERS_CHIRHO + 5),
        "headline": "Bienvenue",
        "summary": "\u{65e5}\u{672c}\u{8a9e} <b>caf\u{e9}</b>  ".repeat(200),
        "contact_phone": kept_phone_chirho,
        "contact_email": "",
        "contact_details": "Lundi \u{e0} vendredi",
    })
    .to_string();
    let candidate_chirho = draft_candidate_from_completion_chirho(&completion_chirho).unwrap();
    let document_chirho = &candidate_chirho.document_chirho;
    assert_eq!(
        document_chirho.business_name_chirho.chars().count(),
        BUSINESS_NAME_MAX_CHARACTERS_CHIRHO
    );
    assert_eq!(
        document_chirho.summary_chirho.chars().count(),
        SUMMARY_MAX_CHARACTERS_CHIRHO
    );
    assert!(document_chirho
        .summary_chirho
        .starts_with("\u{65e5}\u{672c}\u{8a9e} caf\u{e9} "));
    assert!(!document_chirho.summary_chirho.contains('<'));
    assert_eq!(document_chirho.contact_phone_chirho, kept_phone_chirho);
    assert!(!candidate_chirho.dropped_contact_chirho);

    let one_more_chirho = serde_json::json!({
        "business_name": "A", "headline": "B", "summary": "C",
        "contact_phone": "\u{ff15}".repeat(CONTACT_PHONE_MAX_CHARACTERS_CHIRHO + 1),
        "contact_email": "a@b.example",
    })
    .to_string();
    let dropped_chirho = draft_candidate_from_completion_chirho(&one_more_chirho).unwrap();
    assert_eq!(dropped_chirho.document_chirho.contact_phone_chirho, "");
    assert!(dropped_chirho.dropped_contact_chirho);

    // a completion of exactly the ceiling in bytes, mostly two-byte characters
    let wrap_chirho = |completion_chirho: &str| {
        serde_json::json!({"schema_chirho": "llm-response-v1-chirho", "completion_chirho": completion_chirho})
            .to_string()
    };
    let mut at_ceiling_chirho = "\u{e9}".repeat(COMPLETION_MAX_BYTES_CHIRHO / 2 - 1);
    at_ceiling_chirho.push_str("xx");
    assert_eq!(at_ceiling_chirho.len(), COMPLETION_MAX_BYTES_CHIRHO);
    assert_eq!(
        decode_llm_response_chirho(wrap_chirho(&at_ceiling_chirho).as_bytes())
            .map(|c_chirho| c_chirho.len()),
        Ok(COMPLETION_MAX_BYTES_CHIRHO)
    );
    at_ceiling_chirho.push('x');
    assert_eq!(
        decode_llm_response_chirho(wrap_chirho(&at_ceiling_chirho).as_bytes()),
        Err(DraftMappingErrorChirho::NotAPageChirho)
    );
}

#[test]
fn the_engines_hidden_caller_key_is_tolerated_and_goes_nowhere_chirho() {
    use perffection_workspace_chirho::draft_page_chirho::{
        DraftPageV1Chirho, WORKSPACE_DRAFT_ACTION_CHIRHO,
    };
    use perffection_workspace_chirho::first_draft_chirho::{
        read_draft_brief_chirho, DraftBriefOutcomeChirho,
    };
    use perffection_workspace_chirho::page_edit_request_chirho::FORM_CALLER_KEY_FIELD_CHIRHO;
    use perffection_workspace_chirho::page_render_chirho::render_draft_chirho;
    let key_chirho = "KEY-marker-7f3a9c";
    let with_key_chirho = |body_chirho: &[u8]| {
        let mut bytes_chirho = body_chirho.to_vec();
        bytes_chirho
            .extend_from_slice(format!("&{FORM_CALLER_KEY_FIELD_CHIRHO}={key_chirho}").as_bytes());
        bytes_chirho
    };
    // a brief that carries the engine's field is still a brief, and the request to the card is
    // byte-identical with and without it: the key never becomes part of the prompt
    let keyed_chirho = with_key_chirho(GOOD_BRIEF_CHIRHO);
    assert!(matches!(
        read_draft_brief_chirho(&keyed_chirho),
        DraftBriefOutcomeChirho::ReadyChirho(_)
    ));
    let request_chirho = build_draft_request_chirho(&keyed_chirho);
    assert_ne!(request_chirho, SKIPPED_STONE_CHIRHO);
    assert_eq!(
        request_chirho,
        build_draft_request_chirho(GOOD_BRIEF_CHIRHO)
    );
    assert!(!String::from_utf8_lossy(&request_chirho).contains(key_chirho));
    // a brief the page cannot use stays REFUSED (not unreadable), and the submitted key is not
    // copied into the form the product hands back: each render gets its own from the engine
    let refused_chirho =
        with_key_chirho(b"business_name_chirho=A&about_chirho=B&tone_chirho=warm-chirho");
    let DraftBriefOutcomeChirho::RefusedChirho {
        words_chirho,
        issues_chirho,
    } = read_draft_brief_chirho(&refused_chirho)
    else {
        panic!("a contact-less brief with the engine's field must be refused, not unreadable");
    };
    let html_chirho = render_draft_chirho(&DraftPageV1Chirho::refused_brief_chirho(
        words_chirho,
        &issues_chirho,
        Some(WORKSPACE_DRAFT_ACTION_CHIRHO),
    ))
    .unwrap();
    assert!(!html_chirho.contains(key_chirho));
    // any other field our form never sends still makes the body unreadable
    let mut surprise_chirho = GOOD_BRIEF_CHIRHO.to_vec();
    surprise_chirho.extend_from_slice(b"&idempotency_key_two_chirho=x");
    assert_eq!(
        read_draft_brief_chirho(&surprise_chirho),
        DraftBriefOutcomeChirho::UnreadableChirho
    );
}

/// A refused brief comes back around her own words (the bound door; was draft_brief_form_chirho.rs).
mod brief_form_chirho {
    // A brief the page cannot use yet comes back to her as the form around her own words. These pin
    // behavior, never sentences: every problem is reported at once and linked to a control that
    // exists; her words return exactly, escaped, and only in a readable brief's own form; an
    // unreadable body gets nothing of hers back.

    use perffection_workspace_chirho::draft_page_chirho::{
        DraftPageV1Chirho, WORKSPACE_DRAFT_ACTION_CHIRHO,
    };
    use perffection_workspace_chirho::first_draft_chirho::{
        read_draft_brief_chirho, DraftBriefFieldChirho, DraftBriefIssueChirho,
        DraftBriefOutcomeChirho, BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO,
    };
    use perffection_workspace_chirho::implementations_chirho::{
        build_draft_request_chirho::build_draft_request_chirho,
        render_draft_brief_rejection_chirho::render_draft_brief_rejection_chirho,
    };
    use perffection_workspace_chirho::page_render_chirho::render_draft_chirho;

    use super::support_chirho::{
        assert_page_wiring_chirho, attribute_tokens_chirho, opening_tag_chirho,
    };

    const SKIPPED_STONE_CHIRHO: &[u8] = cairn_node_sdk_chirho::SKIPPED_STONE_BYTES_CHIRHO;

    /// What the rejection branch renders the day the draft door is bound.
    fn bound_refusal_chirho(body_chirho: &[u8]) -> (String, Vec<DraftBriefIssueChirho>) {
        let DraftBriefOutcomeChirho::RefusedChirho {
            words_chirho,
            issues_chirho,
        } = read_draft_brief_chirho(body_chirho)
        else {
            panic!("the body under test must be a readable brief the page cannot use");
        };
        let model_chirho = DraftPageV1Chirho::refused_brief_chirho(
            words_chirho,
            &issues_chirho,
            Some(WORKSPACE_DRAFT_ACTION_CHIRHO),
        );
        (render_draft_chirho(&model_chirho).unwrap(), issues_chirho)
    }

    fn summary_chirho(html_chirho: &str) -> &str {
        let start_chirho = html_chirho.find("draft-rejected-heading-chirho").unwrap();
        let rest_chirho = &html_chirho[start_chirho..];
        &rest_chirho[..rest_chirho.find("</section>").unwrap()]
    }

    #[test]
    fn every_problem_is_reported_at_once_and_linked_to_a_control_chirho() {
        // nameless, wordless and contact-less in one submission
        let (html_chirho, issues_chirho) =
            bound_refusal_chirho(b"audience_chirho=Homes+around+Mesa&tone_chirho=plain-chirho");
        assert_eq!(
            issues_chirho,
            vec![
                DraftBriefIssueChirho::NameMissingChirho,
                DraftBriefIssueChirho::AboutMissingChirho,
                DraftBriefIssueChirho::ContactMissingChirho,
            ]
        );
        let summary_chirho = summary_chirho(&html_chirho);
        assert_eq!(summary_chirho.matches("<li>").count(), issues_chirho.len());
        let targets_chirho: Vec<String> = attribute_tokens_chirho(summary_chirho, "href")
            .into_iter()
            .filter_map(|href_chirho| href_chirho.strip_prefix('#').map(str::to_string))
            .collect();
        assert_eq!(targets_chirho.len(), issues_chirho.len());
        for target_chirho in &targets_chirho {
            let tag_chirho = opening_tag_chirho(&html_chirho, target_chirho);
            assert!(
                tag_chirho.starts_with("<input") || tag_chirho.starts_with("<textarea"),
                "{tag_chirho}"
            );
            assert!(
                tag_chirho.contains(" aria-invalid=\"true\""),
                "{tag_chirho}"
            );
        }
        assert!(assert_page_wiring_chirho("refused brief", &html_chirho) >= 5);
        // the brief form it hands back carries the engine's caller-key placeholder once
        assert_eq!(
            super::support_chirho::assert_caller_key_forms_chirho("refused brief", &html_chirho),
            1
        );
    }

    #[test]
    fn her_words_come_back_exactly_in_the_form_and_the_tone_stays_chosen_chirho() {
        let body_chirho = b"business_name_chirho=North+Street+Repairs&about_chirho=We+fix+what+breaks.&audience_chirho=Homes+around+Mesa&tone_chirho=confident-chirho&service_1_chirho=Repairs&service_2_chirho=&service_3_chirho=Callouts&contact_phone_chirho=&contact_email_chirho=&contact_details_chirho=Weekdays+8+to+5";
        let (html_chirho, issues_chirho) = bound_refusal_chirho(body_chirho);
        assert_eq!(
            issues_chirho,
            vec![DraftBriefIssueChirho::ContactMissingChirho]
        );
        for (control_chirho, value_chirho) in [
            ("draft-business-name-chirho", "North Street Repairs"),
            ("draft-audience-chirho", "Homes around Mesa"),
            ("draft-service-1-chirho", "Repairs"),
            ("draft-service-3-chirho", "Callouts"),
            ("draft-contact-details-chirho", "Weekdays 8 to 5"),
        ] {
            assert!(
                opening_tag_chirho(&html_chirho, control_chirho)
                    .contains(&format!(" value=\"{value_chirho}\"")),
                "{control_chirho}"
            );
        }
        assert!(opening_tag_chirho(&html_chirho, "draft-service-2-chirho").contains(" value=\"\""));
        assert!(html_chirho.contains(">We fix what breaks.</textarea>"));
        assert_eq!(html_chirho.matches(" selected").count(), 1);
        assert!(html_chirho.contains("<option value=\"confident-chirho\" selected>"));
        // the form posts to the draft door again
        assert!(html_chirho.contains(&format!("action=\"{WORKSPACE_DRAFT_ACTION_CHIRHO}\"")));
    }

    #[test]
    fn the_contact_rule_is_said_once_on_the_pair_and_described_into_both_chirho() {
        let (html_chirho, _) =
            bound_refusal_chirho(b"business_name_chirho=A&about_chirho=B&tone_chirho=warm-chirho");
        assert_eq!(
            html_chirho
                .matches(" id=\"draft-contact-method-error-chirho\"")
                .count(),
            1
        );
        for member_chirho in ["draft-contact-phone-chirho", "draft-contact-email-chirho"] {
            let tag_chirho = opening_tag_chirho(&html_chirho, member_chirho);
            assert!(
                attribute_tokens_chirho(tag_chirho, "aria-describedby")
                    .contains(&"draft-contact-method-error-chirho".to_string()),
                "{tag_chirho}"
            );
            assert!(tag_chirho.contains(" aria-invalid=\"true\""));
        }
        assert!(!opening_tag_chirho(&html_chirho, "draft-about-chirho").contains("aria-invalid"));
    }

    #[test]
    fn an_over_long_answer_lands_on_its_control_and_her_words_are_not_cut_chirho() {
        let long_chirho = "x".repeat(BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO + 100);
        let body_chirho =
            format!("business_name_chirho=A&about_chirho={long_chirho}&contact_phone_chirho=1");
        let (html_chirho, issues_chirho) = bound_refusal_chirho(body_chirho.as_bytes());
        assert_eq!(
            issues_chirho,
            vec![DraftBriefIssueChirho::TooLongChirho {
                field_chirho: DraftBriefFieldChirho::AboutChirho,
                max_characters_chirho: BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO,
            }]
        );
        assert!(opening_tag_chirho(&html_chirho, "draft-about-chirho").contains("aria-invalid"));
        assert!(html_chirho.contains(&format!(">{long_chirho}</textarea>")));
        // the limit she has to meet is said in the message, as a number
        let message_chirho = &html_chirho[html_chirho
            .find(" id=\"draft-about-error-chirho\"")
            .unwrap()..];
        let message_chirho = &message_chirho[..message_chirho.find("</p>").unwrap()];
        assert!(message_chirho.contains(&BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO.to_string()));
    }

    #[test]
    fn her_words_are_escaped_wherever_they_return_chirho() {
        let body_chirho = b"business_name_chirho=%22%3E%3Cscript%3Ealert(1)%3C%2Fscript%3E&about_chirho=%3C%2Ftextarea%3E%3Cimg+src%3Dx+onerror%3Dalert(2)%3E&tone_chirho=warm-chirho";
        let (html_chirho, _) = bound_refusal_chirho(body_chirho);
        assert!(!html_chirho.contains("<script>"));
        assert!(!html_chirho.contains("<img"));
        assert_eq!(html_chirho.matches("</textarea>").count(), 1);
    }

    #[test]
    fn an_unreadable_body_gets_a_sentence_and_nothing_of_hers_chirho() {
        for body_chirho in [
            &b"business_name_chirho=Marker+Bakery&about_chirho=B&contact_phone_chirho=1&surprise_chirho=1"[..],
            &b"not=a&form"[..],
        ] {
            assert_eq!(
                read_draft_brief_chirho(body_chirho),
                DraftBriefOutcomeChirho::UnreadableChirho
            );
            assert_eq!(build_draft_request_chirho(body_chirho), SKIPPED_STONE_CHIRHO);
            let html_chirho =
                String::from_utf8(render_draft_brief_rejection_chirho(body_chirho)).unwrap();
            assert!(!html_chirho.contains("Marker Bakery"));
            assert!(!html_chirho.contains("<form"));
        }
    }

    #[test]
    fn a_blank_brief_carries_no_invalid_state_and_a_refused_one_says_so_in_the_tab_title_chirho() {
        let blank_chirho = render_draft_chirho(&DraftPageV1Chirho::brief_chirho(Some(
            WORKSPACE_DRAFT_ACTION_CHIRHO,
        )))
        .unwrap();
        // the carrier's four dormant notices are hidden until targeted; everything else must be clean
        let clean_chirho = super::support_chirho::without_carrier_notices_chirho(&blank_chirho);
        assert!(!clean_chirho.contains("aria-invalid"));
        assert!(!clean_chirho.contains("-error-chirho"));
        assert!(!clean_chirho.contains("role=\"alert\""));
        assert!(assert_page_wiring_chirho("blank brief", &blank_chirho) >= 5);
        let (refused_chirho, _) =
            bound_refusal_chirho(b"business_name_chirho=A&about_chirho=B&tone_chirho=warm-chirho");
        let title_chirho = |html_chirho: &str| {
            let start_chirho = html_chirho.find("<title>").unwrap() + "<title>".len();
            html_chirho[start_chirho..html_chirho.find("</title>").unwrap()].to_string()
        };
        assert_ne!(title_chirho(&blank_chirho), title_chirho(&refused_chirho));
    }
}

/// What she can draft with, beside the brief form: one read of the credit view port, in the Credits page's own words.
mod credit_line_chirho {
    use super::support_chirho::assert_page_wiring_chirho;
    use perffection_credits_chirho::credit_view_port_chirho::{
        CreditStatementChirho, CreditViewChirho, CreditViewUnavailableChirho, CreditsChirho,
    };
    use perffection_ui_chirho::{CreditsCopyChirho, UiLocaleChirho};
    use perffection_workspace_chirho::draft_page_chirho::{
        DraftPageKindChirho, DraftPageV1Chirho, SERVED_CREDIT_VIEW_CHIRHO,
        WORKSPACE_DRAFT_ACTION_CHIRHO,
    };
    use perffection_workspace_chirho::page_render_chirho::render_draft_chirho;

    fn known_chirho(credits_chirho: u64) -> CreditViewChirho {
        CreditViewChirho::KnownChirho(
            CreditStatementChirho::new_chirho(
                CreditsChirho::new_chirho(credits_chirho),
                Vec::new(),
                false,
            )
            .expect("an empty history keeps the port's rules"),
        )
    }

    fn bound_page_chirho(view_chirho: &CreditViewChirho) -> String {
        render_draft_chirho(
            &DraftPageV1Chirho::brief_chirho(Some(WORKSPACE_DRAFT_ACTION_CHIRHO))
                .with_credits_chirho(view_chirho, UiLocaleChirho::EnglishChirho),
        )
        .unwrap()
    }

    /// The credit line's own element text, or `None` when the page has none.
    fn credit_line_chirho(html_chirho: &str) -> Option<&str> {
        let at_chirho = html_chirho.find(r#" id="draft-credits-chirho""#)?;
        let open_chirho = at_chirho + html_chirho[at_chirho..].find('>')? + 1;
        let close_chirho = open_chirho + html_chirho[open_chirho..].find("</p>")?;
        Some(&html_chirho[open_chirho..close_chirho])
    }

    /// The text a reader sees: numeric entities (the escaper's `&#39;`) are one character, not digits.
    fn visible_chirho(html_chirho: &str) -> String {
        let mut text_chirho = String::new();
        let mut rest_chirho = html_chirho;
        while let Some(at_chirho) = rest_chirho.find("&#") {
            text_chirho.push_str(&rest_chirho[..at_chirho]);
            let end_chirho = at_chirho
                + rest_chirho[at_chirho..]
                    .find(';')
                    .expect("an entity closes");
            text_chirho.push('·');
            rest_chirho = &rest_chirho[end_chirho + 1..];
        }
        text_chirho + rest_chirho
    }

    /// A catalog value as the page's html escaper writes it.
    fn escaped_chirho(text_chirho: &str) -> String {
        text_chirho
            .replace('&', "&#38;")
            .replace('<', "&#60;")
            .replace('>', "&#62;")
            .replace('"', "&#34;")
            .replace('\'', "&#39;")
    }

    #[test]
    fn a_known_balance_shows_whole_grouped_credits_and_zero_is_a_number_chirho() {
        let copy_chirho = CreditsCopyChirho::english_chirho();
        let html_chirho = bound_page_chirho(&known_chirho(1250));
        let line_chirho =
            credit_line_chirho(&html_chirho).expect("a live brief form carries the line");
        assert_eq!(
            line_chirho,
            format!(
                "{} <strong>{}</strong>",
                escaped_chirho(&copy_chirho.balance_heading_chirho),
                escaped_chirho(&copy_chirho.balance_chirho(1250))
            )
        );
        assert_eq!(
            html_chirho.matches(r#" id="draft-credits-chirho""#).count(),
            1
        );
        assert_page_wiring_chirho("brief with a known balance", &html_chirho);
        let zero_chirho = bound_page_chirho(&known_chirho(0));
        assert!(credit_line_chirho(&zero_chirho)
            .unwrap()
            .ends_with(&format!(
                "<strong>{}</strong>",
                escaped_chirho(&copy_chirho.balance_chirho(0))
            )));
    }

    #[test]
    fn every_unavailable_read_is_the_sentence_and_never_a_number_chirho() {
        let copy_chirho = CreditsCopyChirho::english_chirho();
        for reason_chirho in [
            CreditViewUnavailableChirho::NotConnectedChirho,
            CreditViewUnavailableChirho::ReadFailedChirho,
            CreditViewUnavailableChirho::UnverifiableChirho,
        ] {
            let html_chirho =
                bound_page_chirho(&CreditViewChirho::UnavailableChirho(reason_chirho));
            let line_chirho = credit_line_chirho(&html_chirho).expect("unavailable still says so");
            assert_eq!(
                line_chirho,
                escaped_chirho(&copy_chirho.unavailable_heading_chirho),
                "{reason_chirho:?}"
            );
            assert!(
                !visible_chirho(line_chirho)
                    .chars()
                    .any(|character_chirho| character_chirho.is_ascii_digit()),
                "{reason_chirho:?}"
            );
        }
        // The served renderers read exactly the unconnected port's answer until a payer can reach them (W3).
        assert_eq!(
            SERVED_CREDIT_VIEW_CHIRHO,
            CreditViewChirho::UnavailableChirho(CreditViewUnavailableChirho::NotConnectedChirho)
        );
    }

    #[test]
    fn a_page_with_no_form_to_send_carries_no_credit_line_chirho() {
        // A page of another kind handed a draft door still has no brief form, so it carries none either.
        let other_kind_with_door_chirho = DraftPageV1Chirho::for_locale_chirho(
            DraftPageKindChirho::HasPageChirho { version_chirho: 4 },
            Some(WORKSPACE_DRAFT_ACTION_CHIRHO),
            UiLocaleChirho::EnglishChirho,
        );
        for model_chirho in [
            DraftPageV1Chirho::brief_chirho(None),
            DraftPageV1Chirho::has_page_chirho(4),
            other_kind_with_door_chirho,
        ] {
            let html_chirho = render_draft_chirho(
                &model_chirho.with_credits_chirho(&known_chirho(5), UiLocaleChirho::EnglishChirho),
            )
            .unwrap();
            assert_eq!(credit_line_chirho(&html_chirho), None);
        }
    }
}
