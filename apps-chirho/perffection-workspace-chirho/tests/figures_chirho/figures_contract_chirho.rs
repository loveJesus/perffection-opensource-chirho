// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures through the viewer-state door: their own logical key, the exact validated bytes on
//! the wire, a verified state and a conflict that hands her attempted figures back. A page answer
//! and a figures answer never cross.

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use perffection_workspace_chirho::figures_flow_chirho::figures_csv_chirho::import_figures_csv_chirho;
use perffection_workspace_chirho::figures_flow_chirho::figures_document_chirho::ValidatedFiguresChirho;
use perffection_workspace_chirho::viewer_state_chirho::figures_contract_chirho::{
    build_figures_read_request_chirho, build_figures_write_request_chirho,
    decode_figures_outcome_chirho, decode_figures_state_chirho, FiguresOutcomeChirho,
    FIGURES_LOGICAL_KEY_CHIRHO,
};
use perffection_workspace_chirho::viewer_state_contract_chirho::{
    decode_page_state_response_chirho, ViewerStateContractErrorChirho,
    PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
};
use serde_json::{json, Value};

fn figures_chirho(rows_chirho: &str) -> ValidatedFiguresChirho {
    import_figures_csv_chirho(
        &format!("month,kind,money_in,costs,jobs\n{rows_chirho}"),
        "USD",
        "",
    )
    .kept_chirho
    .unwrap()
}

fn state_chirho(key_chirho: &str, values_chirho: &[(u64, &[u8])]) -> Vec<u8> {
    let history_chirho: Vec<Value> = values_chirho.iter().map(|(version_chirho, bytes_chirho)| json!({"version_chirho": version_chirho, "value_base64_chirho": BASE64_CHIRHO.encode(bytes_chirho)})).collect();
    serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": key_chirho, "current_chirho": history_chirho[0], "history_chirho": history_chirho})).unwrap()
}

#[test]
fn the_requests_name_only_the_figures_key_and_carry_the_measured_bytes_chirho() {
    assert_eq!(
        build_figures_read_request_chirho().unwrap(),
        br#"{"schema_chirho":"viewer-state-request-v1-chirho","action_chirho":"read-chirho","key_chirho":"business-figures-chirho"}"#
    );
    let validated_chirho = figures_chirho("2026-01,Bread,100,40,10\n");
    let request_chirho: Value = serde_json::from_slice(
        &build_figures_write_request_chirho(&validated_chirho, Some(3)).unwrap(),
    )
    .unwrap();
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
    assert_eq!(
        BASE64_CHIRHO
            .decode(request_chirho["value_base64_chirho"].as_str().unwrap())
            .unwrap(),
        validated_chirho.stored_bytes_chirho()
    );
    let first_chirho: Value = serde_json::from_slice(
        &build_figures_write_request_chirho(&validated_chirho, None).unwrap(),
    )
    .unwrap();
    assert!(first_chirho
        .get("expected_current_version_chirho")
        .is_none());
    assert_eq!(
        build_figures_write_request_chirho(&validated_chirho, Some(0)),
        Err(ViewerStateContractErrorChirho::InvalidExpectedVersionChirho)
    );
}

#[test]
fn a_verified_state_gives_the_head_and_the_kept_versions_chirho() {
    let (older_chirho, newer_chirho) = (
        figures_chirho("2026-01,Bread,100,40,10\n"),
        figures_chirho("2026-01,Bread,150,40,10\n"),
    );
    let state_bytes_chirho = state_chirho(
        FIGURES_LOGICAL_KEY_CHIRHO,
        &[
            (2, newer_chirho.stored_bytes_chirho()),
            (1, older_chirho.stored_bytes_chirho()),
        ],
    );
    let state_chirho = decode_figures_state_chirho(&state_bytes_chirho)
        .unwrap()
        .unwrap();
    assert_eq!(
        (
            state_chirho.current_version_chirho,
            state_chirho.kept_versions_chirho.clone()
        ),
        (2, vec![2, 1])
    );
    assert_eq!(&state_chirho.figures_chirho, newer_chirho.document_chirho());
    let empty_chirho = serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "history_chirho": []})).unwrap();
    assert_eq!(decode_figures_state_chirho(&empty_chirho), Ok(None));
}

