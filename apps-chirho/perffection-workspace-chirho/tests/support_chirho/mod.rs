// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared reading of rendered workspace pages: which ids exist, and whether everything that
//! points at an id points at one the same document renders.

#![allow(dead_code)]

use perffection_workspace_chirho::page_edit_request_chirho::{
    CARRIER_REFUSAL_TOKENS_CHIRHO, FORM_CALLER_KEY_PLACEHOLDER_CHIRHO,
};
use std::collections::BTreeSet;

/// Every value of `attribute_chirho="…"` in the document, split on whitespace.
pub fn attribute_tokens_chirho(html_chirho: &str, attribute_chirho: &str) -> Vec<String> {
    let needle_chirho = format!(" {attribute_chirho}=\"");
    html_chirho
        .match_indices(&needle_chirho)
        .flat_map(|(start_chirho, _)| {
            let value_chirho = &html_chirho[start_chirho + needle_chirho.len()..];
            let end_chirho = value_chirho.find('"').expect("an attribute value closes");
            value_chirho[..end_chirho]
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn ids_chirho(html_chirho: &str) -> BTreeSet<String> {
    attribute_tokens_chirho(html_chirho, "id")
        .into_iter()
        .collect()
}

/// The element that opens with `id="…"`: its opening tag's text.
pub fn opening_tag_chirho<'html_chirho>(
    html_chirho: &'html_chirho str,
    id_chirho: &str,
) -> &'html_chirho str {
    let at_chirho = html_chirho
        .find(&format!(" id=\"{id_chirho}\""))
        .unwrap_or_else(|| panic!("no element with id {id_chirho}"));
    let start_chirho = html_chirho[..at_chirho].rfind('<').unwrap();
    let end_chirho = at_chirho + html_chirho[at_chirho..].find('>').unwrap();
    &html_chirho[start_chirho..=end_chirho]
}

/// What a page owes the engine's browser caller-key carrier: every POST form whose action is one of
/// Perffection's app routes (`/app-chirho/…`) carries the empty placeholder exactly once, and no
/// other form, nor anything outside a form, carries it. Escaped content can't forge one either.
/// Returns how many app-route POST forms were found, so a caller can refuse a vacuous pass. The contract:
/// spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md
pub fn assert_caller_key_forms_chirho(page_chirho: &str, html_chirho: &str) -> usize {
    let placeholder_chirho = FORM_CALLER_KEY_PLACEHOLDER_CHIRHO;
    let mut app_forms_chirho = 0;
    let mut carried_chirho = 0;
    let mut rest_chirho = html_chirho;
    while let Some(open_chirho) = rest_chirho.find("<form") {
        let tag_end_chirho = open_chirho
            + rest_chirho[open_chirho..]
                .find('>')
                .expect("a form tag closes");
        let close_chirho = tag_end_chirho
            + rest_chirho[tag_end_chirho..]
                .find("</form>")
                .expect("a form element closes");
        let tag_chirho = &rest_chirho[open_chirho..=tag_end_chirho];
        let inside_chirho = rest_chirho[tag_end_chirho..close_chirho]
            .matches(placeholder_chirho)
            .count();
        let app_post_chirho = tag_chirho.contains(r#"method="post""#)
            && tag_chirho.contains(r#"action="/app-chirho/"#);
        assert_eq!(
            inside_chirho,
            usize::from(app_post_chirho),
            "{page_chirho}: {tag_chirho} carries {inside_chirho} caller-key placeholders"
        );
        app_forms_chirho += usize::from(app_post_chirho);
        carried_chirho += inside_chirho;
        rest_chirho = &rest_chirho[close_chirho + "</form>".len()..];
    }
    assert_eq!(
        html_chirho.matches(placeholder_chirho).count(),
        carried_chirho,
        "{page_chirho}: a caller-key placeholder outside an app-route form"
    );
    // A page with a write form also pre-renders each of the carrier's four refusal notices exactly once.
    if app_forms_chirho > 0 {
        for token_chirho in CARRIER_REFUSAL_TOKENS_CHIRHO {
            assert_eq!(
                html_chirho
                    .matches(&format!(r#" id="refusal-{token_chirho}""#))
                    .count(),
                1,
                "{page_chirho}: the {token_chirho} notice"
            );
        }
    }
    app_forms_chirho
}

/// The page without the carrier's four dormant refusal notices, which stay hidden until a `#refusal-<token>`
/// targets one. This is what a clean page shows, so "no alert, no invalid state" can still be asserted about
/// everything else on it.
pub fn without_carrier_notices_chirho(html_chirho: &str) -> String {
    let mut clean_chirho = html_chirho.to_string();
    for token_chirho in CARRIER_REFUSAL_TOKENS_CHIRHO {
        let open_chirho = format!(r#"<section id="refusal-{token_chirho}""#);
        if let Some(start_chirho) = clean_chirho.find(&open_chirho) {
            let end_chirho = start_chirho
                + clean_chirho[start_chirho..]
                    .find("</section>")
                    .expect("a notice closes")
                + "</section>".len();
            clean_chirho.replace_range(start_chirho..end_chirho, "");
        }
    }
    clean_chirho
}

/// What a page owes a reader who cannot see it: every label, description and same-page link
/// names an element the page renders, and help text under a control is wired to that control.
/// Every page checked here is also held to the caller-key carrier (assert_caller_key_forms_chirho).
/// Returns how many help texts were found, so a caller can refuse a vacuous pass.
pub fn assert_page_wiring_chirho(page_chirho: &str, html_chirho: &str) -> usize {
    assert_caller_key_forms_chirho(page_chirho, html_chirho);
    let ids_chirho = ids_chirho(html_chirho);
    for attribute_chirho in ["aria-describedby", "aria-labelledby", "for"] {
        for id_chirho in attribute_tokens_chirho(html_chirho, attribute_chirho) {
            assert!(
                ids_chirho.contains(&id_chirho),
                "{page_chirho}: dangling {attribute_chirho} {id_chirho}"
            );
        }
    }
    for href_chirho in attribute_tokens_chirho(html_chirho, "href") {
        if let Some(fragment_chirho) = href_chirho.strip_prefix('#') {
            assert!(
                ids_chirho.contains(fragment_chirho),
                "{page_chirho}: dangling #{fragment_chirho}"
            );
        }
    }
    let described_chirho: BTreeSet<String> =
        attribute_tokens_chirho(html_chirho, "aria-describedby")
            .into_iter()
            .collect();
    // `workspace-help-chirho` is the page's own help aside, not a control's help text.
    let help_ids_chirho: Vec<&String> = ids_chirho
        .iter()
        .filter(|id_chirho| {
            id_chirho.ends_with("-help-chirho") && id_chirho.as_str() != "workspace-help-chirho"
        })
        .collect();
    for help_id_chirho in &help_ids_chirho {
        assert!(
            described_chirho.contains(*help_id_chirho),
            "{page_chirho}: {help_id_chirho} describes nothing"
        );
    }
    // No control's help text hides inside its label, where it would lengthen the control's name.
    assert!(
        !html_chirho.contains("<label class=\"workspace-field-chirho\""),
        "{page_chirho}: a field still wraps its control in the label"
    );
    help_ids_chirho.len()
}
