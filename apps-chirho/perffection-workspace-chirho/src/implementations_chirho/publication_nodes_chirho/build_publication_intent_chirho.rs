// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Intent branch of the publish route in the page-document lifecycle workflow: her approval form
//! becomes the door request, or the Skipped stone when the rejection branch is the live one.

use crate::publication_intent_chirho::{
    build_publication_intent_request_chirho, parse_publication_form_chirho,
};

pub fn build_publication_intent_chirho(request_bytes_chirho: &[u8]) -> Vec<u8> {
    match parse_publication_form_chirho(request_bytes_chirho) {
        Ok(approval_chirho) => build_publication_intent_request_chirho(&approval_chirho),
        Err(_error_chirho) => cairn_node_sdk_chirho::skipped_bytes_chirho(),
    }
}
