// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed, actionless-until-review view model for bounded PageDocumentV2 content recovery.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::field_view_chirho::FieldViewChirho;
pub use crate::page_import_request_chirho::{
    ImportNoticeChirho,
    PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO as PAGE_DOCUMENT_IMPORT_MAX_BYTES_CHIRHO,
};
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
    WORKSPACE_HOME_HREF_CHIRHO, WORKSPACE_IMPORT_HREF_CHIRHO,
};
use perffection_ui_chirho::{
    SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho, WorkspaceImportCopyChirho,
};

pub const PAGE_DOCUMENT_IMPORT_MAX_CHARACTERS_CHIRHO: usize = PAGE_DOCUMENT_IMPORT_MAX_BYTES_CHIRHO;
pub const WORKSPACE_IMPORT_REVIEW_ACTION_CHIRHO: &str =
    "/app-chirho/perffection-workspace/review-page-document-chirho";

/// The import form's one control and what describes it.
pub const IMPORT_DOCUMENT_CONTROL_ID_CHIRHO: &str = "page-document-import-chirho";
const IMPORT_DOCUMENT_HELP_ID_CHIRHO: &str = "page-document-import-help-chirho";
const IMPORT_DOCUMENT_MESSAGE_ID_CHIRHO: &str = "page-document-import-error-chirho";

impl ImportNoticeChirho {
    fn message_chirho(self, copy_chirho: &WorkspaceImportCopyChirho) -> &str {
        match self {
            Self::InvalidFormChirho => &copy_chirho.invalid_form_notice_chirho,
            Self::InvalidJsonChirho => &copy_chirho.invalid_json_notice_chirho,
            Self::UnsupportedSchemaChirho => &copy_chirho.unsupported_schema_notice_chirho,
            Self::DocumentTooLargeChirho => &copy_chirho.document_too_large_notice_chirho,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageDocumentImportV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) status_chirho: String,
    pub(crate) expected_version_chirho: u64,
    pub(crate) document_json_chirho: String,
    /// Why the pasted document could not be reviewed: said at the top as a link to the field,
    /// and again beside it through `document_field_chirho`.
    pub(crate) notice_chirho: Option<String>,
    /// The one control of the form: described by its help text, and by the notice when there is
    /// one. The template's literal ids must match (the render tests resolve them).
    pub(crate) document_field_chirho: FieldViewChirho,
    pub(crate) review_action_chirho: Option<String>,
    pub(crate) reopen_href_chirho: Option<String>,
    pub(crate) back_href_chirho: String,
    pub(crate) max_document_characters_chirho: usize,
    pub(crate) failure_heading_chirho: String,
    pub(crate) failure_body_chirho: String,
}

impl PageDocumentImportV1Chirho {
    pub fn ready_chirho(expected_version_chirho: u64) -> Self {
        Self::ready_for_locale_chirho(expected_version_chirho, UiLocaleChirho::EnglishChirho)
    }

    pub fn ready_for_locale_chirho(
        expected_version_chirho: u64,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho.import_chirho.status_ready_chirho.clone();
        Self::form_chirho(
            copy_chirho,
            status_chirho,
            expected_version_chirho,
            String::new(),
            None,
        )
    }

