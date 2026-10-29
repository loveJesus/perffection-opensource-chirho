// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
//! Native fuel probe for the first-draft review renderer (workflow: first-draft-chirho.md).
//!
//! Runs the COMPILED guest under the hard fuel cap the emitted manifest gives every pure
//! workspace node and prints what each shape of the card's answer consumes: the stub's document,
//! a document at the completion ceiling, markup noise at the ceiling, prose at the ceiling that
//! is not a page, and a typed refusal. Dev evidence for the bounded-growth claim; it never serves and never rebuilds.
//!
//! usage: draft_review_fuel_probe_native_chirho <render_draft_review_guest_chirho.wasm> [fuel-cap]
use perffection_workspace_chirho::first_draft_chirho::{
    COMPLETION_MAX_BYTES_CHIRHO, LLM_REFUSAL_SCHEMA_CHIRHO, LLM_RESPONSE_SCHEMA_CHIRHO,
};

const ENTRY_CHIRHO: &str = "render_draft_review_chirho";
/// The emitted manifest's `fuel_chirho` for a pure workspace node.
const DEFAULT_FUEL_CAP_CHIRHO: u64 = 50_000_000;
const MAX_OUTPUT_BYTES_CHIRHO: usize = 1024 * 1024;
const MAX_MEMORY_BYTES_CHIRHO: usize = 64 * 1024 * 1024;
/// The deterministic stub's real completion for a North Street Repairs brief (cairn bcd1894d).
const STUB_DOCUMENT_CHIRHO: &str = r#"{"business_name":"North Street Repairs","contact_details":"Weekdays 8 to 5","contact_email":"","contact_heading":"Get in touch","contact_phone":"555-0124","headline":"Welcome to North Street Repairs","services":[{"body":"Repairs at North Street Repairs.","title":"Repairs"}],"summary":"North Street Repairs - a starting draft from your brief. Reference 5bc2268f."}"#;

fn response_bytes_chirho(completion_chirho: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "schema_chirho": LLM_RESPONSE_SCHEMA_CHIRHO,
        "completion_chirho": completion_chirho,
    }))
    .expect("a response body serializes")
}

/// The stub document with its summary padded so the completion is exactly the ceiling.
fn ceiling_document_chirho() -> String {
    let mut document_chirho: serde_json::Value =
        serde_json::from_str(STUB_DOCUMENT_CHIRHO).expect("the stub document parses");
    document_chirho["summary"] = serde_json::Value::String(String::new());
    let base_len_chirho = serde_json::to_string(&document_chirho)
        .expect("serializes")
        .len();
    let pad_chirho = COMPLETION_MAX_BYTES_CHIRHO - base_len_chirho;
    document_chirho["summary"] = serde_json::Value::String("s".repeat(pad_chirho));
    let text_chirho = serde_json::to_string(&document_chirho).expect("serializes");
    assert_eq!(text_chirho.len(), COMPLETION_MAX_BYTES_CHIRHO);
    text_chirho
}

/// The stub document whose summary is markup noise up to the ceiling: the tag-stripping road.
fn ceiling_markup_chirho() -> String {
    let mut document_chirho: serde_json::Value =
        serde_json::from_str(STUB_DOCUMENT_CHIRHO).expect("the stub document parses");
    document_chirho["summary"] = serde_json::Value::String(String::new());
    let base_len_chirho = serde_json::to_string(&document_chirho)
        .expect("serializes")
        .len();
    let pad_chirho = COMPLETION_MAX_BYTES_CHIRHO - base_len_chirho;
    document_chirho["summary"] = serde_json::Value::String("<b>".repeat(pad_chirho / 3));
    serde_json::to_string(&document_chirho).expect("serializes")
}

/// Prose at the ceiling with no JSON object in it: the "not a page" road.
fn ceiling_prose_chirho() -> String {
    let mut text_chirho = "The helper wrote a long answer instead of a page. ".repeat(1024);
    text_chirho.truncate(COMPLETION_MAX_BYTES_CHIRHO);
    text_chirho
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments_chirho: Vec<String> = std::env::args().collect();
    let wasm_path_chirho = arguments_chirho
        .get(1)
        .ok_or("usage: draft_review_fuel_probe_native_chirho <guest.wasm> [fuel-cap]")?;
    let fuel_cap_chirho: u64 = match arguments_chirho.get(2) {
        Some(cap_chirho) => cap_chirho.parse()?,
        None => DEFAULT_FUEL_CAP_CHIRHO,
    };
    let wasm_chirho = std::fs::read(wasm_path_chirho)?;
    let refusal_chirho = serde_json::to_vec(&serde_json::json!({
        "schema_chirho": LLM_REFUSAL_SCHEMA_CHIRHO,
        "refusal_code_chirho": "provider-unavailable-chirho",
    }))?;
    let shapes_chirho: [(&str, Vec<u8>); 5] = [
        ("stub-document", response_bytes_chirho(STUB_DOCUMENT_CHIRHO)),
        (
            "ceiling-document",
            response_bytes_chirho(&ceiling_document_chirho()),
        ),
        (
            "ceiling-markup",
            response_bytes_chirho(&ceiling_markup_chirho()),
        ),
        (
            "ceiling-prose",
            response_bytes_chirho(&ceiling_prose_chirho()),
        ),
        ("provider-refusal", refusal_chirho),
    ];
    let mut worst_chirho: u64 = 0;
    let mut failed_chirho = 0usize;
    for (name_chirho, input_chirho) in &shapes_chirho {
        match cairn_wasmtime_runner_chirho::runner_chirho::run_node_bytes_with_limits_chirho(
            &wasm_chirho,
            ENTRY_CHIRHO,
            input_chirho,
            fuel_cap_chirho,
            MAX_OUTPUT_BYTES_CHIRHO,
            MAX_MEMORY_BYTES_CHIRHO,
        ) {
            Ok(outcome_chirho) => {
                worst_chirho = worst_chirho.max(outcome_chirho.fuel_used_chirho);
                println!(
                    "draft review guest fuel {name_chirho}: {} of cap {} (input {} bytes, output {} bytes)",
                    outcome_chirho.fuel_used_chirho,
                    fuel_cap_chirho,
                    input_chirho.len(),
                    outcome_chirho.output_chirho.len()
                );
            }
            Err(error_chirho) => {
                failed_chirho += 1;
                // The alternate form carries the cause chain (fuel exhausted, or a guest trap).
                let text_chirho = format!("{error_chirho:#}");
                let first_line_chirho = text_chirho.lines().next().unwrap_or("");
                let cause_chirho = text_chirho
                    .lines()
                    .rev()
                    .find(|line_chirho| !line_chirho.trim().is_empty())
                    .unwrap_or("");
                println!(
                    "draft review guest fuel {name_chirho}: FAILED (input {} bytes): {first_line_chirho} | cause: {}",
                    input_chirho.len(),
                    cause_chirho.trim()
                );
            }
        }
    }
    println!(
        "draft review guest fuel worst: {} = {:.1}% of cap {}; shapes failed: {failed_chirho}",
        worst_chirho,
        worst_chirho as f64 * 100.0 / fuel_cap_chirho as f64,
        fuel_cap_chirho
    );
    if failed_chirho > 0 {
        return Err(format!("{failed_chirho} shape(s) did not complete under the cap").into());
    }
    Ok(())
}
