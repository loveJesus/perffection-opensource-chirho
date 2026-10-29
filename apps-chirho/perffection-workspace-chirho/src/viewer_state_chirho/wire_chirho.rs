// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Cairn's audited viewer-state wire, once, for every product document kept under its door.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`. Cairn
//! derives the app and authenticated viewer scopes. A request carries only one product-owned logical
//! key, an action, the optimistic version coordinate, and either bounded value bytes or one earlier
//! retained version. An answer is verified here before any value decoder sees it: its schema and
//! key, a history no longer than the door keeps, newest first with no gap, and a head equal to the
//! newest. The page contract and the figures contract each bring only their key and value decoder.

use base64::{engine::general_purpose::STANDARD as BASE64_CHIRHO, Engine as _};
use serde::{Deserialize, Serialize};

pub const VIEWER_STATE_REQUEST_SCHEMA_CHIRHO: &str = "viewer-state-request-v1-chirho";
pub const VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO: &str = "viewer-state-response-v2-chirho";
/// The typed stale-write answer of the viewer-state door (CAIRN brick after option A, #21578):
/// head unchanged, no append, the verified head state plus an echo of the attempted value.
pub const VIEWER_STATE_CONFLICT_SCHEMA_CHIRHO: &str = "viewer-state-conflict-v1-chirho";
/// The door's history limit (`VIEWER_STATE_HISTORY_LIMIT_CHIRHO`): an append past it retires the oldest.
pub const VIEWER_STATE_HISTORY_MAX_VERSIONS_CHIRHO: usize = 16;

const ACTION_READ_CHIRHO: &str = "read-chirho";
const ACTION_WRITE_CHIRHO: &str = "write-chirho";
const ACTION_RESTORE_CHIRHO: &str = "restore-chirho";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewerStateContractErrorChirho {
    SerializeRequestChirho,
    MalformedResponseChirho,
    UnsupportedResponseSchemaChirho,
    WrongLogicalKeyChirho,
    InvalidExpectedVersionChirho,
    InvalidRestoreVersionChirho,
    InvalidCurrentVersionChirho,
    InvalidHistoryLengthChirho,
    InconsistentCurrentHistoryChirho,
    InvalidHistoryVersionChirho,
    NonContiguousHistoryChirho,
    InvalidBase64ValueChirho,
    MalformedPageDocumentChirho,
    InvalidPageDocumentChirho,
    MalformedFiguresChirho,
    InvalidFiguresChirho,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RequestV1Chirho<'key_chirho> {
    schema_chirho: &'static str,
    action_chirho: &'static str,
    key_chirho: &'key_chirho str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_current_version_chirho: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_base64_chirho: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore_from_version_chirho: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct VersionV2Chirho {
    version_chirho: u64,
    value_base64_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ResponseV2Chirho {
    schema_chirho: String,
    key_chirho: String,
    #[serde(default)]
    current_chirho: Option<VersionV2Chirho>,
    history_chirho: Vec<VersionV2Chirho>,
}

/// The door's conflict answer (engine `ViewerStateConflictChirho`): the expected version is absent
/// when the request was a first write against a key that already had a head, and the head version
/// is stated alongside the full verified head body. Unknown extra fields are tolerated.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ConflictV1Chirho {
    schema_chirho: String,
    key_chirho: String,
    #[serde(default)]
    expected_current_version_chirho: Option<u64>,
    #[serde(default)]
    current_version_chirho: Option<u64>,
    current_chirho: ResponseV2Chirho,
    #[serde(default)]
    attempted_value_base64_chirho: Option<String>,
}

/// One verified retained version and its decoded value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireVersionChirho<D> {
    pub version_chirho: u64,
    pub value_chirho: D,
}

/// A verified state: `history_chirho[0]` is the head, newest first, contiguous.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireSnapshotChirho<D> {
    pub history_chirho: Vec<WireVersionChirho<D>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireConflictChirho<D, A> {
    pub expected_current_version_chirho: Option<u64>,
    pub head_chirho: WireSnapshotChirho<D>,
    /// The echoed attempt, only when its decoder accepted it exactly.
    pub attempted_chirho: Option<A>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WireOutcomeChirho<D, A> {
    StateChirho(Option<WireSnapshotChirho<D>>),
    ConflictChirho(WireConflictChirho<D, A>),
}

pub fn read_request_chirho(key_chirho: &str) -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    encode_chirho(RequestV1Chirho {
        schema_chirho: VIEWER_STATE_REQUEST_SCHEMA_CHIRHO,
        action_chirho: ACTION_READ_CHIRHO,
        key_chirho,
        expected_current_version_chirho: None,
        value_base64_chirho: None,
        restore_from_version_chirho: None,
    })
}

