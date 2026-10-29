// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her assistant's figures Save (F6c, gpt #26625): the complete document it read, one validator, one
//! versioned wire. The write request is the very one her browser's Save builds for the same figures;
//! every refusal has its code and reaches no write; exactly one branch is live; the door's answers read
//! as saved, stale or unknown. Behaviour, never wording.

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use perffection_workspace_chirho::figures_flow_chirho::agent_figures_chirho::AgentFiguresV1Chirho;
use perffection_workspace_chirho::figures_flow_chirho::agent_figures_sample_chirho::agent_sample_figures_chirho;
use perffection_workspace_chirho::figures_flow_chirho::agent_figures_save_chirho::{
    agent_figures_save_outcome_from_door_chirho, classify_agent_figures_save_chirho,
    AgentFiguresSaveDecisionChirho, AgentFiguresSaveRefusalChirho,
    AGENT_FIGURES_SAVE_MAX_BODY_BYTES_CHIRHO,
};
use perffection_workspace_chirho::figures_flow_chirho::figures_csv_chirho::import_figures_csv_chirho;
use perffection_workspace_chirho::figures_flow_chirho::figures_form_chirho::{
    classify_figures_save_chirho, FiguresSaveDecisionChirho,
};
use perffection_workspace_chirho::implementations_chirho::{
    build_agent_figures_write_chirho::build_agent_figures_write_chirho,
    render_agent_figures_refusal_chirho::render_agent_figures_refusal_chirho,
    render_agent_figures_saved_chirho::render_agent_figures_saved_chirho,
};
use perffection_workspace_chirho::viewer_state_chirho::figures_contract_chirho::FIGURES_LOGICAL_KEY_CHIRHO;
use serde_json::{json, Value};

const CSV_CHIRHO: &str =
    "month,kind,money_in,costs,jobs\n2026-01,Bread,1200.50,400,300\n2026-02,Bread,1300,,320\n";

/// Her figures as the read answers them: the stored document's own JSON.
fn figures_json_chirho() -> Value {
    let kept_chirho = import_figures_csv_chirho(CSV_CHIRHO, "USD", "from my books")
        .kept_chirho
        .unwrap();
    serde_json::to_value(kept_chirho.document_chirho()).unwrap()
}

fn body_chirho(version_chirho: u64, figures_chirho: Value) -> Vec<u8> {
    serde_json::to_vec(
        &json!({"expected_version_chirho": version_chirho, "figures_chirho": figures_chirho}),
    )
    .unwrap()
}

fn refusal_of_chirho(body_chirho: &[u8]) -> Value {
    match classify_agent_figures_save_chirho(body_chirho) {
        AgentFiguresSaveDecisionChirho::RefuseChirho(outcome_chirho) => {
            serde_json::from_slice(&outcome_chirho.to_json_bytes_chirho()).unwrap()
        }
        AgentFiguresSaveDecisionChirho::WriteChirho(_request_chirho) => {
            panic!("expected a refusal")
        }
    }
}

#[test]
fn her_assistants_save_is_the_very_write_her_browsers_save_builds_chirho() {
    let AgentFiguresSaveDecisionChirho::WriteChirho(agent_chirho) =
        classify_agent_figures_save_chirho(&body_chirho(3, figures_json_chirho()))
    else {
        panic!("expected a write");
    };
    let encode_chirho = |text_chirho: &str| {
        text_chirho
            .bytes()
            .map(|byte_chirho| format!("%{byte_chirho:02X}"))
            .collect::<String>()
    };
    let form_chirho = format!(
        "csv_chirho={}&currency_chirho=USD&note_chirho={}&expected_version_chirho=3",
        encode_chirho(CSV_CHIRHO),
        encode_chirho("from my books")
    );
    let FiguresSaveDecisionChirho::WriteChirho(browser_chirho) =
        classify_figures_save_chirho(form_chirho.as_bytes())
    else {
        panic!("expected the browser's write");
    };
    assert_eq!(agent_chirho, browser_chirho);
    let request_chirho: Value = serde_json::from_slice(&agent_chirho).unwrap();
    assert_eq!(
        (
            request_chirho["action_chirho"].as_str(),
            request_chirho["key_chirho"].as_str(),
            request_chirho["expected_current_version_chirho"].as_u64()
        ),
        (
            Some("write-chirho"),
            Some(FIGURES_LOGICAL_KEY_CHIRHO),
            Some(3)
        )
    );
    // 0 is "nothing saved yet": no precondition, the same convention as her browser
    let AgentFiguresSaveDecisionChirho::WriteChirho(first_chirho) =
        classify_agent_figures_save_chirho(&body_chirho(0, figures_json_chirho()))
    else {
        panic!("expected a write");
    };
    let first_chirho: Value = serde_json::from_slice(&first_chirho).unwrap();
    assert!(first_chirho
        .get("expected_current_version_chirho")
        .is_none());
}

