// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The agent API's pure layer: what it describes is what a save accepts, a save is strict and
//! reaches the SAME write request her browser's save builds, and a refusal never echoes the body.

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use perffection_workspace_chirho::agent_page_chirho::{AgentPageStateChirho, AgentPageV1Chirho};
use perffection_workspace_chirho::agent_save_chirho::{
    agent_save_outcome_from_door_chirho, classify_agent_save_chirho, AgentSaveDecisionChirho,
    AgentSaveOutcomeV1Chirho, AgentSaveRefusalChirho, AGENT_SAVE_MAX_BODY_BYTES_CHIRHO,
};
use perffection_workspace_chirho::agent_surface_chirho::{
    agent_field_limit_chirho, agent_sample_page_chirho, agent_surface_chirho, AgentFieldNeedChirho,
    AGENT_ISSUE_CODES_CHIRHO,
};
use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PageValidationCodeChirho,
    PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_workspace_chirho::save_page_decision_chirho::{
    classify_page_edit_form_chirho, SavePageDecisionChirho,
};
use perffection_workspace_chirho::viewer_state_contract_chirho::{
    PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO, VIEWER_STATE_CONFLICT_SCHEMA_CHIRHO,
    VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
};
use serde_json::{json, Value};

const MARKER_CHIRHO: &str = "agent-body-marker-7f3a-chirho";

fn page_chirho() -> PageDocumentV2Chirho {
    PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "North Street Repairs".to_string(),
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
    }
}

fn body_chirho(expected_version_chirho: u64, page_chirho: &PageDocumentV2Chirho) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "expected_version_chirho": expected_version_chirho,
        "page_chirho": page_chirho,
    }))
    .unwrap()
}

/// The same page as `page_chirho`, as her browser's editor form posts it.
fn browser_form_chirho(expected_version_chirho: u64) -> Vec<u8> {
    format!(
        "schema_version_chirho=2&expected_version_chirho={expected_version_chirho}&business_name_chirho=North+Street+Repairs&headline_chirho=Clear+help&summary_chirho=Straightforward+local+service&service_1_title_chirho=Repairs&service_1_body_chirho=Homes+and+small+shops&service_2_title_chirho=&service_2_body_chirho=&service_3_title_chirho=&service_3_body_chirho=&contact_heading_chirho=Tell+us+what+needs+attention&contact_phone_chirho=555-0124&contact_email_chirho=&contact_details_chirho=During+business+hours"
    )
    .into_bytes()
}

fn refusal_of_chirho(decision_chirho: AgentSaveDecisionChirho) -> AgentSaveOutcomeV1Chirho {
    match decision_chirho {
        AgentSaveDecisionChirho::RefuseChirho(outcome_chirho) => *outcome_chirho,
        AgentSaveDecisionChirho::WriteChirho(_) => panic!("expected a refusal, got a write"),
    }
}

fn assert_refused_chirho(body_chirho: &[u8], refusal_chirho: AgentSaveRefusalChirho) {
    let outcome_chirho = refusal_of_chirho(classify_agent_save_chirho(body_chirho));
    assert!(!outcome_chirho.saved_chirho);
    assert_eq!(
        outcome_chirho.refusal_chirho,
        Some(refusal_chirho.code_chirho())
    );
    assert_eq!(outcome_chirho.page_chirho, None);
    let answer_chirho = String::from_utf8(outcome_chirho.to_json_bytes_chirho()).unwrap();
    assert!(
        !answer_chirho.contains(MARKER_CHIRHO),
        "a refusal must never echo the body"
    );
}

fn state_response_chirho(version_chirho: u64, page_chirho: &PageDocumentV2Chirho) -> Vec<u8> {
    let current_chirho = json!({
        "version_chirho": version_chirho,
        "value_base64_chirho": BASE64_CHIRHO.encode(serde_json::to_vec(page_chirho).unwrap()),
    });
    serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": current_chirho.clone(),
        "history_chirho": [current_chirho],
    }))
    .unwrap()
}