/// `None` is a first write; a stated version must be a real one, never 0.
pub fn write_request_chirho(
    key_chirho: &str,
    value_bytes_chirho: &[u8],
    expected_current_version_chirho: Option<u64>,
) -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    if expected_current_version_chirho == Some(0) {
        return Err(ViewerStateContractErrorChirho::InvalidExpectedVersionChirho);
    }
    encode_chirho(RequestV1Chirho {
        schema_chirho: VIEWER_STATE_REQUEST_SCHEMA_CHIRHO,
        action_chirho: ACTION_WRITE_CHIRHO,
        key_chirho,
        expected_current_version_chirho,
        value_base64_chirho: Some(BASE64_CHIRHO.encode(value_bytes_chirho)),
        restore_from_version_chirho: None,
    })
}

pub fn restore_request_chirho(
    key_chirho: &str,
    expected_current_version_chirho: u64,
    restore_from_version_chirho: u64,
) -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    if expected_current_version_chirho == 0 {
        return Err(ViewerStateContractErrorChirho::InvalidExpectedVersionChirho);
    }
    if restore_from_version_chirho == 0
        || restore_from_version_chirho >= expected_current_version_chirho
    {
        return Err(ViewerStateContractErrorChirho::InvalidRestoreVersionChirho);
    }
    encode_chirho(RequestV1Chirho {
        schema_chirho: VIEWER_STATE_REQUEST_SCHEMA_CHIRHO,
        action_chirho: ACTION_RESTORE_CHIRHO,
        key_chirho,
        expected_current_version_chirho: Some(expected_current_version_chirho),
        value_base64_chirho: None,
        restore_from_version_chirho: Some(restore_from_version_chirho),
    })
}

/// A read or mutation answer for `key_chirho`; `None` when nothing is saved under it yet.
pub fn decode_state_chirho<D>(
    key_chirho: &str,
    response_bytes_chirho: &[u8],
    decode_value_chirho: &dyn Fn(&[u8]) -> Result<D, ViewerStateContractErrorChirho>,
) -> Result<Option<WireSnapshotChirho<D>>, ViewerStateContractErrorChirho> {
    let response_chirho: ResponseV2Chirho = serde_json::from_slice(response_bytes_chirho)
        .map_err(|_| ViewerStateContractErrorChirho::MalformedResponseChirho)?;
    verify_chirho(key_chirho, response_chirho, decode_value_chirho)
}

