// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her AI-assistant connections on the Account page. Held until the engine's list read is bound: no
//! control at all. An unavailable list is never shown as an empty one. A disconnect form exists only
//! for a live connection, only while the disconnect road is bound, and posts her opaque key id and the
//! way back. The list read's answer is typed, and anything unexpected reads as unavailable.

use perffection_ui_chirho::{UiLocaleChirho, WorkspaceCopyChirho};
use perffection_workspace_chirho::account_page_chirho::{
    AccountBindingV1Chirho, AccountPageV1Chirho,
};
use perffection_workspace_chirho::connections_chirho::{
    connect_entry_href_chirho, KEY_REVOKE_ACTION_CHIRHO, OWN_KEYS_READ_REQUEST_CHIRHO,
};
use perffection_workspace_chirho::connections_chirho::{
    connections_state_from_read_chirho, utc_date_chirho, ConnectionRowChirho,
    ConnectionsStateChirho, CONNECTION_ENDED_TOKEN_CHIRHO, DISCONNECT_REFUSAL_TOKENS_CHIRHO,
};
use perffection_workspace_chirho::implementations_chirho::build_own_keys_read_chirho::build_own_keys_read_chirho;
use perffection_workspace_chirho::implementations_chirho::render_account_chirho::render_account_chirho;
use perffection_workspace_chirho::implementations_chirho::render_account_connect_chirho::render_account_connect_chirho;
use perffection_workspace_chirho::implementations_chirho::render_account_keys_chirho::render_account_keys_chirho;
use perffection_workspace_chirho::page_render_chirho::render_account_chirho as render_model_chirho;
use perffection_workspace_chirho::workspace_page_chirho::WORKSPACE_ACCOUNT_HREF_CHIRHO;
use serde_json::{json, Value};

const DISCONNECT_CHIRHO: &str = "/auth-chirho/key-revoke-chirho";

fn row_chirho(key_id_chirho: &str, live_chirho: bool) -> ConnectionRowChirho {
    ConnectionRowChirho {
        key_id_chirho: key_id_chirho.to_string(),
        app_chirho: "perffection-workspace".to_string(),
        scopes_chirho: vec!["page-draft-chirho".to_string()],
        issued_ms_chirho: 1_791_501_940_488,
        expires_ms_chirho: 1_791_588_340_488,
        live_chirho,
    }
}

fn page_chirho(
    binding_chirho: AccountBindingV1Chirho,
    state_chirho: ConnectionsStateChirho,
) -> String {
    render_model_chirho(&AccountPageV1Chirho::with_connections_chirho(
        binding_chirho,
        state_chirho,
        UiLocaleChirho::EnglishChirho,
    ))
    .unwrap()
}

fn disconnect_forms_chirho(html_chirho: &str) -> usize {
    html_chirho
        .matches(&format!("action=\"{DISCONNECT_CHIRHO}\""))
        .count()
}

#[test]
fn the_deployed_page_holds_connections_and_offers_no_control_for_them_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let html_chirho = String::from_utf8(render_account_chirho(b"ignored request bytes")).unwrap();
    assert!(html_chirho.contains(&copy_chirho.connections_chirho.heading_chirho));
    assert!(html_chirho.contains(&copy_chirho.connections_chirho.held_chirho));
    // still only the password form: no disconnect, no create, no key id anywhere
    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert_eq!(disconnect_forms_chirho(&html_chirho), 0);
    assert!(!html_chirho.contains("key_id_chirho"));
    assert!(!html_chirho.contains(CONNECTION_ENDED_TOKEN_CHIRHO));
    for token_chirho in ["no-such-key-chirho", "keys-failed-chirho"] {
        assert!(!html_chirho.contains(&format!("refusal-{token_chirho}")));
    }
}

#[test]
fn an_unavailable_list_is_never_shown_as_an_empty_one_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let binding_chirho =
        AccountBindingV1Chirho::held_chirho().with_disconnect_action_chirho(DISCONNECT_CHIRHO);
    let html_chirho = page_chirho(binding_chirho, ConnectionsStateChirho::UnavailableChirho);
    assert!(html_chirho.contains(&copy_chirho.connections_chirho.unavailable_chirho));
    assert!(!html_chirho.contains(&copy_chirho.connections_chirho.empty_chirho));
    assert!(!html_chirho.contains(&copy_chirho.connections_chirho.held_chirho));
    // nothing to disconnect, so a bound road offers nothing: no form, no notice, no refusals
    assert_eq!(disconnect_forms_chirho(&html_chirho), 0);
    assert!(!html_chirho.contains(CONNECTION_ENDED_TOKEN_CHIRHO));
    assert!(!html_chirho.contains("refusal-no-such-key-chirho"));
}

