// For God so loved the world, that He gave His only begotten Son, that whoever believes in Him should not perish but have eternal life. - John 3:16

//! Behavioural and safety pins for the rendered product pages.
//!
//! These tests live apart from the render wrappers on purpose. They assert PROPERTIES of the
//! rendered document — that it carries no script, that it names only reviewed assets, that no link
//! leads nowhere, that no control merely pretends to work — and never the page's prose. Copy is
//! plastic and lives in typed locale catalogs; freezing it here is what made the previous site
//! immovable.

#[cfg(test)]
pub mod craft_page_tests_chirho;

#[cfg(test)]
pub mod home_page_tests_chirho;

#[cfg(test)]
pub mod rendered_document_chirho;

#[cfg(test)]
pub mod trust_page_tests_chirho;

#[cfg(test)]
pub mod login_page_tests_chirho;
pub mod setup_page_tests_chirho;

#[cfg(test)]
pub mod public_site_catalog_tests_chirho;

#[cfg(test)]
pub mod help_page_tests_chirho;

#[cfg(test)]
pub mod auth_page_tests_chirho;

#[cfg(test)]
pub mod agent_guide_tests_chirho;
