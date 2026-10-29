// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Branded auth view over the generic display-only guest boundary. No auth or challenge issuance.
//! Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/signup-chirho.md.

use super::auth_page_input_chirho::{
    AuthPageChallengeChirho, AuthPageFormChirho, AuthPageInputChirho, AuthPageKindChirho,
    AuthPagePostureChirho,
};
use super::login_page_v1_chirho::{
    LoginNoticeChirho, LoginPageV1Chirho, LOGIN_ACTION_CHIRHO, WORKSPACE_REDIRECT_CHIRHO,
};
use super::site_chrome_chirho::SiteChromeChirho;
use perffection_ui_chirho::{
    AuthCopyChirho, AuthNoticeCopyChirho, LoginCopyChirho, UiLocaleChirho,
};

pub const REGISTER_ACTION_CHIRHO: &str = "/auth-chirho/register-chirho";
pub const MAX_AUTH_VIEW_BYTES_CHIRHO: usize = 16 * 1024;

pub struct AuthPageV1Chirho {
    pub chrome_chirho: SiteChromeChirho,
    pub copy_chirho: AuthCopyChirho,
    pub common_copy_chirho: LoginCopyChirho,
    pub introduction_chirho: AuthNoticeCopyChirho,
    pub notice_chirho: Option<AuthNoticeCopyChirho>,
    pub fragment_notices_chirho: Vec<LoginNoticeChirho>,
    pub challenge_chirho: Option<AuthPageChallengeChirho>,
    pub signin_chirho: bool,
    pub start_again_chirho: bool,
    pub next_route_chirho: Option<String>,
    pub register_action_chirho: &'static str,
    pub login_action_chirho: &'static str,
    pub workspace_redirect_chirho: &'static str,
}

