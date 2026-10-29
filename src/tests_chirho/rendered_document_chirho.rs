// For God so loved the world, that He gave His only begotten Son, that whoever believes in Him should not perish but have eternal life. - John 3:16

//! Shared helpers for asserting properties of a rendered page.
//!
//! Several needles here would match the assertion's own source if written as plain literals — a
//! test that forbids an inline style marker must not itself contain one — so every needle of that
//! kind is assembled at runtime.

/// The declared asset names for this app, read from the descriptor the manifest is emitted from.
/// A page may reference an app asset only if the descriptor declares it; anything else would 404
/// at serve time and, worse, would silently strip a directive from the route's CSP.
pub fn declared_asset_names_chirho() -> Vec<String> {
    let descriptor_chirho = include_str!("../../app-chirho.toml");
    descriptor_chirho
        .lines()
        .filter_map(|line_chirho| {
            let line_chirho = line_chirho.trim();
            let rest_chirho = line_chirho.strip_prefix("name_chirho")?;
            let rest_chirho = rest_chirho.trim_start().strip_prefix('=')?;
            let rest_chirho = rest_chirho.trim().strip_prefix('"')?;
            let end_chirho = rest_chirho.find('"')?;
            Some(rest_chirho[..end_chirho].to_string())
        })
        .collect()
}

/// Every `src=` and `href=` value in the document, in source order.
pub fn referenced_urls_chirho(html_chirho: &str) -> Vec<String> {
    let mut urls_chirho = Vec::new();
    for attribute_chirho in ["src=\"", "href=\""] {
        let mut cursor_chirho = 0usize;
        while let Some(found_chirho) = html_chirho[cursor_chirho..].find(attribute_chirho) {
            let start_chirho = cursor_chirho + found_chirho + attribute_chirho.len();
            let Some(end_chirho) = html_chirho[start_chirho..].find('"') else {
                break;
            };
            urls_chirho.push(html_chirho[start_chirho..start_chirho + end_chirho].to_string());
            cursor_chirho = start_chirho + end_chirho;
        }
    }
    urls_chirho
}

/// Every `id="…"` value in the document.
pub fn element_ids_chirho(html_chirho: &str) -> Vec<String> {
    let mut ids_chirho = Vec::new();
    let needle_chirho = "id=\"";
    let mut cursor_chirho = 0usize;
    while let Some(found_chirho) = html_chirho[cursor_chirho..].find(needle_chirho) {
        let start_chirho = cursor_chirho + found_chirho + needle_chirho.len();
        let Some(end_chirho) = html_chirho[start_chirho..].find('"') else {
            break;
        };
        ids_chirho.push(html_chirho[start_chirho..start_chirho + end_chirho].to_string());
        cursor_chirho = start_chirho + end_chirho;
    }
    ids_chirho
}

/// The inline-style marker the engine scans for. Built at runtime so this file does not trip the
/// very check it exists to enforce.
pub fn inline_style_marker_chirho() -> String {
    format!("{}=", "style")
}

/// The opening of a style element, likewise assembled rather than written.
pub fn style_element_marker_chirho() -> String {
    format!("<{}", "style")
}

/// The opening of a script element.
pub fn script_element_marker_chirho() -> String {
    format!("<{}", "script")
}

/// An opening tag for an arbitrary element name, for asserting that hand-written Rust contains no
/// page markup.
pub fn open_tag_marker_chirho(name_chirho: &str) -> String {
    format!("<{name_chirho}")
}

