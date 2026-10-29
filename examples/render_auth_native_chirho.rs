// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native display QA only. Runs the production template, never an auth handler.
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let locale_chirho = if std::env::args().any(|arg_chirho| arg_chirho == "--rtl-chirho") {
        perffection_ui_chirho::UiLocaleChirho::PseudolocaleRtlChirho
    } else {
        perffection_ui_chirho::UiLocaleChirho::EnglishChirho
    };
    let mut input_chirho = Vec::new();
    std::io::stdin()
        .take(
            (perffection_site_chirho::types_chirho::auth_page_v1_chirho::MAX_AUTH_VIEW_BYTES_CHIRHO
                + 1) as u64,
        )
        .read_to_end(&mut input_chirho)?;
    let arguments_chirho: Vec<_> = std::env::args().collect();
    let html_chirho = if let Some(index_chirho) = arguments_chirho
        .iter()
        .position(|arg_chirho| arg_chirho == "--guest-chirho")
    {
        if locale_chirho != perffection_ui_chirho::UiLocaleChirho::EnglishChirho {
            return Err(
                "the compiled guest uses its release locale; RTL is native template QA".into(),
            );
        }
        let path_chirho = arguments_chirho
            .get(index_chirho + 1)
            .ok_or("missing guest path")?;
        let bytes_chirho = std::fs::read(path_chirho)?;
        let outcome_chirho =
            cairn_wasmtime_runner_chirho::runner_chirho::run_node_bytes_with_limits_chirho(
                &bytes_chirho,
                "render_auth_chirho",
                &input_chirho,
                1_000_000,
                256 * 1024,
                16 * 1024 * 1024,
            )?;
        eprintln!(
            "auth display guest fuel: {}",
            outcome_chirho.fuel_used_chirho
        );
        outcome_chirho.output_chirho
    } else {
        perffection_site_chirho::page_render_chirho::render_auth_input_chirho(
            &input_chirho,
            locale_chirho,
        )?
        .into_bytes_chirho()
    };
    std::io::stdout().write_all(&html_chirho)?;
    Ok(())
}
