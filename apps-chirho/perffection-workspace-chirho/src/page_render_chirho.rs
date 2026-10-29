// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Thin typed wrappers around markup owned by the checked-in templates.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::{
    account_page_chirho::AccountPageV1Chirho,
    draft_page_chirho::{DraftCreditsChirho, DraftPageKindChirho, DraftPageV1Chirho},
    figures_flow_chirho::figures_page_chirho::{FiguresPageV1Chirho, FiguresReviewV1Chirho},
    page_document_export_chirho::PageDocumentExportV1Chirho,
    page_document_import_chirho::PageDocumentImportV1Chirho,
    publication_intent_chirho::PublicationIntentV1Chirho,
    publication_status_chirho::PublicationStatusV1Chirho,
    workspace_page_chirho::WorkspacePageV1Chirho,
};
use askama::Template;
pub use perffection_page_template_chirho::page_render_chirho::render_page_chirho;

#[derive(Template)]
#[template(path = "workspace_chirho.html", escape = "html")]
struct WorkspaceTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho WorkspacePageV1Chirho,
}

#[derive(Template)]
#[template(path = "page_document_export_chirho.html", escape = "html")]
struct PageDocumentExportTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho PageDocumentExportV1Chirho,
}

#[derive(Template)]
#[template(path = "page_document_import_chirho.html", escape = "html")]
struct PageDocumentImportTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho PageDocumentImportV1Chirho,
}

#[derive(Template)]
#[template(path = "publication_status_chirho.html", escape = "html")]
struct PublicationStatusTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho PublicationStatusV1Chirho,
}

#[derive(Template)]
#[template(path = "publication_intent_chirho.html", escape = "html")]
struct PublicationIntentTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho PublicationIntentV1Chirho,
}

#[derive(Template)]
#[template(path = "account_chirho.html", escape = "html")]
struct AccountTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho AccountPageV1Chirho,
}

pub fn render_account_chirho(model_chirho: &AccountPageV1Chirho) -> Result<String, askama::Error> {
    AccountTemplateChirho { model_chirho }.render()
}

#[derive(Template)]
#[template(path = "draft_chirho.html", escape = "html")]
struct DraftTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho DraftPageV1Chirho,
}

pub fn render_draft_chirho(model_chirho: &DraftPageV1Chirho) -> Result<String, askama::Error> {
    DraftTemplateChirho { model_chirho }.render()
}

pub fn render_publication_intent_chirho(
    model_chirho: &PublicationIntentV1Chirho,
) -> Result<String, askama::Error> {
    PublicationIntentTemplateChirho { model_chirho }.render()
}

pub fn render_publication_status_chirho(
    model_chirho: &PublicationStatusV1Chirho,
) -> Result<String, askama::Error> {
    PublicationStatusTemplateChirho { model_chirho }.render()
}

pub fn render_page_document_import_chirho(
    model_chirho: &PageDocumentImportV1Chirho,
) -> Result<String, askama::Error> {
    PageDocumentImportTemplateChirho { model_chirho }.render()
}

pub fn render_workspace_chirho(
    model_chirho: &WorkspacePageV1Chirho,
) -> Result<String, askama::Error> {
    WorkspaceTemplateChirho { model_chirho }.render()
}

pub fn render_page_document_export_chirho(
    model_chirho: &PageDocumentExportV1Chirho,
) -> Result<String, askama::Error> {
    PageDocumentExportTemplateChirho { model_chirho }.render()
}

#[derive(Template)]
#[template(path = "figures_chirho.html", escape = "html")]
struct FiguresTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho FiguresPageV1Chirho,
}

/// Her figures: the report of her saved version and the paste form (her-figures-chirho.md).
pub fn render_figures_chirho(model_chirho: &FiguresPageV1Chirho) -> Result<String, askama::Error> {
    FiguresTemplateChirho { model_chirho }.render()
}

#[derive(Template)]
#[template(path = "figures_review_chirho.html", escape = "html")]
struct FiguresReviewTemplateChirho<'model_chirho> {
    model_chirho: &'model_chirho FiguresReviewV1Chirho,
}

/// The review of her pasted figures before she saves (her-figures-chirho.md).
pub fn render_figures_review_chirho(
    model_chirho: &FiguresReviewV1Chirho,
) -> Result<String, askama::Error> {
    FiguresReviewTemplateChirho { model_chirho }.render()
}