/// The invariants EVERY public page on this site must hold. Asserted once here and called from
/// each page's own test file, so adding a page cannot quietly skip them.
///
/// These are properties and boundaries, never prose: what the document may not contain, what it
/// may reference, and where its links may lead.
pub fn assert_public_page_invariants_chirho(html_chirho: &str, page_chirho: &str) {
    // 1. A document, and a script-free one. The route serves `script-src 'none'`.
    assert!(
        html_chirho.trim_start().starts_with("<!doctype html>"),
        "{page_chirho}: page must start with a doctype"
    );
    assert!(
        !html_chirho.contains(&script_element_marker_chirho()),
        "{page_chirho}: a script element cannot run under this route's policy"
    );

    // 2. The strict style-source policy. Any inline style marker in the payload silently downgrades
    //    `style-src 'self'` to also permit inline styles, with nothing surfaced anywhere.
    assert!(
        !html_chirho.contains(&inline_style_marker_chirho()),
        "{page_chirho}: an inline style attribute silently weakens the route CSP"
    );
    assert!(
        !html_chirho.contains(&style_element_marker_chirho()),
        "{page_chirho}: a style element silently weakens the route CSP"
    );

    // 3. Only declared assets. An undeclared name 404s, and a failed asset record makes the CSP
    //    derivation fail closed — which strips img-src and font-src and takes the page's pictures
    //    and typefaces down with it.
    let declared_chirho = declared_asset_names_chirho();
    let asset_prefix_chirho = "/app-assets-chirho/perffection/by-name-chirho/";
    for url_chirho in referenced_urls_chirho(html_chirho) {
        if let Some(name_chirho) = url_chirho.strip_prefix(asset_prefix_chirho) {
            assert!(
                declared_chirho
                    .iter()
                    .any(|d_chirho| d_chirho == name_chirho),
                "{page_chirho}: references undeclared app asset {name_chirho}"
            );
        }
    }

    // 4. Every destination is an element on this page, another public app page, an asset, the
    //    canonical sign-in display, the reviewed invitation mailto, or L.J.'s explicitly requested
    //    product-builder destination. No generic off-site allowlist or domain-prefix matching.
    let ids_chirho = element_ids_chirho(html_chirho);
    let app_prefix_chirho = "/app-chirho/perffection/";
    for url_chirho in referenced_urls_chirho(html_chirho) {
        if url_chirho.starts_with("/app-assets-chirho/")
            || url_chirho.starts_with(app_prefix_chirho)
            || url_chirho == crate::types_chirho::login_page_v1_chirho::LOGIN_ACTION_CHIRHO
            || url_chirho == crate::types_chirho::site_chrome_chirho::INVITATION_MAILTO_CHIRHO
            || url_chirho == perffection_ui_chirho::LOVEJESUS_SOFTWARE_URL_CHIRHO
        {
            continue;
        }
        let Some(fragment_chirho) = url_chirho.strip_prefix('#') else {
            panic!("{page_chirho}: unexpected off-site link {url_chirho}");
        };
        assert!(
            ids_chirho
                .iter()
                .any(|id_chirho| id_chirho == fragment_chirho),
            "{page_chirho}: in-page link points at a missing element #{fragment_chirho}"
        );
    }

    // 5. Nothing that only pretends to submit. No form, no button, no submit control anywhere on a
    //    release whose money, sign-up and email paths are all closed.
    assert!(
        !html_chirho.contains(&open_tag_marker_chirho("form")),
        "{page_chirho}: no form may appear while sign-up and email are closed"
    );
    assert!(
        !html_chirho.contains(&open_tag_marker_chirho("button")),
        "{page_chirho}: a button here would imply an action this release cannot take"
    );
    assert!(
        !html_chirho.contains("type=\"submit\""),
        "{page_chirho}: no submit control"
    );

    // 6. Every image is described, or explicitly marked decorative with an empty alt.
    assert_eq!(
        html_chirho.matches("<img").count(),
        html_chirho.matches("alt=\"").count(),
        "{page_chirho}: every image needs an alt attribute"
    );
}

/// Hand-written Rust must not own page markup. Given a wrapper module's source, this fails if HTML
/// has migrated back into it.
pub fn assert_markup_stays_in_the_template_chirho(wrapper_chirho: &str, page_chirho: &str) {
    for element_chirho in [
        "html", "body", "main", "section", "div", "h1", "h2", "span", "img",
    ] {
        assert!(
            !wrapper_chirho.contains(&open_tag_marker_chirho(element_chirho)),
            "{page_chirho}: page markup has leaked into the Rust wrapper ({element_chirho})"
        );
    }
    assert!(
        !wrapper_chirho.contains("push_str"),
        "{page_chirho}: the wrapper is assembling a string"
    );
    assert!(
        !wrapper_chirho.contains("format!"),
        "{page_chirho}: the wrapper is formatting markup"
    );
}