#[test]
fn an_available_empty_list_says_none_and_is_not_unavailable_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let binding_chirho =
        AccountBindingV1Chirho::held_chirho().with_disconnect_action_chirho(DISCONNECT_CHIRHO);
    let html_chirho = page_chirho(
        binding_chirho,
        ConnectionsStateChirho::ListedChirho(Vec::new()),
    );
    assert!(html_chirho.contains(&copy_chirho.connections_chirho.empty_chirho));
    assert!(!html_chirho.contains(&copy_chirho.connections_chirho.unavailable_chirho));
    assert_eq!(disconnect_forms_chirho(&html_chirho), 0);
}

#[test]
fn each_live_connection_and_only_a_live_one_can_be_disconnected_chirho() {
    let binding_chirho =
        AccountBindingV1Chirho::held_chirho().with_disconnect_action_chirho(DISCONNECT_CHIRHO);
    let rows_chirho = vec![
        row_chirho("0a1b2c", true),
        row_chirho("3d4e5f", false),
        row_chirho("6a7b8c", true),
    ];
    let html_chirho = page_chirho(
        binding_chirho,
        ConnectionsStateChirho::ListedChirho(rows_chirho),
    );
    assert_eq!(disconnect_forms_chirho(&html_chirho), 2);
    assert_eq!(html_chirho.matches("name=\"key_id_chirho\"").count(), 2);
    assert!(html_chirho.contains("name=\"key_id_chirho\" value=\"0a1b2c\""));
    assert!(html_chirho.contains("name=\"key_id_chirho\" value=\"6a7b8c\""));
    // the ended one is listed, with no form naming it
    assert!(!html_chirho.contains("value=\"3d4e5f\""));
    // each form comes back to this page, where the road's fragments are waiting
    assert_eq!(
        html_chirho
            .matches(&format!(
                "name=\"redirect_chirho\" value=\"{WORKSPACE_ACCOUNT_HREF_CHIRHO}\""
            ))
            .count(),
        2
    );
    assert_eq!(
        html_chirho
            .matches(&format!("id=\"{CONNECTION_ENDED_TOKEN_CHIRHO}\""))
            .count(),
        1
    );
    for token_chirho in DISCONNECT_REFUSAL_TOKENS_CHIRHO {
        assert_eq!(
            html_chirho
                .matches(&format!("id=\"refusal-{token_chirho}\""))
                .count(),
            1,
            "{token_chirho}"
        );
    }
}

#[test]
fn a_list_without_the_disconnect_road_shows_rows_and_offers_no_form_chirho() {
    let html_chirho = page_chirho(
        AccountBindingV1Chirho::held_chirho(),
        ConnectionsStateChirho::ListedChirho(vec![row_chirho("0a1b2c", true)]),
    );
    assert!(html_chirho.contains(&utc_date_chirho(1_791_501_940_488)));
    assert_eq!(disconnect_forms_chirho(&html_chirho), 0);
    assert!(!html_chirho.contains("key_id_chirho"));
    assert!(!html_chirho.contains(CONNECTION_ENDED_TOKEN_CHIRHO));
}

#[test]
fn a_refusal_two_roads_share_renders_once_chirho() {
    let binding_chirho = AccountBindingV1Chirho::held_chirho()
        .with_change_password_action_chirho("/auth-chirho/credential-change-chirho")
        .with_disconnect_action_chirho(DISCONNECT_CHIRHO);
    let html_chirho = page_chirho(
        binding_chirho,
        ConnectionsStateChirho::ListedChirho(vec![row_chirho("0a1b2c", true)]),
    );
    assert_eq!(
        html_chirho
            .matches("id=\"refusal-request-invalid-chirho\"")
            .count(),
        1
    );
    assert_eq!(html_chirho.matches("<form").count(), 2);
}

