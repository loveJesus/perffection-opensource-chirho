// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Native stdin/stdout adapter for local browser verification of the exact public renderer.

use perffection_page_template_chirho::implementations_chirho::render_published_page_chirho::render_published_page_chirho;
use perffection_page_template_chirho::page_render_chirho::render_page_chirho;
use perffection_page_template_chirho::page_view_chirho::PagePreviewV1Chirho;
use perffection_page_template_chirho::published_request_chirho::{
    decode_published_page_request_chirho, PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO,
};
use perffection_ui_chirho::UiLocaleChirho;
use std::io::{self, Read, Write};

fn main() {
    // Explicit local authoring modes only. The served guest accepts neither of these request fields.
    let mut preview_chirho = false;
    let mut locale_chirho = UiLocaleChirho::EnglishChirho;
    for argument_chirho in std::env::args().skip(1) {
        match argument_chirho.as_str() {
            "--preview-chirho" => preview_chirho = true,
            "--rtl-pseudolocale-chirho" => locale_chirho = UiLocaleChirho::PseudolocaleRtlChirho,
            _ => panic!("unknown local renderer option: {argument_chirho}"),
        }
    }
    let mut input_chirho = Vec::new();
    io::stdin()
        .take(PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO as u64 + 1)
        .read_to_end(&mut input_chirho)
        .expect("read the host-shaped public-page request from stdin");
    let output_chirho = if !preview_chirho && locale_chirho == UiLocaleChirho::EnglishChirho {
        render_published_page_chirho(&input_chirho)
    } else {
        let document_chirho = (input_chirho.len() <= PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO)
            .then(|| decode_published_page_request_chirho(&input_chirho).ok())
            .flatten()
            .and_then(|request_chirho| request_chirho.app_params_chirho)
            .and_then(|document_chirho| document_chirho.validate_chirho().ok());
        let model_chirho = match document_chirho {
            Some(document_chirho) if preview_chirho => {
                PagePreviewV1Chirho::private_preview_for_locale_chirho(
                    document_chirho,
                    "/app-chirho/perffection-workspace/",
                    locale_chirho,
                )
            }
            Some(document_chirho) => {
                PagePreviewV1Chirho::published_for_locale_chirho(document_chirho, locale_chirho)
            }
            None => PagePreviewV1Chirho::public_unavailable_for_locale_chirho(locale_chirho),
        };
        render_page_chirho(&model_chirho)
            .expect("render the typed local page")
            .into_bytes()
    };
    io::stdout()
        .write_all(&output_chirho)
        .expect("write the rendered public page to stdout");
}