#[test]
fn every_refusal_has_its_code_and_reaches_no_write_chirho() {
    let mut unknown_top_chirho: Value =
        serde_json::from_slice(&body_chirho(1, figures_json_chirho())).unwrap();
    unknown_top_chirho["publish_chirho"] = json!(true);
    let mut unknown_inner_chirho = figures_json_chirho();
    unknown_inner_chirho["owner_chirho"] = json!("someone else");
    let mut other_schema_chirho = figures_json_chirho();
    other_schema_chirho["schema_chirho"] = json!("perffection-figures-v9-chirho");
    let mut bad_currency_chirho = figures_json_chirho();
    bad_currency_chirho["currency_chirho"] = json!("XYZ");
    let mut empty_month_chirho = figures_json_chirho();
    empty_month_chirho["money_in_chirho"][0][1] = Value::Null;
    let cases_chirho: Vec<(&str, Vec<u8>, &str, Vec<&str>)> = vec![
        (
            "oversize",
            vec![b' '; AGENT_FIGURES_SAVE_MAX_BODY_BYTES_CHIRHO + 1],
            "oversize-chirho",
            vec![],
        ),
        (
            "not JSON",
            b"not json".to_vec(),
            "unreadable-chirho",
            vec![],
        ),
        (
            "an unknown key beside the figures",
            serde_json::to_vec(&unknown_top_chirho).unwrap(),
            "unreadable-chirho",
            vec![],
        ),
        (
            "an unknown key inside the figures",
            body_chirho(1, unknown_inner_chirho),
            "unreadable-chirho",
            vec![],
        ),
        (
            "no version",
            serde_json::to_vec(&json!({"figures_chirho": figures_json_chirho()})).unwrap(),
            "unreadable-chirho",
            vec![],
        ),
        (
            "another schema",
            body_chirho(1, other_schema_chirho),
            "unsupported-schema-chirho",
            vec![],
        ),
        (
            "an unsupported currency",
            body_chirho(1, bad_currency_chirho),
            "invalid-figures-chirho",
            vec!["unsupported-currency-chirho"],
        ),
        (
            "a month left with no money in",
            body_chirho(1, empty_month_chirho),
            "invalid-figures-chirho",
            vec!["month-without-figures-chirho"],
        ),
    ];
    let skipped_chirho = cairn_node_sdk_chirho::skipped_bytes_chirho();
    for (label_chirho, request_chirho, code_chirho, issues_chirho) in cases_chirho {
        let refusal_chirho = refusal_of_chirho(&request_chirho);
        assert_eq!(
            refusal_chirho["saved_chirho"],
            json!(false),
            "{label_chirho}"
        );
        assert_eq!(
            refusal_chirho["refusal_chirho"],
            json!(code_chirho),
            "{label_chirho}"
        );
        assert_eq!(
            refusal_chirho["issues_chirho"],
            json!(issues_chirho),
            "{label_chirho}"
        );
        assert!(
            refusal_chirho["current_version_chirho"].is_null(),
            "{label_chirho}"
        );
        assert!(refusal_chirho["figures_chirho"].is_null(), "{label_chirho}");
        // exactly one branch is live: the write branch skips, the refusal answers
        assert_eq!(
            build_agent_figures_write_chirho(&request_chirho),
            skipped_chirho,
            "{label_chirho}"
        );
        assert_ne!(
            render_agent_figures_refusal_chirho(&request_chirho),
            skipped_chirho,
            "{label_chirho}"
        );
    }
    // and for a good body the other way round
    let good_chirho = body_chirho(1, figures_json_chirho());
    assert_ne!(
        build_agent_figures_write_chirho(&good_chirho),
        skipped_chirho
    );
    assert_eq!(
        render_agent_figures_refusal_chirho(&good_chirho),
        skipped_chirho
    );
}

fn stored_value_chirho() -> String {
    let kept_chirho = import_figures_csv_chirho(CSV_CHIRHO, "USD", "from my books")
        .kept_chirho
        .unwrap();
    BASE64_CHIRHO.encode(kept_chirho.stored_bytes_chirho())
}