#[test]
fn the_list_read_is_typed_and_anything_else_reads_unavailable_chirho() {
    let available_chirho = br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"available-chirho","keys_chirho":[
        {"key_id_chirho":"0a1b2c","app_chirho":"perffection-workspace","scopes_chirho":["page-draft-chirho"],"issued_ms_chirho":1,"expires_ms_chirho":2,"rate_limit_chirho":null,"live_chirho":true},
        {"key_id_chirho":"3d4e5f","app_chirho":"perffection-workspace","scopes_chirho":[],"issued_ms_chirho":3,"expires_ms_chirho":4,"rate_limit_chirho":{"window_ms_chirho":3600000,"max_invokes_chirho":400},"live_chirho":false}]}"#;
    let ConnectionsStateChirho::ListedChirho(rows_chirho) =
        connections_state_from_read_chirho(available_chirho)
    else {
        panic!("an available answer lists her rows");
    };
    assert_eq!(
        rows_chirho
            .iter()
            .map(|row_chirho| (row_chirho.key_id_chirho.as_str(), row_chirho.live_chirho))
            .collect::<Vec<_>>(),
        [("0a1b2c", true), ("3d4e5f", false)]
    );
    assert_eq!(
        connections_state_from_read_chirho(br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"available-chirho","keys_chirho":[]}"#),
        ConnectionsStateChirho::ListedChirho(Vec::new())
    );
    for unavailable_chirho in [
        &br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"unavailable-chirho","reason_chirho":"keys-unavailable-chirho"}"#[..],
        br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"unavailable-chirho","keys_chirho":[]}"#,
        br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"available-chirho"}"#,
        br#"{"schema_chirho":"own-keys-read-v2-chirho","state_chirho":"available-chirho","keys_chirho":[]}"#,
        br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"available-chirho","keys_chirho":[{"key_id_chirho":"0a1b2c"}]}"#,
        b"not json",
        b"",
    ] {
        assert_eq!(connections_state_from_read_chirho(unavailable_chirho), ConnectionsStateChirho::UnavailableChirho, "{}", String::from_utf8_lossy(unavailable_chirho));
    }
}

#[test]
fn dates_are_utc_calendar_days_chirho() {
    assert_eq!(utc_date_chirho(0), "1970-01-01");
    assert_eq!(utc_date_chirho(951_868_800_000), "2000-03-01");
    assert_eq!(utc_date_chirho(1_709_164_800_000), "2024-02-29");
    assert_eq!(utc_date_chirho(1_709_251_199_999), "2024-02-29");
    assert_eq!(utc_date_chirho(1_791_501_940_488), "2026-10-08");
}

#[test]
fn the_section_takes_the_next_number_after_those_shown_above_it_chirho() {
    let deployed_chirho = String::from_utf8(render_account_chirho(b"")).unwrap();
    assert!(deployed_chirho.contains("<p>02</p>"));
    assert!(!deployed_chirho.contains("<p>04</p>"));
    let all_bound_chirho = page_chirho(
        AccountBindingV1Chirho::held_chirho()
            .with_change_password_action_chirho("/auth-chirho/credential-change-chirho")
            .with_recovery_code_action_chirho("/auth-chirho/recovery-code-chirho")
            .with_close_account_action_chirho("/auth-chirho/delete-account-chirho"),
        ConnectionsStateChirho::HeldChirho,
    );
    assert!(all_bound_chirho.contains("<p>04</p>"));
}

// ---- the bound page (dag account_keys_chirho): her list from the engine's own-keys read (Cairn K3) ----

fn own_keys_answer_chirho(rows_chirho: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({"schema_chirho": "own-keys-read-v1-chirho", "state_chirho": "available-chirho", "keys_chirho": rows_chirho})).unwrap()
}

fn engine_row_chirho(
    key_id_chirho: &str,
    app_chirho: &str,
    scopes_chirho: &[&str],
    live_chirho: bool,
) -> Value {
    json!({"key_id_chirho": key_id_chirho, "app_chirho": app_chirho, "scopes_chirho": scopes_chirho, "issued_ms_chirho": 1_791_501_940_488u64, "expires_ms_chirho": 1_791_588_340_488u64, "rate_limit_chirho": null, "live_chirho": live_chirho})
}

fn bound_page_chirho(answer_chirho: &[u8]) -> String {
    String::from_utf8(render_account_keys_chirho(answer_chirho)).unwrap()
}

/// The list itself: from its label to its end, so a word elsewhere on the page never counts.
fn list_region_chirho(html_chirho: &str) -> &str {
    let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
    let start_chirho = html_chirho
        .find(&format!(
            "aria-label=\"{}\"",
            copy_chirho.connections_chirho.list_label_chirho
        ))
        .expect("the list is shown");
    let end_chirho = start_chirho + html_chirho[start_chirho..].find("</ul>").unwrap();
    &html_chirho[start_chirho..end_chirho]
}