    pub fn rejected_chirho(
        expected_version_chirho: u64,
        document_json_chirho: String,
        notice_chirho: ImportNoticeChirho,
    ) -> Self {
        Self::rejected_for_locale_chirho(
            expected_version_chirho,
            document_json_chirho,
            notice_chirho,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn rejected_for_locale_chirho(
        expected_version_chirho: u64,
        document_json_chirho: String,
        notice_chirho: ImportNoticeChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho.import_chirho.status_rejected_chirho.clone();
        let (document_json_chirho, notice_chirho) =
            if document_json_chirho.len() > PAGE_DOCUMENT_IMPORT_MAX_BYTES_CHIRHO {
                (String::new(), ImportNoticeChirho::DocumentTooLargeChirho)
            } else {
                (document_json_chirho, notice_chirho)
            };
        let notice_message_chirho = notice_chirho
            .message_chirho(&copy_chirho.import_chirho)
            .to_string();
        Self::form_chirho(
            copy_chirho,
            status_chirho,
            expected_version_chirho,
            document_json_chirho,
            Some(notice_message_chirho),
        )
    }

    pub fn unreadable_form_chirho() -> Self {
        Self::unreadable_form_for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    pub fn unreadable_form_for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho.import_chirho.status_unreadable_chirho.clone();
        let failure_heading_chirho = copy_chirho.import_chirho.unreadable_heading_chirho.clone();
        let failure_body_chirho = copy_chirho.import_chirho.unreadable_body_chirho.clone();
        Self::actionless_chirho(
            copy_chirho,
            status_chirho,
            failure_heading_chirho,
            failure_body_chirho,
            Some(WORKSPACE_IMPORT_HREF_CHIRHO.to_string()),
        )
    }

    pub fn unavailable_chirho() -> Self {
        Self::unavailable_for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    pub fn unavailable_for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho.import_chirho.status_unavailable_chirho.clone();
        let failure_heading_chirho = copy_chirho.import_chirho.unavailable_heading_chirho.clone();
        let failure_body_chirho = copy_chirho.import_chirho.unavailable_body_chirho.clone();
        Self::actionless_chirho(
            copy_chirho,
            status_chirho,
            failure_heading_chirho,
            failure_body_chirho,
            None,
        )
    }

    fn form_chirho(
        copy_chirho: WorkspaceCopyChirho,
        status_chirho: String,
        expected_version_chirho: u64,
        document_json_chirho: String,
        notice_chirho: Option<String>,
    ) -> Self {
        let chrome_chirho = import_chrome_chirho(&copy_chirho);
        let mut document_field_chirho =
            FieldViewChirho::clean_chirho(IMPORT_DOCUMENT_MESSAGE_ID_CHIRHO.to_string());
        document_field_chirho
            .messages_chirho
            .extend(notice_chirho.iter().cloned());
        document_field_chirho.settle_chirho(Some(IMPORT_DOCUMENT_HELP_ID_CHIRHO.to_string()), None);
        Self {
            chrome_chirho,
            copy_chirho,
            status_chirho,
            expected_version_chirho,
            document_json_chirho,
            notice_chirho,
            document_field_chirho,
            review_action_chirho: Some(WORKSPACE_IMPORT_REVIEW_ACTION_CHIRHO.to_string()),
            reopen_href_chirho: None,
            back_href_chirho: WORKSPACE_HOME_HREF_CHIRHO.to_string(),
            max_document_characters_chirho: PAGE_DOCUMENT_IMPORT_MAX_CHARACTERS_CHIRHO,
            failure_heading_chirho: String::new(),
            failure_body_chirho: String::new(),
        }
    }

    fn actionless_chirho(
        copy_chirho: WorkspaceCopyChirho,
        status_chirho: String,
        failure_heading_chirho: String,
        failure_body_chirho: String,
        reopen_href_chirho: Option<String>,
    ) -> Self {
        let chrome_chirho = import_chrome_chirho(&copy_chirho);
        Self {
            chrome_chirho,
            copy_chirho,
            status_chirho,
            expected_version_chirho: 0,
            document_json_chirho: String::new(),
            notice_chirho: None,
            document_field_chirho: FieldViewChirho::clean_chirho(
                IMPORT_DOCUMENT_MESSAGE_ID_CHIRHO.to_string(),
            ),
            review_action_chirho: None,
            reopen_href_chirho,
            back_href_chirho: WORKSPACE_HOME_HREF_CHIRHO.to_string(),
            max_document_characters_chirho: PAGE_DOCUMENT_IMPORT_MAX_CHARACTERS_CHIRHO,
            failure_heading_chirho,
            failure_body_chirho,
        }
    }
}

fn import_chrome_chirho(copy_chirho: &WorkspaceCopyChirho) -> SiteChromeChirho {
    workspace_chrome_chirho(
        copy_chirho,
        &copy_chirho.import_chirho.page_title_chirho,
        &copy_chirho.import_chirho.page_description_chirho,
        WorkspaceNavPageChirho::ImportChirho,
        WorkspaceNavRoadsChirho::deployed_chirho(),
    )
}
