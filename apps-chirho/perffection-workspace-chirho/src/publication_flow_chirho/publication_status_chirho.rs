// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Publication status: what is live at her address, what is on its way, and what was refused.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.
//!
//! The tenant door reads her own publication intents and the platform root's applied records
//! (verified there) through the `read-store` capability on the reserved `publication-chirho/`
//! prefix (CAIRN #21753). Nothing here mutates anything; the shape tolerates additive fields and
//! fails closed on an unknown schema.
//!
//! This page is the one place that knows each address's approval base (CAIRN #21800), so every
//! approval form — new address, republish, rollback — is rendered here from the same read and
//! carries that base; the editor only points here.

use crate::field_view_chirho::{FieldViewChirho, FormNoticeChirho};
use crate::publication_intent_chirho::{
    ADDRESS_CONTROL_ID_CHIRHO, ADDRESS_HELP_ID_CHIRHO, ADDRESS_MESSAGE_ID_CHIRHO, DARK_BASE_CHIRHO,
};
use crate::viewer_state_contract_chirho::PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO;
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
    WORKSPACE_PREVIEW_HREF_CHIRHO,
};
use perffection_ui_chirho::{SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho};
use serde::{Deserialize, Serialize};

pub const PUBLICATION_STATUS_REQUEST_SCHEMA_CHIRHO: &str = "publication-status-request-v1-chirho";
pub const PUBLICATION_STATUS_RESPONSE_SCHEMA_CHIRHO: &str = "publication-status-response-v1-chirho";
/// Wire words carry the `-chirho` suffix like every kebab enum on this wire.
pub const APPLIED_OUTCOME_CHIRHO: &str = "applied-chirho";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct PublicationStatusRequestV1Chirho {
    schema_chirho: &'static str,
    /// The viewer-state key whose verified head the road states back (CAIRN #21828), so the
    /// Publishing page can name her saved version without a second read.
    page_key_chirho: &'static str,
}

