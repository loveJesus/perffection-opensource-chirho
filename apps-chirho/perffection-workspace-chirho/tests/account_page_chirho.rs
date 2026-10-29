// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The Account page offers exactly the forms whose cairn road is bound, names the operator path
//! while a road is held, posts each act straight to its road with her session, never echoes a
//! password, and makes closing the account need her password and a typed consent.

use perffection_ui_chirho::{UiLocaleChirho, WorkspaceCopyChirho};
use perffection_workspace_chirho::account_page_chirho::{
    AccountBindingV1Chirho, AccountPageV1Chirho, AUTH_CREDENTIAL_CHANGE_ACTION_CHIRHO,
    CHANGE_PASSWORD_REFUSAL_TOKENS_CHIRHO, CLOSE_ACCOUNT_CONSENT_CHIRHO,
    NEW_PASSWORD_MIN_CHARACTERS_CHIRHO,
};
use perffection_workspace_chirho::implementations_chirho::render_account_chirho::render_account_chirho;
use perffection_workspace_chirho::page_edit_request_chirho::FORM_CALLER_KEY_PLACEHOLDER_CHIRHO;
use perffection_workspace_chirho::page_render_chirho::render_account_chirho as render_account_model_chirho;
use perffection_workspace_chirho::workspace_page_chirho::WORKSPACE_ACCOUNT_HREF_CHIRHO;

const CHANGE_CHIRHO: &str = "/auth-chirho/credential-change-chirho";
const RECOVERY_CHIRHO: &str = "/auth-chirho/recovery-code-chirho";
const CLOSE_CHIRHO: &str = "/auth-chirho/delete-account-chirho";

#[test]
fn the_held_page_names_the_operator_path_and_offers_no_form_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    // the deployed renderer binds the credential-change road and nothing else
    let html_chirho = String::from_utf8(render_account_chirho(b"ignored request bytes")).unwrap();
    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert!(html_chirho.contains(&format!(
        "action=\"{AUTH_CREDENTIAL_CHANGE_ACTION_CHIRHO}\""
    )));
    assert!(!html_chirho.contains(&copy_chirho.account_chirho.held_heading_chirho));
    assert!(!html_chirho.contains("ignored request bytes"));
    // the held model still renders the operator path and no form
    let html_chirho = render_account_model_chirho(&AccountPageV1Chirho::held_chirho()).unwrap();
    assert!(html_chirho.contains(&copy_chirho.account_chirho.held_heading_chirho));
    assert!(html_chirho.contains(&copy_chirho.account_chirho.held_body_chirho));
    assert!(html_chirho.contains(&copy_chirho.account_chirho.status_held_chirho));
    assert!(!html_chirho.contains("<form"));
    assert!(!html_chirho.contains("type=\"password\""));
    assert!(AccountPageV1Chirho::held_chirho().is_held_chirho());
    // the page is reached from the workspace chrome by its own address
    assert!(html_chirho.contains(WORKSPACE_ACCOUNT_HREF_CHIRHO));
}

