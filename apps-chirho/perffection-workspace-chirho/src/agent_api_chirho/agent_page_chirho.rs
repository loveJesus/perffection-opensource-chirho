// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! What the agent reads first: the surface, her current private page and the version to save against.

use crate::agent_surface_chirho::{agent_surface_chirho, AgentSurfaceV1Chirho};
use crate::page_document_chirho::PageDocumentV2Chirho;
use crate::viewer_state_contract_chirho::decode_page_state_response_chirho;
use serde::Serialize;

pub const AGENT_PAGE_SCHEMA_CHIRHO: &str = "perffection-agent-page-v1-chirho";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AgentPageStateChirho {
    #[serde(rename = "saved-chirho")]
    SavedChirho,
    #[serde(rename = "nothing-saved-chirho")]
    NothingSavedChirho,
    /// Her stored page could not be verified. There is no version to save against: do not save.
    #[serde(rename = "unverifiable-chirho")]
    UnverifiableChirho,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentPageV1Chirho {
    pub schema_chirho: &'static str,
    pub state_chirho: AgentPageStateChirho,
    /// The version to send back as `expected_version_chirho`; 0 when nothing is saved yet.
    pub current_version_chirho: Option<u64>,
    pub page_chirho: Option<PageDocumentV2Chirho>,
    pub surface_chirho: AgentSurfaceV1Chirho,
}

impl AgentPageV1Chirho {
    pub fn from_state_response_chirho(response_bytes_chirho: &[u8]) -> Self {
        let (state_chirho, current_version_chirho, page_chirho) =
            match decode_page_state_response_chirho(response_bytes_chirho) {
                Ok(Some(snapshot_chirho)) => (
                    AgentPageStateChirho::SavedChirho,
                    Some(snapshot_chirho.current_version_chirho()),
                    Some(snapshot_chirho.document_chirho().to_document_chirho()),
                ),
                Ok(None) => (AgentPageStateChirho::NothingSavedChirho, Some(0), None),
                Err(_error_chirho) => (AgentPageStateChirho::UnverifiableChirho, None, None),
            };
        Self {
            schema_chirho: AGENT_PAGE_SCHEMA_CHIRHO,
            state_chirho,
            current_version_chirho,
            page_chirho,
            surface_chirho: agent_surface_chirho(),
        }
    }

    pub fn to_json_bytes_chirho(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_default()
    }
}
