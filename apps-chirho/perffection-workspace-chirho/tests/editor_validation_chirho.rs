// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Validation beside the field: what the editor owes her when the validator refuses her words.
//! These pin behavior, never sentences: every reported issue is shown; every link the summary
//! offers and every id a control is described by exists in the same document; help text is
//! always wired to its control; a clean page carries no invalid state.

use perffection_workspace_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PageValidationCodeChirho,
    PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
};
use perffection_workspace_chirho::page_render_chirho::render_workspace_chirho;
use perffection_workspace_chirho::workspace_page_chirho::{
    WorkspaceEditorBindingV1Chirho, WorkspacePageV1Chirho,
};
use std::collections::BTreeSet;

mod support_chirho;
use support_chirho::{attribute_tokens_chirho, ids_chirho, opening_tag_chirho};

fn valid_document_chirho() -> PageDocumentV2Chirho {
    let mut document_chirho = PageDocumentV2Chirho {
        schema_version_chirho: PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO,
        business_name_chirho: "North Street Repairs".to_string(),
        headline_chirho: "Clear help".to_string(),
        summary_chirho: "Straightforward local service".to_string(),
        contact_heading_chirho: "Tell us what needs attention".to_string(),
        contact_phone_chirho: "555-0124".to_string(),
        ..PageDocumentV2Chirho::default()
    };
    document_chirho.services_chirho[0] = PageServiceV1Chirho {
        title_chirho: "Repairs".to_string(),
        body_chirho: "Same-week visits".to_string(),
    };
    document_chirho
}

fn rejected_html_chirho(document_chirho: &PageDocumentV2Chirho) -> (String, usize) {
    let error_chirho = document_chirho
        .validate_chirho()
        .expect_err("the document under test must be refused");
    let issue_count_chirho = error_chirho.issues_chirho.len();
    let model_chirho = WorkspacePageV1Chirho::rejected_chirho(
        document_chirho.clone(),
        4,
        WorkspaceEditorBindingV1Chirho::published_contract_chirho(),
        &error_chirho,
    );
    (
        render_workspace_chirho(&model_chirho).unwrap(),
        issue_count_chirho,
    )
}

/// A document wrong in every way the form can show: three empty required fields, a half-filled
/// second service, a bad phone, a bad email and an over-long contact note.
fn broken_everywhere_chirho() -> PageDocumentV2Chirho {
    let mut document_chirho = valid_document_chirho();
    document_chirho.business_name_chirho = "   ".to_string();
    document_chirho.headline_chirho = String::new();
    document_chirho.summary_chirho = String::new();
    document_chirho.services_chirho[1] = PageServiceV1Chirho {
        title_chirho: "Callouts".to_string(),
        body_chirho: String::new(),
    };
    document_chirho.contact_phone_chirho = "call me".to_string();
    document_chirho.contact_email_chirho = "not-an-address".to_string();
    document_chirho.contact_details_chirho = "x".repeat(401);
    document_chirho
}

#[test]
fn every_reported_issue_is_shown_in_the_summary_and_none_is_dropped_chirho() {
    let (html_chirho, issue_count_chirho) = rejected_html_chirho(&broken_everywhere_chirho());
    assert!(issue_count_chirho >= 7, "the fixture should fail widely");
    let summary_chirho =
        &html_chirho[html_chirho.find("workspace-errors-heading-chirho").unwrap()..];
    let summary_chirho = &summary_chirho[..summary_chirho.find("</section>").unwrap()];
    assert_eq!(summary_chirho.matches("<li>").count(), issue_count_chirho);
}

#[test]
fn an_issue_with_no_control_to_point_at_still_reaches_the_summary_chirho() {
    let mut document_chirho = valid_document_chirho();
    document_chirho.schema_version_chirho = PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO + 1;
    let error_chirho = document_chirho.validate_chirho().unwrap_err();
    assert!(error_chirho.issues_chirho.iter().any(|issue_chirho| {
        issue_chirho.code_chirho == PageValidationCodeChirho::UnsupportedSchemaChirho
    }));
    let (html_chirho, issue_count_chirho) = rejected_html_chirho(&document_chirho);
    let summary_chirho =
        &html_chirho[html_chirho.find("workspace-errors-heading-chirho").unwrap()..];
    let summary_chirho = &summary_chirho[..summary_chirho.find("</section>").unwrap()];
    assert_eq!(summary_chirho.matches("<li>").count(), issue_count_chirho);
    // It has no home in the form, so the line is words, not a link to nowhere.
    assert_eq!(
        summary_chirho.matches("<a href=\"#").count(),
        issue_count_chirho - 1
    );
}