fn with_field_chirho(path_chirho: &str, value_chirho: String) -> PageDocumentV2Chirho {
    let mut page_chirho = page_chirho();
    match path_chirho {
        "business_name_chirho" => page_chirho.business_name_chirho = value_chirho,
        "headline_chirho" => page_chirho.headline_chirho = value_chirho,
        "summary_chirho" => page_chirho.summary_chirho = value_chirho,
        "services_chirho[0].title_chirho" => {
            page_chirho.services_chirho[0].title_chirho = value_chirho
        }
        "services_chirho[0].body_chirho" => {
            page_chirho.services_chirho[0].body_chirho = value_chirho
        }
        "contact_heading_chirho" => page_chirho.contact_heading_chirho = value_chirho,
        "contact_details_chirho" => page_chirho.contact_details_chirho = value_chirho,
        other_chirho => panic!("no text filler for {other_chirho}"),
    }
    page_chirho
}

#[test]
fn every_described_limit_is_the_limit_a_save_enforces_chirho() {
    let surface_chirho = agent_surface_chirho();
    let mut checked_chirho = 0;
    for field_chirho in &surface_chirho.page_document_chirho.fields_chirho {
        // The owner's own contact values have formats of their own; their limits are covered by
        // the validator's tests. Every free-text field is put AT and one PAST its described limit.
        if field_chirho.copy_exactly_from_owner_chirho {
            continue;
        }
        let path_chirho = field_chirho.field_path_chirho.replace("[N]", "[0]");
        let at_limit_chirho = "a".repeat(field_chirho.max_characters_chirho);
        let at_chirho = body_chirho(0, &with_field_chirho(&path_chirho, at_limit_chirho));
        assert!(
            matches!(
                classify_agent_save_chirho(&at_chirho),
                AgentSaveDecisionChirho::WriteChirho(_)
            ),
            "{path_chirho} at its described limit must save"
        );
        let past_limit_chirho = "a".repeat(field_chirho.max_characters_chirho + 1);
        let past_chirho = body_chirho(0, &with_field_chirho(&path_chirho, past_limit_chirho));
        let outcome_chirho = refusal_of_chirho(classify_agent_save_chirho(&past_chirho));
        assert_eq!(
            outcome_chirho.refusal_chirho,
            Some(AgentSaveRefusalChirho::InvalidPageChirho.code_chirho())
        );
        assert!(
            outcome_chirho.issues_chirho.iter().any(|issue_chirho| {
                issue_chirho.field_path_chirho == path_chirho
                    && issue_chirho.code_chirho == PageValidationCodeChirho::TooManyCharactersChirho
            }),
            "{path_chirho} one past its described limit must be refused BY NAME, never clamped"
        );
        assert!(
            outcome_chirho.issues_chirho.iter().any(|issue_chirho| {
                issue_chirho.field_path_chirho == path_chirho
                    && issue_chirho.max_characters_chirho
                        == Some(field_chirho.max_characters_chirho)
            }),
            "{path_chirho}: the length refusal carries the limit the surface describes"
        );
        checked_chirho += 1;
    }
    assert_eq!(checked_chirho, 7, "every free-text field was exercised");
}

