// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

use perffection_workspace_chirho::implementations_chirho::render_figures_saved_chirho::render_figures_saved_chirho as render_figures_saved_impl_chirho;

fn render_figures_saved_guest_chirho(input_chirho: &[u8]) -> Vec<u8> {
    render_figures_saved_impl_chirho(input_chirho)
}

cairn_node_sdk_chirho::export_bytes_node_chirho!(
    render_figures_saved_chirho,
    render_figures_saved_guest_chirho
);
