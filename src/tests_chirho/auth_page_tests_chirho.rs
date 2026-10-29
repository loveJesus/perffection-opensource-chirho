// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use crate::page_render_chirho::{render_auth_input_chirho, render_auth_template_chirho};
use crate::types_chirho::auth_page_v1_chirho::AuthPageV1Chirho;
use perffection_ui_chirho::{AuthCopyChirho, UiLocaleChirho};
use serde_json::{json, Value};

// Canonical spelling from Cairn's a948efce handoff #23648, not our model's serializer.
fn challenge_chirho() -> Value {
    json!({"page_kind_chirho":"challenge_chirho", "posture_chirho":"registration_open_chirho",
        "challenge_chirho":{"question_chirho":"2 + 3", "token_chirho":"synthetic-once-chirho"},
        "form_chirho":{"action_route_chirho":"/auth-chirho/register-chirho", "field_names_chirho":[
            "kind_chirho","username_chirho","credential_chirho","human_check_chirho","human_check_token_chirho"]}})
}

fn login_chirho() -> Value {
    json!({"page_kind_chirho":"username_login_chirho", "posture_chirho":"sign_in_chirho",
        "form_chirho":{"action_route_chirho":"/auth-chirho/login-chirho", "field_names_chirho":["username_chirho","credential_chirho","redirect_chirho"]}})
}

fn render_chirho(input_chirho: &Value) -> Result<String, &'static str> {
    render_auth_input_chirho(
        &serde_json::to_vec(input_chirho).unwrap(),
        UiLocaleChirho::EnglishChirho,
    )
    .map(|rendered_chirho| rendered_chirho.html_chirho)
}

#[test]
fn signup_posts_only_the_username_contract_directly_to_cairn_chirho() {
    let html_chirho = render_chirho(&challenge_chirho()).unwrap();
    assert!(html_chirho.contains("action=\"/auth-chirho/register-chirho\""));
    for field_chirho in [
        "kind_chirho",
        "username_chirho",
        "credential_chirho",
        "human_check_chirho",
        "human_check_token_chirho",
    ] {
        assert!(html_chirho.contains(&format!("name=\"{field_chirho}\"")));
    }
    assert!(html_chirho.contains("autocomplete=\"new-password\""));
    assert!(!html_chirho.contains("name=\"registration_token_chirho\""));
    assert!(!html_chirho.contains("principal_chirho"));
    assert!(!html_chirho.contains("<script"));
    assert!(!html_chirho.contains("<style"));
}

#[test]
fn username_login_has_no_legacy_principal_or_reflected_credentials_chirho() {
    let html_chirho = render_chirho(&login_chirho()).unwrap();
    assert!(html_chirho.contains("action=\"/auth-chirho/login-chirho\""));
    assert!(html_chirho.contains("name=\"username_chirho\""));
    assert!(html_chirho.contains("autocomplete=\"current-password\""));
    assert!(html_chirho
        .contains("name=\"redirect_chirho\" value=\"/app-chirho/perffection-workspace/\""));
    assert!(!html_chirho.contains("principal_chirho"));
    assert!(!html_chirho.contains("/app-chirho/perffection/login-chirho"));
    for field_chirho in [
        "credential_chirho",
        "username_chirho",
        "principal_chirho",
        "request_chirho",
    ] {
        let mut invalid_chirho = login_chirho();
        invalid_chirho[field_chirho] = json!("must-not-reach-a-renderer-chirho");
        assert!(render_chirho(&invalid_chirho).is_err());
    }
}

#[test]
fn username_entry_offers_availability_without_inventing_registration_authority_chirho() {
    for locale_chirho in [
        UiLocaleChirho::EnglishChirho,
        UiLocaleChirho::PseudolocaleRtlChirho,
    ] {
        let mut model_chirho = AuthPageV1Chirho::from_input_chirho(
            serde_json::from_value(login_chirho()).unwrap(),
            locale_chirho,
        )
        .unwrap();
        model_chirho.copy_chirho.availability_link_chirho =
            "<script>availability-marker-chirho</script>".into();
        let html_chirho = render_auth_template_chirho(&model_chirho).html_chirho;
        assert!(html_chirho.contains("href=\"/auth-chirho/register-chirho\""));
        assert!(html_chirho.contains("availability-marker-chirho"));
        assert!(!html_chirho.contains("<script"));
        assert_eq!(html_chirho.matches("<form ").count(), 1);
        assert!(!html_chirho.contains("human_check_token_chirho"));
        assert!(model_chirho
            .chrome_chirho
            .nav_links_chirho
            .iter()
            .any(|link_chirho| {
                link_chirho.href_chirho == "/auth-chirho/login-chirho"
                    && link_chirho.is_current_chirho
            }));
    }
    let registration_chirho = AuthPageV1Chirho::from_input_chirho(
        serde_json::from_value(challenge_chirho()).unwrap(),
        UiLocaleChirho::EnglishChirho,
    )
    .unwrap();
    assert!(registration_chirho
        .chrome_chirho
        .nav_links_chirho
        .iter()
        .all(|link_chirho| !link_chirho.is_current_chirho));
}

#[test]
fn closed_full_and_refused_forms_do_not_issue_challenges_or_replay_chirho() {
    for (posture_chirho, code_chirho) in [
        ("registration_closed_chirho", "registration-disabled-chirho"),
        (
            "registration_full_chirho",
            "registration-cohort-full-chirho",
        ),
        ("registration_open_chirho", "human-check-rejected-chirho"),
    ] {
        let html_chirho = render_chirho(&json!({"page_kind_chirho":"refusal_chirho",
            "posture_chirho":posture_chirho,"refusal_code_chirho":code_chirho}))
        .unwrap();
        assert!(!html_chirho.contains("<form"));
        assert!(!html_chirho.contains("human_check_token_chirho"));
        assert!(html_chirho.contains("role=\"alert\""));
        assert!(html_chirho.contains("href=\"/auth-chirho/login-chirho\""));
        assert_eq!(
            html_chirho.contains("href=\"/auth-chirho/register-chirho\""),
            posture_chirho == "registration_open_chirho"
        );
    }
}

