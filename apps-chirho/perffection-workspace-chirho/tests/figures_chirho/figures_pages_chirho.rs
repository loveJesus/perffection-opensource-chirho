// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures through the guest nodes the DAGs run: the page for nothing saved, a saved version and
//! an unverifiable state; the review and its Save form; exactly one live branch of the save; the save's
//! answers (saved, someone saved first, unknown); and her assistant's JSON. Behaviour, never wording:
//! sentences come from the catalog.

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use perffection_ui_chirho::WorkspaceCopyChirho;
use perffection_workspace_chirho::figures_flow_chirho::figures_csv_chirho::import_figures_csv_chirho;
use perffection_workspace_chirho::figures_flow_chirho::figures_form_chirho::figures_to_csv_chirho;
use perffection_workspace_chirho::figures_flow_chirho::figures_page_chirho::{
    FIGURES_REVIEW_ACTION_CHIRHO, FIGURES_SAVE_ACTION_CHIRHO,
};
use perffection_workspace_chirho::implementations_chirho::{
    build_figures_read_chirho::build_figures_read_chirho,
    build_figures_write_chirho::build_figures_write_chirho,
    render_agent_figures_chirho::render_agent_figures_chirho,
    render_figures_page_chirho::render_figures_page_chirho,
    render_figures_rejection_chirho::render_figures_rejection_chirho,
    render_figures_saved_chirho::render_figures_saved_chirho,
    review_figures_chirho::review_figures_chirho,
};
use perffection_workspace_chirho::viewer_state_chirho::figures_contract_chirho::FIGURES_LOGICAL_KEY_CHIRHO;
use serde_json::{json, Value};

#[path = "../support_chirho/mod.rs"]
mod support_chirho;

const CSV_CHIRHO: &str = "month,kind,money_in,costs,jobs\n2026-01,Bread,1200.50,400,300\n2026-01,\"Weddings, large\",900,600,2\n";

fn copy_chirho() -> WorkspaceCopyChirho {
    WorkspaceCopyChirho::english_chirho()
}

fn form_chirho(csv_chirho: &str, version_chirho: u64) -> Vec<u8> {
    let encode_chirho = |text_chirho: &str| {
        text_chirho
            .bytes()
            .map(|byte_chirho| format!("%{byte_chirho:02X}"))
            .collect::<String>()
    };
    format!(
        "csv_chirho={}&currency_chirho=USD&note_chirho={}&expected_version_chirho={version_chirho}",
        encode_chirho(csv_chirho),
        encode_chirho("from my books")
    )
    .into_bytes()
}

fn state_chirho(version_chirho: u64) -> Vec<u8> {
    state_of_chirho(CSV_CHIRHO, version_chirho)
}

fn state_of_chirho(csv_chirho: &str, version_chirho: u64) -> Vec<u8> {
    let stored_chirho = import_figures_csv_chirho(csv_chirho, "USD", "from my books")
        .kept_chirho
        .unwrap()
        .stored_bytes_chirho()
        .to_vec();
    let current_chirho = json!({"version_chirho": version_chirho, "value_base64_chirho": BASE64_CHIRHO.encode(stored_chirho)});
    serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "current_chirho": current_chirho, "history_chirho": [current_chirho]})).unwrap()
}