#[test]
fn the_bound_page_reads_her_list_through_one_fixed_request_chirho() {
    let request_chirho =
        build_own_keys_read_chirho(br#"{"user_principal_chirho":"user/someone-else"}"#);
    assert_eq!(request_chirho, OWN_KEYS_READ_REQUEST_CHIRHO.as_bytes());
    assert_eq!(
        serde_json::from_slice::<Value>(&request_chirho).unwrap(),
        json!({"schema_chirho": "own-keys-read-request-v1-chirho"})
    );
}

#[test]
fn each_row_names_its_app_and_what_it_may_do_and_never_a_key_chirho() {
    let copy_chirho =
        WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho).connections_chirho;
    let mut limited_chirho = engine_row_chirho(
        "0a1b2c",
        "perffection-workspace",
        &["figures-read-chirho", "figures-save-chirho"],
        true,
    );
    // a field the engine might add beside a row is never shown, and no key ever is
    limited_chirho["token_chirho"] = json!("planted-secret-0123");
    let answer_chirho = own_keys_answer_chirho(json!([
        limited_chirho,
        engine_row_chirho(
            "3d4e5f",
            "another-app-chirho",
            &["future-thing-chirho"],
            false
        ),
    ]));
    let html_chirho = bound_page_chirho(&answer_chirho);
    let list_chirho = list_region_chirho(&html_chirho);
    assert!(list_chirho.contains(&copy_chirho.app_labels_chirho["perffection-workspace"]));
    for action_chirho in ["figures-read-chirho", "figures-save-chirho"] {
        assert!(
            list_chirho.contains(&copy_chirho.action_labels_chirho[action_chirho]),
            "{action_chirho}"
        );
    }
    // an app or a permission the page has no words for is shown by the engine's own name, never hidden
    assert!(list_chirho.contains("another-app-chirho"));
    assert!(list_chirho.contains(&format!(
        "{} future-thing-chirho",
        copy_chirho.unknown_action_label_chirho
    )));
    assert!(!html_chirho.contains("planted-secret-0123"));
}

#[test]
fn the_bound_page_offers_a_disconnect_per_live_row_and_no_other_control_chirho() {
    assert_eq!(KEY_REVOKE_ACTION_CHIRHO, DISCONNECT_CHIRHO);
    let answer_chirho = own_keys_answer_chirho(json!([
        engine_row_chirho(
            "0a1b2c",
            "perffection-workspace",
            &["page-read-chirho"],
            true
        ),
        engine_row_chirho(
            "3d4e5f",
            "perffection-workspace",
            &["page-draft-chirho"],
            false
        ),
        engine_row_chirho(
            "6a7b8c",
            "perffection-workspace",
            &["figures-read-chirho"],
            true
        ),
    ]));
    let html_chirho = bound_page_chirho(&answer_chirho);
    assert_eq!(disconnect_forms_chirho(&html_chirho), 2);
    assert!(html_chirho.contains("name=\"key_id_chirho\" value=\"0a1b2c\""));
    assert!(html_chirho.contains("name=\"key_id_chirho\" value=\"6a7b8c\""));
    assert!(!html_chirho.contains("value=\"3d4e5f\""));
    // no control creates a connection: the page's forms are her password form and the two Disconnects,
    // and every one of them returns her to this page
    let forms_chirho = html_chirho.matches("<form").count();
    let returns_chirho = html_chirho
        .matches(&format!(
            "name=\"redirect_chirho\" value=\"{WORKSPACE_ACCOUNT_HREF_CHIRHO}\""
        ))
        .count();
    assert_eq!((forms_chirho, returns_chirho), (1 + 2, 1 + 2));
}

#[test]
fn the_bound_page_shows_an_unavailable_or_unreadable_list_as_unavailable_never_none_chirho() {
    let copy_chirho =
        WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho).connections_chirho;
    let unavailable_answers_chirho: [&[u8]; 4] = [
        br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"unavailable-chirho","reason_chirho":"keys-unavailable-chirho"}"#,
        br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"available-chirho"}"#,
        b"the read failed",
        b"",
    ];
    for answer_chirho in unavailable_answers_chirho {
        let html_chirho = bound_page_chirho(answer_chirho);
        let label_chirho = String::from_utf8_lossy(answer_chirho);
        assert!(
            html_chirho.contains(&copy_chirho.unavailable_chirho),
            "{label_chirho}"
        );
        assert!(
            !html_chirho.contains(&copy_chirho.empty_chirho),
            "{label_chirho}"
        );
        assert_eq!(
            (
                disconnect_forms_chirho(&html_chirho),
                html_chirho.matches("<form").count()
            ),
            (0, 1),
            "{label_chirho}"
        );
    }
    // an available empty list says none, and is not unavailable
    let html_chirho = bound_page_chirho(&own_keys_answer_chirho(json!([])));
    assert!(html_chirho.contains(&copy_chirho.empty_chirho));
    assert!(!html_chirho.contains(&copy_chirho.unavailable_chirho));
}

