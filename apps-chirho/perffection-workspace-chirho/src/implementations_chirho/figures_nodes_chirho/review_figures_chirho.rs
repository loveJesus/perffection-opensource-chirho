// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The pure review of her figures, pasted or chosen as a file: exactly what would be kept, and why
//! anything was combined or left out; or, when the form gives nothing to review, why. No effect: Save
//! is a separate route she must choose.

use crate::figures_flow_chirho::figures_form_chirho::read_figures_review_form_chirho;
use crate::figures_flow_chirho::figures_page_chirho::FiguresReviewV1Chirho;
use crate::page_render_chirho::render_figures_review_chirho;

pub fn review_figures_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    let model_chirho = match read_figures_review_form_chirho(request_bytes_chirho) {
        Ok(form_chirho) => {
            FiguresReviewV1Chirho::from_import_chirho(&form_chirho, &form_chirho.import_chirho())
        }
        Err(refusal_chirho) => FiguresReviewV1Chirho::refused_form_chirho(refusal_chirho),
    };
    render_figures_review_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