fn empty_state_chirho() -> Vec<u8> {
    serde_json::to_vec(&json!({"schema_chirho": "viewer-state-response-v2-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "history_chirho": []})).unwrap()
}

fn html_chirho(bytes_chirho: Vec<u8>) -> String {
    String::from_utf8(bytes_chirho).unwrap()
}

#[test]
fn the_read_names_only_the_figures_key_chirho() {
    let request_chirho: Value =
        serde_json::from_slice(&build_figures_read_chirho(b"anything")).unwrap();
    assert_eq!(
        (
            request_chirho["action_chirho"].as_str(),
            request_chirho["key_chirho"].as_str()
        ),
        (Some("read-chirho"), Some(FIGURES_LOGICAL_KEY_CHIRHO))
    );
}

#[test]
fn nothing_saved_unverifiable_and_saved_each_read_as_themselves_chirho() {
    let copy_chirho = copy_chirho();
    let nothing_chirho = html_chirho(render_figures_page_chirho(&empty_state_chirho()));
    assert!(nothing_chirho.contains(&copy_chirho.figures_chirho.nothing_saved_chirho));
    assert!(nothing_chirho.contains(&format!("action=\"{FIGURES_REVIEW_ACTION_CHIRHO}\"")));
    assert!(nothing_chirho.contains("name=\"expected_version_chirho\" value=\"0\""));
    assert!(!nothing_chirho.contains("<table"));
    // unverifiable is said plainly, never as "nothing saved"
    let unverifiable_chirho = html_chirho(render_figures_page_chirho(b"not a state"));
    assert!(unverifiable_chirho.contains(&copy_chirho.figures_chirho.unavailable_chirho));
    assert!(!unverifiable_chirho.contains(&copy_chirho.figures_chirho.nothing_saved_chirho));
    assert!(!unverifiable_chirho.contains("<table"));
    // a saved version: its report, and her figures back in the paste box against that version
    let saved_chirho = html_chirho(render_figures_page_chirho(&state_chirho(3)));
    assert!(saved_chirho.contains("<th scope=\"row\">Bread</th>"));
    assert!(saved_chirho.contains("1,200.50"));
    assert!(saved_chirho.contains("name=\"expected_version_chirho\" value=\"3\""));
    assert!(
        saved_chirho.contains("2026-01,&#34;Weddings, large&#34;,900.00,600.00,2")
            || saved_chirho.contains("2026-01,&quot;Weddings, large&quot;,900.00,600.00,2")
    );
    // the page posts only to the review, never straight to a save
    assert!(!saved_chirho.contains(FIGURES_SAVE_ACTION_CHIRHO));
}

#[test]
fn the_review_shows_what_is_kept_and_its_save_posts_exactly_those_fields_chirho() {
    let copy_chirho = copy_chirho();
    let csv_chirho = format!("{CSV_CHIRHO}2026-01,Bread,10,,\n2026-13,Cakes,5,,\n");
    let review_chirho = html_chirho(review_figures_chirho(&form_chirho(&csv_chirho, 3)));
    assert!(review_chirho.contains(&copy_chirho.figures_chirho.kept_heading_chirho));
    assert!(review_chirho.contains(&copy_chirho.figures_chirho.combined_heading_chirho));
    assert!(review_chirho.contains(&copy_chirho.figures_chirho.refused_heading_chirho));
    assert!(review_chirho.contains(
        copy_chirho
            .figures_chirho
            .row_sentence_chirho("invalid-month-chirho")
    ));
    assert_eq!(review_chirho.matches("<form").count(), 1);
    assert!(review_chirho.contains(&format!("action=\"{FIGURES_SAVE_ACTION_CHIRHO}\"")));
    assert!(review_chirho.contains("name=\"expected_version_chirho\" value=\"3\""));
    assert!(review_chirho.contains("name=\"currency_chirho\" value=\"USD\""));
    // nothing can be kept: the reason, and no Save at all
    let refused_chirho = html_chirho(review_figures_chirho(&form_chirho(
        "month,kind,amount\n2026-01,Bread,1\n",
        0,
    )));
    assert!(refused_chirho.contains(
        copy_chirho
            .figures_chirho
            .refusal_sentence_chirho("wrong-header-chirho")
    ));
    assert!(!refused_chirho.contains("<form"));
    let unreadable_chirho = html_chirho(review_figures_chirho(b"csv_chirho=x&csv_chirho=y"));
    assert!(unreadable_chirho.contains(
        copy_chirho
            .figures_chirho
            .refusal_sentence_chirho("unreadable-form-chirho")
    ));
    assert!(!unreadable_chirho.contains("<form"));
}

#[test]
fn exactly_one_branch_of_the_save_is_live_chirho() {
    let skipped_chirho = cairn_node_sdk_chirho::skipped_bytes_chirho();
    let good_chirho = form_chirho(CSV_CHIRHO, 3);
    let request_chirho: Value =
        serde_json::from_slice(&build_figures_write_chirho(&good_chirho)).unwrap();
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
        render_figures_rejection_chirho(&good_chirho),
        skipped_chirho
    );
    let first_save_chirho: Value =
        serde_json::from_slice(&build_figures_write_chirho(&form_chirho(CSV_CHIRHO, 0))).unwrap();
    assert!(first_save_chirho
        .get("expected_current_version_chirho")
        .is_none());
    for bad_chirho in [
        form_chirho("month,kind,amount\n", 3),
        b"not a form".to_vec(),
        form_chirho(CSV_CHIRHO, 3)
            .into_iter()
            .chain(*b"&extra_chirho=1")
            .collect(),
    ] {
        assert_eq!(build_figures_write_chirho(&bad_chirho), skipped_chirho);
        assert_ne!(render_figures_rejection_chirho(&bad_chirho), skipped_chirho);
    }
}

#[test]
fn the_save_answer_is_saved_someone_saved_first_or_unknown_chirho() {
    let copy_chirho = copy_chirho();
    let saved_chirho = html_chirho(render_figures_saved_chirho(&state_chirho(4)));
    assert!(saved_chirho.contains(&copy_chirho.figures_chirho.saved_notice_chirho));
    assert!(saved_chirho.contains("name=\"expected_version_chirho\" value=\"4\""));
    // someone saved first: the head's report, and her attempt back in the paste box against the head
    let attempted_chirho = import_figures_csv_chirho(
        "month,kind,money_in,costs,jobs\n2026-02,Pies,77,,\n",
        "USD",
        "",
    )
    .kept_chirho
    .unwrap();
    let head_chirho: Value = serde_json::from_slice(&state_chirho(5)).unwrap();
    let conflict_chirho = serde_json::to_vec(&json!({"schema_chirho": "viewer-state-conflict-v1-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "expected_current_version_chirho": 4, "current_version_chirho": 5, "current_chirho": head_chirho, "attempted_value_base64_chirho": BASE64_CHIRHO.encode(attempted_chirho.stored_bytes_chirho())})).unwrap();
    let conflict_page_chirho = html_chirho(render_figures_saved_chirho(&conflict_chirho));
    assert!(conflict_page_chirho.contains(&copy_chirho.figures_chirho.conflict_notice_chirho));
    assert!(conflict_page_chirho.contains("2026-02,Pies,77.00,,"));
    assert!(conflict_page_chirho.contains("name=\"expected_version_chirho\" value=\"5\""));
    // an answer that cannot be verified is unknown: no report, never "saved", never "nothing saved"
    let unknown_chirho = html_chirho(render_figures_saved_chirho(b"{\"half\":"));
    assert!(unknown_chirho.contains(&copy_chirho.figures_chirho.unknown_notice_chirho));
    assert!(!unknown_chirho.contains(&copy_chirho.figures_chirho.saved_notice_chirho));
    assert!(!unknown_chirho.contains(&copy_chirho.figures_chirho.nothing_saved_chirho));
    assert!(!unknown_chirho.contains("<table"));
}

#[test]
fn her_assistant_reads_the_report_the_figures_and_how_to_read_them_chirho() {
    let saved_chirho: Value =
        serde_json::from_slice(&render_agent_figures_chirho(&state_chirho(6))).unwrap();
    assert_eq!(
        (
            saved_chirho["state_chirho"].as_str(),
            saved_chirho["current_version_chirho"].as_u64(),
            saved_chirho["as_of_month_chirho"].as_str()
        ),
        (Some("saved-chirho"), Some(6), Some("2026-01"))
    );
    assert_eq!(
        saved_chirho["report_chirho"]["kinds_chirho"][0]["money_in_minor_chirho"].as_i64(),
        Some(120_050)
    );
    assert_eq!(
        saved_chirho["figures_chirho"]["kinds_chirho"][1].as_str(),
        Some("Weddings, large")
    );
    assert!(saved_chirho["meanings_chirho"]["unknown_chirho"].is_string());
    let nothing_chirho: Value =
        serde_json::from_slice(&render_agent_figures_chirho(&empty_state_chirho())).unwrap();
    assert_eq!(
        (
            nothing_chirho["state_chirho"].as_str(),
            nothing_chirho["current_version_chirho"].as_u64(),
            nothing_chirho["figures_chirho"].is_null()
        ),
        (Some("nothing-saved-chirho"), Some(0), true)
    );
    let unverifiable_chirho: Value =
        serde_json::from_slice(&render_agent_figures_chirho(b"garbage")).unwrap();
    assert_eq!(
        (
            unverifiable_chirho["state_chirho"].as_str(),
            unverifiable_chirho["current_version_chirho"].is_null(),
            unverifiable_chirho["report_chirho"].is_null()
        ),
        (Some("unverifiable-chirho"), true, true)
    );
}

#[test]
fn her_assistant_is_told_when_the_figures_are_incomplete_chirho() {
    // gpt #26575: an incomplete report is never handed over as whole-business totals. Cakes has no
    // January figure, so Bread's share "falling" from 100% to 50% is no trend, the report says it is
    // incomplete, and the meanings send her assistant to that flag for what the shares cover.
    let flags_chirho = |answer_chirho: &Value| -> Vec<(String, String)> {
        answer_chirho["report_chirho"]["flags_chirho"]
            .as_array()
            .unwrap()
            .iter()
            .map(|flag_chirho| {
                (
                    flag_chirho["code_chirho"].as_str().unwrap().to_string(),
                    flag_chirho["kind_chirho"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                )
            })
            .collect()
    };
    let falling_bread_chirho = ("falling-share-chirho".to_string(), "Bread".to_string());
    let incomplete_chirho: Value = serde_json::from_slice(&render_agent_figures_chirho(&state_of_chirho(
        "month,kind,money_in,costs,jobs\n2026-01,Bread,100,40,\n2026-02,Bread,100,40,\n2026-02,Cakes,100,40,\n",
        2,
    )))
    .unwrap();
    assert_eq!(
        incomplete_chirho["report_chirho"]["complete_chirho"].as_bool(),
        Some(false)
    );
    assert!(!flags_chirho(&incomplete_chirho).contains(&falling_bread_chirho));
    for meaning_chirho in ["share_chirho", "unknown_chirho"] {
        assert!(
            incomplete_chirho["meanings_chirho"][meaning_chirho]
                .as_str()
                .is_some_and(|text_chirho| text_chirho.contains("complete")),
            "{meaning_chirho} must point at completeness"
        );
    }
    // control: with Cakes' January given, the same fall is a trend, and the report is complete
    let complete_chirho: Value = serde_json::from_slice(&render_agent_figures_chirho(&state_of_chirho(
        "month,kind,money_in,costs,jobs\n2026-01,Bread,100,40,\n2026-01,Cakes,100,40,\n2026-02,Bread,100,40,\n2026-02,Cakes,300,40,\n",
        2,
    )))
    .unwrap();
    assert_eq!(
        complete_chirho["report_chirho"]["complete_chirho"].as_bool(),
        Some(true)
    );
    assert!(flags_chirho(&complete_chirho).contains(&falling_bread_chirho));
}

#[test]
fn her_figures_written_back_as_csv_read_back_to_the_same_figures_chirho() {
    let original_chirho = import_figures_csv_chirho("month,kind,money_in,costs,jobs\n2026-01,\"Say \"\"hi\"\", ok\",5.05,,1\n2026-02,Bread,7,1,\n", "EUR", "n").kept_chirho.unwrap();
    let again_chirho = import_figures_csv_chirho(
        &figures_to_csv_chirho(original_chirho.document_chirho()),
        "EUR",
        "n",
    )
    .kept_chirho
    .unwrap();
    assert_eq!(
        again_chirho.document_chirho(),
        original_chirho.document_chirho()
    );
    assert_eq!(
        again_chirho.stored_bytes_chirho(),
        original_chirho.stored_bytes_chirho()
    );
}

#[test]
fn the_review_shows_costs_and_jobs_kept_with_each_unknown_named_chirho() {
    // gpt #26573: the review shows everything that will be kept, not money in alone
    let copy_chirho = copy_chirho();
    let review_chirho = html_chirho(review_figures_chirho(&form_chirho(
        "month,kind,money_in,costs,jobs\n2026-01,Bread,1200.50,400,300\n2026-01,Cakes,90,,\n",
        0,
    )));
    for caption_chirho in [
        &copy_chirho.figures_chirho.money_in_column_chirho,
        &copy_chirho.figures_chirho.costs_column_chirho,
        &copy_chirho.figures_chirho.jobs_column_chirho,
    ] {
        assert!(
            review_chirho.contains(&format!(">{caption_chirho}</caption>")),
            "{caption_chirho}"
        );
    }
    assert!(review_chirho.contains("<td>1,200.50</td>"));
    assert!(review_chirho.contains("<td>400.00</td>"));
    assert!(review_chirho.contains("<td>300</td>"));
    // and the currency and her note exactly as they will be kept
    assert!(review_chirho.contains("<dd>USD</dd>"));
    assert!(review_chirho.contains("<dd>from my books</dd>"));
    // Cakes gave money in but no costs and no jobs: both named unknown, never blank, never 0
    assert_eq!(
        review_chirho
            .matches(&format!(
                "<td>{}</td>",
                copy_chirho.figures_chirho.unknown_chirho
            ))
            .count(),
        2
    );
}

#[test]
fn a_conflict_hands_back_her_whole_attempt_its_currency_and_note_too_chirho() {
    // gpt #26573: never the head's units or provenance under her attempted figures
    let attempted_chirho = import_figures_csv_chirho(
        "month,kind,money_in,costs,jobs\n2026-02,Pies,77,,\n",
        "EUR",
        "my other books",
    )
    .kept_chirho
    .unwrap();
    let head_chirho: Value = serde_json::from_slice(&state_chirho(5)).unwrap();
    let conflict_chirho = serde_json::to_vec(&json!({"schema_chirho": "viewer-state-conflict-v1-chirho", "key_chirho": FIGURES_LOGICAL_KEY_CHIRHO, "expected_current_version_chirho": 4, "current_version_chirho": 5, "current_chirho": head_chirho, "attempted_value_base64_chirho": BASE64_CHIRHO.encode(attempted_chirho.stored_bytes_chirho())})).unwrap();
    let page_chirho = html_chirho(render_figures_saved_chirho(&conflict_chirho));
    assert!(page_chirho.contains("name=\"currency_chirho\" value=\"EUR\""));
    assert!(page_chirho.contains("name=\"note_chirho\" value=\"my other books\""));
    assert!(!page_chirho.contains("name=\"note_chirho\" value=\"from my books\""));
    // the head's own report still reads in the head's currency
    assert!(page_chirho.contains("(USD)</caption>"));
}

/// The browser caller-key carrier (the contract:
/// spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md):
/// the page's paste and file forms and the review's Save each carry the engine's empty placeholder
/// once. A review that keeps nothing offers no Save, so it carries none.
#[test]
fn every_figures_write_form_carries_one_caller_key_placeholder_chirho() {
    let page_chirho = html_chirho(render_figures_page_chirho(&state_chirho(3)));
    assert_eq!(
        support_chirho::assert_caller_key_forms_chirho("figures page", &page_chirho),
        2
    );
    let review_chirho = html_chirho(review_figures_chirho(&form_chirho(CSV_CHIRHO, 3)));
    assert_eq!(
        support_chirho::assert_caller_key_forms_chirho("figures review", &review_chirho),
        1
    );
    let nothing_kept_chirho = html_chirho(review_figures_chirho(&form_chirho(
        "month,kind,amount\n2026-01,Bread,1\n",
        3,
    )));
    assert_eq!(
        support_chirho::assert_caller_key_forms_chirho("nothing kept", &nothing_kept_chirho),
        0
    );
}

#[test]
fn her_figures_page_offers_paste_and_file_as_two_forms_chirho() {
    // gpt #26616: choosing a file never collides with her saved figures, and nothing clears or
    // chooses for her. The paste form keeps her saved figures; the file form sends an empty paste.
    let page_chirho = html_chirho(render_figures_page_chirho(&state_chirho(3)));
    let forms_chirho: Vec<&str> = page_chirho
        .split("<form")
        .skip(1)
        .map(|rest_chirho| &rest_chirho[..rest_chirho.find("</form>").unwrap()])
        .collect();
    assert_eq!(forms_chirho.len(), 2);
    let (paste_chirho, file_chirho) = (forms_chirho[0], forms_chirho[1]);
    for form_chirho in [paste_chirho, file_chirho] {
        assert!(form_chirho.contains(&format!("action=\"{FIGURES_REVIEW_ACTION_CHIRHO}\"")));
        assert!(form_chirho.contains("name=\"expected_version_chirho\" value=\"3\""));
        assert!(form_chirho.contains("name=\"currency_chirho\" value=\"USD\""));
        assert!(form_chirho.contains("name=\"note_chirho\" value=\"from my books\""));
    }
    assert!(!paste_chirho.contains("multipart/form-data"));
    assert!(paste_chirho.contains("2026-01,Bread,1200.50,400.00,300"));
    assert!(!paste_chirho.contains("csv_file_chirho"));
    assert!(file_chirho.contains("enctype=\"multipart/form-data\""));
    assert!(file_chirho.contains("<input type=\"hidden\" name=\"csv_chirho\" value=\"\">"));
    assert!(file_chirho.contains("name=\"csv_file_chirho\""));
    assert!(!file_chirho.contains("Bread"));
    // one rendering of the shared fields, every id on the page still unique
    let mut ids_chirho: Vec<&str> = page_chirho
        .split(" id=\"")
        .skip(1)
        .map(|rest_chirho| &rest_chirho[..rest_chirho.find('"').unwrap()])
        .collect();
    let all_chirho = ids_chirho.len();
    ids_chirho.sort_unstable();
    ids_chirho.dedup();
    assert_eq!(ids_chirho.len(), all_chirho, "a duplicate id");
}

#[test]
fn the_review_counts_in_words_that_agree_with_the_counts_chirho() {
    let copy_chirho = copy_chirho();
    let words_chirho = &copy_chirho.figures_chirho;
    assert_eq!(
        words_chirho.kept_counts_chirho(1, 1),
        format!(
            "1 {} · 1 {}",
            words_chirho.kept_kind_one_chirho, words_chirho.kept_month_one_chirho
        )
    );
    assert_eq!(
        words_chirho.kept_counts_chirho(2, 3),
        format!(
            "2 {} · 3 {}",
            words_chirho.kept_kinds_chirho, words_chirho.kept_months_chirho
        )
    );
    // her review: two kinds of work in one month
    let review_chirho = html_chirho(review_figures_chirho(&form_chirho(CSV_CHIRHO, 0)));
    assert!(review_chirho.contains(&format!("{} · USD", words_chirho.kept_counts_chirho(2, 1))));
}
