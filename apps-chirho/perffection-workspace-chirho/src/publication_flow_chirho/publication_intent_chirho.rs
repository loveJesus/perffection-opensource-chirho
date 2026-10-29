// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Self-service publication intents: her explicit approval act, the door request that carries
//! only a version binding (never the document), and the queued answer.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.
//!
//! Contract (cairn branch `claude-publication-intent-chirho`, CAIRN #21783 + R1 pick (b) #21789):
//! on a tenant root the blessed `template-app-deploy` card records a signed publication intent.
//! With `viewer_state_chirho` present the door re-reads her verified head, refuses if it moved,
//! and publishes THAT head's bytes; `params_chirho` must then be absent. Rollback names one of her
//! own prior params records by hash. The host injects `user_principal_chirho`; we never add it.

use crate::field_view_chirho::{FieldViewChirho, FormNoticeChirho};
use crate::page_edit_request_chirho::{parse_form_fields_chirho, PageEditRequestErrorChirho};
use crate::viewer_state_contract_chirho::PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO;
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
    WORKSPACE_PUBLICATION_HREF_CHIRHO, WORKSPACE_PUBLISH_ACTION_CHIRHO,
};
use perffection_ui_chirho::{SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho};
use serde::{Deserialize, Serialize};

pub const PUBLICATION_INTENT_REQUEST_SCHEMA_CHIRHO: &str = "template-app-deploy-request-v1-chirho";
pub const PUBLICATION_INTENT_RESPONSE_SCHEMA_CHIRHO: &str = "publication-intent-response-v1-chirho";
/// The page template registered once on the platform root; the blessed card's scope names it.
pub const PUBLICATION_TEMPLATE_ID_CHIRHO: &str = "perffection-page-template-chirho";
/// The door's refusal of a name that is another account's (no intent queued).
pub const SLUG_TAKEN_REFUSAL_CODE_CHIRHO: &str = "slug-taken-chirho";
/// The address form's one control and what describes it, wherever that form is rendered (the
/// Publishing page and the door's refusal page); the templates' literal ids must match.
pub const ADDRESS_CONTROL_ID_CHIRHO: &str = "publish-slug-chirho";
pub const ADDRESS_HELP_ID_CHIRHO: &str = "publish-slug-help-chirho";
pub const ADDRESS_MESSAGE_ID_CHIRHO: &str = "publish-slug-error-chirho";
pub const PUBLISH_KIND_CHIRHO: &str = "publish-chirho";
pub const REPUBLISH_KIND_CHIRHO: &str = "republish-chirho";
pub const ROLLBACK_KIND_CHIRHO: &str = "rollback-chirho";
pub const PUBLICATION_FORM_MAX_BYTES_CHIRHO: usize = 4 * 1024;
pub const SLUG_MAX_CHARACTERS_CHIRHO: usize = 48;
/// The approval base of an address that has never been applied (CAIRN #21800).
pub const DARK_BASE_CHIRHO: &str = "dark-chirho";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationFormErrorChirho {
    UnreadableChirho,
    InvalidSlugChirho,
    NothingSavedChirho,
    InvalidRollbackTargetChirho,
    UnknownKindChirho,
}

/// One approval act, parsed from the exact product form. The document is never a field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationApprovalChirho {
    /// Publish or republish the saved version she was looking at.
    VersionChirho {
        kind_chirho: &'static str,
        slug_chirho: String,
        version_chirho: u64,
        receipt_digest_chirho: Option<String>,
        /// The address's approval base as the status page rendered it (live applied intent
        /// digest, or `dark-chirho`); the door refuses before any write if it moved.
        base_publication_chirho: String,
    },
    /// Make one of this address's own earlier published records live again.
    RollbackChirho {
        slug_chirho: String,
        params_hash_chirho: String,
        version_chirho: Option<u64>,
        base_publication_chirho: String,
    },
}

impl PublicationApprovalChirho {
    pub fn slug_chirho(&self) -> &str {
        match self {
            Self::VersionChirho { slug_chirho, .. } | Self::RollbackChirho { slug_chirho, .. } => {
                slug_chirho
            }
        }
    }