#[test]
fn every_summary_link_and_every_describing_id_resolves_in_the_same_document_chirho() {
    for document_chirho in [broken_everywhere_chirho(), {
        // The two group rules: no service at all, and neither a phone nor an email.
        let mut document_chirho = valid_document_chirho();
        document_chirho.services_chirho[0] = PageServiceV1Chirho::default();
        document_chirho.contact_phone_chirho = String::new();
        document_chirho
    }] {
        let (html_chirho, _) = rejected_html_chirho(&document_chirho);
        let ids_chirho = ids_chirho(&html_chirho);
        let fragments_chirho: Vec<String> = attribute_tokens_chirho(&html_chirho, "href")
            .into_iter()
            .filter_map(|href_chirho| href_chirho.strip_prefix('#').map(str::to_string))
            .collect();
        assert!(!fragments_chirho.is_empty());
        for fragment_chirho in &fragments_chirho {
            assert!(
                ids_chirho.contains(fragment_chirho),
                "dangling #{fragment_chirho}"
            );
        }
        for attribute_chirho in ["aria-describedby", "aria-labelledby", "for"] {
            for id_chirho in attribute_tokens_chirho(&html_chirho, attribute_chirho) {
                assert!(
                    ids_chirho.contains(&id_chirho),
                    "dangling {attribute_chirho} {id_chirho}"
                );
            }
        }
    }
}

#[test]
fn a_summary_link_lands_on_a_control_she_can_type_in_and_that_control_is_marked_chirho() {
    let (html_chirho, _) = rejected_html_chirho(&broken_everywhere_chirho());
    let summary_chirho =
        &html_chirho[html_chirho.find("workspace-errors-heading-chirho").unwrap()..];
    let summary_chirho = &summary_chirho[..summary_chirho.find("</section>").unwrap()];
    let targets_chirho: BTreeSet<String> = attribute_tokens_chirho(summary_chirho, "href")
        .into_iter()
        .filter_map(|href_chirho| href_chirho.strip_prefix('#').map(str::to_string))
        .collect();
    assert!(targets_chirho.len() >= 6);
    for target_chirho in &targets_chirho {
        let tag_chirho = opening_tag_chirho(&html_chirho, target_chirho);
        assert!(
            tag_chirho.starts_with("<input") || tag_chirho.starts_with("<textarea"),
            "{target_chirho} is not a control: {tag_chirho}"
        );
        assert!(tag_chirho.contains(" name=\""), "{tag_chirho}");
        assert!(
            tag_chirho.contains(" aria-invalid=\"true\""),
            "{tag_chirho}"
        );
        // Its own message, or its group's, is among what describes it.
        let described_chirho = attribute_tokens_chirho(tag_chirho, "aria-describedby");
        assert!(
            described_chirho
                .iter()
                .any(|id_chirho| id_chirho.ends_with("-error-chirho")),
            "{tag_chirho}"
        );
    }
}

/// The text of the first element that opens with `opening_chirho`, up to `closing_chirho`.
fn text_after_chirho(html_chirho: &str, opening_chirho: &str, closing_chirho: &str) -> String {
    let start_chirho = html_chirho
        .find(opening_chirho)
        .unwrap_or_else(|| panic!("no {opening_chirho}"))
        + opening_chirho.len();
    let after_chirho = &html_chirho[start_chirho..];
    after_chirho[..after_chirho.find(closing_chirho).unwrap()].to_string()
}

