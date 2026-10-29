// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The set-up page's safety and contract, not its words (they are a draft for L.J.): ONE form to R2's recover road
//! (revision 3) with exactly the sign-in name, the code and a first password, redirecting back to itself; no request
//! reflection (so a code in a URL is never rendered back), no code in any link, one fixed notice per refusal, escaping.

use crate::implementations_chirho::render_setup_chirho::render_setup_chirho;
use crate::page_render_chirho::render_setup_template_chirho;
use crate::tests_chirho::rendered_document_chirho::{
    assert_markup_stays_in_the_template_chirho, declared_asset_names_chirho, referenced_urls_chirho,
};
use crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho;
use crate::types_chirho::login_page_v1_chirho::PRINCIPAL_FIELD_CHIRHO;
use crate::types_chirho::setup_page_v1_chirho::{
    SetupPageV1Chirho, CONFIRMATION_MISMATCH_FRAGMENT_CHIRHO, CREDENTIAL_NOT_SET_FRAGMENT_CHIRHO,
    CREDENTIAL_TOO_WEAK_FRAGMENT_CHIRHO, RECOVERY_FAILED_FRAGMENT_CHIRHO,
    RECOVERY_REFUSED_FRAGMENT_CHIRHO, RECOVER_RATE_LIMITED_FRAGMENT_CHIRHO,
    REQUEST_INVALID_FRAGMENT_CHIRHO, SETUP_ACTION_CHIRHO, SETUP_CODE_FIELD_CHIRHO,
    SETUP_CONFIRM_CREDENTIAL_FIELD_CHIRHO, SETUP_DISPLAY_PATH_CHIRHO,
    SETUP_NEW_CREDENTIAL_FIELD_CHIRHO, SETUP_REDIRECT_CHIRHO,
};
use perffection_ui_chirho::NEW_PASSWORD_MIN_CHARACTERS_CHIRHO;

fn rendered_chirho() -> String {
    render_setup_chirho(&HttpRequestV1Chirho::default()).html_chirho
}

#[test]
fn setup_is_one_form_with_name_code_and_first_password_back_to_itself_chirho() {
    let html_chirho = rendered_chirho();
    assert_eq!(html_chirho.matches("<form ").count(), 1);
    assert!(html_chirho.contains(&format!("method=\"post\" action=\"{SETUP_ACTION_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!("name=\"{PRINCIPAL_FIELD_CHIRHO}\"")));
    assert!(html_chirho.contains(&format!("name=\"{SETUP_CODE_FIELD_CHIRHO}\"")));
    // Refusals come back here, so the redirect is this page and nowhere else.
    assert_eq!(SETUP_REDIRECT_CHIRHO, SETUP_DISPLAY_PATH_CHIRHO);
    assert!(html_chirho.contains(&format!(
        "name=\"redirect_chirho\" value=\"{SETUP_REDIRECT_CHIRHO}\""
    )));
    assert!(html_chirho.contains("autocomplete=\"one-time-code\""));
    // Two password fields, both new: the password and its confirmation, each asking the shared minimum, so a typing
    // slip is refused before anything is written and the code stays live.
    assert_eq!(html_chirho.matches("type=\"password\"").count(), 2);
    for field_chirho in [
        SETUP_NEW_CREDENTIAL_FIELD_CHIRHO,
        SETUP_CONFIRM_CREDENTIAL_FIELD_CHIRHO,
    ] {
        assert!(html_chirho.contains(&format!(
            "type=\"password\" name=\"{field_chirho}\" autocomplete=\"new-password\" minlength=\"{NEW_PASSWORD_MIN_CHARACTERS_CHIRHO}\""
        )));
    }
}

#[test]
fn setup_never_reflects_the_request_so_a_code_in_a_url_is_never_rendered_chirho() {
    let request_chirho = HttpRequestV1Chirho {
        path_chirho: "/setup-chirho?recovery_code_chirho=secret-code-marker-chirho&redirect_chirho=https://untrusted.invalid"
            .to_string(),
        user_principal_chirho: Some("user/private-marker-chirho".to_string()),
    };
    let html_chirho = render_setup_chirho(&request_chirho).html_chirho;
    assert_eq!(html_chirho, rendered_chirho());
    assert!(!html_chirho.contains("secret-code-marker-chirho"));
    // No link anywhere carries a code.
    for url_chirho in referenced_urls_chirho(&html_chirho) {
        assert!(
            !url_chirho.contains(SETUP_CODE_FIELD_CHIRHO),
            "{url_chirho}"
        );
    }
}

#[test]
fn notices_are_fixed_engine_tokens_and_copy_stays_escaped_chirho() {
    let html_chirho = rendered_chirho();
    // One notice per token the recover road can return here (one rate token for every limit, Cairn #26040).
    for fragment_chirho in [
        CONFIRMATION_MISMATCH_FRAGMENT_CHIRHO,
        CREDENTIAL_TOO_WEAK_FRAGMENT_CHIRHO,
        RECOVERY_REFUSED_FRAGMENT_CHIRHO,
        RECOVER_RATE_LIMITED_FRAGMENT_CHIRHO,
        REQUEST_INVALID_FRAGMENT_CHIRHO,
        CREDENTIAL_NOT_SET_FRAGMENT_CHIRHO,
        RECOVERY_FAILED_FRAGMENT_CHIRHO,
    ] {
        assert_eq!(
            html_chirho
                .matches(&format!("id=\"{fragment_chirho}\""))
                .count(),
            1,
            "{fragment_chirho}"
        );
    }
    let mut copy_chirho = SetupPageV1Chirho::default_public_chirho().copy_chirho;
    copy_chirho.lang_chirho = "ar".to_string();
    copy_chirho.dir_chirho = "rtl".to_string();
    copy_chirho.heading_chirho = "<script>not-markup-chirho</script>".to_string();
    let model_chirho = SetupPageV1Chirho::from_copy_chirho(copy_chirho);
    let html_chirho = render_setup_template_chirho(&model_chirho).html_chirho;
    assert!(html_chirho.contains("lang=\"ar\" dir=\"rtl\""));
    assert!(!html_chirho.contains("<script>"));
    assert_eq!(model_chirho.action_chirho, SETUP_ACTION_CHIRHO);
    assert_eq!(model_chirho.redirect_chirho, SETUP_REDIRECT_CHIRHO);
}

#[test]
fn setup_stays_template_authored_and_uses_declared_assets_chirho() {
    assert_markup_stays_in_the_template_chirho(
        include_str!("../implementations_chirho/render_setup_chirho.rs"),
        "setup",
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