    /// One key per approval act, folded from what she approved: kind, address, version binding
    /// and the approval base the page showed her (CAIRN #21800). Identical form bytes replay the
    /// original receipt; a fresh approval after the address moved carries a new base and is a
    /// new act. Hashed to the engine's short-id grammar (64 hex).
    pub fn idem_key_chirho(&self) -> String {
        let material_chirho = match self {
            Self::VersionChirho {
                kind_chirho,
                slug_chirho,
                version_chirho,
                receipt_digest_chirho,
                base_publication_chirho,
            } => format!(
                "perffection-publication-attempt-v2-chirho\n{kind_chirho}\n{slug_chirho}\n{version_chirho}\n{}\n{base_publication_chirho}",
                receipt_digest_chirho.as_deref().unwrap_or("")
            ),
            Self::RollbackChirho {
                slug_chirho,
                params_hash_chirho,
                base_publication_chirho,
                ..
            } => format!(
                "perffection-publication-attempt-v2-chirho\n{ROLLBACK_KIND_CHIRHO}\n{slug_chirho}\n{params_hash_chirho}\n{base_publication_chirho}"
            ),
        };
        blake3::hash(material_chirho.as_bytes())
            .to_hex()
            .to_string()
    }
}

pub fn valid_slug_chirho(slug_chirho: &str) -> bool {
    !slug_chirho.is_empty()
        && slug_chirho.len() <= SLUG_MAX_CHARACTERS_CHIRHO
        && !slug_chirho.starts_with('-')
        && !slug_chirho.ends_with('-')
        && slug_chirho.chars().all(|character_chirho| {
            character_chirho.is_ascii_lowercase()
                || character_chirho.is_ascii_digit()
                || character_chirho == '-'
        })
}

/// The engine's content-address grammar (`validate_hex_hash_chirho`): 64 lowercase hex chars.
fn valid_params_hash_chirho(value_chirho: &str) -> bool {
    value_chirho.len() == 64
        && value_chirho.chars().all(|character_chirho| {
            character_chirho.is_ascii_digit() || matches!(character_chirho, 'a'..='f')
        })
}

fn valid_base_chirho(value_chirho: &str) -> bool {
    value_chirho == DARK_BASE_CHIRHO || valid_params_hash_chirho(value_chirho)
}

