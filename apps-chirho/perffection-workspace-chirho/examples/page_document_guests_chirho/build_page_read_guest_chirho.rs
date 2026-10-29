// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use perffection_workspace_chirho::implementations_chirho::build_page_read_chirho::build_page_read_chirho as build_page_read_impl_chirho;

fn build_page_read_guest_chirho(input_chirho: &[u8]) -> Vec<u8> {
    build_page_read_impl_chirho(input_chirho)
}

cairn_node_sdk_chirho::export_bytes_node_chirho!(
    build_page_read_chirho,
    build_page_read_guest_chirho
);
