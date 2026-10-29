// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
//! Her AI-assistant connections, a section of the Account page.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/agent-api-chirho.md` (her connection's
//! life). The list and the disconnect are Cairn's owner key roads (4b-2). This private route uses a
//! no-script pattern, so it binds them through the contract Cairn fixed (#1712, its K3, built after
//! K1): a DAG node holding the `read-store:own-keys-chirho/` capability reads the list, and a form POST
//! to the disconnect road comes back by 303 and `:target`. The deployed Account route stays HELD (it says
//! what a connection will do and offers no control) until the pin carries K3; the bound route,
//! `account_keys_chirho`, renders her list and is staged only where the engine has it. Each row names the
//! app it reaches and what it may do, in her words. Three rules hold in every state:
//! - an unavailable list is never shown as an empty one;
//! - a row carries only what the engine lets an owner see, never a key, its hash or its session;
//! - a disconnect form exists only for a live row, and only while the disconnect road is bound.

use crate::workspace_page_chirho::WORKSPACE_ACCOUNT_HREF_CHIRHO;
use perffection_ui_chirho::WorkspaceConnectionsCopyChirho;
use serde::Deserialize;

/// The engine's form road that ends one of her connections (Cairn K3, #1712): `key_id_chirho` and
/// `redirect_chirho`, answered by a 303 back here.
pub const KEY_REVOKE_ACTION_CHIRHO: &str = "/auth-chirho/key-revoke-chirho";
/// The engine's own form that creates a connection (Cairn K4 Part 3): her password again, the actions she chooses,
/// and the key shown once on the engine's page, never here.
pub const CONNECTION_NEW_ROUTE_CHIRHO: &str = "/auth-chirho/connection-new-chirho";
/// The app the engine resolves the connection client for: this product's own app (app-chirho.toml).
pub const CONNECTION_APP_CHIRHO: &str = "perffection-workspace";

/// The Create entry (gpt #26824, #26825): the engine's form for this app, with her way back to this page. It is
/// navigation only: it names no client and carries no credential, and the engine checks the app and the return again.
pub fn connect_entry_href_chirho() -> String {
    format!(
        "{CONNECTION_NEW_ROUTE_CHIRHO}?app_chirho={}&return_chirho={}",
        query_value_chirho(CONNECTION_APP_CHIRHO),
        query_value_chirho(WORKSPACE_ACCOUNT_HREF_CHIRHO)
    )
}

/// One query value, percent-encoded: every byte but the unreserved ones (RFC 3986).
fn query_value_chirho(value_chirho: &str) -> String {
    value_chirho
        .bytes()
        .map(|byte_chirho| match byte_chirho {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                char::from(byte_chirho).to_string()
            }
            _ => format!("%{byte_chirho:02X}"),
        })
        .collect()
}

/// The fragment the disconnect road's success (or repeat) redirect targets, revealing the ended notice.
pub const CONNECTION_ENDED_TOKEN_CHIRHO: &str = "notice-key-ended-chirho";
/// The read the list node sends through `read-store:own-keys-chirho/` (Cairn #1712).
pub const OWN_KEYS_READ_REQUEST_CHIRHO: &str =
    r#"{"schema_chirho":"own-keys-read-request-v1-chirho"}"#;
/// The disconnect road's refusals that return to the Account page as `#refusal-<token>` (Cairn #1712).
/// A key that expired between the page and her click answers `no-such-key-chirho`: already ended.
pub const DISCONNECT_REFUSAL_TOKENS_CHIRHO: [&str; 3] = [
    "no-such-key-chirho",
    "request-invalid-chirho",
    "keys-failed-chirho",
];

/// One connection as the engine lists it for its owner, in Cairn's `ScopedKeyRowChirho` shape. Fields
/// the engine adds later are ignored, never shown: the page reads only these.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ConnectionRowChirho {
    pub key_id_chirho: String,
    pub app_chirho: String,
    pub scopes_chirho: Vec<String>,
    pub issued_ms_chirho: u64,
    pub expires_ms_chirho: u64,
    /// Cairn lists an ended key until it expires, so she sees it ended rather than vanish.
    pub live_chirho: bool,
}

/// What the page knows of her connections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionsStateChirho {
    /// No list input on this engine yet: nothing of hers is read.
    HeldChirho,
    /// The list input is bound but gave no list this time. Never rendered as "none".
    UnavailableChirho,
    /// Her own connections, possibly none.
    ListedChirho(Vec<ConnectionRowChirho>),
}

/// One row as the page shows it: the app it reaches and what it may do, in her words; the opaque id the
/// disconnect form names; and dates. Never the key itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionRowViewChirho {
    pub(crate) key_id_chirho: String,
    pub(crate) live_chirho: bool,
    pub(crate) app_chirho: String,
    pub(crate) actions_chirho: Vec<String>,
    pub(crate) since_chirho: String,
    pub(crate) until_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionsViewChirho {
    pub(crate) held_chirho: bool,
    pub(crate) unavailable_chirho: bool,
    pub(crate) listed_chirho: bool,
    pub(crate) rows_chirho: Vec<ConnectionRowViewChirho>,
    /// The engine road a disconnect form posts to; `None` until that road is bound.
    pub(crate) disconnect_action_chirho: Option<String>,
    /// The Create entry: only on a page bound for creating AND listing; never held or unavailable.
    pub(crate) connect_href_chirho: Option<String>,
    pub(crate) ended_id_chirho: &'static str,
}

