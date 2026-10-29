// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her AI-assistant connections on the Account page. The words live in the workspace catalog
//! (`connections_chirho`); which of them a page shows follows the engine roads that are bound.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceConnectionsCopyChirho {
    pub heading_chirho: String,
    pub intro_chirho: String,
    pub can_heading_chirho: String,
    pub can_items_chirho: Vec<String>,
    pub never_heading_chirho: String,
    pub never_items_chirho: Vec<String>,
    /// While no list is bound: connecting is not open yet. No control, no promise of a date.
    pub held_chirho: String,
    /// The list is bound but could not be read now. Never worded as "you have none".
    pub unavailable_chirho: String,
    pub empty_chirho: String,
    pub list_label_chirho: String,
    pub connected_label_chirho: String,
    pub ended_label_chirho: String,
    pub since_label_chirho: String,
    pub until_label_chirho: String,
    pub disconnect_submit_chirho: String,
    pub disconnect_boundary_chirho: String,
    /// Revealed by `:target` when the disconnect road sends her back after an end.
    pub ended_notice_chirho: String,
    pub refusal_no_such_key_chirho: String,
    pub refusal_failed_chirho: String,
    /// Her words for the app a connection reaches, by the engine's app name.
    pub app_labels_chirho: BTreeMap<String, String>,
    /// Her words for what a connection may do, by the engine's action name (or an older key's scope).
    pub action_labels_chirho: BTreeMap<String, String>,
    /// Shown before a permission this page has no words for yet; the engine's own name follows, never hidden.
    pub unknown_action_label_chirho: String,
    /// The entry to the engine's form that creates a connection, and what she will do there.
    pub connect_link_chirho: String,
    pub connect_note_chirho: String,
}
