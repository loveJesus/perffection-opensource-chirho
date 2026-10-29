// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use crate::implementations_chirho::render_login_chirho::render_login_chirho;
use crate::page_render_chirho::render_login_template_chirho;
use crate::tests_chirho::rendered_document_chirho::{
    assert_markup_stays_in_the_template_chirho, declared_asset_names_chirho, referenced_urls_chirho,
};
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;
use crate::types_chirho::login_page_v1_chirho::{
    LoginPageV1Chirho, ACCOUNT_CLOSED_NOTICE_FRAGMENT_CHIRHO, CREDENTIAL_FIELD_CHIRHO,
    EXHAUSTED_FRAGMENT_CHIRHO, INVALID_CREDENTIAL_FRAGMENT_CHIRHO, LOGIN_ACTION_CHIRHO,
    NO_SESSION_FRAGMENT_CHIRHO, PASSWORD_SET_NOTICE_FRAGMENT_CHIRHO, PRINCIPAL_FIELD_CHIRHO,
    RATE_LIMITED_FRAGMENT_CHIRHO, SIGNIN_DISPLAY_PATH_CHIRHO, WORKSPACE_REDIRECT_CHIRHO,
};
use crate::types_chirho::setup_page_v1_chirho::SETUP_DISPLAY_PATH_CHIRHO;

fn rendered_chirho() -> String {
    render_login_chirho(&HttpRequestV1Chirho::default()).html_chirho
}

#[test]
fn login_uses_canonical_post_and_fixed_workspace_redirect_chirho() {
    let html_chirho = rendered_chirho();
    assert_eq!(html_chirho.matches("<form ").count(), 1);
    assert!(html_chirho.contains(&format!("method=\"post\" action=\"{LOGIN_ACTION_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!("name=\"{PRINCIPAL_FIELD_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!("name=\"{CREDENTIAL_FIELD_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!(
        "name=\"redirect_chirho\" value=\"{WORKSPACE_REDIRECT_CHIRHO}\""
    )));
    assert!(html_chirho.contains("autocomplete=\"current-password\""));
    assert!(!html_chirho.contains("/auth-chirho/register-chirho"));
    assert!(!html_chirho.contains("/app-chirho/perffection/signup-chirho"));
}

#[test]
fn login_does_not_reflect_request_or_accept_redirect_input_chirho() {
    let request_chirho = HttpRequestV1Chirho {
        path_chirho: "/login-chirho?redirect_chirho=https://untrusted.invalid".to_string(),
        user_principal_chirho: Some("user/private-marker-chirho".to_string()),
    };
    assert_eq!(
        render_login_chirho(&request_chirho).html_chirho,
        rendered_chirho()
    );
}

#[test]
fn locale_changes_text_not_auth_destination_and_remains_escaped_chirho() {
    let mut copy_chirho = LoginPageV1Chirho::default_public_chirho().copy_chirho;
    copy_chirho.lang_chirho = "ar".to_string();
    copy_chirho.dir_chirho = "rtl".to_string();
    copy_chirho.heading_chirho = "<script>not-markup-chirho</script>".to_string();
    let model_chirho = LoginPageV1Chirho::from_copy_chirho(copy_chirho);
    let html_chirho = render_login_template_chirho(&model_chirho).html_chirho;
    assert!(html_chirho.contains("lang=\"ar\" dir=\"rtl\""));
    assert!(html_chirho.contains("not-markup-chirho"));
    assert!(!html_chirho.contains("<script>"));
    assert_eq!(model_chirho.action_chirho, LOGIN_ACTION_CHIRHO);
    assert_eq!(model_chirho.redirect_chirho, WORKSPACE_REDIRECT_CHIRHO);
}