impl ConnectionsViewChirho {
    pub fn new_chirho(
        state_chirho: ConnectionsStateChirho,
        disconnect_action_chirho: Option<String>,
        connect_href_chirho: Option<String>,
        copy_chirho: &WorkspaceConnectionsCopyChirho,
    ) -> Self {
        let (held_chirho, unavailable_chirho, rows_chirho) = match state_chirho {
            ConnectionsStateChirho::HeldChirho => (true, false, None),
            ConnectionsStateChirho::UnavailableChirho => (false, true, None),
            ConnectionsStateChirho::ListedChirho(rows_chirho) => (false, false, Some(rows_chirho)),
        };
        let listed_chirho = rows_chirho.is_some();
        let rows_chirho = rows_chirho
            .unwrap_or_default()
            .into_iter()
            .map(|row_chirho| ConnectionRowViewChirho {
                since_chirho: utc_date_chirho(row_chirho.issued_ms_chirho),
                until_chirho: utc_date_chirho(row_chirho.expires_ms_chirho),
                app_chirho: app_label_chirho(copy_chirho, &row_chirho.app_chirho),
                actions_chirho: row_chirho
                    .scopes_chirho
                    .iter()
                    .map(|scope_chirho| action_label_chirho(copy_chirho, scope_chirho))
                    .collect(),
                key_id_chirho: row_chirho.key_id_chirho,
                live_chirho: row_chirho.live_chirho,
            })
            .collect();
        Self {
            held_chirho,
            unavailable_chirho,
            listed_chirho,
            rows_chirho,
            // Without a list there is no row to disconnect, so a bound road offers nothing; nor, in this
            // product's choice, a way to create one (gpt #26825).
            disconnect_action_chirho: disconnect_action_chirho.filter(|_| listed_chirho),
            connect_href_chirho: connect_href_chirho.filter(|_| listed_chirho),
            ended_id_chirho: CONNECTION_ENDED_TOKEN_CHIRHO,
        }
    }

    pub fn held_chirho(copy_chirho: &WorkspaceConnectionsCopyChirho) -> Self {
        Self::new_chirho(ConnectionsStateChirho::HeldChirho, None, None, copy_chirho)
    }

    pub fn disconnect_bound_chirho(&self) -> bool {
        self.disconnect_action_chirho.is_some()
    }
}

/// Her words for an app; an app the catalog does not name is shown by the engine's name.
fn app_label_chirho(copy_chirho: &WorkspaceConnectionsCopyChirho, app_chirho: &str) -> String {
    copy_chirho
        .app_labels_chirho
        .get(app_chirho)
        .cloned()
        .unwrap_or_else(|| app_chirho.to_string())
}

/// Her words for what a connection may do. A limited key's scopes are its action names (Cairn #1740);
/// an older key's are its scope. One the catalog does not name is shown with the engine's own name, so
/// nothing a connection may do is ever hidden.
fn action_label_chirho(copy_chirho: &WorkspaceConnectionsCopyChirho, scope_chirho: &str) -> String {
    copy_chirho
        .action_labels_chirho
        .get(scope_chirho)
        .cloned()
        .unwrap_or_else(|| format!("{} {scope_chirho}", copy_chirho.unknown_action_label_chirho))
}

/// `YYYY-MM-DD` in UTC for milliseconds since the epoch (days-to-civil, proleptic Gregorian).
pub fn utc_date_chirho(ms_chirho: u64) -> String {
    let days_chirho = (ms_chirho / 86_400_000) as i64 + 719_468;
    let era_chirho = days_chirho.div_euclid(146_097);
    let day_of_era_chirho = days_chirho.rem_euclid(146_097);
    let year_of_era_chirho = (day_of_era_chirho - day_of_era_chirho / 1_460
        + day_of_era_chirho / 36_524
        - day_of_era_chirho / 146_096)
        / 365;
    let day_of_year_chirho = day_of_era_chirho
        - (365 * year_of_era_chirho + year_of_era_chirho / 4 - year_of_era_chirho / 100);
    let month_index_chirho = (5 * day_of_year_chirho + 2) / 153;
    let day_chirho = day_of_year_chirho - (153 * month_index_chirho + 2) / 5 + 1;
    let month_chirho = if month_index_chirho < 10 {
        month_index_chirho + 3
    } else {
        month_index_chirho - 9
    };
    let year_chirho = year_of_era_chirho + era_chirho * 400 + i64::from(month_chirho <= 2);
    format!("{year_chirho:04}-{month_chirho:02}-{day_chirho:02}")
}

/// The list read's answer (Cairn #1712), typed. Available carries her rows, possibly none; unavailable
/// carries no list at all. Anything else, a malformed or unknown answer included, is unavailable: the
/// page never turns a failed read into "you have none".
#[derive(Deserialize)]
struct OwnKeysReadChirho {
    schema_chirho: String,
    state_chirho: String,
    keys_chirho: Option<Vec<ConnectionRowChirho>>,
}

pub fn connections_state_from_read_chirho(answer_chirho: &[u8]) -> ConnectionsStateChirho {
    match serde_json::from_slice::<OwnKeysReadChirho>(answer_chirho) {
        Ok(read_chirho) if read_chirho.schema_chirho == "own-keys-read-v1-chirho" => {
            match (read_chirho.state_chirho.as_str(), read_chirho.keys_chirho) {
                ("available-chirho", Some(rows_chirho)) => {
                    ConnectionsStateChirho::ListedChirho(rows_chirho)
                }
                _ => ConnectionsStateChirho::UnavailableChirho,
            }
        }
        _ => ConnectionsStateChirho::UnavailableChirho,
    }
}