#[test]
fn each_form_appears_only_with_its_road_and_posts_there_with_the_way_back_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let only_password_chirho = AccountPageV1Chirho::bound_chirho(
        AccountBindingV1Chirho::held_chirho().with_change_password_action_chirho(CHANGE_CHIRHO),
    );
    assert!(!only_password_chirho.is_held_chirho());
    let html_chirho = render_account_model_chirho(&only_password_chirho).unwrap();
    assert_eq!(html_chirho.matches("<form").count(), 1);
    assert!(html_chirho.contains(&format!("action=\"{CHANGE_CHIRHO}\"")));
    assert!(!html_chirho.contains(RECOVERY_CHIRHO));
    assert!(!html_chirho.contains(CLOSE_CHIRHO));
    assert!(!html_chirho.contains(&copy_chirho.account_chirho.held_heading_chirho));
    assert!(html_chirho.contains(&copy_chirho.account_chirho.status_ready_chirho));
    assert!(html_chirho.contains(&format!(
        "name=\"redirect_chirho\" value=\"{WORKSPACE_ACCOUNT_HREF_CHIRHO}\""
    )));
    // password fields: masked, browser-managed, never pre-filled, the new one with the floor
    assert!(html_chirho.contains(
        "type=\"password\" name=\"credential_chirho\" autocomplete=\"current-password\" required"
    ));
    assert!(html_chirho.contains(&format!(
        "type=\"password\" name=\"new_credential_chirho\" autocomplete=\"new-password\" minlength=\"{NEW_PASSWORD_MIN_CHARACTERS_CHIRHO}\""
    )));
    assert!(!html_chirho.contains("name=\"credential_chirho\" value="));
    assert!(!html_chirho.contains("name=\"new_credential_chirho\" value="));

    let all_chirho = AccountPageV1Chirho::bound_chirho(
        AccountBindingV1Chirho::held_chirho()
            .with_change_password_action_chirho(CHANGE_CHIRHO)
            .with_recovery_code_action_chirho(RECOVERY_CHIRHO)
            .with_close_account_action_chirho(CLOSE_CHIRHO),
    );
    let html_chirho = render_account_model_chirho(&all_chirho).unwrap();
    assert_eq!(html_chirho.matches("<form").count(), 3);
    for action_chirho in [CHANGE_CHIRHO, RECOVERY_CHIRHO, CLOSE_CHIRHO] {
        assert!(html_chirho.contains(&format!("action=\"{action_chirho}\"")));
    }
    // These three post to the engine's own roads, not an app route, so none carries the browser
    // caller-key placeholder: the carrier covers app routes only, until Cairn extends it to its roads.
    assert!(!html_chirho.contains(FORM_CALLER_KEY_PLACEHOLDER_CHIRHO));
    assert_eq!(
        html_chirho
            .matches(&format!(
                "name=\"redirect_chirho\" value=\"{WORKSPACE_ACCOUNT_HREF_CHIRHO}\""
            ))
            .count(),
        3
    );
    // closing needs her password and the typed consent, both required
    assert!(html_chirho.contains(&format!(
        "type=\"checkbox\" name=\"consent_chirho\" value=\"{CLOSE_ACCOUNT_CONSENT_CHIRHO}\" required"
    )));
    // every road proves the password: change, recovery code, closing
    assert_eq!(
        html_chirho
            .matches("name=\"credential_chirho\" autocomplete=\"current-password\" required")
            .count(),
        3
    );
    // each bound road pre-renders its refusals; a token shared by two roads is one alert
    for id_chirho in [
        "refusal-consent-missing-chirho",
        "refusal-publication-pending-chirho",
        "refusal-close-account-failed-chirho",
        "refusal-recovery-code-failed-chirho",
    ] {
        assert_eq!(
            html_chirho.matches(&format!("id=\"{id_chirho}\"")).count(),
            1
        );
    }
    assert_eq!(
        html_chirho
            .matches("id=\"refusal-credential-mismatch-chirho\"")
            .count(),
        1
    );
    assert!(html_chirho.contains(&copy_chirho.account_chirho.close_boundary_chirho));
}

#[test]
fn every_refusal_of_the_password_road_is_pre_rendered_hidden_and_addressable_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let held_html_chirho =
        render_account_model_chirho(&AccountPageV1Chirho::held_chirho()).unwrap();
    assert!(!held_html_chirho.contains("workspace-refusal-chirho"));

    let bound_chirho = AccountPageV1Chirho::bound_chirho(
        AccountBindingV1Chirho::held_chirho()
            .with_change_password_action_chirho(AUTH_CREDENTIAL_CHANGE_ACTION_CHIRHO),
    );
    let html_chirho = render_account_model_chirho(&bound_chirho).unwrap();
    assert_eq!(
        html_chirho.matches("workspace-refusal-chirho").count(),
        CHANGE_PASSWORD_REFUSAL_TOKENS_CHIRHO.len()
    );
    for token_chirho in CHANGE_PASSWORD_REFUSAL_TOKENS_CHIRHO {
        assert!(html_chirho.contains(&format!("id=\"refusal-{token_chirho}\"")));
        assert!(html_chirho.contains(
            copy_chirho
                .account_chirho
                .refusal_sentence_chirho(token_chirho)
        ));
    }
    // the five sentences are distinct except the generic fallback, and an unknown token falls to it
    assert_ne!(
        copy_chirho
            .account_chirho
            .refusal_sentence_chirho("credential-mismatch-chirho"),
        copy_chirho
            .account_chirho
            .refusal_sentence_chirho("credential-too-weak-chirho")
    );
    assert_eq!(
        copy_chirho
            .account_chirho
            .refusal_sentence_chirho("some-future-token-chirho"),
        copy_chirho.account_chirho.refusal_change_failed_chirho
    );
    // the road's session refusal lands on the login page, so it is not one of ours
    assert!(!html_chirho.contains("no-session-chirho"));
    assert!(html_chirho.contains(&format!(
        "action=\"{AUTH_CREDENTIAL_CHANGE_ACTION_CHIRHO}\""
    )));
}

#[test]
fn the_catalog_carries_the_page_in_every_locale_chirho() {
    let rtl_chirho = WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::PseudolocaleRtlChirho);
    assert!(!rtl_chirho.account_chirho.heading_chirho.is_empty());
    assert_ne!(
        rtl_chirho.account_chirho.held_body_chirho,
        WorkspaceCopyChirho::english_chirho()
            .account_chirho
            .held_body_chirho
    );
    let page_chirho = AccountPageV1Chirho::for_locale_chirho(
        AccountBindingV1Chirho::held_chirho(),
        UiLocaleChirho::PseudolocaleRtlChirho,
    );
    let html_chirho = render_account_model_chirho(&page_chirho).unwrap();
    assert!(html_chirho.contains("dir=\"rtl\""));
    assert!(!html_chirho.contains("<form"));
}