#[test]
fn a_stored_value_that_is_not_valid_figures_is_refused_by_name_chirho() {
    let out_of_order_chirho = br#"{"schema_chirho":"perffection-figures-v1-chirho","currency_chirho":"USD","note_chirho":"","months_chirho":["2026-02","2026-01"],"kinds_chirho":["Bread"],"money_in_chirho":[[1,1]],"costs_chirho":[[null,null]],"jobs_chirho":[[null,null]]}"#;
    assert_eq!(
        decode_figures_state_chirho(&state_chirho(
            FIGURES_LOGICAL_KEY_CHIRHO,
            &[(1, out_of_order_chirho)]
        )),
        Err(ViewerStateContractErrorChirho::InvalidFiguresChirho)
    );
    assert_eq!(
        decode_figures_state_chirho(&state_chirho(
            FIGURES_LOGICAL_KEY_CHIRHO,
            &[(1, b"not json")]
        )),
        Err(ViewerStateContractErrorChirho::MalformedFiguresChirho)
    );
}

#[test]
fn a_page_answer_and_a_figures_answer_never_cross_chirho() {
    let figures_bytes_chirho = figures_chirho("2026-01,Bread,100,40,10\n");
    let figures_state_chirho = state_chirho(
        FIGURES_LOGICAL_KEY_CHIRHO,
        &[(1, figures_bytes_chirho.stored_bytes_chirho())],
    );
    assert_eq!(
        decode_page_state_response_chirho(&figures_state_chirho),
        Err(ViewerStateContractErrorChirho::WrongLogicalKeyChirho)
    );
    let page_state_chirho = state_chirho(
        PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        &[(1, figures_bytes_chirho.stored_bytes_chirho())],
    );
    assert_eq!(
        decode_figures_state_chirho(&page_state_chirho),
        Err(ViewerStateContractErrorChirho::WrongLogicalKeyChirho)
    );
}

#[test]
fn a_conflict_keeps_the_head_and_hands_her_attempted_figures_back_chirho() {
    let (head_chirho, attempted_chirho) = (
        figures_chirho("2026-01,Bread,100,40,10\n"),
        figures_chirho("2026-01,Bread,999,40,10\n"),
    );
    let head_state_chirho: Value = serde_json::from_slice(&state_chirho(
        FIGURES_LOGICAL_KEY_CHIRHO,
        &[(3, head_chirho.stored_bytes_chirho())],
    ))
    .unwrap();
    let conflict_chirho = serde_json::to_vec(&json!({
        "schema_chirho": "viewer-state-conflict-v1-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO,
        "expected_current_version_chirho": 2, "current_version_chirho": 3, "current_chirho": head_state_chirho,
        "attempted_value_base64_chirho": BASE64_CHIRHO.encode(attempted_chirho.stored_bytes_chirho()),
    }))
    .unwrap();
    let FiguresOutcomeChirho::ConflictChirho(conflict_chirho) =
        decode_figures_outcome_chirho(&conflict_chirho).unwrap()
    else {
        panic!("a stale save is a conflict");
    };
    assert_eq!(
        (
            conflict_chirho.expected_current_version_chirho,
            conflict_chirho.head_chirho.current_version_chirho
        ),
        (Some(2), 3)
    );
    assert_eq!(
        conflict_chirho.attempted_chirho.as_ref(),
        Some(attempted_chirho.document_chirho())
    );
    assert_eq!(
        &conflict_chirho.head_chirho.figures_chirho,
        head_chirho.document_chirho()
    );
}
