// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The pure guest-node implementations, grouped by the route family they serve. Every node keeps
//! its old path (`implementations_chirho::<node>::<fn>`), which the guest examples and the
//! tests name, through the re-exports below.

pub mod account_nodes_chirho;
pub mod agent_api_nodes_chirho;
pub mod figures_nodes_chirho;
pub mod first_draft_nodes_chirho;
pub mod page_document_nodes_chirho;
pub mod publication_nodes_chirho;

pub use account_nodes_chirho::{
    build_own_keys_read_chirho, render_account_chirho, render_account_connect_chirho,
    render_account_keys_chirho,
};
pub use agent_api_nodes_chirho::{
    build_agent_page_write_chirho, render_agent_page_chirho, render_agent_save_outcome_chirho,
    render_agent_save_refusal_chirho,
};
pub use figures_nodes_chirho::{
    build_agent_figures_write_chirho, build_figures_read_chirho, build_figures_write_chirho,
    render_agent_figures_chirho, render_agent_figures_refusal_chirho,
    render_agent_figures_saved_chirho, render_figures_page_chirho, render_figures_rejection_chirho,
    render_figures_saved_chirho, review_figures_chirho,
};
pub use first_draft_nodes_chirho::{
    build_draft_request_chirho, render_draft_brief_chirho, render_draft_brief_rejection_chirho,
    render_draft_review_chirho,
};
pub use page_document_nodes_chirho::{
    build_page_read_chirho, build_page_restore_chirho, build_page_write_chirho,
    render_page_document_export_chirho, render_page_document_import_chirho,
    render_page_rejection_chirho, render_private_preview_chirho, render_workspace_state_chirho,
    review_page_document_chirho,
};
pub use publication_nodes_chirho::{
    build_publication_intent_chirho, build_publication_status_read_chirho,
    render_publication_form_rejection_chirho, render_publication_intent_chirho,
    render_publication_status_chirho,
};
