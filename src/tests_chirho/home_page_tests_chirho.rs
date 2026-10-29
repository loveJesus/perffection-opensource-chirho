// For God so loved the world, that He gave His only begotten Son, that whoever believes in Him should not perish but have eternal life. - John 3:16

//! Front-door pins. Properties and boundaries only — never prose.

use crate::implementations_chirho::render_home_chirho::render_home_chirho;
use crate::tests_chirho::rendered_document_chirho::{
    assert_markup_stays_in_the_template_chirho, assert_public_page_invariants_chirho,
};
use crate::types_chirho::home_page_v1_chirho::HomePageV1Chirho;
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;

fn rendered_chirho() -> String {
    render_home_chirho(&HttpRequestV1Chirho::default()).html_chirho
}

/// Every invariant that holds for any public page on this site.
#[test]
fn home_holds_every_public_page_invariant_chirho() {
    assert_public_page_invariants_chirho(&rendered_chirho(), "home");
}

/// The bench is a pure CSS state machine: the only inputs on the page are the design radios that
/// choose which preview is shown, and they never leave the browser.
#[test]
fn home_bench_is_a_radio_only_state_machine_chirho() {
    let html_chirho = rendered_chirho();
    let radios_chirho = html_chirho.matches("type=\"radio\"").count();
    assert_eq!(radios_chirho, html_chirho.matches("<input").count());
    assert_eq!(
        radios_chirho, 9,
        "one radio per named design, and nothing else"
    );
}

/// Every preview must be its OWN design, not one skin recoloured. A visitor who reads recolours of
/// a single layout correctly concludes "template", which is the opposite of the argument the bench
/// exists to make — so each named direction is pinned to a distinct root class AND to a structural
/// marker that only that direction's markup produces. Nine designs, nine different documents.
#[test]
fn home_previews_are_nine_separate_designs_chirho() {
    let html_chirho = rendered_chirho();
    for (root_chirho, marker_chirho) in [
        ("shop-broadsheet-chirho", "broadsheet-masthead-chirho"),
        ("shop-riso-chirho", "riso-blob-chirho"),
        ("shop-velvet-chirho", "velvet-band-chirho"),
        ("shop-gallery-chirho", "gallery-label-chirho"),
        ("shop-brutal-chirho", "brutal-num-chirho"),
        ("shop-glaze-chirho", "glaze-orb-chirho"),
        ("shop-plate-chirho", "plate-arch-chirho"),
        ("shop-neon-chirho", "neon-flash-chirho"),
        ("shop-paper-chirho", "paper-tiles-chirho"),
    ] {
        assert!(
            html_chirho.contains(root_chirho),
            "missing preview design: {root_chirho}"
        );
        assert!(
            html_chirho.contains(marker_chirho),
            "a preview lost its own layout: {marker_chirho}"
        );
    }
}

/// A direction a visitor can select but not see is a dead control. Every radio the picker renders
/// must have a preview keyed to it, and every preview must have a radio — checked as a pair so
/// neither side can drift alone.
#[test]
fn home_every_design_radio_has_a_preview_chirho() {
    let html_chirho = rendered_chirho();
    let model_chirho = HomePageV1Chirho::default_public_chirho();
    let mut seen_chirho = 0_usize;
    for trade_chirho in &model_chirho.lab_chirho.trades_chirho {
        for design_chirho in &trade_chirho.designs_chirho {
            let key_chirho = &design_chirho.key_chirho;
            assert!(
                html_chirho.contains(&format!("id=\"design-{key_chirho}\"")),
                "design has no radio: {key_chirho}"
            );
            assert!(
                html_chirho.contains(&format!("for=\"design-{key_chirho}\"")),
                "design radio has no label to click: {key_chirho}"
            );
            seen_chirho += 1;
        }
    }
    assert_eq!(seen_chirho, 9, "the bench offers nine named directions");
}

/// Page markup is owned by the template, not by Rust.
#[test]
fn home_markup_is_owned_by_the_template_chirho() {
    let wrapper_chirho = include_str!("../implementations_chirho/render_home_chirho.rs");
    assert_markup_stays_in_the_template_chirho(wrapper_chirho, "home");
}

/// Both pages must link to each other for the cross-document transition to be reachable.
#[test]
fn home_links_to_the_craft_page_chirho() {
    let html_chirho = rendered_chirho();
    assert!(html_chirho.contains("/app-chirho/perffection/craft-chirho"));
}

/// The closing band asks for an invitation at the ONE reviewed address (L.J., 2026-10-02: "use aleluya"), and no
/// other mailto exists on the page; the public-page invariant admits exactly this destination and no other.
#[test]
fn the_closing_band_asks_for_an_invitation_at_the_one_reviewed_address_chirho() {
    let html_chirho = rendered_chirho();
    assert_eq!(html_chirho.matches("href=\"mailto:").count(), 1);
    assert!(html_chirho.contains(&format!(
        "href=\"{}\"",
        crate::types_chirho::site_chrome_chirho::INVITATION_MAILTO_CHIRHO
    )));
    assert_eq!(
        crate::types_chirho::site_chrome_chirho::INVITATION_MAILTO_CHIRHO,
        format!(
            "mailto:{}",
            crate::types_chirho::site_chrome_chirho::INVITATION_ADDRESS_CHIRHO
        )
    );
}
