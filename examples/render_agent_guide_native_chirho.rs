// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Native authoring aid for expanded RTL and the same contract the public guest renders.
use perffection_site_chirho::{
    page_render_chirho::render_agent_guide_template_chirho,
    types_chirho::agent_guide_page_v1_chirho::AgentGuidePageV1Chirho,
};
use perffection_ui_chirho::UiLocaleChirho;

fn main() {
    let locale_chirho = match std::env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [] => UiLocaleChirho::EnglishChirho,
        [flag_chirho] if flag_chirho == "--pseudolocale-chirho" => {
            UiLocaleChirho::PseudolocaleRtlChirho
        }
        _ => {
            eprintln!("usage: render_agent_guide_native_chirho [--pseudolocale-chirho]");
            std::process::exit(2);
        }
    };
    print!(
        "{}",
        render_agent_guide_template_chirho(&AgentGuidePageV1Chirho::for_locale_chirho(
            locale_chirho
        ))
        .html_chirho
    );
}
