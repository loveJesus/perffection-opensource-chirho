// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

fn render_auth_input_chirho(input_chirho: &[u8]) -> Vec<u8> {
    perffection_site_chirho::page_render_chirho::render_auth_input_chirho(
        input_chirho,
        perffection_ui_chirho::UiLocaleChirho::EnglishChirho,
    )
    .expect("auth display input must satisfy the typed contract")
    .into_bytes_chirho()
}

cairn_node_sdk_chirho::export_bytes_node_chirho!(render_auth_chirho, render_auth_input_chirho);