#[test]
fn a_summary_line_names_its_control_by_the_label_the_form_shows_chirho() {
    let (html_chirho, _) = rejected_html_chirho(&broken_everywhere_chirho());
    let mut compared_chirho = 0;
    for target_chirho in [
        "business-name-chirho",
        "headline-chirho",
        "contact-phone-chirho",
        "contact-email-chirho",
        "contact-details-chirho",
    ] {
        let link_chirho = text_after_chirho(
            &html_chirho,
            &format!("<a href=\"#{target_chirho}\">"),
            "</a>",
        );
        let label_chirho = text_after_chirho(
            &html_chirho,
            &format!("<label for=\"{target_chirho}\">"),
            "</label>",
        );
        assert_eq!(link_chirho, label_chirho, "{target_chirho}");
        compared_chirho += 1;
    }
    assert_eq!(compared_chirho, 5);

    // A group rule is named by the group's own legend.
    let mut document_chirho = valid_document_chirho();
    document_chirho.contact_phone_chirho = String::new();
    let (html_chirho, _) = rejected_html_chirho(&document_chirho);
    let link_chirho = text_after_chirho(&html_chirho, "<a href=\"#contact-phone-chirho\">", "</a>");
    let group_chirho = &html_chirho[html_chirho
        .find("<fieldset class=\"workspace-field-group-chirho\"")
        .unwrap()..];
    assert_eq!(
        link_chirho,
        text_after_chirho(group_chirho, "<legend>", "</legend>")
    );
}

#[test]
fn a_group_rule_is_said_once_on_the_group_and_described_into_each_member_chirho() {
    let mut document_chirho = valid_document_chirho();
    document_chirho.contact_phone_chirho = String::new();
    let (html_chirho, issue_count_chirho) = rejected_html_chirho(&document_chirho);
    assert_eq!(issue_count_chirho, 1);
    assert_eq!(
        html_chirho
            .matches(" id=\"contact-method-error-chirho\"")
            .count(),
        1
    );
    for member_chirho in ["contact-phone-chirho", "contact-email-chirho"] {
        let tag_chirho = opening_tag_chirho(&html_chirho, member_chirho);
        assert!(
            attribute_tokens_chirho(tag_chirho, "aria-describedby")
                .contains(&"contact-method-error-chirho".to_string()),
            "{tag_chirho}"
        );
    }
    // The group keeps its own semantics: a fieldset described by the rule and its message.
    let group_chirho = &html_chirho[..html_chirho
        .find(" id=\"contact-method-help-chirho\"")
        .unwrap()];
    let group_chirho = &group_chirho[group_chirho.rfind("<fieldset").unwrap()..];
    assert!(attribute_tokens_chirho(group_chirho, "aria-describedby")
        .contains(&"contact-method-error-chirho".to_string()));
    // A control outside the group is untouched by it.
    assert!(!opening_tag_chirho(&html_chirho, "contact-details-chirho").contains("aria-invalid"));
}

#[test]
fn a_problem_in_a_service_pair_says_which_pair_chirho() {
    let mut document_chirho = valid_document_chirho();
    for slot_chirho in [1, 2] {
        document_chirho.services_chirho[slot_chirho] = PageServiceV1Chirho {
            title_chirho: "Named".to_string(),
            body_chirho: String::new(),
        };
    }
    let (html_chirho, issue_count_chirho) = rejected_html_chirho(&document_chirho);
    assert_eq!(issue_count_chirho, 2);
    let label_chirho = |target_chirho: &str| {
        let link_chirho = format!("<a href=\"#{target_chirho}\">");
        let after_chirho =
            &html_chirho[html_chirho.find(&link_chirho).unwrap() + link_chirho.len()..];
        after_chirho[..after_chirho.find("</a>").unwrap()].to_string()
    };
    assert_ne!(
        label_chirho("service-2-body-chirho"),
        label_chirho("service-3-body-chirho")
    );
    assert!(opening_tag_chirho(&html_chirho, "service-2-body-chirho").contains("aria-invalid"));
    assert!(!opening_tag_chirho(&html_chirho, "service-1-body-chirho").contains("aria-invalid"));
}

