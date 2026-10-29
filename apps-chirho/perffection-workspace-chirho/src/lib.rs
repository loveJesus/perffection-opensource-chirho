// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Product-owned pure layer for Perffection's first invite-only workspace.
//!
//! This crate owns the bounded page document, validation, view models, editor-friendly templates,
//! and pure guest-node implementations. It depends on Cairn only through the narrow node ABI SDK;
//! the reviewed DAG and host runtime supply authenticated viewer state and effects without bypassing
//! these validation and render boundaries.

pub mod account_page_chirho;
pub mod agent_api_chirho;
pub mod connections_chirho;
pub mod credits_flow_chirho;
pub mod draft_flow_chirho;
pub mod figures_flow_chirho;
pub mod form_views_chirho;
pub mod implementations_chirho;
pub mod page_document_flow_chirho;
pub mod page_render_chirho;
pub mod publication_flow_chirho;
pub mod viewer_state_chirho;
pub use viewer_state_chirho::page_contract_chirho as viewer_state_contract_chirho;
pub mod workspace_page_chirho;

// The modules grouped above keep the paths every node, guest example and test already names
// (`crate::first_draft_chirho`, `perffection_workspace_chirho::publication_status_chirho`, ...).
pub use agent_api_chirho::{agent_page_chirho, agent_save_chirho, agent_surface_chirho};
pub use draft_flow_chirho::{draft_brief_form_chirho, draft_page_chirho, first_draft_chirho};
pub use form_views_chirho::{editor_validation_chirho, field_view_chirho};
pub use page_document_flow_chirho::{
    page_document_chirho, page_document_export_chirho, page_document_import_chirho,
    page_edit_request_chirho, page_import_request_chirho, save_page_decision_chirho,
};
pub use publication_flow_chirho::{publication_intent_chirho, publication_status_chirho};
