// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Host-injected request contract for the public template.
//!
//! The no-body app surface supplies `app_params_chirho` from Cairn's signed owner-scoped params
//! record. Browser input cannot select or replace that field.

use crate::page_document_chirho::PageDocumentV2Chirho;
use serde::Deserialize;

pub const PUBLISHED_PAGE_REQUEST_MAX_BYTES_CHIRHO: usize = 16 * 1024;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedPageRequestV1Chirho {
    #[serde(default)]
    pub path_chirho: String,
    #[serde(default)]
    pub user_principal_chirho: Option<String>,
    #[serde(default)]
    pub app_params_chirho: Option<PageDocumentV2Chirho>,
}

pub fn decode_published_page_request_chirho(
    input_chirho: &[u8],
) -> Result<PublishedPageRequestV1Chirho, serde_json::Error> {
    serde_json::from_slice(input_chirho)
}