#[test]
fn the_doors_answer_reads_as_saved_stale_or_unknown_chirho() {
    let current_chirho = json!({"version_chirho": 4, "value_base64_chirho": stored_value_chirho()});
    let state_chirho = serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "current_chirho": current_chirho, "history_chirho": [current_chirho]})).unwrap();
    let saved_chirho: Value =
        serde_json::from_slice(&render_agent_figures_saved_chirho(&state_chirho)).unwrap();
    assert_eq!(
        (
            saved_chirho["saved_chirho"].as_bool(),
            saved_chirho["current_version_chirho"].as_u64(),
            saved_chirho["figures_chirho"]["kinds_chirho"][0].as_str(),
            saved_chirho["report_chirho"]["version_chirho"].as_u64()
        ),
        (Some(true), Some(4), Some("Bread"), Some(4))
    );
    let head_chirho: Value = serde_json::from_slice(&state_chirho).unwrap();
    let conflict_chirho = serde_json::to_vec(&json!({"schema_chirho": "viewer-state-conflict-v1-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "expected_current_version_chirho": 3, "current_version_chirho": 4, "current_chirho": head_chirho, "attempted_value_base64_chirho": stored_value_chirho()})).unwrap();
    let stale_chirho = agent_figures_save_outcome_from_door_chirho(&conflict_chirho);
    assert_eq!(
        (
            stale_chirho.saved_chirho,
            stale_chirho.refusal_chirho,
            stale_chirho.current_version_chirho,
            stale_chirho.figures_chirho.is_none()
        ),
        (false, Some("stale-version-chirho"), Some(4), true)
    );
    // after a write, an answer that is not a verified state, or a state saying nothing is saved, is unknown
    let nothing_chirho = serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "current_chirho": null, "history_chirho": []})).unwrap();
    for answer_chirho in [b"garbage".to_vec(), nothing_chirho] {
        let unknown_chirho = agent_figures_save_outcome_from_door_chirho(&answer_chirho);
        assert_eq!(
            (
                unknown_chirho.saved_chirho,
                unknown_chirho.refusal_chirho,
                unknown_chirho.current_version_chirho
            ),
            (false, Some("unverifiable-chirho"), None)
        );
    }
}

#[test]
fn her_assistant_starts_from_an_illustration_never_from_her_figures_chirho() {
    let sample_chirho = agent_sample_figures_chirho();
    assert!(sample_chirho.clone().validate_chirho().is_ok());
    let sample_json_chirho = serde_json::to_value(&sample_chirho).unwrap();
    // the illustration itself is refused against any version, and no write branch is live
    let skipped_chirho = cairn_node_sdk_chirho::skipped_bytes_chirho();
    for version_chirho in [0, 1, 7] {
        let request_chirho = body_chirho(version_chirho, sample_json_chirho.clone());
        let refusal_chirho = refusal_of_chirho(&request_chirho);
        assert_eq!(
            (
                refusal_chirho["saved_chirho"].clone(),
                refusal_chirho["refusal_chirho"].clone(),
                refusal_chirho["current_version_chirho"].clone(),
                refusal_chirho["issues_chirho"].clone()
            ),
            (
                json!(false),
                json!("sample-not-her-figures-chirho"),
                Value::Null,
                json!([])
            )
        );
        assert_eq!(
            build_agent_figures_write_chirho(&request_chirho),
            skipped_chirho
        );
        assert_ne!(
            render_agent_figures_refusal_chirho(&request_chirho),
            skipped_chirho
        );
    }
    // her own figures in the sample's shape are a write: one changed value is enough
    let mut hers_chirho = sample_json_chirho.clone();
    hers_chirho["kinds_chirho"][0] = json!("Bookbinding");
    assert!(matches!(
        classify_agent_figures_save_chirho(&body_chirho(0, hers_chirho)),
        AgentFiguresSaveDecisionChirho::WriteChirho(_)
    ));
    // the read carries the start only while nothing is saved; a saved or unverifiable answer is as before
    let read_chirho = |response_chirho: &[u8]| -> Value {
        serde_json::from_slice(
            &AgentFiguresV1Chirho::from_state_response_chirho(response_chirho)
                .to_json_bytes_chirho(),
        )
        .unwrap()
    };
    let nothing_chirho = read_chirho(&serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "current_chirho": null, "history_chirho": []})).unwrap());
    let start_chirho = &nothing_chirho["start_chirho"];
    assert_eq!(
        (
            nothing_chirho["state_chirho"].as_str(),
            start_chirho["first_save_expected_version_chirho"].as_u64(),
            &start_chirho["sample_figures_chirho"]
        ),
        (Some("nothing-saved-chirho"), Some(0), &sample_json_chirho)
    );
    let codes_chirho: Vec<&str> = start_chirho["refusals_chirho"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|refusal_chirho| refusal_chirho["code_chirho"].as_str())
        .collect();
    let expected_codes_chirho: Vec<&str> = AgentFiguresSaveRefusalChirho::ALL_CHIRHO
        .iter()
        .map(|refusal_chirho| refusal_chirho.code_chirho())
        .collect();
    assert_eq!(codes_chirho, expected_codes_chirho);
    let current_chirho = json!({"version_chirho": 2, "value_base64_chirho": stored_value_chirho()});
    let saved_chirho = read_chirho(&serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "current_chirho": current_chirho, "history_chirho": [current_chirho]})).unwrap());
    let unverifiable_chirho = read_chirho(b"garbage");
    assert_eq!(
        (
            saved_chirho["state_chirho"].as_str(),
            saved_chirho.get("start_chirho").is_none(),
            unverifiable_chirho["state_chirho"].as_str(),
            unverifiable_chirho.get("start_chirho").is_none()
        ),
        (
            Some("saved-chirho"),
            true,
            Some("unverifiable-chirho"),
            true
        )
    );
}
