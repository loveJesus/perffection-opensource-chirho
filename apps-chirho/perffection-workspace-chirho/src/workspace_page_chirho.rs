// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Pure view models for the private editor and exact page preview.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::editor_validation_chirho::{EditorFieldsChirho, EditorValidationViewChirho};
use crate::field_view_chirho::FormNoticeChirho;
use crate::page_document_chirho::{
    PageDocumentV2Chirho, PageDocumentValidationErrorChirho, BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
    CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO, CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
    CONTACT_HEADING_MAX_CHARACTERS_CHIRHO, CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
    HEADLINE_MAX_CHARACTERS_CHIRHO, SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
    SERVICE_TITLE_MAX_CHARACTERS_CHIRHO, SUMMARY_MAX_CHARACTERS_CHIRHO,
};
use crate::viewer_state_contract_chirho::{
    PageStateConflictV1Chirho, PageStateHistoryVersionV1Chirho,
};
pub use perffection_page_template_chirho::page_view_chirho::{
    PagePreviewV1Chirho, PAGE_STYLESHEET_HREF_CHIRHO,
};
use perffection_ui_chirho::{
    ChromeNavLinkChirho, SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho,
};

pub const WORKSPACE_STYLESHEET_HREF_CHIRHO: &str =
    "/app-assets-chirho/perffection-workspace/by-name-chirho/workspace-chirho.css";
pub const WORKSPACE_STYLESHEET_NAME_CHIRHO: &str = "workspace-chirho.css";
pub const WORKSPACE_HOME_HREF_CHIRHO: &str = "/app-chirho/perffection-workspace/";
pub const WORKSPACE_SAVE_ACTION_CHIRHO: &str = "/app-chirho/perffection-workspace/edit-page-chirho";
pub const WORKSPACE_RESTORE_ACTION_CHIRHO: &str =
    "/app-chirho/perffection-workspace/restore-page-chirho";
pub const WORKSPACE_PREVIEW_HREF_CHIRHO: &str = "/app-chirho/perffection-workspace/preview-chirho";
pub const WORKSPACE_EXPORT_HREF_CHIRHO: &str =
    "/app-chirho/perffection-workspace/page-document-chirho";
pub const WORKSPACE_IMPORT_HREF_CHIRHO: &str =
    "/app-chirho/perffection-workspace/import-page-document-chirho";
/// The Publishing page: every approval form lives there. The editor points at it only through
/// `with_publication_href_chirho`, the day Cairn's publication road is on the pin (route held).
pub const WORKSPACE_PUBLICATION_HREF_CHIRHO: &str =
    "/app-chirho/perffection-workspace/publication-status-chirho";
/// The publish door the Publishing page's forms post to (route held with it).
pub const WORKSPACE_PUBLISH_ACTION_CHIRHO: &str =
    "/app-chirho/perffection-workspace/publish-chirho";
/// The Account page (password, recovery code, closing the account); route held until cairn's
/// credential-change road lands (CAIRN #21830, R1 first), then linked like Publishing.
pub const WORKSPACE_ACCOUNT_HREF_CHIRHO: &str = "/app-chirho/perffection-workspace/account-chirho";
/// The public, actionless Help page of the customer site (gpt #22441); the in-page help asides
/// keep their own `#workspace-help-chirho` anchors.
pub const WORKSPACE_HELP_HREF_CHIRHO: &str = "/app-chirho/perffection/help-chirho";
pub const AUTH_LOGOUT_HREF_CHIRHO: &str = "/auth-chirho/logout-chirho";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceEditorBindingV1Chirho {
    pub(crate) save_action_chirho: String,
    pub(crate) restore_action_chirho: String,
    pub(crate) preview_href_chirho: String,
    pub(crate) export_href_chirho: String,
    /// The Publishing page; `None` until Cairn's publication road is on the pin, so the editor
    /// shows no publishing pointer at all rather than a link to a refused route.
    pub(crate) publication_href_chirho: Option<String>,
}

