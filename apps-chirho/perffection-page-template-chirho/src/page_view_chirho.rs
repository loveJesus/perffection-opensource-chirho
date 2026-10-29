// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Typed view model for a private preview or a published customer page.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.
//! Rendering/copy: `spec-chirho/workflows-chirho/product-delivery-chirho/customer-page-rendering-chirho.md`.

use crate::page_document_chirho::ValidatedPageDocumentV2Chirho;
use perffection_ui_chirho::{PageCopyChirho, UiLocaleChirho};

pub const PAGE_STYLESHEET_HREF_CHIRHO: &str =
    "/app-assets-chirho/perffection-page-template-chirho/by-name-chirho/page-chirho.css";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PagePreviewV1Chirho {
    pub(crate) copy_chirho: PageCopyChirho,
    pub(crate) page_title_chirho: String,
    pub(crate) page_description_chirho: String,
    pub(crate) stylesheet_href_chirho: String,
    pub(crate) document_chirho: Option<ValidatedPageDocumentV2Chirho>,
    pub(crate) preview_return_href_chirho: Option<String>,
}

impl PagePreviewV1Chirho {
    pub fn private_preview_chirho(
        document_chirho: ValidatedPageDocumentV2Chirho,
        preview_return_href_chirho: impl Into<String>,
    ) -> Self {
        Self::private_preview_for_locale_chirho(
            document_chirho,
            preview_return_href_chirho,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn private_preview_for_locale_chirho(
        document_chirho: ValidatedPageDocumentV2Chirho,
        preview_return_href_chirho: impl Into<String>,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = PageCopyChirho::for_locale_chirho(locale_chirho);
        Self {
            page_title_chirho: copy_chirho
                .preview_title_chirho(document_chirho.business_name_chirho()),
            copy_chirho,
            page_description_chirho: document_chirho.summary_chirho().to_string(),
            stylesheet_href_chirho: PAGE_STYLESHEET_HREF_CHIRHO.to_string(),
            document_chirho: Some(document_chirho),
            preview_return_href_chirho: Some(preview_return_href_chirho.into()),
        }
    }

    pub fn published_chirho(document_chirho: ValidatedPageDocumentV2Chirho) -> Self {
        Self::published_for_locale_chirho(document_chirho, UiLocaleChirho::EnglishChirho)
    }

    /// Locale is a trusted presentation choice, not a new field in signed PageDocumentV2 params.
    pub fn published_for_locale_chirho(
        document_chirho: ValidatedPageDocumentV2Chirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        Self {
            copy_chirho: PageCopyChirho::for_locale_chirho(locale_chirho),
            page_title_chirho: document_chirho.business_name_chirho().to_string(),
            page_description_chirho: document_chirho.summary_chirho().to_string(),
            stylesheet_href_chirho: PAGE_STYLESHEET_HREF_CHIRHO.to_string(),
            document_chirho: Some(document_chirho),
            preview_return_href_chirho: None,
        }
    }

    pub fn public_unavailable_chirho() -> Self {
        Self::public_unavailable_for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    pub fn public_unavailable_for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let copy_chirho = PageCopyChirho::for_locale_chirho(locale_chirho);
        Self {
            page_title_chirho: copy_chirho.unavailable_title_chirho.clone(),
            page_description_chirho: copy_chirho.unavailable_heading_chirho.clone(),
            copy_chirho,
            stylesheet_href_chirho: PAGE_STYLESHEET_HREF_CHIRHO.to_string(),
            document_chirho: None,
            preview_return_href_chirho: None,
        }
    }
}
