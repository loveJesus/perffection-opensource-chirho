// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use perffection_workspace_chirho::implementations_chirho::review_figures_chirho::review_figures_chirho as review_figures_impl_chirho;

fn review_figures_guest_chirho(input_chirho: &[u8]) -> Vec<u8> {
    review_figures_impl_chirho(input_chirho)
}

cairn_node_sdk_chirho::export_bytes_node_chirho!(
    review_figures_chirho,
    review_figures_guest_chirho
);