#[test]
fn a_clean_page_carries_no_invalid_state_and_every_help_text_describes_its_control_chirho() {
    let document_chirho = valid_document_chirho();
    assert!(document_chirho.validate_chirho().is_ok());
    let model_chirho = WorkspacePageV1Chirho::editable_chirho(
        document_chirho,
        4,
        WorkspaceEditorBindingV1Chirho::published_contract_chirho(),
        &[],
        None,
    );
    let html_chirho = render_workspace_chirho(&model_chirho).unwrap();
    // the carrier's four dormant notices are hidden until targeted; everything else must be clean
    let clean_chirho = support_chirho::without_carrier_notices_chirho(&html_chirho);
    assert!(!clean_chirho.contains("aria-invalid"));
    assert!(!clean_chirho.contains("-error-chirho"));
    assert!(!clean_chirho.contains("role=\"alert\""));
    // Help text is a control's description, so each rendered help id is referenced by one.
    let described_chirho: BTreeSet<String> =
        attribute_tokens_chirho(&html_chirho, "aria-describedby")
            .into_iter()
            .collect();
    let help_ids_chirho: Vec<String> = ids_chirho(&html_chirho)
        .into_iter()
        .filter(|id_chirho| {
            id_chirho.ends_with("-help-chirho") && id_chirho != "workspace-help-chirho"
        })
        .collect();
    assert!(help_ids_chirho.len() >= 6, "{help_ids_chirho:?}");
    for help_id_chirho in &help_ids_chirho {
        assert!(
            described_chirho.contains(help_id_chirho),
            "{help_id_chirho} describes nothing"
        );
    }
    for id_chirho in &described_chirho {
        assert!(
            ids_chirho(&html_chirho).contains(id_chirho),
            "dangling {id_chirho}"
        );
    }
}

#[test]
fn a_refused_save_says_so_in_the_tab_title_and_a_saved_page_does_not_chirho() {
    let (rejected_chirho, _) = rejected_html_chirho(&broken_everywhere_chirho());
    let model_chirho = WorkspacePageV1Chirho::editable_chirho(
        valid_document_chirho(),
        4,
        WorkspaceEditorBindingV1Chirho::published_contract_chirho(),
        &[],
        None,
    );
    let saved_chirho = render_workspace_chirho(&model_chirho).unwrap();
    let title_chirho = |html_chirho: &str| {
        let start_chirho = html_chirho.find("<title>").unwrap() + "<title>".len();
        html_chirho[start_chirho..html_chirho.find("</title>").unwrap()].to_string()
    };
    assert_ne!(title_chirho(&rejected_chirho), title_chirho(&saved_chirho));
}

/// One field component, one shape, on every workspace page (was field_wiring_chirho.rs).
mod field_wiring_chirho {
    // One field component, one shape, on every workspace page: the label names the control, the help
    // text describes it, and nothing points at an id the page does not render.

    use perffection_workspace_chirho::account_page_chirho::{
        AccountBindingV1Chirho, AccountPageV1Chirho,
    };
    use perffection_workspace_chirho::draft_page_chirho::{
        DraftPageV1Chirho, WORKSPACE_DRAFT_ACTION_CHIRHO,
    };
    use perffection_workspace_chirho::page_document_import_chirho::PageDocumentImportV1Chirho;
    use perffection_workspace_chirho::page_render_chirho::{
        render_account_chirho, render_draft_chirho, render_page_document_import_chirho,
    };

    use super::support_chirho::assert_page_wiring_chirho;

    #[test]
    fn the_account_brief_and_import_pages_wire_every_label_and_help_text_chirho() {
        let account_chirho = render_account_chirho(&AccountPageV1Chirho::bound_chirho(
            AccountBindingV1Chirho::held_chirho()
                .with_change_password_action_chirho("/auth-chirho/credential-change-chirho")
                .with_recovery_code_action_chirho("/auth-chirho/recovery-code-chirho")
                .with_close_account_action_chirho("/auth-chirho/delete-account-chirho"),
        ))
        .unwrap();
        let brief_chirho = render_draft_chirho(&DraftPageV1Chirho::brief_chirho(Some(
            WORKSPACE_DRAFT_ACTION_CHIRHO,
        )))
        .unwrap();
        let import_chirho =
            render_page_document_import_chirho(&PageDocumentImportV1Chirho::ready_chirho(4))
                .unwrap();

        let mut help_texts_chirho = 0;
        for (page_chirho, html_chirho) in [
            ("account", &account_chirho),
            ("brief", &brief_chirho),
            ("import", &import_chirho),
        ] {
            assert!(
                html_chirho.contains("<form"),
                "{page_chirho} rendered no form"
            );
            help_texts_chirho += assert_page_wiring_chirho(page_chirho, html_chirho);
        }
        // The three pages carry help under their controls; zero would mean the check saw nothing.
        assert!(help_texts_chirho >= 6, "{help_texts_chirho}");
    }
}
