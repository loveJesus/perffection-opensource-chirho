// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures page from a verified read: the report and the paste form, nothing saved yet, or an
//! unverifiable state said plainly (never as "nothing saved").

use crate::figures_flow_chirho::figures_page_chirho::FiguresPageV1Chirho;
use crate::page_render_chirho::render_figures_chirho;
use crate::viewer_state_chirho::figures_contract_chirho::decode_figures_state_chirho;

pub fn render_figures_page_chirho(response_bytes_chirho: &[u8]) -> Vec<u8> {
    let state_chirho = decode_figures_state_chirho(response_bytes_chirho);
    let model_chirho = FiguresPageV1Chirho::from_state_chirho(
        state_chirho.as_ref().map(Option::as_ref).map_err(|_| ()),
        None,
    );
    render_figures_chirho(&model_chirho)
        .map(String::into_bytes)
        .unwrap_or_default()
}