impl WorkspaceEditorBindingV1Chirho {
    /// The host may use this only after every named workspace route is installed and reviewed.
    pub fn route_contract_chirho() -> Self {
        Self {
            save_action_chirho: WORKSPACE_SAVE_ACTION_CHIRHO.to_string(),
            restore_action_chirho: WORKSPACE_RESTORE_ACTION_CHIRHO.to_string(),
            preview_href_chirho: WORKSPACE_PREVIEW_HREF_CHIRHO.to_string(),
            export_href_chirho: WORKSPACE_EXPORT_HREF_CHIRHO.to_string(),
            publication_href_chirho: None,
        }
    }

    /// The contract once the publication-status and publish routes are deployed alongside the
    /// others: the editor gains the Publishing nav link and a saved-state pointer.
    pub fn with_publication_href_chirho(mut self, publication_href_chirho: &str) -> Self {
        self.publication_href_chirho = Some(publication_href_chirho.to_string());
        self
    }

    /// The deployed contract: every workspace route including publishing (app-chirho.toml binds
    /// `publication-status-chirho` and `publish-chirho` together).
    pub fn published_contract_chirho() -> Self {
        Self::route_contract_chirho()
            .with_publication_href_chirho(WORKSPACE_PUBLICATION_HREF_CHIRHO)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceHistoryVersionV1Chirho {
    pub(crate) version_chirho: u64,
    pub(crate) business_name_chirho: String,
    pub(crate) headline_chirho: String,
    pub(crate) is_current_chirho: bool,
}

impl WorkspaceHistoryVersionV1Chirho {
    fn from_state_chirho(
        state_chirho: &PageStateHistoryVersionV1Chirho,
        current_version_chirho: u64,
    ) -> Self {
        Self {
            version_chirho: state_chirho.version_chirho(),
            business_name_chirho: state_chirho
                .document_chirho()
                .business_name_chirho()
                .to_string(),
            headline_chirho: state_chirho.document_chirho().headline_chirho().to_string(),
            is_current_chirho: state_chirho.version_chirho() == current_version_chirho,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkspaceFieldLimitsV1Chirho {
    pub(crate) business_name_chirho: usize,
    pub(crate) headline_chirho: usize,
    pub(crate) summary_chirho: usize,
    pub(crate) service_title_chirho: usize,
    pub(crate) service_body_chirho: usize,
    pub(crate) contact_heading_chirho: usize,
    pub(crate) contact_phone_chirho: usize,
    pub(crate) contact_email_chirho: usize,
    pub(crate) contact_details_chirho: usize,
}

impl Default for WorkspaceFieldLimitsV1Chirho {
    fn default() -> Self {
        Self {
            business_name_chirho: BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
            headline_chirho: HEADLINE_MAX_CHARACTERS_CHIRHO,
            summary_chirho: SUMMARY_MAX_CHARACTERS_CHIRHO,
            service_title_chirho: SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
            service_body_chirho: SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
            contact_heading_chirho: CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
            contact_phone_chirho: CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
            contact_email_chirho: CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
            contact_details_chirho: CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePageV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) draft_chirho: PageDocumentV2Chirho,
    pub(crate) current_version_chirho: u64,
    /// True only for the plain saved state: a version above zero, no validation notice, no
    /// page notice. Rejected, conflict and import-review renders echo unsaved words, so they
    /// never point at publishing as if those words were the approved page (gpt #21798).
    pub(crate) publishable_chirho: bool,
    /// "Publish version N" for the pointer to the Publishing page.
    pub(crate) publish_link_label_chirho: String,
    pub(crate) publish_pointer_body_chirho: String,
    pub(crate) limits_chirho: WorkspaceFieldLimitsV1Chirho,
    pub(crate) binding_chirho: Option<WorkspaceEditorBindingV1Chirho>,
    pub(crate) status_chirho: String,
    /// One page-level sentence when the whole submission was refused (a stale save).
    pub(crate) page_notice_chirho: Option<String>,
    /// Every validator issue, once: the summary at the top of the page.
    pub(crate) validation_notices_chirho: Vec<FormNoticeChirho>,
    /// The same issues beside the control or group each belongs to.
    pub(crate) fields_chirho: EditorFieldsChirho,
    /// The road that costs nothing: her own AI agent writes or improves the page.
    /// False when a refused submission is shown without pretending its retained history is empty.
    pub(crate) history_available_chirho: bool,
    pub(crate) history_chirho: Vec<WorkspaceHistoryVersionV1Chirho>,
}

impl WorkspacePageV1Chirho {
    /// Review customer-supplied content without treating it as a saved version or loading history.
    pub fn imported_chirho(
        draft_chirho: PageDocumentV2Chirho,
        expected_version_chirho: u64,
    ) -> Self {
        let validation_error_chirho = draft_chirho.validate_chirho().err();
        let mut model_chirho = Self::editable_chirho(
            draft_chirho,
            expected_version_chirho,
            WorkspaceEditorBindingV1Chirho::route_contract_chirho(),
            &[],
            validation_error_chirho.as_ref(),
        );
        model_chirho.status_chirho = model_chirho
            .copy_chirho
            .editor_chirho
            .status_imported_chirho
            .clone();
        model_chirho.page_notice_chirho = Some(
            model_chirho
                .copy_chirho
                .editor_chirho
                .import_review_notice_chirho
                .clone(),
        );
        model_chirho.history_available_chirho = false;
        model_chirho.publishable_chirho = false;
        model_chirho
    }

    /// Review the writing helper's first draft: the imported state at version 0 with the draft's
    /// own notice, plus a visible sentence when a helper-supplied contact value was not usable and
    /// was left blank for her to type (never a silent loss).
    pub fn drafted_chirho(
        draft_chirho: PageDocumentV2Chirho,
        dropped_contact_chirho: bool,
    ) -> Self {
        let mut model_chirho = Self::imported_chirho(draft_chirho, 0);
        let mut notice_chirho = model_chirho
            .copy_chirho
            .draft_chirho
            .review_notice_chirho
            .clone();
        if dropped_contact_chirho {
            notice_chirho.push(' ');
            notice_chirho.push_str(
                &model_chirho
                    .copy_chirho
                    .draft_chirho
                    .dropped_contact_notice_chirho,
            );
        }
        model_chirho.page_notice_chirho = Some(notice_chirho);
        model_chirho
    }

    pub fn editable_chirho(
        draft_chirho: PageDocumentV2Chirho,
        current_version_chirho: u64,
        binding_chirho: WorkspaceEditorBindingV1Chirho,
        history_chirho: &[PageStateHistoryVersionV1Chirho],
        validation_error_chirho: Option<&PageDocumentValidationErrorChirho>,
    ) -> Self {
        Self::editable_for_locale_chirho(
            draft_chirho,
            current_version_chirho,
            binding_chirho,
            history_chirho,
            validation_error_chirho,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn editable_for_locale_chirho(
        draft_chirho: PageDocumentV2Chirho,
        current_version_chirho: u64,
        binding_chirho: WorkspaceEditorBindingV1Chirho,
        history_chirho: &[PageStateHistoryVersionV1Chirho],
        validation_error_chirho: Option<&PageDocumentValidationErrorChirho>,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let EditorValidationViewChirho {
            notices_chirho: validation_notices_chirho,
            fields_chirho,
        } = EditorValidationViewChirho::from_error_chirho(validation_error_chirho, &copy_chirho);
        let chrome_chirho = editor_chrome_chirho(&copy_chirho, Some(&binding_chirho));
        let status_chirho = copy_chirho.editor_chirho.status_private_chirho.clone();

        let publishable_chirho = current_version_chirho > 0 && validation_notices_chirho.is_empty();
        // The editor reads page state only and cannot see what is live, so its pointer never
        // offers to publish a version: it opens the Publishing page, which knows.
        let publish_link_label_chirho = copy_chirho
            .publication_chirho
            .editor_publishing_link_chirho
            .clone();
        let publish_pointer_body_chirho = copy_chirho
            .publication_chirho
            .editor_pointer_chirho(current_version_chirho);
        Self {
            chrome_chirho,
            copy_chirho,
            draft_chirho,
            current_version_chirho,
            publishable_chirho,
            publish_link_label_chirho,
            publish_pointer_body_chirho,
            limits_chirho: WorkspaceFieldLimitsV1Chirho::default(),
            binding_chirho: Some(binding_chirho),
            status_chirho,
            page_notice_chirho: None,
            validation_notices_chirho,
            fields_chirho,
            history_available_chirho: true,
            history_chirho: history_chirho
                .iter()
                .map(|state_chirho| {
                    WorkspaceHistoryVersionV1Chirho::from_state_chirho(
                        state_chirho,
                        current_version_chirho,
                    )
                })
                .collect(),
        }
    }

    /// Re-renders a refused submission without changing the optimistic save contract or history.
    pub fn rejected_chirho(
        draft_chirho: PageDocumentV2Chirho,
        expected_current_version_chirho: u64,
        binding_chirho: WorkspaceEditorBindingV1Chirho,
        validation_error_chirho: &PageDocumentValidationErrorChirho,
    ) -> Self {
        Self::rejected_for_locale_chirho(
            draft_chirho,
            expected_current_version_chirho,
            binding_chirho,
            validation_error_chirho,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn rejected_for_locale_chirho(
        draft_chirho: PageDocumentV2Chirho,
        expected_current_version_chirho: u64,
        binding_chirho: WorkspaceEditorBindingV1Chirho,
        validation_error_chirho: &PageDocumentValidationErrorChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let mut model_chirho = Self::editable_for_locale_chirho(
            draft_chirho,
            expected_current_version_chirho,
            binding_chirho,
            &[],
            Some(validation_error_chirho),
            locale_chirho,
        );
        model_chirho.status_chirho = model_chirho
            .copy_chirho
            .editor_chirho
            .status_rejected_chirho
            .clone();
        model_chirho.mark_not_saved_title_chirho();
        model_chirho.history_available_chirho = false;
        model_chirho.publishable_chirho = false;
        model_chirho
    }

    pub fn unavailable_chirho(
        draft_chirho: PageDocumentV2Chirho,
        current_version_chirho: u64,
        reason_chirho: impl Into<String>,
    ) -> Self {
        Self::unavailable_for_locale_chirho(
            draft_chirho,
            current_version_chirho,
            reason_chirho,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn unavailable_for_locale_chirho(
        draft_chirho: PageDocumentV2Chirho,
        current_version_chirho: u64,
        reason_chirho: impl Into<String>,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let chrome_chirho = unavailable_editor_chrome_chirho(&copy_chirho);
        Self {
            chrome_chirho,
            copy_chirho,
            draft_chirho,
            current_version_chirho,
            publishable_chirho: false,
            publish_link_label_chirho: String::new(),
            publish_pointer_body_chirho: String::new(),
            limits_chirho: WorkspaceFieldLimitsV1Chirho::default(),
            binding_chirho: None,
            status_chirho: reason_chirho.into(),
            page_notice_chirho: None,
            validation_notices_chirho: Vec::new(),
            fields_chirho: EditorFieldsChirho::without_issues_chirho(),
            history_available_chirho: false,
            history_chirho: Vec::new(),
        }
    }

    pub fn unverifiable_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho.editor_chirho.unverifiable_status_chirho.clone();
        Self::unavailable_for_locale_chirho(
            PageDocumentV2Chirho::default(),
            0,
            status_chirho,
            locale_chirho,
        )
    }

    /// Renders a stale optimistic write at the verified head while keeping a verified attempt.
    pub fn conflict_chirho(
        conflict_chirho: &PageStateConflictV1Chirho,
        binding_chirho: WorkspaceEditorBindingV1Chirho,
    ) -> Self {
        Self::conflict_for_locale_chirho(
            conflict_chirho,
            binding_chirho,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn conflict_for_locale_chirho(
        conflict_chirho: &PageStateConflictV1Chirho,
        binding_chirho: WorkspaceEditorBindingV1Chirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let head_chirho = conflict_chirho.head_chirho();
        let head_version_chirho = head_chirho.current_version_chirho();
        let attempted_document_chirho = conflict_chirho.attempted_document_chirho().cloned();
        let draft_chirho = attempted_document_chirho
            .clone()
            .unwrap_or_else(|| head_chirho.document_chirho().to_document_chirho());
        let mut model_chirho = Self::editable_for_locale_chirho(
            draft_chirho,
            head_version_chirho,
            binding_chirho,
            head_chirho.history_chirho(),
            None,
            locale_chirho,
        );
        model_chirho.status_chirho = model_chirho
            .copy_chirho
            .editor_chirho
            .status_conflict_chirho
            .clone();
        model_chirho.mark_not_saved_title_chirho();
        model_chirho.page_notice_chirho = Some(if attempted_document_chirho.is_some() {
            model_chirho
                .copy_chirho
                .editor_chirho
                .conflict_attempted_chirho(head_version_chirho)
        } else {
            model_chirho
                .copy_chirho
                .editor_chirho
                .conflict_head_chirho(head_version_chirho)
        });
        model_chirho.publishable_chirho = false;
        model_chirho
    }

    /// A refused or raced save says so in the tab's title, which a screen reader announces first.
    fn mark_not_saved_title_chirho(&mut self) {
        self.chrome_chirho.page_title_chirho = self
            .copy_chirho
            .editor_chirho
            .not_saved_page_title_chirho
            .clone();
    }

    pub fn is_saved_state_chirho(&self) -> bool {
        self.history_available_chirho
    }

    pub fn page_notice_chirho(&self) -> Option<&str> {
        self.page_notice_chirho.as_deref()
    }
}

/// Which workspace page the chrome is on; that page's link renders as the current one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspaceNavPageChirho {
    EditorChirho,
    DraftChirho,
    PublishingChirho,
    ExportChirho,
    ImportChirho,
    AccountChirho,
    /// Her figures: no nav link while the routes are held (unrouted until the K ruling).
    FiguresChirho,
    /// Her credits: no nav link while the page is held (unrouted until Cairn's ledger port and L.J.'s word).
    CreditsChirho,
}

/// Which held roads are on the pin. A held road's link is omitted rather than shown dead, and
/// the deployed set lives in one place so a landing flips one switch for every page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorkspaceNavRoadsChirho {
    pub publication_chirho: bool,
    pub account_chirho: bool,
    /// The first-draft brief: held until the served `llm` card runs.
    pub draft_chirho: bool,
}

impl WorkspaceNavRoadsChirho {
    /// The roads bound on the current pin: publication landed 2026-09-10; the Account page is
    /// on the pin with cairn's credential-change road (CAIRN #21925).
    pub const fn deployed_chirho() -> Self {
        Self {
            publication_chirho: true,
            account_chirho: true,
            draft_chirho: false,
        }
    }
}

/// One nav for every workspace page, in one order: Back to your page, Preview, Publishing, Copy my
/// page document, Recover page content, Account, Help, Log out — the current page marked, held
/// roads omitted.
pub(crate) fn workspace_chrome_chirho(
    copy_chirho: &WorkspaceCopyChirho,
    page_title_chirho: &str,
    page_description_chirho: &str,
    page_chirho: WorkspaceNavPageChirho,
    roads_chirho: WorkspaceNavRoadsChirho,
) -> SiteChromeChirho {
    let chrome_chirho = &copy_chirho.chrome_chirho;
    let mut links_chirho = Vec::with_capacity(8);
    let mut push_chirho = |label_chirho: &str, href_chirho: &str, current_chirho: bool| {
        links_chirho.push(if current_chirho {
            ChromeNavLinkChirho::current_chirho(label_chirho, href_chirho)
        } else {
            ChromeNavLinkChirho::link_chirho(label_chirho, href_chirho)
        });
    };
    if page_chirho != WorkspaceNavPageChirho::EditorChirho {
        push_chirho(
            &chrome_chirho.editor_label_chirho,
            WORKSPACE_HOME_HREF_CHIRHO,
            false,
        );
    }
    push_chirho(
        &chrome_chirho.preview_label_chirho,
        WORKSPACE_PREVIEW_HREF_CHIRHO,
        false,
    );
    if roads_chirho.draft_chirho {
        push_chirho(
            &chrome_chirho.draft_label_chirho,
            crate::draft_page_chirho::WORKSPACE_DRAFT_HREF_CHIRHO,
            page_chirho == WorkspaceNavPageChirho::DraftChirho,
        );
    }
    if roads_chirho.publication_chirho {
        push_chirho(
            &chrome_chirho.publish_label_chirho,
            WORKSPACE_PUBLICATION_HREF_CHIRHO,
            page_chirho == WorkspaceNavPageChirho::PublishingChirho,
        );
    }
    push_chirho(
        &chrome_chirho.export_label_chirho,
        WORKSPACE_EXPORT_HREF_CHIRHO,
        page_chirho == WorkspaceNavPageChirho::ExportChirho,
    );
    push_chirho(
        &chrome_chirho.import_label_chirho,
        WORKSPACE_IMPORT_HREF_CHIRHO,
        page_chirho == WorkspaceNavPageChirho::ImportChirho,
    );
    if roads_chirho.account_chirho {
        push_chirho(
            &chrome_chirho.account_label_chirho,
            WORKSPACE_ACCOUNT_HREF_CHIRHO,
            page_chirho == WorkspaceNavPageChirho::AccountChirho,
        );
    }
    push_chirho(
        &chrome_chirho.help_label_chirho,
        WORKSPACE_HELP_HREF_CHIRHO,
        false,
    );
    links_chirho.push(ChromeNavLinkChirho::cta_chirho(
        &chrome_chirho.logout_label_chirho,
        AUTH_LOGOUT_HREF_CHIRHO,
    ));
    copy_chirho.site_chrome_chirho(
        page_title_chirho,
        page_description_chirho,
        WORKSPACE_STYLESHEET_NAME_CHIRHO,
        WORKSPACE_HOME_HREF_CHIRHO,
        links_chirho,
    )
}

/// The chrome of a page that could not verify her state: Help and Log out only, no private links.
pub(crate) fn workspace_chrome_without_private_links_chirho(
    copy_chirho: &WorkspaceCopyChirho,
    page_title_chirho: &str,
    page_description_chirho: &str,
) -> SiteChromeChirho {
    copy_chirho.site_chrome_chirho(
        page_title_chirho,
        page_description_chirho,
        WORKSPACE_STYLESHEET_NAME_CHIRHO,
        WORKSPACE_HOME_HREF_CHIRHO,
        vec![
            ChromeNavLinkChirho::link_chirho(
                &copy_chirho.chrome_chirho.help_label_chirho,
                WORKSPACE_HELP_HREF_CHIRHO,
            ),
            ChromeNavLinkChirho::cta_chirho(
                &copy_chirho.chrome_chirho.logout_label_chirho,
                AUTH_LOGOUT_HREF_CHIRHO,
            ),
        ],
    )
}

fn editor_chrome_chirho(
    copy_chirho: &WorkspaceCopyChirho,
    binding_chirho: Option<&WorkspaceEditorBindingV1Chirho>,
) -> SiteChromeChirho {
    match binding_chirho {
        Some(binding_chirho) => workspace_chrome_chirho(
            copy_chirho,
            &copy_chirho.editor_chirho.page_title_chirho,
            &copy_chirho.editor_chirho.page_description_chirho,
            WorkspaceNavPageChirho::EditorChirho,
            WorkspaceNavRoadsChirho {
                publication_chirho: binding_chirho.publication_href_chirho.is_some(),
                ..WorkspaceNavRoadsChirho::deployed_chirho()
            },
        ),
        None => workspace_chrome_without_private_links_chirho(
            copy_chirho,
            &copy_chirho.editor_chirho.page_title_chirho,
            &copy_chirho.editor_chirho.page_description_chirho,
        ),
    }
}

fn unavailable_editor_chrome_chirho(copy_chirho: &WorkspaceCopyChirho) -> SiteChromeChirho {
    let mut chrome_chirho = editor_chrome_chirho(copy_chirho, None);
    chrome_chirho.page_title_chirho = copy_chirho
        .editor_chirho
        .unavailable_page_title_chirho
        .clone();
    chrome_chirho
}