// ---- the Create entry (Cairn K4; gpt #26824, #26825): a separately bound page, a link to the engine's own form ----

const CONNECTION_FORM_CHIRHO: &str = "/auth-chirho/connection-new-chirho";

fn connect_page_chirho(answer_chirho: &[u8]) -> String {
    String::from_utf8(render_account_connect_chirho(answer_chirho)).unwrap()
}

/// Every link to the engine's connection form on a page, its href as a browser reads it (entities decoded).
fn connection_form_links_chirho(html_chirho: &str) -> Vec<String> {
    html_chirho
        .split("href=\"")
        .skip(1)
        .filter_map(|rest_chirho| {
            rest_chirho
                .split_once('"')
                .map(|(href_chirho, _)| href_chirho)
        })
        .filter(|href_chirho| href_chirho.starts_with(CONNECTION_FORM_CHIRHO))
        .map(|href_chirho| href_chirho.replace("&#38;", "&").replace("&amp;", "&"))
        .collect()
}

#[test]
fn the_entry_is_the_engines_form_for_this_app_returning_here_chirho() {
    let href_chirho = connect_entry_href_chirho();
    let (path_chirho, query_chirho) = href_chirho.split_once('?').unwrap();
    assert_eq!(path_chirho, CONNECTION_FORM_CHIRHO);
    let pairs_chirho: Vec<(&str, &str)> = query_chirho
        .split('&')
        .map(|pair_chirho| pair_chirho.split_once('=').unwrap())
        .collect();
    // it names the app, never a client id, and its way back is this very page; nothing else rides along
    assert_eq!(
        pairs_chirho,
        vec![
            ("app_chirho", "perffection-workspace"),
            (
                "return_chirho",
                "%2Fapp-chirho%2Fperffection-workspace%2Faccount-chirho"
            ),
        ]
    );
    assert!(!href_chirho.contains("client"));
}

#[test]
fn only_the_page_bound_for_creating_offers_the_entry_and_only_beside_a_list_chirho() {
    let listed_chirho = own_keys_answer_chirho(json!([engine_row_chirho(
        "0a1b2c",
        "perffection-workspace",
        &["page-read-chirho"],
        true
    )]));
    let empty_chirho = own_keys_answer_chirho(json!([]));
    let unavailable_chirho: &[u8] = br#"{"schema_chirho":"own-keys-read-v1-chirho","state_chirho":"unavailable-chirho","reason_chirho":"keys-unavailable-chirho"}"#;
    // the page bound for creating: one link beside a list (an empty one too), never beside an unavailable one
    for (label_chirho, answer_chirho, expected_chirho) in [
        ("listed", listed_chirho.as_slice(), 1),
        ("empty", empty_chirho.as_slice(), 1),
        ("unavailable", unavailable_chirho, 0),
        ("unreadable", b"the read failed".as_slice(), 0),
    ] {
        let html_chirho = connect_page_chirho(answer_chirho);
        let expected_links_chirho = vec![connect_entry_href_chirho(); expected_chirho];
        assert_eq!(
            (
                label_chirho,
                connection_form_links_chirho(&html_chirho),
                html_chirho.matches(CONNECTION_FORM_CHIRHO).count()
            ),
            (label_chirho, expected_links_chirho, expected_chirho)
        );
    }
    // a link, not a form: the forms are still her password form and one Disconnect per live row
    let html_chirho = connect_page_chirho(&listed_chirho);
    assert_eq!(
        (
            html_chirho.matches("<form").count(),
            disconnect_forms_chirho(&html_chirho)
        ),
        (1 + 1, 1)
    );
    // binding the list alone never offers it, nor does the held page
    for html_chirho in [
        bound_page_chirho(&listed_chirho),
        bound_page_chirho(&empty_chirho),
        String::from_utf8(render_account_chirho(b"")).unwrap(),
    ] {
        assert_eq!(html_chirho.matches(CONNECTION_FORM_CHIRHO).count(), 0);
    }
}