/// A mutation answer: the state on success, or the typed conflict when the door refused a stale
/// optimistic version without touching the head.
pub fn decode_outcome_chirho<D, A>(
    key_chirho: &str,
    response_bytes_chirho: &[u8],
    decode_value_chirho: &dyn Fn(&[u8]) -> Result<D, ViewerStateContractErrorChirho>,
    decode_attempted_chirho: &dyn Fn(&[u8]) -> Option<A>,
) -> Result<WireOutcomeChirho<D, A>, ViewerStateContractErrorChirho> {
    #[derive(Deserialize)]
    struct SchemaOnlyChirho {
        schema_chirho: String,
    }
    let schema_chirho: SchemaOnlyChirho = serde_json::from_slice(response_bytes_chirho)
        .map_err(|_| ViewerStateContractErrorChirho::MalformedResponseChirho)?;
    if schema_chirho.schema_chirho != VIEWER_STATE_CONFLICT_SCHEMA_CHIRHO {
        return decode_state_chirho(key_chirho, response_bytes_chirho, decode_value_chirho)
            .map(WireOutcomeChirho::StateChirho);
    }
    let conflict_chirho: ConflictV1Chirho = serde_json::from_slice(response_bytes_chirho)
        .map_err(|_| ViewerStateContractErrorChirho::MalformedResponseChirho)?;
    if conflict_chirho.key_chirho != key_chirho {
        return Err(ViewerStateContractErrorChirho::WrongLogicalKeyChirho);
    }
    let head_chirho = verify_chirho(
        key_chirho,
        conflict_chirho.current_chirho,
        decode_value_chirho,
    )?
    .ok_or(ViewerStateContractErrorChirho::InconsistentCurrentHistoryChirho)?;
    let head_version_chirho = head_chirho.history_chirho[0].version_chirho;
    if conflict_chirho.expected_current_version_chirho == Some(head_version_chirho) {
        return Err(ViewerStateContractErrorChirho::InvalidCurrentVersionChirho);
    }
    if conflict_chirho
        .current_version_chirho
        .is_some_and(|stated_chirho| stated_chirho != head_version_chirho)
    {
        return Err(ViewerStateContractErrorChirho::InconsistentCurrentHistoryChirho);
    }
    let attempted_chirho = conflict_chirho
        .attempted_value_base64_chirho
        .and_then(|value_chirho| BASE64_CHIRHO.decode(value_chirho).ok())
        .and_then(|bytes_chirho| decode_attempted_chirho(&bytes_chirho));
    Ok(WireOutcomeChirho::ConflictChirho(WireConflictChirho {
        expected_current_version_chirho: conflict_chirho.expected_current_version_chirho,
        head_chirho,
        attempted_chirho,
    }))
}

fn verify_chirho<D>(
    key_chirho: &str,
    response_chirho: ResponseV2Chirho,
    decode_value_chirho: &dyn Fn(&[u8]) -> Result<D, ViewerStateContractErrorChirho>,
) -> Result<Option<WireSnapshotChirho<D>>, ViewerStateContractErrorChirho> {
    if response_chirho.schema_chirho != VIEWER_STATE_RESPONSE_SCHEMA_CHIRHO {
        return Err(ViewerStateContractErrorChirho::UnsupportedResponseSchemaChirho);
    }
    if response_chirho.key_chirho != key_chirho {
        return Err(ViewerStateContractErrorChirho::WrongLogicalKeyChirho);
    }
    if response_chirho.history_chirho.len() > VIEWER_STATE_HISTORY_MAX_VERSIONS_CHIRHO {
        return Err(ViewerStateContractErrorChirho::InvalidHistoryLengthChirho);
    }
    let Some(current_chirho) = response_chirho.current_chirho else {
        return if response_chirho.history_chirho.is_empty() {
            Ok(None)
        } else {
            Err(ViewerStateContractErrorChirho::InconsistentCurrentHistoryChirho)
        };
    };
    if current_chirho.version_chirho == 0 {
        return Err(ViewerStateContractErrorChirho::InvalidCurrentVersionChirho);
    }
    if response_chirho.history_chirho.first() != Some(&current_chirho) {
        return Err(ViewerStateContractErrorChirho::InconsistentCurrentHistoryChirho);
    }
    let history_chirho = response_chirho
        .history_chirho
        .into_iter()
        .map(|version_chirho| {
            if version_chirho.version_chirho == 0 {
                return Err(ViewerStateContractErrorChirho::InvalidHistoryVersionChirho);
            }
            let value_bytes_chirho = BASE64_CHIRHO
                .decode(version_chirho.value_base64_chirho)
                .map_err(|_| ViewerStateContractErrorChirho::InvalidBase64ValueChirho)?;
            Ok(WireVersionChirho {
                version_chirho: version_chirho.version_chirho,
                value_chirho: decode_value_chirho(&value_bytes_chirho)?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    for adjacent_chirho in history_chirho.windows(2) {
        if adjacent_chirho[1].version_chirho.checked_add(1)
            != Some(adjacent_chirho[0].version_chirho)
        {
            return Err(ViewerStateContractErrorChirho::NonContiguousHistoryChirho);
        }
    }
    Ok(Some(WireSnapshotChirho { history_chirho }))
}

fn encode_chirho(
    request_chirho: RequestV1Chirho<'_>,
) -> Result<Vec<u8>, ViewerStateContractErrorChirho> {
    serde_json::to_vec(&request_chirho)
        .map_err(|_| ViewerStateContractErrorChirho::SerializeRequestChirho)
}
