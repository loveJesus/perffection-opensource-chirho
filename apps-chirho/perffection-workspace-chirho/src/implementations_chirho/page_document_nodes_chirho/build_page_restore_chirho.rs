// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Private restore-form builder in the page-document lifecycle workflow.

use crate::page_edit_request_chirho::parse_page_restore_form_chirho;
use crate::viewer_state_contract_chirho::build_page_restore_request_chirho;

const INVALID_PAGE_RESTORE_REQUEST_CHIRHO: &[u8] =
    br#"{"schema_chirho":"invalid-page-restore-request-v1-chirho"}"#;

pub fn build_page_restore_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    let request_chirho = match parse_page_restore_form_chirho(request_bytes_chirho) {
        Ok(request_chirho) => request_chirho,
        Err(_error_chirho) => return INVALID_PAGE_RESTORE_REQUEST_CHIRHO.to_vec(),
    };
    build_page_restore_request_chirho(
        request_chirho.expected_current_version_chirho(),
        request_chirho.restore_from_version_chirho(),
    )
    .unwrap_or_else(|_error_chirho| INVALID_PAGE_RESTORE_REQUEST_CHIRHO.to_vec())
}
