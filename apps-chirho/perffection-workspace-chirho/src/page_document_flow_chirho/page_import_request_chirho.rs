// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Bounded, data-only import review; never creates a storage request or publication authority.
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::page_document_chirho::{PageDocumentV2Chirho, PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO};
use crate::page_edit_request_chirho::parse_form_fields_chirho;

pub const PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO: usize = 16 * 1024;
pub const PAGE_IMPORT_FORM_MAX_BYTES_CHIRHO: usize =
    PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO * 3 + 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportNoticeChirho {
    InvalidFormChirho,
    InvalidJsonChirho,
    UnsupportedSchemaChirho,
    DocumentTooLargeChirho,
}

#[derive(Debug)]
pub enum PageImportOutcomeChirho {
    /// Submitted version is only an optimistic-concurrency precondition, never viewer authority.
    DraftChirho {
        document_chirho: Box<PageDocumentV2Chirho>,
        expected_version_chirho: u64,
    },
    RejectedChirho {
        expected_version_chirho: u64,
        document_json_chirho: String,
        notice_chirho: ImportNoticeChirho,
    },
    UnreadableFormChirho,
}

pub fn parse_page_import_chirho(body_chirho: &[u8]) -> PageImportOutcomeChirho {
    let Ok(mut fields_chirho) =
        parse_form_fields_chirho(body_chirho, PAGE_IMPORT_FORM_MAX_BYTES_CHIRHO)
    else {
        return PageImportOutcomeChirho::UnreadableFormChirho;
    };
    let Some(expected_version_chirho) = fields_chirho
        .remove("expected_version_chirho")
        .and_then(|value_chirho| value_chirho.parse::<u64>().ok())
    else {
        return PageImportOutcomeChirho::UnreadableFormChirho;
    };
    let Some(document_json_chirho) = fields_chirho.remove("page_document_chirho") else {
        return PageImportOutcomeChirho::UnreadableFormChirho;
    };
    if !fields_chirho.is_empty() {
        return PageImportOutcomeChirho::UnreadableFormChirho;
    }
    if document_json_chirho.len() > PAGE_IMPORT_DOCUMENT_MAX_BYTES_CHIRHO {
        return PageImportOutcomeChirho::RejectedChirho {
            expected_version_chirho,
            // Do not amplify an over-limit input into the response.
            document_json_chirho: String::new(),
            notice_chirho: ImportNoticeChirho::DocumentTooLargeChirho,
        };
    }
    let notice_chirho = match serde_json::from_str::<PageDocumentV2Chirho>(&document_json_chirho) {
        Ok(document_chirho)
            if document_chirho.schema_version_chirho == PAGE_DOCUMENT_SCHEMA_VERSION_CHIRHO =>
        {
            return PageImportOutcomeChirho::DraftChirho {
                document_chirho: Box::new(document_chirho),
                expected_version_chirho,
            };
        }
        Ok(_) => ImportNoticeChirho::UnsupportedSchemaChirho,
        Err(_) => ImportNoticeChirho::InvalidJsonChirho,
    };
    PageImportOutcomeChirho::RejectedChirho {
        expected_version_chirho,
        document_json_chirho,
        notice_chirho,
    }
}
