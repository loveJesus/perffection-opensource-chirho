// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Public display nodes ignore request bytes. This shared adapter keeps the bytes ABI out of pages.
//! Workflow: spec-chirho/workflows-chirho/product-delivery-chirho/product-authoring-chirho.md.

#[macro_export]
macro_rules! export_public_page_chirho {
    ($entry_chirho:ident) => {
        fn render_public_input_chirho(_input_chirho: &[u8]) -> Vec<u8> {
            $crate::implementations_chirho::$entry_chirho::$entry_chirho(
                &$crate::types_chirho::http_request_v1_chirho::HttpRequestV1Chirho::default(),
            )
            .into_bytes_chirho()
        }
        cairn_node_sdk_chirho::export_bytes_node_chirho!($entry_chirho, render_public_input_chirho);
    };
}