#[test]
fn login_stays_template_authored_and_uses_declared_assets_chirho() {
    assert_markup_stays_in_the_template_chirho(
        include_str!("../implementations_chirho/render_login_chirho.rs"),
        "login",
    );
    let html_chirho = rendered_chirho();
    assert!(html_chirho.trim_start().starts_with("<!doctype html>"));
    assert!(!html_chirho.contains("<script"));
    assert!(!html_chirho.contains("<style"));
    assert!(!html_chirho.contains("style="));
    let declared_chirho = declared_asset_names_chirho();
    for url_chirho in referenced_urls_chirho(&html_chirho) {
        if let Some(asset_chirho) =
            url_chirho.strip_prefix("/app-assets-chirho/perffection/by-name-chirho/")
        {
            assert!(declared_chirho
                .iter()
                .any(|declared_chirho| declared_chirho == asset_chirho));
        }
    }
}

#[test]
fn reentry_notice_is_fixed_display_copy_not_request_or_authority_chirho() {
    let mut copy_chirho = LoginPageV1Chirho::default_public_chirho().copy_chirho;
    copy_chirho.no_session_body_chirho = "<script>display-only-chirho</script>".into();
    copy_chirho.exhausted_heading_chirho = "<script>exhausted-heading-chirho</script>".into();
    copy_chirho.exhausted_body_chirho = "<script>exhausted-body-chirho</script>".into();
    copy_chirho.invalid_credential_heading_chirho =
        "<script>invalid-heading-chirho</script>".into();
    copy_chirho.invalid_credential_body_chirho = "<script>invalid-body-chirho</script>".into();
    copy_chirho.rate_limited_heading_chirho = "<script>rate-heading-chirho</script>".into();
    copy_chirho.rate_limited_body_chirho = "<script>rate-body-chirho</script>".into();
    let model_chirho = LoginPageV1Chirho::from_copy_chirho(copy_chirho);
    let html_chirho = render_login_template_chirho(&model_chirho).html_chirho;
    assert_eq!(
        model_chirho
            .notices_chirho
            .iter()
            .map(|notice_chirho| notice_chirho.fragment_chirho.as_str())
            .collect::<std::collections::BTreeSet<_>>(),
        [
            NO_SESSION_FRAGMENT_CHIRHO,
            EXHAUSTED_FRAGMENT_CHIRHO,
            INVALID_CREDENTIAL_FRAGMENT_CHIRHO,
            RATE_LIMITED_FRAGMENT_CHIRHO
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(model_chirho.display_path_chirho, SIGNIN_DISPLAY_PATH_CHIRHO);
    assert!(html_chirho.contains(&format!("id=\"{NO_SESSION_FRAGMENT_CHIRHO}\"")));
    assert!(html_chirho.contains("role=\"alert\""));
    assert!(html_chirho.contains("display-only-chirho"));
    assert!(html_chirho.contains("exhausted-heading-chirho"));
    assert!(html_chirho.contains("exhausted-body-chirho"));
    for marker_chirho in [
        "invalid-heading-chirho",
        "invalid-body-chirho",
        "rate-heading-chirho",
        "rate-body-chirho",
    ] {
        assert!(html_chirho.contains(marker_chirho));
    }
    for notice_chirho in &model_chirho.notices_chirho {
        assert_eq!(
            html_chirho
                .matches(&format!("id=\"{}\"", notice_chirho.fragment_chirho))
                .count(),
            1
        );
        let heading_id_chirho = format!("{}-heading-chirho", notice_chirho.fragment_chirho);
        assert_eq!(
            html_chirho
                .matches(&format!("id=\"{heading_id_chirho}\""))
                .count(),
            1
        );
        assert!(html_chirho.contains(&format!("aria-labelledby=\"{heading_id_chirho}\"")));
    }
    assert!(!html_chirho.contains("<script>"));
    assert_eq!(html_chirho.matches("<form ").count(), 1);
    assert_eq!(model_chirho.action_chirho, LOGIN_ACTION_CHIRHO);
    assert_eq!(model_chirho.redirect_chirho, WORKSPACE_REDIRECT_CHIRHO);
}

#[test]
fn login_help_and_return_copy_follow_the_selected_catalog_chirho() {
    let copy_chirho = perffection_ui_chirho::LoginCopyChirho::for_locale_chirho(
        perffection_ui_chirho::UiLocaleChirho::PseudolocaleRtlChirho,
    );
    let model_chirho = LoginPageV1Chirho::from_copy_chirho(copy_chirho);
    let html_chirho = render_login_template_chirho(&model_chirho).html_chirho;
    assert!(html_chirho.contains("dir=\"rtl\""));
    assert!(html_chirho.contains("autocomplete=\"username\""));
    assert!(html_chirho.contains("autocomplete=\"current-password\""));
    assert!(html_chirho.contains("dir=\"ltr\""));
    assert_eq!(model_chirho.action_chirho, LOGIN_ACTION_CHIRHO);
    assert_eq!(model_chirho.redirect_chirho, WORKSPACE_REDIRECT_CHIRHO);
    assert!(!model_chirho.copy_chirho.help_body_chirho.is_empty());
    assert!(!model_chirho.copy_chirho.no_session_body_chirho.is_empty());
    assert_eq!(
        model_chirho.notices_chirho[1].body_chirho,
        model_chirho.copy_chirho.exhausted_body_chirho
    );
    for (fragment_chirho, body_chirho) in [
        (
            INVALID_CREDENTIAL_FRAGMENT_CHIRHO,
            &model_chirho.copy_chirho.invalid_credential_body_chirho,
        ),
        (
            RATE_LIMITED_FRAGMENT_CHIRHO,
            &model_chirho.copy_chirho.rate_limited_body_chirho,
        ),
    ] {
        assert_eq!(
            &model_chirho
                .notices_chirho
                .iter()
                .find(|notice_chirho| notice_chirho.fragment_chirho == fragment_chirho)
                .unwrap()
                .body_chirho,
            body_chirho
        );
        assert!(body_chirho.starts_with("⟦!!"));
    }
}

/// The recover road's success lands here (R2 revision 3): one calm notice with a fixed id, a status rather than an alert,
/// with no "try again" inside it, and its copy escaped like every notice's.
#[test]
fn password_set_notice_is_one_fixed_status_not_an_alert_chirho() {
    let mut copy_chirho = LoginPageV1Chirho::default_public_chirho().copy_chirho;
    copy_chirho.password_set_heading_chirho = "<script>set-heading-chirho</script>".into();
    let model_chirho = LoginPageV1Chirho::from_copy_chirho(copy_chirho);
    let html_chirho = render_login_template_chirho(&model_chirho).html_chirho;
    let id_chirho = format!("id=\"{PASSWORD_SET_NOTICE_FRAGMENT_CHIRHO}\"");
    assert_eq!(html_chirho.matches(&id_chirho).count(), 1);
    let notice_chirho = &html_chirho[html_chirho.find(&id_chirho).unwrap()..];
    let notice_chirho = &notice_chirho[..notice_chirho.find("</section>").unwrap()];
    assert!(notice_chirho.contains("role=\"status\""));
    assert!(!notice_chirho.contains("role=\"alert\""));
    assert!(!notice_chirho.contains("<a "));
    assert!(html_chirho.contains("set-heading-chirho"));
    assert!(!html_chirho.contains("<script>"));
}

/// An invitee who has a setup code but no password yet finds the Set-up page from here: the help panel links to its one
/// display path, a public app page, while the sign-in form keeps its own destination.
#[test]
fn login_help_points_an_invitee_with_a_setup_code_to_setup_chirho() {
    let model_chirho = LoginPageV1Chirho::default_public_chirho();
    assert_eq!(model_chirho.setup_path_chirho, SETUP_DISPLAY_PATH_CHIRHO);
    let html_chirho = rendered_chirho();
    let help_chirho = &html_chirho[html_chirho.find("id=\"login-help-chirho\"").unwrap()..];
    let help_chirho = &help_chirho[..help_chirho.find("</aside>").unwrap()];
    assert!(referenced_urls_chirho(help_chirho).contains(&SETUP_DISPLAY_PATH_CHIRHO.to_string()));
    assert!(html_chirho.contains(&format!("method=\"post\" action=\"{LOGIN_ACTION_CHIRHO}\"")));
}

/// The opening tag and the body of one rendered notice section, found by its fixed id.
fn notice_section_chirho<'html_chirho>(
    html_chirho: &'html_chirho str,
    fragment_chirho: &str,
) -> (&'html_chirho str, &'html_chirho str) {
    let id_at_chirho = html_chirho
        .find(&format!("id=\"{fragment_chirho}\""))
        .expect("the notice renders");
    let open_at_chirho = html_chirho[..id_at_chirho]
        .rfind("<section")
        .expect("inside a section");
    let tag_end_chirho = open_at_chirho + html_chirho[open_at_chirho..].find('>').unwrap();
    let section_end_chirho =
        open_at_chirho + html_chirho[open_at_chirho..].find("</section>").unwrap();
    (
        &html_chirho[open_at_chirho..=tag_end_chirho],
        &html_chirho[open_at_chirho..section_end_chirho],
    )
}

/// The close road's accepted closure lands here (R3, account-recovery-chirho.md): one calm notice with a fixed id, a
/// status rather than an alert, with no link inside it, its copy escaped, and dormant like every notice until its own
/// fragment targets it. The page's other notices are unchanged.
#[test]
fn account_closed_notice_is_one_fixed_dormant_status_chirho() {
    let mut copy_chirho = LoginPageV1Chirho::default_public_chirho().copy_chirho;
    copy_chirho.account_closed_heading_chirho = "<script>closed-heading-chirho</script>".into();
    copy_chirho.account_closed_body_chirho = "<script>closed-body-chirho</script>".into();
    let model_chirho = LoginPageV1Chirho::from_copy_chirho(copy_chirho);
    let html_chirho = render_login_template_chirho(&model_chirho).html_chirho;
    assert_eq!(
        html_chirho
            .matches(&format!("id=\"{ACCOUNT_CLOSED_NOTICE_FRAGMENT_CHIRHO}\""))
            .count(),
        1
    );
    let (open_tag_chirho, notice_chirho) =
        notice_section_chirho(&html_chirho, ACCOUNT_CLOSED_NOTICE_FRAGMENT_CHIRHO);
    assert!(open_tag_chirho.contains("class=\"login-notice-chirho login-notice-ok-chirho\""));
    assert!(open_tag_chirho.contains("role=\"status\""));
    assert!(!notice_chirho.contains("role=\"alert\""));
    assert!(!notice_chirho.contains("<a "));
    assert!(notice_chirho.contains("closed-heading-chirho"));
    assert!(notice_chirho.contains("closed-body-chirho"));
    assert!(!html_chirho.contains("<script>"));

    // Dormant except its fragment: the stylesheet hides every notice and shows only the one its fragment targets.
    let styles_chirho = include_str!("../../styles_chirho/pages-chirho/login-chirho.scss");
    let rule_chirho = |selector_chirho: &str| {
        let start_chirho = styles_chirho
            .find(&format!("{selector_chirho} {{"))
            .expect("the rule exists");
        let end_chirho = start_chirho + styles_chirho[start_chirho..].find('}').unwrap();
        styles_chirho[start_chirho..end_chirho].to_string()
    };
    assert!(rule_chirho(".login-notice-chirho").contains("display: none"));
    let target_rule_chirho = rule_chirho(".login-notice-chirho:target");
    assert!(target_rule_chirho.contains("display:") && !target_rule_chirho.contains("none"));

    // The other notices still render once each, the password-set one as a status too.
    for fragment_chirho in [
        PASSWORD_SET_NOTICE_FRAGMENT_CHIRHO,
        NO_SESSION_FRAGMENT_CHIRHO,
        EXHAUSTED_FRAGMENT_CHIRHO,
        INVALID_CREDENTIAL_FRAGMENT_CHIRHO,
        RATE_LIMITED_FRAGMENT_CHIRHO,
    ] {
        assert_eq!(
            html_chirho
                .matches(&format!("id=\"{fragment_chirho}\""))
                .count(),
            1,
            "{fragment_chirho}"
        );
    }
    let (password_set_tag_chirho, _) =
        notice_section_chirho(&html_chirho, PASSWORD_SET_NOTICE_FRAGMENT_CHIRHO);
    assert!(password_set_tag_chirho.contains("role=\"status\""));
}