#[test]
fn the_described_needs_are_the_ones_a_save_enforces_chirho() {
    let surface_chirho = agent_surface_chirho();
    for field_chirho in &surface_chirho.page_document_chirho.fields_chirho {
        if field_chirho.need_chirho != AgentFieldNeedChirho::RequiredChirho {
            continue;
        }
        let emptied_chirho = with_field_chirho(field_chirho.field_path_chirho, String::new());
        let outcome_chirho =
            refusal_of_chirho(classify_agent_save_chirho(&body_chirho(0, &emptied_chirho)));
        assert!(outcome_chirho.issues_chirho.iter().any(|issue_chirho| {
            issue_chirho.field_path_chirho == field_chirho.field_path_chirho
                && issue_chirho.code_chirho == PageValidationCodeChirho::RequiredChirho
        }));
    }
    // no service at all, and neither a phone nor an email: each named, nothing invented
    let mut bare_chirho = page_chirho();
    bare_chirho.services_chirho[0] = PageServiceV1Chirho::default();
    bare_chirho.contact_phone_chirho = String::new();
    let outcome_chirho =
        refusal_of_chirho(classify_agent_save_chirho(&body_chirho(0, &bare_chirho)));
    let named_chirho: Vec<&str> = outcome_chirho
        .issues_chirho
        .iter()
        .map(|issue_chirho| issue_chirho.field_path_chirho.as_str())
        .collect();
    assert!(named_chirho.contains(&"services_chirho"));
    assert!(named_chirho.contains(&"contact_chirho"));
    // optional means optional
    let mut no_details_chirho = page_chirho();
    no_details_chirho.contact_details_chirho = String::new();
    assert!(matches!(
        classify_agent_save_chirho(&body_chirho(0, &no_details_chirho)),
        AgentSaveDecisionChirho::WriteChirho(_)
    ));
}

#[test]
fn the_sample_is_complete_and_the_refusal_list_is_whole_chirho() {
    let surface_chirho = agent_surface_chirho();
    assert_eq!(
        surface_chirho.sample_page_chirho,
        agent_sample_page_chirho()
    );
    assert!(surface_chirho.sample_page_chirho.validate_chirho().is_ok());
    assert!(matches!(
        classify_agent_save_chirho(&body_chirho(0, &surface_chirho.sample_page_chirho)),
        AgentSaveDecisionChirho::WriteChirho(_)
    ));
    let listed_chirho = surface_chirho.save_chirho.refusals_chirho;
    for refusal_chirho in AgentSaveRefusalChirho::ALL_CHIRHO {
        assert!(listed_chirho.iter().any(|entry_chirho| {
            entry_chirho.code_chirho == refusal_chirho.code_chirho()
                && !entry_chirho.meaning_chirho.is_empty()
        }));
    }
    assert_eq!(
        listed_chirho.len(),
        AgentSaveRefusalChirho::ALL_CHIRHO.len()
    );
    assert_eq!(
        surface_chirho.save_chirho.max_body_bytes_chirho,
        AGENT_SAVE_MAX_BODY_BYTES_CHIRHO
    );
}

#[test]
fn an_agent_save_is_the_very_write_her_browser_makes_chirho() {
    for expected_version_chirho in [0_u64, 3] {
        let AgentSaveDecisionChirho::WriteChirho(agent_write_chirho) =
            classify_agent_save_chirho(&body_chirho(expected_version_chirho, &page_chirho()))
        else {
            panic!("a complete page must become a write");
        };
        let SavePageDecisionChirho::WriteChirho(browser_write_chirho) =
            classify_page_edit_form_chirho(&browser_form_chirho(expected_version_chirho))
        else {
            panic!("the control form must become a write");
        };
        assert_eq!(
            agent_write_chirho, browser_write_chirho,
            "same page, same version: one write request, whichever road it came by"
        );
    }
}

