// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed view model for the exact saved page document handed into reviewed publication.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::viewer_state_contract_chirho::PageStateSnapshotV1Chirho;
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
};
use perffection_ui_chirho::{SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho};
use std::fmt::Write as _;

pub const PAGE_DOCUMENT_DOWNLOAD_MEDIA_TYPE_CHIRHO: &str = "application/json;charset=utf-8";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageDocumentExportV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) status_chirho: String,
    pub(crate) current_version_chirho: u64,
    pub(crate) document_json_chirho: Option<String>,
    pub(crate) download_href_chirho: Option<String>,
    pub(crate) download_name_chirho: Option<String>,
}

impl PageDocumentExportV1Chirho {
    pub fn available_chirho(
        snapshot_chirho: &PageStateSnapshotV1Chirho,
    ) -> Result<Self, serde_json::Error> {
        Self::available_for_locale_chirho(snapshot_chirho, UiLocaleChirho::EnglishChirho)
    }

    pub fn available_for_locale_chirho(
        snapshot_chirho: &PageStateSnapshotV1Chirho,
        locale_chirho: UiLocaleChirho,
    ) -> Result<Self, serde_json::Error> {
        let version_chirho = snapshot_chirho.current_version_chirho();
        let document_json_chirho =
            serde_json::to_string_pretty(&snapshot_chirho.document_chirho().to_document_chirho())?;
        let download_href_chirho = format!(
            "data:{PAGE_DOCUMENT_DOWNLOAD_MEDIA_TYPE_CHIRHO},{}",
            percent_encode_chirho(&document_json_chirho)
        );
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let chrome_chirho = export_chrome_chirho(&copy_chirho);
        let status_chirho = copy_chirho
            .export_chirho
            .saved_status_chirho(version_chirho);
        Ok(Self {
            chrome_chirho,
            copy_chirho,
            status_chirho,
            current_version_chirho: version_chirho,
            document_json_chirho: Some(document_json_chirho),
            download_href_chirho: Some(download_href_chirho),
            download_name_chirho: Some(format!("perffection-page-document-v{version_chirho}.json")),
        })
    }

    pub fn unavailable_chirho(status_chirho: impl Into<String>) -> Self {
        Self::unavailable_for_locale_chirho(status_chirho, UiLocaleChirho::EnglishChirho)
    }

    pub fn unavailable_for_locale_chirho(
        status_chirho: impl Into<String>,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let chrome_chirho = export_chrome_chirho(&copy_chirho);
        Self {
            chrome_chirho,
            copy_chirho,
            status_chirho: status_chirho.into(),
            current_version_chirho: 0,
            document_json_chirho: None,
            download_href_chirho: None,
            download_name_chirho: None,
        }
    }

    pub fn nothing_saved_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho
            .export_chirho
            .nothing_saved_status_chirho
            .clone();
        Self::unavailable_for_locale_chirho(status_chirho, locale_chirho)
    }

    pub fn unverifiable_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho.export_chirho.unverifiable_status_chirho.clone();
        Self::unavailable_for_locale_chirho(status_chirho, locale_chirho)
    }

    pub fn document_json_chirho(&self) -> Option<&str> {
        self.document_json_chirho.as_deref()
    }

    pub fn download_href_chirho(&self) -> Option<&str> {
        self.download_href_chirho.as_deref()
    }
}

fn export_chrome_chirho(copy_chirho: &WorkspaceCopyChirho) -> SiteChromeChirho {
    workspace_chrome_chirho(
        copy_chirho,
        &copy_chirho.export_chirho.page_title_chirho,
        &copy_chirho.export_chirho.page_description_chirho,
        WorkspaceNavPageChirho::ExportChirho,
        WorkspaceNavRoadsChirho::deployed_chirho(),
    )
}

/// RFC 3986 percent-encoding of every byte outside the unreserved set.
pub fn percent_encode_chirho(value_chirho: &str) -> String {
    let mut encoded_chirho = String::with_capacity(value_chirho.len() * 3);
    for byte_chirho in value_chirho.bytes() {
        if byte_chirho.is_ascii_alphanumeric() || matches!(byte_chirho, b'-' | b'_' | b'.' | b'~') {
            encoded_chirho.push(byte_chirho as char);
        } else {
            let _ = write!(encoded_chirho, "%{byte_chirho:02X}");
        }
    }
    encoded_chirho
}