/// The one fixed read request: her own record, every slug, plus her page head. No
/// caller-selected scope.
pub fn build_publication_status_request_chirho() -> Vec<u8> {
    serde_json::to_vec(&PublicationStatusRequestV1Chirho {
        schema_chirho: PUBLICATION_STATUS_REQUEST_SCHEMA_CHIRHO,
        page_key_chirho: PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
    })
    .expect("a fixed two-field request serializes")
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ViewerStateRefV1Chirho {
    #[serde(default)]
    key_chirho: Option<String>,
    version_chirho: u64,
    #[serde(default)]
    receipt_digest_chirho: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct IntentRecordV1Chirho {
    seq_chirho: u64,
    intent_digest_chirho: String,
    kind_chirho: String,
    #[serde(default)]
    params_chirho: Option<String>,
    #[serde(default)]
    viewer_state_chirho: Option<ViewerStateRefV1Chirho>,
    #[serde(default)]
    at_ms_chirho: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct AppliedRecordV1Chirho {
    intent_digest_chirho: String,
    outcome_chirho: String,
    #[serde(default)]
    refusal_code_chirho: Option<String>,
    #[serde(default)]
    params_chirho: Option<String>,
    #[serde(default)]
    at_ms_chirho: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct LiveRecordV1Chirho {
    #[serde(default)]
    params_chirho: Option<String>,
    applied_intent_digest_chirho: String,
    #[serde(default)]
    at_ms_chirho: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct SlugStatusV1Chirho {
    slug_chirho: String,
    address_chirho: String,
    #[serde(default)]
    intents_chirho: Vec<IntentRecordV1Chirho>,
    #[serde(default)]
    applied_chirho: Vec<AppliedRecordV1Chirho>,
    #[serde(default)]
    live_chirho: Option<LiveRecordV1Chirho>,
    /// The approval base the door expects back (CAIRN #21800): the live applied intent digest,
    /// or `dark-chirho`.
    #[serde(default)]
    approval_base_chirho: Option<String>,
}

/// Her verified page head as the status road states it for the requested key (CAIRN #21828):
/// version and receipt digest, never value bytes; OMITTED before her first save.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct PageHeadV1Chirho {
    #[serde(default)]
    key_chirho: Option<String>,
    version_chirho: u64,
    #[serde(default)]
    receipt_digest_chirho: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct PublicationStatusResponseV1Chirho {
    schema_chirho: String,
    #[serde(default)]
    slugs_chirho: Vec<SlugStatusV1Chirho>,
    #[serde(default)]
    page_head_chirho: Option<PageHeadV1Chirho>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationStatusErrorChirho {
    MalformedResponseChirho,
    UnsupportedResponseSchemaChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationOutcomeChirho {
    PendingChirho,
    AppliedChirho,
    RefusedChirho(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationEventV1Chirho {
    pub kind_chirho: String,
    pub version_chirho: Option<u64>,
    pub outcome_chirho: PublicationOutcomeChirho,
    pub at_ms_chirho: u64,
    /// The params record this event published or applied; a rollback target once applied.
    pub params_chirho: Option<String>,
    /// The applied record the address serves right now; every other applied record is history
    /// (measured 2026-09-09 in real Chrome: all of them read "Live" before this flag).
    pub is_live_chirho: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationSiteStatusV1Chirho {
    pub slug_chirho: String,
    pub address_chirho: String,
    /// Live applied intent digest, or `dark-chirho` before the first apply.
    pub approval_base_chirho: String,
    /// The params record that is live; rolling back to it would change nothing.
    pub live_params_chirho: Option<String>,
    /// The version that is live, resolved through the applied intent; `None` before the first
    /// apply, or when the live record names an intent this read did not return.
    pub live_version_chirho: Option<u64>,
    pub is_live_chirho: bool,
    pub pending_chirho: Vec<PublicationEventV1Chirho>,
    pub history_chirho: Vec<PublicationEventV1Chirho>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationStatusSnapshotV1Chirho {
    pub sites_chirho: Vec<PublicationSiteStatusV1Chirho>,
    /// Her saved version above zero; `None` when the road omitted the head (nothing saved yet)
    /// or named a head for another key, and then no approval form names a version.
    pub head_version_chirho: Option<u64>,
}

pub fn decode_publication_status_chirho(
    response_bytes_chirho: &[u8],
) -> Result<PublicationStatusSnapshotV1Chirho, PublicationStatusErrorChirho> {
    let response_chirho: PublicationStatusResponseV1Chirho =
        serde_json::from_slice(response_bytes_chirho)
            .map_err(|_| PublicationStatusErrorChirho::MalformedResponseChirho)?;
    if response_chirho.schema_chirho != PUBLICATION_STATUS_RESPONSE_SCHEMA_CHIRHO {
        return Err(PublicationStatusErrorChirho::UnsupportedResponseSchemaChirho);
    }
    let sites_chirho = response_chirho
        .slugs_chirho
        .into_iter()
        .map(site_status_chirho)
        .collect();
    Ok(PublicationStatusSnapshotV1Chirho {
        sites_chirho,
        head_version_chirho: response_chirho
            .page_head_chirho
            .filter(|head_chirho| {
                head_chirho
                    .key_chirho
                    .as_deref()
                    .is_none_or(|key_chirho| key_chirho == PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO)
            })
            .map(|head_chirho| head_chirho.version_chirho)
            .filter(|version_chirho| *version_chirho > 0),
    })
}

fn site_status_chirho(slug_chirho: SlugStatusV1Chirho) -> PublicationSiteStatusV1Chirho {
    let mut intents_chirho = slug_chirho.intents_chirho;
    intents_chirho.sort_by_key(|intent_chirho| intent_chirho.seq_chirho);
    // The live version is named by the applied intent's version binding; a rollback intent
    // carries only the params hash, so fall back to any of her intents that published the same
    // params record under a binding (measured 2026-09-09 rehearsal: "Live" without a number).
    let live_version_chirho = slug_chirho.live_chirho.as_ref().and_then(|live_chirho| {
        intents_chirho
            .iter()
            .find(|intent_chirho| {
                intent_chirho.intent_digest_chirho == live_chirho.applied_intent_digest_chirho
            })
            .and_then(|intent_chirho| intent_chirho.viewer_state_chirho.as_ref())
            .map(|state_chirho| state_chirho.version_chirho)
            .or_else(|| {
                let live_params_chirho = live_chirho.params_chirho.as_deref()?;
                intents_chirho
                    .iter()
                    .filter(|intent_chirho| {
                        intent_chirho.params_chirho.as_deref() == Some(live_params_chirho)
                    })
                    .find_map(|intent_chirho| {
                        intent_chirho
                            .viewer_state_chirho
                            .as_ref()
                            .map(|state_chirho| state_chirho.version_chirho)
                    })
            })
    });
    let mut pending_chirho = Vec::new();
    let mut history_chirho = Vec::new();
    let live_digest_chirho = slug_chirho
        .live_chirho
        .as_ref()
        .map(|live_chirho| live_chirho.applied_intent_digest_chirho.as_str());
    for intent_chirho in &intents_chirho {
        let applied_chirho = slug_chirho.applied_chirho.iter().find(|applied_chirho| {
            applied_chirho.intent_digest_chirho == intent_chirho.intent_digest_chirho
        });
        let version_chirho = intent_chirho
            .viewer_state_chirho
            .as_ref()
            .map(|state_chirho| state_chirho.version_chirho);
        match applied_chirho {
            None => pending_chirho.push(PublicationEventV1Chirho {
                kind_chirho: intent_chirho.kind_chirho.clone(),
                version_chirho,
                outcome_chirho: PublicationOutcomeChirho::PendingChirho,
                at_ms_chirho: intent_chirho.at_ms_chirho,
                params_chirho: intent_chirho.params_chirho.clone(),
                is_live_chirho: false,
            }),
            Some(applied_chirho) => history_chirho.push(PublicationEventV1Chirho {
                kind_chirho: intent_chirho.kind_chirho.clone(),
                version_chirho,
                outcome_chirho: if applied_chirho.outcome_chirho == APPLIED_OUTCOME_CHIRHO {
                    PublicationOutcomeChirho::AppliedChirho
                } else {
                    PublicationOutcomeChirho::RefusedChirho(
                        applied_chirho
                            .refusal_code_chirho
                            .clone()
                            .unwrap_or_default(),
                    )
                },
                at_ms_chirho: applied_chirho.at_ms_chirho,
                params_chirho: applied_chirho
                    .params_chirho
                    .clone()
                    .or_else(|| intent_chirho.params_chirho.clone()),
                is_live_chirho: applied_chirho.outcome_chirho == APPLIED_OUTCOME_CHIRHO
                    && live_digest_chirho == Some(intent_chirho.intent_digest_chirho.as_str()),
            }),
        }
    }
    history_chirho.reverse();
    let approval_base_chirho = slug_chirho
        .approval_base_chirho
        .clone()
        .or_else(|| {
            slug_chirho
                .live_chirho
                .as_ref()
                .map(|live_chirho| live_chirho.applied_intent_digest_chirho.clone())
        })
        .unwrap_or_else(|| DARK_BASE_CHIRHO.to_string());
    let live_params_chirho = slug_chirho
        .live_chirho
        .as_ref()
        .and_then(|live_chirho| live_chirho.params_chirho.clone());
    PublicationSiteStatusV1Chirho {
        slug_chirho: slug_chirho.slug_chirho,
        address_chirho: slug_chirho.address_chirho,
        approval_base_chirho,
        live_params_chirho,
        live_version_chirho,
        is_live_chirho: slug_chirho.live_chirho.is_some(),
        pending_chirho,
        history_chirho,
    }
}

/// Refusals that are about the address she chose, so a different name is the way forward.
const ADDRESS_REFUSAL_CODES_CHIRHO: [&str; 3] = [
    "slug-reserved-chirho",
    "slug-taken-chirho",
    "not-your-app-chirho",
];

/// Her most recent decided event across every address, when it was refused for a reason about
/// the address: the name she typed and the platform's code. The kind of request does not matter
/// (the caller shows this only when the page offers the new-address form, so a different name is
/// the way forward either way); a later event of any other outcome means that refusal is no
/// longer the last thing that happened.
fn latest_address_refusal_chirho(
    snapshot_chirho: &PublicationStatusSnapshotV1Chirho,
) -> Option<(&str, &str)> {
    let (site_chirho, event_chirho) = snapshot_chirho
        .sites_chirho
        .iter()
        .flat_map(|site_chirho| {
            site_chirho
                .history_chirho
                .iter()
                .map(move |event_chirho| (site_chirho, event_chirho))
        })
        .max_by_key(|(_, event_chirho)| event_chirho.at_ms_chirho)?;
    match &event_chirho.outcome_chirho {
        PublicationOutcomeChirho::RefusedChirho(code_chirho)
            if ADDRESS_REFUSAL_CODES_CHIRHO.contains(&code_chirho.as_str()) =>
        {
            Some((site_chirho.slug_chirho.as_str(), code_chirho.as_str()))
        }
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationEventViewV1Chirho {
    pub(crate) kind_label_chirho: String,
    pub(crate) version_chirho: Option<u64>,
    pub(crate) outcome_label_chirho: String,
    pub(crate) detail_chirho: Option<String>,
    /// Present only for an applied event with a params record: the rollback target.
    pub(crate) rollback_params_chirho: Option<String>,
    pub(crate) rollback_button_chirho: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationSiteViewV1Chirho {
    pub(crate) slug_chirho: String,
    pub(crate) address_chirho: String,
    pub(crate) approval_base_chirho: String,
    pub(crate) is_live_chirho: bool,
    /// The address is or was hers, or a request for it is still on its way. False for a name she
    /// only asked for and was refused: that address may be someone else's, so the page neither
    /// calls it hers nor shows its path.
    pub(crate) held_chirho: bool,
    /// Live with nothing on its way, and NOT known to have her saved version live already: the
    /// republish form. When both the saved and the live version are stated and equal, the form
    /// gives way to `live_is_saved_line_chirho`; when either is absent nothing is guessed.
    pub(crate) can_republish_chirho: bool,
    /// "Version N is what is live at this address…" in place of a redundant republish form.
    pub(crate) live_is_saved_line_chirho: Option<String>,
    /// "Publishing makes version N public at ADDRESS…" for the republish form.
    pub(crate) consent_chirho: String,
    pub(crate) live_version_chirho: Option<u64>,
    pub(crate) pending_chirho: Vec<PublicationEventViewV1Chirho>,
    pub(crate) history_chirho: Vec<PublicationEventViewV1Chirho>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationStatusV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) status_chirho: String,
    pub(crate) available_chirho: bool,
    pub(crate) sites_chirho: Vec<PublicationSiteViewV1Chirho>,
    /// The publish door's route for every approval form; `None` until the road is on the pin.
    pub(crate) publish_action_chirho: Option<String>,
    /// Her saved version once the road states one above zero; every approval form names it.
    pub(crate) publishable_version_chirho: Option<u64>,
    /// "Saved version N is ready to publish.", or "Save a version… before publishing" when the
    /// road omitted her head.
    pub(crate) saved_version_line_chirho: String,
    pub(crate) review_href_chirho: String,
    /// Nothing live and nothing on its way: offer the new-address form.
    pub(crate) new_address_form_chirho: bool,
    /// The address field of that form: described by its help text, and, when the address she
    /// last chose was refused for a reason about the address itself, carrying that message.
    pub(crate) address_field_chirho: FieldViewChirho,
    /// The name she typed, handed back so she amends it instead of starting blank; empty unless
    /// `address_field_chirho` carries a message.
    pub(crate) refused_address_chirho: String,
    /// The same refusal as one line at the top of the page, linked to the field.
    pub(crate) address_notices_chirho: Vec<FormNoticeChirho>,
    pub(crate) new_address_number_chirho: String,
    pub(crate) dark_base_chirho: &'static str,
    pub(crate) publish_button_label_chirho: String,
    pub(crate) republish_button_label_chirho: String,
}

impl PublicationStatusV1Chirho {
    pub fn available_chirho(
        snapshot_chirho: &PublicationStatusSnapshotV1Chirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let publication_chirho = &copy_chirho.publication_chirho;
        let publishable_version_chirho = snapshot_chirho.head_version_chirho;
        let sites_chirho: Vec<PublicationSiteViewV1Chirho> = snapshot_chirho
            .sites_chirho
            .iter()
            .map(|site_chirho| {
                let quiet_and_live_chirho =
                    site_chirho.is_live_chirho && site_chirho.pending_chirho.is_empty();
                // Only a stated saved version and a stated live version can be known equal.
                let saved_is_live_chirho = matches!(
                    (publishable_version_chirho, site_chirho.live_version_chirho),
                    (Some(saved_chirho), Some(live_chirho)) if saved_chirho == live_chirho
                );
                PublicationSiteViewV1Chirho {
                    slug_chirho: site_chirho.slug_chirho.clone(),
                    address_chirho: site_chirho.address_chirho.clone(),
                    approval_base_chirho: site_chirho.approval_base_chirho.clone(),
                    is_live_chirho: site_chirho.is_live_chirho,
                    held_chirho: site_chirho.is_live_chirho
                        || !site_chirho.pending_chirho.is_empty()
                        || site_chirho.history_chirho.iter().any(|event_chirho| {
                            event_chirho.outcome_chirho == PublicationOutcomeChirho::AppliedChirho
                        }),
                    can_republish_chirho: quiet_and_live_chirho && !saved_is_live_chirho,
                    live_is_saved_line_chirho: (quiet_and_live_chirho && saved_is_live_chirho)
                        .then(|| {
                            publication_chirho.live_is_saved_chirho(
                                publishable_version_chirho.unwrap_or_default(),
                            )
                        }),
                    consent_chirho: publishable_version_chirho
                        .map(|version_chirho| {
                            publication_chirho
                                .consent_chirho(version_chirho, &site_chirho.address_chirho)
                        })
                        .unwrap_or_default(),
                    live_version_chirho: site_chirho.live_version_chirho,
                    pending_chirho: site_chirho
                        .pending_chirho
                        .iter()
                        .map(|event_chirho| event_view_chirho(event_chirho, &copy_chirho, None))
                        .collect(),
                    history_chirho: site_chirho
                        .history_chirho
                        .iter()
                        .map(|event_chirho| {
                            event_view_chirho(
                                event_chirho,
                                &copy_chirho,
                                // no approval while a version is on its way
                                site_chirho
                                    .pending_chirho
                                    .is_empty()
                                    .then_some(site_chirho.live_params_chirho.as_deref()),
                            )
                        })
                        .collect(),
                }
            })
            .collect();
        let any_pending_chirho = sites_chirho
            .iter()
            .any(|site_chirho| !site_chirho.pending_chirho.is_empty());
        let live_chirho = sites_chirho
            .iter()
            .find(|site_chirho| site_chirho.is_live_chirho);
        let status_chirho = if any_pending_chirho {
            publication_chirho.status_pending_chirho.clone()
        } else if let Some(site_chirho) = live_chirho {
            match site_chirho.live_version_chirho {
                Some(version_chirho) => publication_chirho.live_status_chirho(version_chirho),
                None => publication_chirho.outcome_live_chirho.clone(),
            }
        } else {
            publication_chirho.status_not_published_chirho.clone()
        };
        let chrome_chirho = publication_chrome_chirho(&copy_chirho);
        // The header says what the panel below will say: nothing saved; the saved version is on
        // its way (an approval of it is queued); it is what is live; or it is ready to publish.
        let saved_version_pending_chirho = |version_chirho: u64| {
            sites_chirho.iter().any(|site_chirho| {
                site_chirho
                    .pending_chirho
                    .iter()
                    .any(|event_chirho| event_chirho.version_chirho == Some(version_chirho))
            })
        };
        let saved_version_live_chirho = |version_chirho: u64| {
            live_chirho
                .is_some_and(|site_chirho| site_chirho.live_version_chirho == Some(version_chirho))
        };
        let saved_version_line_chirho = match publishable_version_chirho {
            None => publication_chirho.nothing_saved_notice_chirho.clone(),
            Some(version_chirho) if saved_version_pending_chirho(version_chirho) => {
                publication_chirho.saved_version_pending_chirho(version_chirho)
            }
            Some(version_chirho)
                if !any_pending_chirho && saved_version_live_chirho(version_chirho) =>
            {
                publication_chirho.saved_version_live_chirho(version_chirho)
            }
            Some(version_chirho) => publication_chirho.saved_version_chirho(version_chirho),
        };
        let new_address_form_chirho =
            publishable_version_chirho.is_some() && live_chirho.is_none() && !any_pending_chirho;
        // The message meets her at the field only when that form is what the page offers next.
        let address_refusal_chirho = new_address_form_chirho
            .then(|| latest_address_refusal_chirho(snapshot_chirho))
            .flatten();
        let mut address_field_chirho =
            FieldViewChirho::clean_chirho(ADDRESS_MESSAGE_ID_CHIRHO.to_string());
        let mut address_notices_chirho = Vec::new();
        let mut refused_address_chirho = String::new();
        if let Some((slug_chirho, code_chirho)) = address_refusal_chirho {
            let message_chirho = publication_chirho
                .refusal_sentence_chirho(code_chirho)
                .to_string();
            address_field_chirho
                .messages_chirho
                .push(message_chirho.clone());
            address_notices_chirho.push(FormNoticeChirho {
                field_label_chirho: publication_chirho.slug_label_chirho.clone(),
                message_chirho,
                control_id_chirho: Some(ADDRESS_CONTROL_ID_CHIRHO.to_string()),
            });
            refused_address_chirho = slug_chirho.to_string();
        }
        address_field_chirho.settle_chirho(Some(ADDRESS_HELP_ID_CHIRHO.to_string()), None);
        let new_address_number_chirho = format!("{:02}", sites_chirho.len() + 1);
        let publish_button_label_chirho = publishable_version_chirho
            .map(|version_chirho| publication_chirho.publish_button_chirho(version_chirho))
            .unwrap_or_default();
        let republish_button_label_chirho = publishable_version_chirho
            .map(|version_chirho| publication_chirho.republish_button_chirho(version_chirho))
            .unwrap_or_default();
        Self {
            chrome_chirho,
            copy_chirho,
            status_chirho,
            available_chirho: true,
            sites_chirho,
            publish_action_chirho: None,
            publishable_version_chirho,
            saved_version_line_chirho,
            review_href_chirho: WORKSPACE_PREVIEW_HREF_CHIRHO.to_string(),
            new_address_form_chirho,
            address_field_chirho,
            refused_address_chirho,
            address_notices_chirho,
            new_address_number_chirho,
            dark_base_chirho: DARK_BASE_CHIRHO,
            publish_button_label_chirho,
            republish_button_label_chirho,
        }
    }

    /// The status page once the publish route is deployed: it hosts every approval form
    /// (new address, republish, rollback), each carrying the address's approval base.
    pub fn with_publish_action_chirho(mut self, action_chirho: &str) -> Self {
        self.publish_action_chirho = Some(action_chirho.to_string());
        self
    }

    pub fn unavailable_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let status_chirho = copy_chirho
            .publication_chirho
            .status_unavailable_chirho
            .clone();
        let chrome_chirho = publication_chrome_chirho(&copy_chirho);
        Self {
            chrome_chirho,
            copy_chirho,
            status_chirho,
            available_chirho: false,
            sites_chirho: Vec::new(),
            publish_action_chirho: None,
            publishable_version_chirho: None,
            saved_version_line_chirho: String::new(),
            review_href_chirho: WORKSPACE_PREVIEW_HREF_CHIRHO.to_string(),
            new_address_form_chirho: false,
            address_field_chirho: FieldViewChirho::clean_chirho(
                ADDRESS_MESSAGE_ID_CHIRHO.to_string(),
            ),
            refused_address_chirho: String::new(),
            address_notices_chirho: Vec::new(),
            new_address_number_chirho: String::new(),
            dark_base_chirho: DARK_BASE_CHIRHO,
            publish_button_label_chirho: String::new(),
            republish_button_label_chirho: String::new(),
        }
    }

    pub fn status_chirho(&self) -> &str {
        &self.status_chirho
    }

    pub fn is_available_chirho(&self) -> bool {
        self.available_chirho
    }
}

/// A rollback target is an applied record other than the live one, offered only when
/// `rollback_against_chirho` names the live record (`None` while a version is on its way).
fn event_view_chirho(
    event_chirho: &PublicationEventV1Chirho,
    copy_chirho: &WorkspaceCopyChirho,
    rollback_against_chirho: Option<Option<&str>>,
) -> PublicationEventViewV1Chirho {
    let publication_chirho = &copy_chirho.publication_chirho;
    let (outcome_label_chirho, detail_chirho) = match &event_chirho.outcome_chirho {
        PublicationOutcomeChirho::PendingChirho => {
            (publication_chirho.outcome_pending_chirho.clone(), None)
        }
        PublicationOutcomeChirho::AppliedChirho if event_chirho.is_live_chirho => {
            (publication_chirho.outcome_live_chirho.clone(), None)
        }
        PublicationOutcomeChirho::AppliedChirho => {
            (publication_chirho.outcome_applied_chirho.clone(), None)
        }
        PublicationOutcomeChirho::RefusedChirho(code_chirho) => (
            publication_chirho.outcome_refused_chirho.clone(),
            Some(
                publication_chirho
                    .refusal_sentence_chirho(code_chirho)
                    .to_string(),
            ),
        ),
    };
    let rollback_params_chirho = match (
        &event_chirho.outcome_chirho,
        &event_chirho.params_chirho,
        rollback_against_chirho,
    ) {
        (
            PublicationOutcomeChirho::AppliedChirho,
            Some(params_chirho),
            Some(live_params_chirho),
        ) if live_params_chirho != Some(params_chirho.as_str()) => Some(params_chirho.clone()),
        _ => None,
    };
    let rollback_button_chirho =
        rollback_params_chirho
            .as_ref()
            .map(|_| match event_chirho.version_chirho {
                Some(version_chirho) => publication_chirho.rollback_button_chirho(version_chirho),
                None => publication_chirho.kind_rollback_chirho.clone(),
            });
    PublicationEventViewV1Chirho {
        kind_label_chirho: publication_chirho
            .kind_label_chirho(&event_chirho.kind_chirho)
            .to_string(),
        version_chirho: event_chirho.version_chirho,
        outcome_label_chirho,
        detail_chirho,
        rollback_params_chirho,
        rollback_button_chirho,
    }
}

fn publication_chrome_chirho(copy_chirho: &WorkspaceCopyChirho) -> SiteChromeChirho {
    workspace_chrome_chirho(
        copy_chirho,
        &copy_chirho.publication_chirho.page_title_chirho,
        &copy_chirho.publication_chirho.page_description_chirho,
        WorkspaceNavPageChirho::PublishingChirho,
        WorkspaceNavRoadsChirho::deployed_chirho(),
    )
}
