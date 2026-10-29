// For God so loved the world, that He gave His only Begotten Son, that whosoever believeth in Him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Trust-page pins. Properties and boundaries only — never prose.
//!
//! This page makes checkable promises, so some pins are unusual and deliberate: the descriptor must
//! still refuse every request body while the page says a public page stores nothing, the limits must
//! always render, and the developer's checks must stay on the page.

use crate::implementations_chirho::render_trust_chirho::render_trust_chirho;
use crate::tests_chirho::rendered_document_chirho::{
    assert_markup_stays_in_the_template_chirho, assert_public_page_invariants_chirho,
};
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;
use crate::types_chirho::trust_page_v1_chirho::TrustPageV1Chirho;

fn rendered_chirho() -> String {
    render_trust_chirho(&HttpRequestV1Chirho::default()).html_chirho
}

#[test]
fn trust_holds_every_public_page_invariant_chirho() {
    assert_public_page_invariants_chirho(&rendered_chirho(), "trust");
}

#[test]
fn trust_markup_is_owned_by_the_template_chirho() {
    let wrapper_chirho = include_str!("../implementations_chirho/render_trust_chirho.rs");
    assert_markup_stays_in_the_template_chirho(wrapper_chirho, "trust");
}

/// The page promises that this site accepts nothing. That promise is only true while the app
/// declares `no-body` on every route it serves, so the descriptor is checked rather than trusted.
#[test]
fn trust_promise_of_accepting_nothing_matches_the_descriptor_chirho() {
    let descriptor_chirho = include_str!("../../app-chirho.toml");
    let declared_routes_chirho = descriptor_chirho.matches("route_chirho = \"").count();
    let no_body_policies_chirho = descriptor_chirho
        .matches("request_body_policy_chirho = \"no-body\"")
        .count();
    assert!(declared_routes_chirho >= 2, "expected the named routes");
    assert_eq!(
        no_body_policies_chirho,
        declared_routes_chirho + 1,
        "every named route plus the root must declare no-body while the page promises that nothing \
         is accepted"
    );
}

/// The limits are the claim most easily dropped. The page must always carry them, rendered: a build
/// that lost them would read as a stronger promise than the one we can keep.
#[test]
fn trust_always_states_its_limits_chirho() {
    let model_chirho = TrustPageV1Chirho::default_public_chirho();
    assert!(
        model_chirho.limits_chirho.points_chirho.len() >= 3,
        "the page must keep naming where its promises stop"
    );
    let html_chirho = rendered_chirho();
    assert!(html_chirho.contains("id=\"limits-chirho\""));
    for point_chirho in &model_chirho.limits_chirho.points_chirho {
        assert!(
            html_chirho.contains(&point_chirho.title_chirho),
            "every limit in the catalog must render"
        );
    }
}

/// Every plain-words promise has its check, folded away for a developer. A trust page whose checks
/// disappeared would be asking to be believed, the exact failure it exists to avoid.
#[test]
fn trust_carries_the_developer_checks_chirho() {
    let model_chirho = TrustPageV1Chirho::default_public_chirho();
    assert!(model_chirho.fold_chirho.points_chirho.len() >= 4);
    let html_chirho = rendered_chirho();
    assert!(html_chirho.contains("<details class=\"fold-chirho\">"));
    assert_eq!(
        html_chirho.matches("<li>").count(),
        model_chirho.fold_chirho.points_chirho.len(),
        "each check in the catalog must render in the fold"
    );
}

/// All three public pages reach each other, so none becomes an orphan a visitor can only find by
/// guessing a URL.
#[test]
fn trust_links_to_both_other_pages_chirho() {
    let html_chirho = rendered_chirho();
    assert!(html_chirho.contains("href=\"/app-chirho/perffection/\""));
    assert!(html_chirho.contains("/app-chirho/perffection/craft-chirho"));
}