#[test]
fn nothing_is_repaired_and_only_the_stated_normalization_happens_chirho() {
    // markup is text: kept as typed (the published template escapes it), never stripped
    let mut marked_up_chirho = page_chirho();
    marked_up_chirho.headline_chirho = "<b>Clear</b> help".to_string();
    // an oddly but validly written phone is hers: stored as given
    marked_up_chirho.contact_phone_chirho = "+1 (555) 0124".to_string();
    let AgentSaveDecisionChirho::WriteChirho(write_chirho) =
        classify_agent_save_chirho(&body_chirho(2, &marked_up_chirho))
    else {
        panic!("valid text must save");
    };
    let request_chirho: Value = serde_json::from_slice(&write_chirho).unwrap();
    let stored_chirho: PageDocumentV2Chirho = serde_json::from_slice(
        &BASE64_CHIRHO
            .decode(request_chirho["value_base64_chirho"].as_str().unwrap())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(stored_chirho.headline_chirho, "<b>Clear</b> help");
    assert_eq!(stored_chirho.contact_phone_chirho, "+1 (555) 0124");
    assert_eq!(request_chirho["expected_current_version_chirho"], json!(2));

    // the ONLY changes a save makes are the ones the surface states: trim, line endings, no gaps
    let mut untidy_chirho = page_chirho();
    untidy_chirho.summary_chirho = "  First line\r\nSecond line\rThird  ".to_string();
    untidy_chirho.services_chirho = [
        PageServiceV1Chirho::default(),
        PageServiceV1Chirho {
            title_chirho: "Repairs".to_string(),
            body_chirho: "Homes and small shops".to_string(),
        },
        PageServiceV1Chirho::default(),
    ];
    let AgentSaveDecisionChirho::WriteChirho(untidy_write_chirho) =
        classify_agent_save_chirho(&body_chirho(0, &untidy_chirho))
    else {
        panic!("untidy but valid text must save");
    };
    let untidy_request_chirho: Value = serde_json::from_slice(&untidy_write_chirho).unwrap();
    let tidied_chirho: PageDocumentV2Chirho = serde_json::from_slice(
        &BASE64_CHIRHO
            .decode(
                untidy_request_chirho["value_base64_chirho"]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        tidied_chirho.summary_chirho,
        "First line\nSecond line\nThird"
    );
    assert_eq!(tidied_chirho.services_chirho[0].title_chirho, "Repairs");
    assert_eq!(
        agent_surface_chirho()
            .page_document_chirho
            .normalization_chirho
            .len(),
        3,
        "each normalization above is stated to the agent"
    );

    // a phone that is not a phone is REFUSED by name, never dropped or rewritten
    let mut bad_phone_chirho = page_chirho();
    bad_phone_chirho.contact_phone_chirho = "call me maybe".to_string();
    let outcome_chirho = refusal_of_chirho(classify_agent_save_chirho(&body_chirho(
        0,
        &bad_phone_chirho,
    )));
    assert!(outcome_chirho.issues_chirho.iter().any(|issue_chirho| {
        issue_chirho.field_path_chirho == "contact_phone_chirho"
            && issue_chirho.code_chirho == PageValidationCodeChirho::InvalidPhoneChirho
    }));
}

#[test]
fn bodies_the_api_never_asked_for_are_refused_without_an_echo_chirho() {
    // bounded BEFORE parsing: a valid page padded past the bound is not even read
    let mut padded_chirho = body_chirho(0, &page_chirho());
    padded_chirho.extend(std::iter::repeat_n(b' ', AGENT_SAVE_MAX_BODY_BYTES_CHIRHO));
    assert_refused_chirho(&padded_chirho, AgentSaveRefusalChirho::OversizeChirho);

    assert_refused_chirho(
        format!("not json {MARKER_CHIRHO}").as_bytes(),
        AgentSaveRefusalChirho::UnreadableChirho,
    );
    // her browser's form is not this API's body
    assert_refused_chirho(
        &browser_form_chirho(0),
        AgentSaveRefusalChirho::UnreadableChirho,
    );
    // an unknown key beside the two, and an unknown key inside the page
    let mut extra_chirho: Value = serde_json::from_slice(&body_chirho(0, &page_chirho())).unwrap();
    extra_chirho["publish_chirho"] = json!(MARKER_CHIRHO);
    assert_refused_chirho(
        &serde_json::to_vec(&extra_chirho).unwrap(),
        AgentSaveRefusalChirho::UnreadableChirho,
    );
    let mut inner_chirho: Value = serde_json::from_slice(&body_chirho(0, &page_chirho())).unwrap();
    inner_chirho["page_chirho"]["slug_chirho"] = json!(MARKER_CHIRHO);
    assert_refused_chirho(
        &serde_json::to_vec(&inner_chirho).unwrap(),
        AgentSaveRefusalChirho::UnreadableChirho,
    );
    // a missing version is not "version 0"
    let missing_chirho = serde_json::to_vec(&json!({ "page_chirho": page_chirho() })).unwrap();
    assert_refused_chirho(&missing_chirho, AgentSaveRefusalChirho::UnreadableChirho);

    let mut old_schema_chirho = page_chirho();
    old_schema_chirho.schema_version_chirho = 1;
    old_schema_chirho.business_name_chirho = MARKER_CHIRHO.to_string();
    assert_refused_chirho(
        &body_chirho(0, &old_schema_chirho),
        AgentSaveRefusalChirho::UnsupportedSchemaChirho,
    );
}

#[test]
fn the_doors_answer_reads_as_saved_stale_or_unverifiable_chirho() {
    let saved_chirho =
        agent_save_outcome_from_door_chirho(&state_response_chirho(4, &page_chirho()));
    assert!(saved_chirho.saved_chirho);
    assert_eq!(saved_chirho.refusal_chirho, None);
    assert_eq!(saved_chirho.current_version_chirho, Some(4));
    assert_eq!(saved_chirho.page_chirho, Some(page_chirho()));

    let head_chirho = json!({
        "version_chirho": 5,
        "value_base64_chirho": BASE64_CHIRHO.encode(serde_json::to_vec(&page_chirho()).unwrap()),
    });
    let conflict_chirho = serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_CONFLICT_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "action_chirho": "write-chirho",
        "expected_current_version_chirho": 3,
        "current_version_chirho": 5,
        "current_chirho": {
            "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
            "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
            "current_chirho": head_chirho.clone(),
            "history_chirho": [head_chirho],
        },
    }))
    .unwrap();
    let stale_chirho = agent_save_outcome_from_door_chirho(&conflict_chirho);
    assert!(!stale_chirho.saved_chirho);
    assert_eq!(
        stale_chirho.refusal_chirho,
        Some(AgentSaveRefusalChirho::StaleVersionChirho.code_chirho())
    );
    assert_eq!(stale_chirho.current_version_chirho, Some(5));
    assert_eq!(stale_chirho.page_chirho, None);

    for unreadable_chirho in [&b"{}"[..], &b"not json"[..], &b""[..]] {
        let outcome_chirho = agent_save_outcome_from_door_chirho(unreadable_chirho);
        assert!(
            !outcome_chirho.saved_chirho,
            "an unverified answer is never 'saved'"
        );
        assert_eq!(
            outcome_chirho.refusal_chirho,
            Some(AgentSaveRefusalChirho::UnverifiableChirho.code_chirho())
        );
    }
}

