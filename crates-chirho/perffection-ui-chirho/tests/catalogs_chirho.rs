// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use perffection_ui_chirho::{
    AuthCopyChirho, LoginCopyChirho, PublicSiteCopyChirho, SetupCopyChirho, UiLocaleChirho,
    WorkspaceCopyChirho, NEW_PASSWORD_MIN_CHARACTERS_CHIRHO, SIGN_IN_WAIT_MINUTES_CHIRHO,
};

#[test]
fn english_catalogs_share_one_default_locale_boundary_chirho() {
    let login_chirho = LoginCopyChirho::english_chirho();
    let workspace_chirho = WorkspaceCopyChirho::english_chirho();

    assert_eq!(
        login_chirho.lang_chirho,
        UiLocaleChirho::default().lang_chirho()
    );
    assert_eq!(
        login_chirho.dir_chirho,
        UiLocaleChirho::default().dir_chirho()
    );
    assert_eq!(workspace_chirho.lang_chirho, "en");
    assert_eq!(workspace_chirho.dir_chirho, "ltr");
}

#[test]
fn pseudolocale_expands_copy_sets_rtl_and_preserves_interpolation_tokens_chirho() {
    let workspace_chirho =
        WorkspaceCopyChirho::for_locale_chirho(UiLocaleChirho::PseudolocaleRtlChirho);

    assert_eq!(workspace_chirho.lang_chirho, "ar-XB");
    assert_eq!(workspace_chirho.dir_chirho, "rtl");
    assert!(workspace_chirho
        .editor_chirho
        .heading_chirho
        .starts_with("⟦!!"));
    assert!(workspace_chirho
        .editor_chirho
        .conflict_attempted_chirho(17)
        .contains("17"));
    assert!(workspace_chirho
        .editor_chirho
        .conflict_attempted_template_chirho
        .contains("{version_chirho}"));
}

#[test]
fn caller_owned_public_catalog_gets_the_same_locale_treatment_chirho() {
    let catalog_json_chirho = r#"{
        "lang_chirho":"en",
        "dir_chirho":"ltr",
        "brand_chirho":"perffection",
        "preview_marker_chirho":"Preview",
        "nav_label_chirho":"Main navigation",
        "home_label_chirho":"Home",
        "craft_label_chirho":"Craft",
        "trust_label_chirho":"Trust",
        "help_label_chirho":"Help",
        "signin_label_chirho":"Sign in",
        "footer_note_chirho":"Footer"
    }"#;
    let copy_chirho = PublicSiteCopyChirho::from_json_for_locale_chirho(
        catalog_json_chirho,
        UiLocaleChirho::PseudolocaleRtlChirho,
    );

    assert_eq!(copy_chirho.lang_chirho, "ar-XB");
    assert_eq!(copy_chirho.dir_chirho, "rtl");
    assert!(copy_chirho.home_label_chirho.starts_with("⟦!!"));
}

/// The new-password minimum has ONE source: copy names it through a placeholder the loader fills from the constant, in
/// every locale, so changing the floor never leaves a stale number in a sentence.
#[test]
fn copy_names_the_password_minimum_from_its_one_constant_chirho() {
    let minimum_chirho = NEW_PASSWORD_MIN_CHARACTERS_CHIRHO.to_string();
    for locale_chirho in [
        UiLocaleChirho::EnglishChirho,
        UiLocaleChirho::PseudolocaleRtlChirho,
    ] {
        let setup_chirho = SetupCopyChirho::for_locale_chirho(locale_chirho);
        let account_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho).account_chirho;
        for text_chirho in [
            &setup_chirho.new_password_help_chirho,
            &setup_chirho.too_weak_body_chirho,
            &account_chirho.new_password_help_chirho,
            &account_chirho.refusal_credential_too_weak_chirho,
        ] {
            assert!(text_chirho.contains(&minimum_chirho), "{text_chirho}");
            assert!(
                !text_chirho.contains("{min_characters_chirho}"),
                "{text_chirho}"
            );
        }
    }
}

/// The sign-in wait has ONE source too: every notice for the engine's sign-in rate refusal names its minutes through a
/// placeholder the loader fills from the constant, in every locale, so the copy and the engine's window move together.
#[test]
fn copy_names_the_sign_in_wait_from_its_one_constant_chirho() {
    let minutes_chirho = SIGN_IN_WAIT_MINUTES_CHIRHO.to_string();
    for locale_chirho in [
        UiLocaleChirho::EnglishChirho,
        UiLocaleChirho::PseudolocaleRtlChirho,
    ] {
        let login_chirho = LoginCopyChirho::for_locale_chirho(locale_chirho);
        let setup_chirho = SetupCopyChirho::for_locale_chirho(locale_chirho);
        let auth_chirho = AuthCopyChirho::for_locale_chirho(locale_chirho);
        for text_chirho in [
            &login_chirho.rate_limited_body_chirho,
            &setup_chirho.rate_limited_body_chirho,
            &auth_chirho
                .refusal_chirho("auth-rate-limited-chirho")
                .body_chirho,
        ] {
            assert!(text_chirho.contains(&minutes_chirho), "{text_chirho}");
            assert!(
                !text_chirho.contains("{wait_minutes_chirho}"),
                "{text_chirho}"
            );
        }
    }
}
