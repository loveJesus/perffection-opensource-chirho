// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use cairn_wasmtime_runner_chirho::runner_chirho::run_node_bytes_with_limits_chirho;
use perffection_page_template_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const WASM_FUEL_LIMIT_CHIRHO: u64 = 10_000_000;
const WASM_OUTPUT_LIMIT_CHIRHO: usize = 1024 * 1024;
const WASM_MEMORY_LIMIT_CHIRHO: usize = 64 * 1024 * 1024;

fn build_guest_wasm_chirho() -> Vec<u8> {
    let manifest_root_chirho = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let manifest_path_chirho = manifest_root_chirho.join("Cargo.toml");
    let cargo_chirho = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output_chirho = Command::new(cargo_chirho)
        .arg("build")
        .arg("--locked")
        .arg("--manifest-path")
        .arg(&manifest_path_chirho)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--example")
        .arg("render_published_page_guest_chirho")
        .arg("--message-format=json-render-diagnostics")
        .output()
        .expect("the public-page guest build should start");
    assert!(
        output_chirho.status.success(),
        "the public-page guest should compile for wasm32-unknown-unknown: {}",
        String::from_utf8_lossy(&output_chirho.stderr)
    );

    // Cargo owns artifact placement (workspace or CARGO_TARGET_DIR). Read its emitted artifact,
    // never a stale app-local target left over from the former three-workspace layout.
    let wasm_path_chirho = String::from_utf8(output_chirho.stdout)
        .expect("Cargo artifact output should be UTF-8")
        .lines()
        .filter_map(|line_chirho| serde_json::from_str::<serde_json::Value>(line_chirho).ok())
        .filter(|message_chirho| {
            message_chirho["reason"] == "compiler-artifact"
                && message_chirho["target"]["name"] == "render_published_page_guest_chirho"
        })
        .flat_map(|message_chirho| {
            message_chirho["filenames"]
                .as_array()
                .expect("Cargo artifacts include filenames")
                .iter()
                .filter_map(|name_chirho| name_chirho.as_str().map(PathBuf::from))
                .collect::<Vec<_>>()
        })
        .find(|path_chirho| {
            path_chirho
                .extension()
                .is_some_and(|ext_chirho| ext_chirho == "wasm")
        })
        .expect("Cargo must report the actual WASM guest artifact");
    fs::read(&wasm_path_chirho).unwrap_or_else(|error_chirho| {
        panic!(
            "read the built public-page guest {}: {error_chirho}",
            wasm_path_chirho.display()
        )
    })
}

fn valid_request_chirho() -> Vec<u8> {
    let document_chirho = PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "Good Earth Bakery".to_string(),
        headline_chirho: "Bread made before the town wakes.".to_string(),
        summary_chirho: "Small-batch loaves for our neighbors.".to_string(),
        services_chirho: [
            PageServiceV1Chirho {
                title_chirho: "Daily bread".to_string(),
                body_chirho: "Naturally leavened and baked each morning.".to_string(),
            },
            PageServiceV1Chirho::default(),
            PageServiceV1Chirho::default(),
        ],
        contact_heading_chirho: "Save a loaf for tomorrow.".to_string(),
        contact_phone_chirho: "+1 (555) 010-0100".to_string(),
        contact_email_chirho: "hello@good-earth.example".to_string(),
        contact_details_chirho: "Open between 7am and 2pm.".to_string(),
    };
    serde_json::to_vec(&serde_json::json!({
        "app_params_chirho": document_chirho
    }))
    .expect("the valid public-page request should serialize")
}

fn run_guest_chirho(wasm_chirho: &[u8], input_chirho: &[u8]) -> (String, u64) {
    let outcome_chirho = run_node_bytes_with_limits_chirho(
        wasm_chirho,
        "render_published_page_chirho",
        input_chirho,
        WASM_FUEL_LIMIT_CHIRHO,
        WASM_OUTPUT_LIMIT_CHIRHO,
        WASM_MEMORY_LIMIT_CHIRHO,
    )
    .expect("the public-page guest should run inside the bounded Wasmtime host");
    let html_chirho = String::from_utf8(outcome_chirho.output_chirho)
        .expect("the public-page guest should return UTF-8 HTML");
    (html_chirho, outcome_chirho.fuel_used_chirho)
}

#[test]
fn real_wasm_guest_renders_valid_params_and_fails_malformed_input_customer_safe_chirho() {
    let wasm_chirho = build_guest_wasm_chirho();

    let (valid_html_chirho, valid_fuel_chirho) =
        run_guest_chirho(&wasm_chirho, &valid_request_chirho());
    assert!(valid_fuel_chirho > 0 && valid_fuel_chirho <= WASM_FUEL_LIMIT_CHIRHO);
    assert!(valid_html_chirho.contains("Good Earth Bakery"));
    assert!(valid_html_chirho.contains("Bread made before the town wakes."));
    assert!(valid_html_chirho.contains("rel=\"stylesheet\""));
    assert!(valid_html_chirho.contains("href=\"tel:+15550100100\""));
    assert!(valid_html_chirho.contains("href=\"mailto:hello@good-earth.example\""));
    assert_eq!(valid_html_chirho.matches("<form").count(), 0);
    assert_eq!(valid_html_chirho.matches("<button").count(), 0);
    assert_eq!(valid_html_chirho.matches("<script").count(), 0);
    assert_eq!(valid_html_chirho.matches("style=").count(), 0);

    let (malformed_html_chirho, malformed_fuel_chirho) = run_guest_chirho(&wasm_chirho, b"{");
    assert!(malformed_fuel_chirho > 0 && malformed_fuel_chirho <= WASM_FUEL_LIMIT_CHIRHO);
    assert!(!malformed_html_chirho.trim().is_empty());
    assert!(malformed_html_chirho.contains("<main"));
    assert!(!malformed_html_chirho.contains("Good Earth Bakery"));
    assert!(!malformed_html_chirho.contains("could not be verified"));
    assert!(!malformed_html_chirho.contains("Perffection page"));
    assert!(!malformed_html_chirho.contains("tel:"));
    assert!(!malformed_html_chirho.contains("mailto:"));
    assert_eq!(malformed_html_chirho.matches("<form").count(), 0);
    assert_eq!(malformed_html_chirho.matches("<button").count(), 0);
    assert_eq!(malformed_html_chirho.matches("<script").count(), 0);
}
