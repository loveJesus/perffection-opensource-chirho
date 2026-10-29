// For God so loved the world, that He gave His only Begotten Son, that whosoever believeth in Him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Craft-page pins. Properties and boundaries only — never prose.

use crate::implementations_chirho::render_craft_chirho::render_craft_chirho;
use crate::tests_chirho::rendered_document_chirho::{
    assert_markup_stays_in_the_template_chirho, assert_public_page_invariants_chirho,
};
use crate::types_chirho::craft_page_v1_chirho::CraftPageV1Chirho;
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;

fn rendered_chirho() -> String {
    render_craft_chirho(&HttpRequestV1Chirho::default()).html_chirho
}

#[test]
fn craft_holds_every_public_page_invariant_chirho() {
    assert_public_page_invariants_chirho(&rendered_chirho(), "craft");
}

#[test]
fn craft_markup_is_owned_by_the_template_chirho() {
    let wrapper_chirho = include_str!("../implementations_chirho/render_craft_chirho.rs");
    assert_markup_stays_in_the_template_chirho(wrapper_chirho, "craft");
}

/// A line that is built but not open to owners must never read like "Included" at a glance: the
/// template marks it from the catalog's flag, and every flagged line renders marked.
#[test]
fn craft_marks_what_is_coming_chirho() {
    let model_chirho = CraftPageV1Chirho::default_public_chirho();
    let coming_chirho = model_chirho
        .inventory_chirho
        .groups_chirho
        .iter()
        .flat_map(|group_chirho| &group_chirho.items_chirho)
        .filter(|item_chirho| item_chirho.coming_next_chirho)
        .count();
    assert_eq!(
        rendered_chirho()
            .matches("inventory-item-coming-chirho")
            .count(),
        coming_chirho,
        "every line flagged as coming must render marked, and no other"
    );
}

/// The public pages reach each other, so none becomes an orphan a visitor can only find by guessing
/// a URL: this one links back to the front door and on to Trust.
#[test]
fn craft_links_to_the_other_pages_chirho() {
    let html_chirho = rendered_chirho();
    assert!(html_chirho.contains("href=\"/app-chirho/perffection/\""));
    assert!(html_chirho.contains("/app-chirho/perffection/trust-chirho"));
}