#[test]
fn the_first_read_carries_her_page_its_version_and_the_surface_chirho() {
    let saved_chirho =
        AgentPageV1Chirho::from_state_response_chirho(&state_response_chirho(3, &page_chirho()));
    assert_eq!(saved_chirho.state_chirho, AgentPageStateChirho::SavedChirho);
    assert_eq!(saved_chirho.current_version_chirho, Some(3));
    assert_eq!(saved_chirho.page_chirho, Some(page_chirho()));
    // what she reads is what she may send back: read -> save round-trips to a write at her version
    let round_trip_chirho = body_chirho(3, saved_chirho.page_chirho.as_ref().unwrap());
    assert!(matches!(
        classify_agent_save_chirho(&round_trip_chirho),
        AgentSaveDecisionChirho::WriteChirho(_)
    ));

    let nothing_chirho = serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": null,
        "history_chirho": [],
    }))
    .unwrap();
    let empty_chirho = AgentPageV1Chirho::from_state_response_chirho(&nothing_chirho);
    assert_eq!(
        empty_chirho.state_chirho,
        AgentPageStateChirho::NothingSavedChirho
    );
    assert_eq!(empty_chirho.current_version_chirho, Some(0));
    assert_eq!(empty_chirho.page_chirho, None);

    let broken_chirho = AgentPageV1Chirho::from_state_response_chirho(b"not json");
    assert_eq!(
        broken_chirho.state_chirho,
        AgentPageStateChirho::UnverifiableChirho
    );
    assert_eq!(
        broken_chirho.current_version_chirho, None,
        "no version to save against when her page cannot be verified"
    );

    let answer_chirho: Value =
        serde_json::from_slice(&saved_chirho.to_json_bytes_chirho()).unwrap();
    assert_eq!(answer_chirho["state_chirho"], json!("saved-chirho"));
    assert!(answer_chirho["surface_chirho"]["routes_chirho"].is_array());
    assert!(answer_chirho["surface_chirho"]["sample_page_chirho"].is_object());
}

