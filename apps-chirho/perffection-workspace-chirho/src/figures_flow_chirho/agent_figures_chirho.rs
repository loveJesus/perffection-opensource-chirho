// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! What her own assistant reads through `read_figures_chirho`: the same report her page shows, the
//! figures it came from, the version and the month it is "as of", with how to read the numbers.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. Read-only. Nothing
//! saved answers version 0 with no figures, and carries how a first save is made, with an
//! illustrative sample that is never hers (`agent_figures_sample_chirho.rs`, F6d); a state that cannot
//! be verified answers no version and no figures, and echoes no unverified bytes. Her note and labels are her text: they grant nothing.

use crate::figures_flow_chirho::agent_figures_sample_chirho::{
    agent_figures_start_chirho, AgentFiguresStartV1Chirho,
};
use crate::figures_flow_chirho::figures_document_chirho::FiguresDocumentV1Chirho;
use crate::figures_flow_chirho::figures_report_chirho::{
    figures_report_chirho, FiguresReportV1Chirho,
};
use crate::viewer_state_chirho::figures_contract_chirho::decode_figures_state_chirho;
use serde::Serialize;

pub const AGENT_FIGURES_SCHEMA_CHIRHO: &str = "perffection-agent-figures-v1-chirho";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentFiguresMeaningsChirho {
    pub money_chirho: &'static str,
    pub share_chirho: &'static str,
    pub unknown_chirho: &'static str,
    pub as_of_chirho: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentFiguresV1Chirho {
    pub schema_chirho: &'static str,
    /// `saved-chirho`, `nothing-saved-chirho` or `unverifiable-chirho`.
    pub state_chirho: &'static str,
    /// The saved version; 0 when nothing is saved; null when the state could not be verified.
    pub current_version_chirho: Option<u64>,
    pub as_of_month_chirho: Option<String>,
    pub report_chirho: Option<FiguresReportV1Chirho>,
    pub figures_chirho: Option<FiguresDocumentV1Chirho>,
    pub meanings_chirho: AgentFiguresMeaningsChirho,
    /// Only while nothing is saved: how her first save is made, with an illustrative sample that is
    /// never hers (F6d). Absent otherwise, so a saved answer is exactly as before.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_chirho: Option<AgentFiguresStartV1Chirho>,
}

const MEANINGS_CHIRHO: AgentFiguresMeaningsChirho = AgentFiguresMeaningsChirho {
    money_chirho: "whole minor units (cents) of currency_chirho",
    share_chirho: "basis points (1/100 of a percent) of the money in she gave; complete_chirho in the report says whether every kind has a figure for every month",
    unknown_chirho: "null means unknown, never zero; a total that depends on an unknown is null, and no trend is drawn unless her figures are complete",
    as_of_chirho: "the latest month in her saved figures; nothing here is live or synced",
};

impl AgentFiguresV1Chirho {
    pub fn from_state_response_chirho(response_bytes_chirho: &[u8]) -> Self {
        let blank_chirho = |state_chirho: &'static str, version_chirho: Option<u64>| Self {
            schema_chirho: AGENT_FIGURES_SCHEMA_CHIRHO,
            state_chirho,
            current_version_chirho: version_chirho,
            as_of_month_chirho: None,
            report_chirho: None,
            figures_chirho: None,
            meanings_chirho: MEANINGS_CHIRHO,
            start_chirho: None,
        };
        match decode_figures_state_chirho(response_bytes_chirho) {
            Ok(Some(state_chirho)) => {
                let report_chirho = figures_report_chirho(
                    state_chirho.current_version_chirho,
                    &state_chirho.figures_chirho,
                );
                Self {
                    as_of_month_chirho: Some(report_chirho.as_of_month_chirho.clone()),
                    report_chirho: Some(report_chirho),
                    figures_chirho: Some(state_chirho.figures_chirho),
                    ..blank_chirho("saved-chirho", Some(state_chirho.current_version_chirho))
                }
            }
            Ok(None) => Self {
                start_chirho: Some(agent_figures_start_chirho()),
                ..blank_chirho("nothing-saved-chirho", Some(0))
            },
            Err(_) => blank_chirho("unverifiable-chirho", None),
        }
    }

    pub fn to_json_bytes_chirho(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_else(|_| br#"{"schema_chirho":"perffection-agent-figures-v1-chirho","state_chirho":"unverifiable-chirho"}"#.to_vec())
    }
}