pub fn parse_publication_form_chirho(
    body_chirho: &[u8],
) -> Result<PublicationApprovalChirho, PublicationFormErrorChirho> {
    let mut fields_chirho =
        parse_form_fields_chirho(body_chirho, PUBLICATION_FORM_MAX_BYTES_CHIRHO).map_err(
            |_error_chirho: PageEditRequestErrorChirho| {
                PublicationFormErrorChirho::UnreadableChirho
            },
        )?;
    let kind_chirho = fields_chirho
        .remove("kind_chirho")
        .ok_or(PublicationFormErrorChirho::UnreadableChirho)?;
    let slug_chirho = fields_chirho
        .remove("slug_chirho")
        .ok_or(PublicationFormErrorChirho::UnreadableChirho)?
        .trim()
        .to_string();
    if !valid_slug_chirho(&slug_chirho) {
        return Err(PublicationFormErrorChirho::InvalidSlugChirho);
    }
    let version_chirho = fields_chirho
        .remove("version_chirho")
        .map(|value_chirho| value_chirho.parse::<u64>())
        .transpose()
        .map_err(|_error_chirho| PublicationFormErrorChirho::UnreadableChirho)?;
    let receipt_digest_chirho = fields_chirho
        .remove("receipt_digest_chirho")
        .filter(|value_chirho| !value_chirho.is_empty());
    let params_hash_chirho = fields_chirho.remove("params_hash_chirho");
    let base_publication_chirho = fields_chirho
        .remove("base_publication_chirho")
        .ok_or(PublicationFormErrorChirho::UnreadableChirho)?;
    if !valid_base_chirho(&base_publication_chirho) {
        return Err(PublicationFormErrorChirho::UnreadableChirho);
    }
    if !fields_chirho.is_empty() {
        return Err(PublicationFormErrorChirho::UnreadableChirho);
    }
    match kind_chirho.as_str() {
        PUBLISH_KIND_CHIRHO | REPUBLISH_KIND_CHIRHO => {
            let version_chirho = version_chirho.unwrap_or(0);
            if version_chirho == 0 {
                return Err(PublicationFormErrorChirho::NothingSavedChirho);
            }
            Ok(PublicationApprovalChirho::VersionChirho {
                kind_chirho: if kind_chirho == PUBLISH_KIND_CHIRHO {
                    PUBLISH_KIND_CHIRHO
                } else {
                    REPUBLISH_KIND_CHIRHO
                },
                slug_chirho,
                version_chirho,
                receipt_digest_chirho,
                base_publication_chirho,
            })
        }
        ROLLBACK_KIND_CHIRHO => {
            let params_hash_chirho = params_hash_chirho
                .filter(|value_chirho| valid_params_hash_chirho(value_chirho))
                .ok_or(PublicationFormErrorChirho::InvalidRollbackTargetChirho)?;
            Ok(PublicationApprovalChirho::RollbackChirho {
                slug_chirho,
                params_hash_chirho,
                version_chirho,
                base_publication_chirho,
            })
        }
        _ => Err(PublicationFormErrorChirho::UnknownKindChirho),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ViewerStateBindingV1Chirho {
    key_chirho: &'static str,
    version_chirho: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_digest_chirho: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct PublicationIntentRequestV1Chirho {
    schema_chirho: &'static str,
    kind_chirho: &'static str,
    slug_chirho: String,
    idem_key_chirho: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    viewer_state_chirho: Option<ViewerStateBindingV1Chirho>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params_hash_chirho: Option<String>,
    base_publication_chirho: String,
    template_id_chirho: &'static str,
}

/// The door request: a version binding for publish/republish (no document on the wire), a prior
/// params hash for rollback. Never a principal, never `params_chirho`.
pub fn build_publication_intent_request_chirho(
    approval_chirho: &PublicationApprovalChirho,
) -> Vec<u8> {
    let request_chirho = match approval_chirho {
        PublicationApprovalChirho::VersionChirho {
            kind_chirho,
            slug_chirho,
            version_chirho,
            receipt_digest_chirho,
            base_publication_chirho,
        } => PublicationIntentRequestV1Chirho {
            schema_chirho: PUBLICATION_INTENT_REQUEST_SCHEMA_CHIRHO,
            kind_chirho,
            slug_chirho: slug_chirho.clone(),
            idem_key_chirho: approval_chirho.idem_key_chirho(),
            viewer_state_chirho: Some(ViewerStateBindingV1Chirho {
                key_chirho: PAGE_DOCUMENT_LOGICAL_KEY_CHIRHO,
                version_chirho: *version_chirho,
                receipt_digest_chirho: receipt_digest_chirho.clone(),
            }),
            params_hash_chirho: None,
            base_publication_chirho: base_publication_chirho.clone(),
            template_id_chirho: PUBLICATION_TEMPLATE_ID_CHIRHO,
        },
        PublicationApprovalChirho::RollbackChirho {
            slug_chirho,
            params_hash_chirho,
            base_publication_chirho,
            ..
        } => PublicationIntentRequestV1Chirho {
            schema_chirho: PUBLICATION_INTENT_REQUEST_SCHEMA_CHIRHO,
            kind_chirho: ROLLBACK_KIND_CHIRHO,
            slug_chirho: slug_chirho.clone(),
            idem_key_chirho: approval_chirho.idem_key_chirho(),
            viewer_state_chirho: None,
            params_hash_chirho: Some(params_hash_chirho.clone()),
            base_publication_chirho: base_publication_chirho.clone(),
            template_id_chirho: PUBLICATION_TEMPLATE_ID_CHIRHO,
        },
    };
    serde_json::to_vec(&request_chirho).expect("a typed intent request serializes")
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ViewerStateEchoV1Chirho {
    #[serde(default)]
    version_chirho: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct PublicationIntentResponseV1Chirho {
    schema_chirho: String,
    intent_digest_chirho: String,
    kind_chirho: String,
    slug_chirho: String,
    #[serde(default)]
    address_chirho: Option<String>,
    #[serde(default)]
    viewer_state_chirho: Option<ViewerStateEchoV1Chirho>,
    #[serde(default)]
    applied_chirho: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationIntentReceiptV1Chirho {
    pub intent_digest_chirho: String,
    pub kind_chirho: String,
    pub slug_chirho: String,
    pub address_chirho: String,
    pub version_chirho: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationIntentErrorChirho {
    MalformedResponseChirho,
    UnsupportedResponseSchemaChirho,
    AlreadyAppliedClaimChirho,
}

/// The door's typed refusal (CAIRN #21828, viewer-state's conflict pattern): a state-dependent
/// refusal — the approval base or her head moved, nothing saved, the saved head is not a
/// document, or the request could not be read — answered as a body the page turns into words
/// instead of a bare host error. It carries no slug and no address; the engine's fixed
/// `message_chirho` is not shown (the catalog owns her words), `live_base_chirho` and
/// `head_version_chirho` are read for the record only.
pub const PUBLICATION_INTENT_REFUSAL_SCHEMA_CHIRHO: &str = "publication-intent-refusal-v1-chirho";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct PublicationIntentRefusalResponseV1Chirho {
    schema_chirho: String,
    refusal_code_chirho: String,
    #[serde(default)]
    live_base_chirho: Option<String>,
    #[serde(default)]
    head_version_chirho: Option<u64>,
    /// Only with `slug-taken-chirho`: the caller's own validated submitted name, echoed so the
    /// page can hand it back. Never anything of the other owner's (CAIRN #24153).
    #[serde(default)]
    slug_chirho: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationIntentRefusalV1Chirho {
    pub refusal_code_chirho: String,
    pub live_base_chirho: Option<String>,
    pub head_version_chirho: Option<u64>,
    pub slug_chirho: Option<String>,
}

/// Everything the door can answer: a queued receipt, or a typed refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationIntentAnswerChirho {
    QueuedChirho(PublicationIntentReceiptV1Chirho),
    RefusedChirho(PublicationIntentRefusalV1Chirho),
}

pub fn decode_publication_intent_answer_chirho(
    response_bytes_chirho: &[u8],
) -> Result<PublicationIntentAnswerChirho, PublicationIntentErrorChirho> {
    #[derive(Deserialize)]
    struct SchemaOnlyChirho {
        schema_chirho: String,
    }
    let schema_chirho: SchemaOnlyChirho = serde_json::from_slice(response_bytes_chirho)
        .map_err(|_| PublicationIntentErrorChirho::MalformedResponseChirho)?;
    if schema_chirho.schema_chirho == PUBLICATION_INTENT_REFUSAL_SCHEMA_CHIRHO {
        let refusal_chirho: PublicationIntentRefusalResponseV1Chirho =
            serde_json::from_slice(response_bytes_chirho)
                .map_err(|_| PublicationIntentErrorChirho::MalformedResponseChirho)?;
        // On a taken-name refusal any live base is the OTHER owner's: dropped at decode, so it
        // never exists in memory (the render never showed it; CAIRN claude2 #24595 asked that the
        // receiver be a second fence, not only the door). On every other refusal the base is hers.
        let live_base_chirho = (refusal_chirho.refusal_code_chirho
            != SLUG_TAKEN_REFUSAL_CODE_CHIRHO)
            .then_some(refusal_chirho.live_base_chirho)
            .flatten();
        return Ok(PublicationIntentAnswerChirho::RefusedChirho(
            PublicationIntentRefusalV1Chirho {
                refusal_code_chirho: refusal_chirho.refusal_code_chirho,
                live_base_chirho,
                head_version_chirho: refusal_chirho.head_version_chirho,
                slug_chirho: refusal_chirho.slug_chirho,
            },
        ));
    }
    decode_publication_intent_response_chirho(response_bytes_chirho)
        .map(PublicationIntentAnswerChirho::QueuedChirho)
}

pub fn decode_publication_intent_response_chirho(
    response_bytes_chirho: &[u8],
) -> Result<PublicationIntentReceiptV1Chirho, PublicationIntentErrorChirho> {
    let response_chirho: PublicationIntentResponseV1Chirho =
        serde_json::from_slice(response_bytes_chirho)
            .map_err(|_| PublicationIntentErrorChirho::MalformedResponseChirho)?;
    if response_chirho.schema_chirho != PUBLICATION_INTENT_RESPONSE_SCHEMA_CHIRHO {
        return Err(PublicationIntentErrorChirho::UnsupportedResponseSchemaChirho);
    }
    if response_chirho.applied_chirho {
        // The door answers queued by construction; a "live" claim here is not ours to render.
        return Err(PublicationIntentErrorChirho::AlreadyAppliedClaimChirho);
    }
    let address_chirho = response_chirho
        .address_chirho
        .unwrap_or_else(|| format!("/app-chirho/{}/", response_chirho.slug_chirho));
    Ok(PublicationIntentReceiptV1Chirho {
        intent_digest_chirho: response_chirho.intent_digest_chirho,
        kind_chirho: response_chirho.kind_chirho,
        slug_chirho: response_chirho.slug_chirho,
        address_chirho,
        version_chirho: response_chirho
            .viewer_state_chirho
            .and_then(|state_chirho| state_chirho.version_chirho),
    })
}

/// The one page both branches of the publish route render: queued, or not sent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationIntentV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) status_chirho: String,
    pub(crate) heading_chirho: String,
    pub(crate) body_chirho: String,
    pub(crate) queued_chirho: bool,
    pub(crate) address_chirho: Option<String>,
    pub(crate) status_href_chirho: String,
    /// The door refused the name as another account's: the address form again, her name in it.
    pub(crate) address_retry_chirho: Option<PublicationAddressRetryV1Chirho>,
}

/// The new-address form on the door's own answer page, when the name she asked for is another
/// account's (`slug-taken-chirho`). Everything in it is HERS: the validated name the door echoed,
/// her verified head version from the door's answer, and the dark base a new name starts from.
/// Nothing of the other owner's is read into it, whatever the answer carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationAddressRetryV1Chirho {
    pub(crate) publish_action_chirho: String,
    pub(crate) slug_chirho: String,
    pub(crate) version_chirho: u64,
    pub(crate) dark_base_chirho: &'static str,
    pub(crate) field_chirho: FieldViewChirho,
    pub(crate) notices_chirho: Vec<FormNoticeChirho>,
    pub(crate) publish_button_label_chirho: String,
}

impl PublicationIntentV1Chirho {
    pub fn queued_chirho(
        receipt_chirho: &PublicationIntentReceiptV1Chirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let publication_chirho = &copy_chirho.publication_chirho;
        let body_chirho = match receipt_chirho.version_chirho {
            Some(version_chirho) if receipt_chirho.kind_chirho != ROLLBACK_KIND_CHIRHO => {
                publication_chirho
                    .queued_body_chirho(version_chirho, &receipt_chirho.address_chirho)
            }
            _ => publication_chirho.queued_rollback_body_chirho.clone(),
        };
        let chrome_chirho = intent_chrome_chirho(&copy_chirho);
        Self {
            chrome_chirho,
            status_chirho: publication_chirho.queued_status_chirho.clone(),
            heading_chirho: publication_chirho.queued_heading_chirho.clone(),
            body_chirho,
            queued_chirho: true,
            address_chirho: Some(receipt_chirho.address_chirho.clone()),
            status_href_chirho: WORKSPACE_PUBLICATION_HREF_CHIRHO.to_string(),
            address_retry_chirho: None,
            copy_chirho,
        }
    }

    pub fn rejected_chirho(
        error_chirho: PublicationFormErrorChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let publication_chirho = &copy_chirho.publication_chirho;
        let body_chirho = match error_chirho {
            PublicationFormErrorChirho::InvalidSlugChirho => {
                publication_chirho.invalid_slug_notice_chirho.clone()
            }
            PublicationFormErrorChirho::NothingSavedChirho => {
                publication_chirho.nothing_saved_notice_chirho.clone()
            }
            PublicationFormErrorChirho::InvalidRollbackTargetChirho => {
                publication_chirho.invalid_rollback_notice_chirho.clone()
            }
            PublicationFormErrorChirho::UnreadableChirho
            | PublicationFormErrorChirho::UnknownKindChirho => {
                publication_chirho.unknown_request_notice_chirho.clone()
            }
        };
        let chrome_chirho = intent_chrome_chirho(&copy_chirho);
        Self {
            chrome_chirho,
            status_chirho: publication_chirho.rejected_status_chirho.clone(),
            heading_chirho: publication_chirho.rejected_heading_chirho.clone(),
            body_chirho,
            queued_chirho: false,
            address_chirho: None,
            status_href_chirho: WORKSPACE_PUBLICATION_HREF_CHIRHO.to_string(),
            address_retry_chirho: None,
            copy_chirho,
        }
    }

    /// A door answer we could not verify: say so, send nothing more, offer the status page.
    /// The door refused before any write: her words for the code, and the way back.
    pub fn refused_chirho(
        refusal_chirho: &PublicationIntentRefusalV1Chirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let mut model_chirho =
            Self::rejected_chirho(PublicationFormErrorChirho::UnreadableChirho, locale_chirho);
        model_chirho.heading_chirho = model_chirho
            .copy_chirho
            .publication_chirho
            .door_refused_heading_chirho
            .clone();
        model_chirho.body_chirho = model_chirho
            .copy_chirho
            .publication_chirho
            .door_refusal_sentence_chirho(&refusal_chirho.refusal_code_chirho)
            .to_string();
        model_chirho.address_retry_chirho = address_retry_chirho(refusal_chirho, &model_chirho);
        model_chirho
    }

    pub fn unverifiable_chirho(locale_chirho: UiLocaleChirho) -> Self {
        let mut model_chirho =
            Self::rejected_chirho(PublicationFormErrorChirho::UnreadableChirho, locale_chirho);
        model_chirho.body_chirho = model_chirho
            .copy_chirho
            .publication_chirho
            .status_unavailable_chirho
            .clone();
        model_chirho
    }

    pub fn is_queued_chirho(&self) -> bool {
        self.queued_chirho
    }
}

/// The address form on the refusal page, only when every part of it is hers and verified: the
/// code is the taken-name refusal, the echoed name passes OUR slug rule (an echo is never trusted
/// into a form value), and the door stated her head version above zero (a version the page did
/// not get from the door is never presented as current). Otherwise the page stays words.
fn address_retry_chirho(
    refusal_chirho: &PublicationIntentRefusalV1Chirho,
    model_chirho: &PublicationIntentV1Chirho,
) -> Option<PublicationAddressRetryV1Chirho> {
    if refusal_chirho.refusal_code_chirho != SLUG_TAKEN_REFUSAL_CODE_CHIRHO {
        return None;
    }
    let slug_chirho = refusal_chirho
        .slug_chirho
        .as_deref()
        .filter(|slug_chirho| valid_slug_chirho(slug_chirho))?;
    let version_chirho = refusal_chirho
        .head_version_chirho
        .filter(|version_chirho| *version_chirho > 0)?;
    let publication_chirho = &model_chirho.copy_chirho.publication_chirho;
    let mut field_chirho = FieldViewChirho::clean_chirho(ADDRESS_MESSAGE_ID_CHIRHO.to_string());
    field_chirho
        .messages_chirho
        .push(model_chirho.body_chirho.clone());
    field_chirho.settle_chirho(Some(ADDRESS_HELP_ID_CHIRHO.to_string()), None);
    Some(PublicationAddressRetryV1Chirho {
        publish_action_chirho: WORKSPACE_PUBLISH_ACTION_CHIRHO.to_string(),
        slug_chirho: slug_chirho.to_string(),
        version_chirho,
        dark_base_chirho: DARK_BASE_CHIRHO,
        field_chirho,
        notices_chirho: vec![FormNoticeChirho {
            field_label_chirho: publication_chirho.slug_label_chirho.clone(),
            message_chirho: model_chirho.body_chirho.clone(),
            control_id_chirho: Some(ADDRESS_CONTROL_ID_CHIRHO.to_string()),
        }],
        publish_button_label_chirho: publication_chirho.publish_button_chirho(version_chirho),
    })
}

fn intent_chrome_chirho(copy_chirho: &WorkspaceCopyChirho) -> SiteChromeChirho {
    workspace_chrome_chirho(
        copy_chirho,
        &copy_chirho.publication_chirho.intent_page_title_chirho,
        &copy_chirho
            .publication_chirho
            .intent_page_description_chirho,
        WorkspaceNavPageChirho::PublishingChirho,
        WorkspaceNavRoadsChirho::deployed_chirho(),
    )
}