#[test]
fn unknown_refusal_does_not_reflect_diagnostics_or_refine_identity_chirho() {
    let mut input_chirho = json!({"page_kind_chirho":"refusal_chirho","posture_chirho":"sign_in_chirho",
        "refusal_code_chirho":"unknown-private-marker-chirho"});
    let first_chirho = render_chirho(&input_chirho).unwrap();
    input_chirho["refusal_code_chirho"] = json!("different-private-marker-chirho");
    assert_eq!(first_chirho, render_chirho(&input_chirho).unwrap());
    assert!(!first_chirho.contains("private-marker-chirho"));
    let catalog_chirho = AuthCopyChirho::for_locale_chirho(UiLocaleChirho::EnglishChirho);
    assert!(catalog_chirho
        .refusals_chirho
        .contains_key("invalid-credential-chirho"));
    assert!(!catalog_chirho
        .refusals_chirho
        .contains_key("unknown-user-chirho"));
}

#[test]
fn rendering_escapes_the_entire_challenge_and_catalog_chirho() {
    let mut input_chirho = challenge_chirho();
    input_chirho["challenge_chirho"]["question_chirho"] =
        json!("<script>question-marker-chirho</script>");
    input_chirho["challenge_chirho"]["token_chirho"] =
        json!("\" autofocus onfocus=\"token-marker-chirho");
    let mut model_chirho = AuthPageV1Chirho::from_input_chirho(
        serde_json::from_value(input_chirho).unwrap(),
        UiLocaleChirho::EnglishChirho,
    )
    .unwrap();
    model_chirho.introduction_chirho.heading_chirho =
        "<script>heading-marker-chirho</script>".into();
    let html_chirho = render_auth_template_chirho(&model_chirho).html_chirho;
    assert!(!html_chirho.contains("<script"));
    assert!(!html_chirho.contains("value=\"\" autofocus"));
    assert!(html_chirho.contains("question-marker-chirho"));
    assert!(html_chirho.contains("heading-marker-chirho"));
}

#[test]
fn inconsistent_view_and_route_field_tampering_refuse_chirho() {
    let mut input_chirho = challenge_chirho();
    input_chirho["posture_chirho"] = json!("registration_closed_chirho");
    assert!(render_chirho(&input_chirho).is_err());
    input_chirho = challenge_chirho();
    input_chirho
        .as_object_mut()
        .unwrap()
        .remove("challenge_chirho");
    assert!(render_chirho(&input_chirho).is_err());
    for route_chirho in [
        "https://untrusted.invalid/",
        "//untrusted.invalid/",
        "/app-chirho/perffection/",
        "/auth-chirho/login-chirho",
    ] {
        let mut input_chirho = challenge_chirho();
        input_chirho["form_chirho"]["action_route_chirho"] = json!(route_chirho);
        assert!(render_chirho(&input_chirho).is_err());
    }
    input_chirho = login_chirho();
    input_chirho["form_chirho"]["field_names_chirho"] =
        json!(["principal_chirho", "credential_chirho"]);
    assert!(render_chirho(&input_chirho).is_err());
    input_chirho = challenge_chirho();
    input_chirho["form_chirho"]["field_names_chirho"]
        .as_array_mut()
        .unwrap()
        .push(json!("registration_token_chirho"));
    assert!(render_chirho(&input_chirho).is_err());
    assert!(
        render_auth_input_chirho(&vec![b' '; 16 * 1024 + 1], UiLocaleChirho::EnglishChirho)
            .is_err()
    );
}

#[test]
fn success_is_a_signin_handoff_never_an_invented_session_or_redirect_chirho() {
    let mut input_chirho = json!({"page_kind_chirho":"success_handoff_chirho","posture_chirho":"registration_open_chirho",
        "success_handoff_chirho":{"next_route_chirho":"/auth-chirho/login-chirho"}});
    let html_chirho = render_chirho(&input_chirho).unwrap();
    assert!(!html_chirho.contains("<form"));
    assert!(!html_chirho.contains("principal_chirho"));
    let mut closed_chirho = input_chirho.clone();
    closed_chirho["posture_chirho"] = json!("registration_closed_chirho");
    assert!(render_chirho(&closed_chirho).is_err());
    for route_chirho in [
        "//untrusted.invalid",
        "/auth-chirho/login-chirho?credential_chirho=secret",
        "/app-chirho/perffection-workspace/",
    ] {
        input_chirho["success_handoff_chirho"]["next_route_chirho"] = json!(route_chirho);
        assert!(render_chirho(&input_chirho).is_err());
    }
}

#[test]
fn pseudolocale_changes_display_not_fields_actions_or_challenge_chirho() {
    let html_chirho = render_auth_input_chirho(
        &serde_json::to_vec(&challenge_chirho()).unwrap(),
        UiLocaleChirho::PseudolocaleRtlChirho,
    )
    .unwrap()
    .html_chirho;
    assert!(html_chirho.contains("dir=\"rtl\""));
    assert!(html_chirho.contains("lang=\"ar-XB\""));
    assert!(html_chirho.contains("name=\"username_chirho\""));
    assert!(html_chirho.contains("action=\"/auth-chirho/register-chirho\""));
    assert!(html_chirho.contains("value=\"synthetic-once-chirho\""));
    assert!(html_chirho.contains("2 + 3"));
}