impl AuthPageV1Chirho {
    pub fn from_input_chirho(
        input_chirho: AuthPageInputChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Result<Self, &'static str> {
        let copy_chirho = AuthCopyChirho::for_locale_chirho(locale_chirho);
        let common_copy_chirho = LoginCopyChirho::for_locale_chirho(locale_chirho);
        let login_chirho = LoginPageV1Chirho::from_copy_chirho(common_copy_chirho.clone());
        let mut chrome_chirho = login_chirho.chrome_chirho;
        // Shared navigation targets the canonical service in both deployment shapes. Its sign-in
        // link is current only on the sign-in view, not on registration or its refusal pages.
        for link_chirho in &mut chrome_chirho.nav_links_chirho {
            if link_chirho.href_chirho == LOGIN_ACTION_CHIRHO {
                link_chirho.is_current_chirho =
                    input_chirho.posture_chirho == AuthPagePostureChirho::SignInChirho;
            }
        }
        chrome_chirho.page_description_chirho = copy_chirho.page_description_chirho.clone();
        chrome_chirho.preview_marker_chirho = copy_chirho.preview_marker_chirho.clone();

        let mut page_chirho = Self {
            chrome_chirho,
            introduction_chirho: copy_chirho.unavailable_chirho.clone(),
            copy_chirho,
            common_copy_chirho,
            notice_chirho: None,
            fragment_notices_chirho: Vec::new(),
            challenge_chirho: None,
            signin_chirho: false,
            start_again_chirho: false,
            next_route_chirho: None,
            register_action_chirho: REGISTER_ACTION_CHIRHO,
            login_action_chirho: LOGIN_ACTION_CHIRHO,
            workspace_redirect_chirho: WORKSPACE_REDIRECT_CHIRHO,
        };
        match input_chirho.page_kind_chirho {
            AuthPageKindChirho::ChallengeChirho => {
                if input_chirho.posture_chirho != AuthPagePostureChirho::RegistrationOpenChirho
                    || input_chirho.refusal_code_chirho.is_some()
                    || input_chirho.success_handoff_chirho.is_some()
                {
                    return Err("inconsistent auth challenge view");
                }
                let challenge_chirho = input_chirho
                    .challenge_chirho
                    .ok_or("auth challenge missing")?;
                validate_form_chirho(
                    input_chirho.form_chirho.as_ref(),
                    REGISTER_ACTION_CHIRHO,
                    &[
                        "kind_chirho",
                        "username_chirho",
                        "credential_chirho",
                        "human_check_chirho",
                        "human_check_token_chirho",
                    ],
                )?;
                if challenge_chirho.question_chirho.trim().is_empty()
                    || challenge_chirho.token_chirho.trim().is_empty()
                {
                    return Err("auth challenge empty");
                }
                page_chirho.introduction_chirho = page_chirho.copy_chirho.signup_chirho.clone();
                page_chirho.challenge_chirho = Some(challenge_chirho);
            }
            AuthPageKindChirho::UsernameLoginChirho => {
                if input_chirho.posture_chirho != AuthPagePostureChirho::SignInChirho
                    || input_chirho.challenge_chirho.is_some()
                    || input_chirho.refusal_code_chirho.is_some()
                    || input_chirho.success_handoff_chirho.is_some()
                {
                    return Err("inconsistent auth sign-in view");
                }
                validate_form_chirho(
                    input_chirho.form_chirho.as_ref(),
                    LOGIN_ACTION_CHIRHO,
                    &["username_chirho", "credential_chirho", "redirect_chirho"],
                )?;
                page_chirho.introduction_chirho = page_chirho.copy_chirho.signin_chirho.clone();
                page_chirho.signin_chirho = true;
                page_chirho.fragment_notices_chirho = login_chirho.notices_chirho;
                // The existing refusal fragments retain their meaning; the new flow uses its
                // username catalog sentence, not operator-issued account instructions.
                for notice_chirho in &mut page_chirho.fragment_notices_chirho {
                    if let Some(code_chirho) =
                        notice_chirho.fragment_chirho.strip_prefix("refusal-")
                    {
                        if let Some(copy_chirho) =
                            page_chirho.copy_chirho.refusals_chirho.get(code_chirho)
                        {
                            notice_chirho.heading_chirho = copy_chirho.heading_chirho.clone();
                            notice_chirho.body_chirho = copy_chirho.body_chirho.clone();
                        }
                    }
                }
            }
            AuthPageKindChirho::RefusalChirho => {
                if input_chirho.challenge_chirho.is_some()
                    || input_chirho.success_handoff_chirho.is_some()
                {
                    return Err("inconsistent auth refusal view");
                }
                let code_chirho = input_chirho
                    .refusal_code_chirho
                    .ok_or("auth refusal missing")?;
                if let Some(form_chirho) = input_chirho.form_chirho.as_ref() {
                    if input_chirho.posture_chirho != AuthPagePostureChirho::SignInChirho {
                        return Err("unexpected auth refusal form");
                    }
                    validate_form_chirho(
                        Some(form_chirho),
                        LOGIN_ACTION_CHIRHO,
                        &["username_chirho", "credential_chirho", "redirect_chirho"],
                    )?;
                    page_chirho.signin_chirho = true;
                }
                let notice_chirho = page_chirho.copy_chirho.refusal_chirho(&code_chirho).clone();
                page_chirho.introduction_chirho = notice_chirho.clone();
                page_chirho.notice_chirho = Some(notice_chirho);
                page_chirho.start_again_chirho =
                    input_chirho.posture_chirho == AuthPagePostureChirho::RegistrationOpenChirho;
            }
            AuthPageKindChirho::SuccessHandoffChirho => {
                if input_chirho.posture_chirho != AuthPagePostureChirho::RegistrationOpenChirho
                    || input_chirho.challenge_chirho.is_some()
                    || input_chirho.refusal_code_chirho.is_some()
                    || input_chirho.form_chirho.is_some()
                {
                    return Err("inconsistent auth success view");
                }
                let next_chirho = input_chirho
                    .success_handoff_chirho
                    .ok_or("auth handoff missing")?;
                // This product currently has an explicit sign-in step after account creation.
                // Neither the catalog nor a renderer input can invent a workspace session.
                if next_chirho.next_route_chirho != LOGIN_ACTION_CHIRHO {
                    return Err("unsupported auth handoff destination");
                }
                page_chirho.introduction_chirho = page_chirho.copy_chirho.success_chirho.clone();
                page_chirho.next_route_chirho = Some(next_chirho.next_route_chirho);
            }
        }
        page_chirho.chrome_chirho.page_title_chirho = format!(
            "{} | {}",
            page_chirho.introduction_chirho.heading_chirho, page_chirho.chrome_chirho.brand_chirho
        );
        Ok(page_chirho)
    }
}

/// Match the engine's display contract, but never trust a supplied form to choose where secrets go.
/// Field order is presentation; destination, identity mode and accepted field set are boundaries.
fn validate_form_chirho(
    form_chirho: Option<&AuthPageFormChirho>,
    action_chirho: &str,
    required_chirho: &[&str],
) -> Result<(), &'static str> {
    let form_chirho = form_chirho.ok_or("auth form contract missing")?;
    let fields_chirho: std::collections::BTreeSet<_> = form_chirho
        .field_names_chirho
        .iter()
        .map(String::as_str)
        .collect();
    if form_chirho.action_route_chirho != action_chirho
        || fields_chirho.len() != form_chirho.field_names_chirho.len()
        || required_chirho
            .iter()
            .any(|field_chirho| !fields_chirho.contains(field_chirho))
        || fields_chirho
            .iter()
            .any(|field_chirho| !required_chirho.contains(field_chirho))
    {
        return Err("unsupported auth form contract");
    }
    Ok(())
}
