// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! "Copy my page document" hands the customer the exact JSON the publication path consumes.

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use perffection_ui_chirho::{UiLocaleChirho, WorkspaceCopyChirho};
use perffection_workspace_chirho::implementations_chirho::render_page_document_export_chirho::render_page_document_export_chirho;
use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_workspace_chirho::page_document_export_chirho::percent_encode_chirho;
use perffection_workspace_chirho::page_document_export_chirho::PageDocumentExportV1Chirho;
use perffection_workspace_chirho::viewer_state_contract_chirho::{
    PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO, VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
};
use serde_json::json;

#[path = "../support_chirho/mod.rs"]
mod support_chirho;

fn stored_document_chirho() -> PageDocumentV2Chirho {
    PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "North Street Repairs".to_string(),
        headline_chirho: "Clear help & honest prices".to_string(),
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

fn current_response_chirho(version_chirho: u64) -> Vec<u8> {
    let current_chirho = json!({
        "version_chirho": version_chirho,
        "value_base64_chirho": BASE64_CHIRHO.encode(serde_json::to_vec(&stored_document_chirho()).unwrap()),
    });
    serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "current_chirho": current_chirho.clone(),
        "history_chirho": [current_chirho],
    }))
    .unwrap()
}

fn textarea_body_chirho(html_chirho: &str) -> String {
    let start_chirho = html_chirho.find("<textarea").unwrap();
    let open_end_chirho = start_chirho + html_chirho[start_chirho..].find('>').unwrap() + 1;
    let close_chirho = html_chirho[open_end_chirho..].find("</textarea>").unwrap();
    html_chirho[open_end_chirho..open_end_chirho + close_chirho]
        .replace("&#34;", "\"")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&#38;", "&")
        .replace("&amp;", "&")
}

#[test]
fn the_export_round_trips_the_exact_stored_document_and_offers_a_download_chirho() {
    let html_chirho = String::from_utf8(render_page_document_export_chirho(
        &current_response_chirho(3),
    ))
    .unwrap();
    assert_eq!(
        support_chirho::assert_page_wiring_chirho("export", &html_chirho),
        1
    );
    assert!(html_chirho.contains("Saved version 3"));
    let exported_chirho: PageDocumentV2Chirho =
        serde_json::from_str(&textarea_body_chirho(&html_chirho)).unwrap();
    assert_eq!(exported_chirho, stored_document_chirho());
    assert!(html_chirho.contains("download=\"perffection-page-document-v3.json\""));
    assert!(html_chirho.contains("href=\"data:application/json;charset=utf-8,"));
    assert!(!html_chirho.contains("<form"));
    assert!(!html_chirho.contains("<script"));
}

#[test]
fn nothing_saved_and_unverifiable_state_export_nothing_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let empty_chirho = serde_json::to_vec(&json!({
        "schema_chirho": VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO,
        "key_chirho": PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
        "history_chirho": [],
    }))
    .unwrap();
    let html_chirho = String::from_utf8(render_page_document_export_chirho(&empty_chirho)).unwrap();
    assert!(html_chirho.contains(&copy_chirho.export_chirho.nothing_saved_status_chirho));
    assert!(!html_chirho.contains("<textarea"));

    let html_chirho = String::from_utf8(render_page_document_export_chirho(
        b"{\"schema_chirho\":\"x\"}",
    ))
    .unwrap();
    assert!(html_chirho.contains(&copy_chirho.export_chirho.unverifiable_status_chirho));
    assert!(!html_chirho.contains("<textarea"));
}

#[test]
fn export_pseudolocale_sets_rtl_without_changing_navigation_authority_chirho() {
    let model_chirho =
        PageDocumentExportV1Chirho::nothing_saved_chirho(UiLocaleChirho::PseudolocaleRtlChirho);
    let html_chirho =
        perffection_workspace_chirho::page_render_chirho::render_page_document_export_chirho(
            &model_chirho,
        )
        .expect("the pseudolocale export should render");

    assert!(html_chirho.contains("lang=\"ar-XB\" dir=\"rtl\""));
    assert!(html_chirho.contains("href=\"/auth-chirho/logout-chirho\""));
    assert!(html_chirho.contains(&format!(
        "href=\"{}\"",
        perffection_workspace_chirho::workspace_page_chirho::WORKSPACE_HELP_HREF_CHIRHO
    )));
    assert!(html_chirho.contains("⟦!!"));
    assert!(!html_chirho.contains("<form"));
}

#[test]
fn percent_encoding_keeps_only_unreserved_bytes_chirho() {
    assert_eq!(percent_encode_chirho("a-b_c.d~e"), "a-b_c.d~e");
    assert_eq!(
        percent_encode_chirho("{\"k\": \"v&w\"}\n"),
        "%7B%22k%22%3A%20%22v%26w%22%7D%0A"
    );
    assert_eq!(percent_encode_chirho("é"), "%C3%A9");
}