/// A real agent, given only the address and a key, had to guess what the codes meant, how many
/// services to send, what a refusal's null version means and how her page goes live (2026-09-21).
/// The description now explains itself; this pins that STRUCTURE, never a sentence.
#[test]
fn the_description_explains_every_code_it_can_answer_with_chirho() {
    let surface_chirho = agent_surface_chirho();
    let document_chirho = &surface_chirho.page_document_chirho;
    // every need a field uses has a meaning
    for field_chirho in &document_chirho.fields_chirho {
        let need_chirho = serde_json::to_value(field_chirho.need_chirho).unwrap();
        assert!(document_chirho.needs_chirho.iter().any(|entry_chirho| {
            json!(entry_chirho.code_chirho) == need_chirho
                && !entry_chirho.meaning_chirho.is_empty()
        }));
    }
    // every issue code the validator can name has a meaning, spelled as a refusal spells it
    assert_eq!(
        document_chirho.issue_codes_chirho.len(),
        AGENT_ISSUE_CODES_CHIRHO.len()
    );
    for code_chirho in AGENT_ISSUE_CODES_CHIRHO {
        let spelled_chirho = serde_json::to_value(code_chirho).unwrap();
        assert!(document_chirho
            .issue_codes_chirho
            .iter()
            .any(|entry_chirho| {
                json!(entry_chirho.code_chirho) == spelled_chirho
                    && !entry_chirho.meaning_chirho.is_empty()
            }));
    }
    // the shape facts an agent otherwise guesses
    assert_eq!(
        document_chirho.service_slots_chirho,
        surface_chirho.sample_page_chirho.services_chirho.len()
    );
    assert!(document_chirho.every_key_required_chirho);
    assert!(surface_chirho.save_chirho.replaces_whole_page_chirho);
    assert_eq!(
        surface_chirho
            .save_chirho
            .first_save_expected_version_chirho,
        0
    );
    assert_eq!(
        surface_chirho.save_chirho.http_status_of_this_answer_chirho,
        200
    );
    assert!(!surface_chirho
        .save_chirho
        .other_http_statuses_chirho
        .is_empty());
    // the answer's keys are described, and they ARE the answer's keys
    let answer_chirho: Value = serde_json::from_slice(
        &agent_save_outcome_from_door_chirho(&state_response_chirho(1, &page_chirho()))
            .to_json_bytes_chirho(),
    )
    .unwrap();
    let mut described_chirho: Vec<&str> = surface_chirho
        .save_chirho
        .answer_keys_chirho
        .iter()
        .map(|entry_chirho| entry_chirho.code_chirho.as_str())
        .collect();
    described_chirho.push("schema_chirho");
    described_chirho.sort_unstable();
    let mut actual_chirho: Vec<&str> = answer_chirho
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    actual_chirho.sort_unstable();
    assert_eq!(described_chirho, actual_chirho);
    // publishing is not offered, and the owner is pointed at where SHE does it
    assert!(surface_chirho
        .not_offered_chirho
        .iter()
        .any(|entry_chirho| {
            entry_chirho.code_chirho == "publishing"
                && entry_chirho.path_chirho
                    == Some("/app-chirho/perffection-workspace/publication-status-chirho")
        }));
    // a slot index reads the generic row; an unknown path has no limit
    assert_eq!(
        agent_field_limit_chirho("services_chirho[2].title_chirho"),
        agent_field_limit_chirho("services_chirho[0].title_chirho")
    );
    assert!(agent_field_limit_chirho("services_chirho[0].title_chirho").is_some());
    assert_eq!(agent_field_limit_chirho("slug_chirho"), None);
}
