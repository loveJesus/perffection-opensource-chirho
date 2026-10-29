// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Typed display catalog for the private workspace surfaces.
//!
//! URLs, form field names and viewer-state contracts intentionally do not appear here. A locale
//! can change what a page says and how it flows, never where a form posts or what it may mutate.

use crate::{
    locale_chirho::load_catalog_chirho, ChromeNavLinkChirho, SiteChromeChirho, UiLocaleChirho,
    WorkspaceConnectionsCopyChirho, WorkspaceFiguresCopyChirho,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceChromeCopyChirho {
    pub brand_chirho: String,
    pub private_marker_chirho: String,
    pub nav_label_chirho: String,
    pub preview_label_chirho: String,
    pub export_label_chirho: String,
    pub import_label_chirho: String,
    pub publish_label_chirho: String,
    pub account_label_chirho: String,
    pub draft_label_chirho: String,
    pub editor_label_chirho: String,
    pub help_label_chirho: String,
    pub logout_label_chirho: String,
    pub footer_note_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceEditorCopyChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub unavailable_page_title_chirho: String,
    /// The tab's title when a save was refused or lost a race: the first thing a reader hears.
    pub not_saved_page_title_chirho: String,
    pub status_private_chirho: String,
    pub status_rejected_chirho: String,
    pub status_conflict_chirho: String,
    pub status_imported_chirho: String,
    pub import_review_notice_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    pub current_version_label_chirho: String,
    pub not_saved_heading_chirho: String,
    pub validation_heading_chirho: String,
    pub validation_body_chirho: String,
    pub business_name_label_chirho: String,
    pub business_name_help_chirho: String,
    pub headline_label_chirho: String,
    pub headline_help_chirho: String,
    pub summary_label_chirho: String,
    pub summary_help_chirho: String,
    pub services_heading_chirho: String,
    pub services_help_chirho: String,
    pub service_label_chirho: String,
    pub service_title_label_chirho: String,
    pub service_body_label_chirho: String,
    pub contact_section_heading_chirho: String,
    pub contact_section_intro_chirho: String,
    pub contact_heading_label_chirho: String,
    /// The phone and email pair: one of the two is required, so the rule lives on the group.
    pub contact_method_label_chirho: String,
    pub contact_method_help_chirho: String,
    pub contact_phone_label_chirho: String,
    pub contact_phone_title_chirho: String,
    pub contact_phone_help_chirho: String,
    pub contact_email_label_chirho: String,
    pub contact_email_help_chirho: String,
    pub contact_details_label_chirho: String,
    pub contact_details_help_chirho: String,
    pub save_boundary_chirho: String,
    pub save_label_chirho: String,
    pub recovery_eyebrow_chirho: String,
    pub history_heading_chirho: String,
    pub history_intro_chirho: String,
    pub history_unknown_chirho: String,
    pub history_empty_chirho: String,
    pub version_label_chirho: String,
    pub current_draft_label_chirho: String,
    pub restore_version_label_chirho: String,
    pub unavailable_heading_chirho: String,
    pub unavailable_safety_chirho: String,
    pub current_draft_heading_chirho: String,
    pub readback_business_label_chirho: String,
    pub readback_headline_label_chirho: String,
    pub readback_summary_label_chirho: String,
    pub readback_phone_label_chirho: String,
    pub readback_email_label_chirho: String,
    pub readback_details_label_chirho: String,
    pub conflict_attempted_template_chirho: String,
    pub conflict_head_template_chirho: String,
    pub unverifiable_status_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceImportCopyChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub status_ready_chirho: String,
    pub status_rejected_chirho: String,
    pub status_unreadable_chirho: String,
    pub status_unavailable_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    pub current_version_label_chirho: String,
    pub rejection_heading_chirho: String,
    pub panel_number_chirho: String,
    pub panel_heading_chirho: String,
    pub panel_intro_chirho: String,
    pub document_label_chirho: String,
    pub document_help_chirho: String,
    pub review_boundary_chirho: String,
    pub review_label_chirho: String,
    pub invalid_form_notice_chirho: String,
    pub invalid_json_notice_chirho: String,
    pub unsupported_schema_notice_chirho: String,
    pub document_too_large_notice_chirho: String,
    pub unreadable_heading_chirho: String,
    pub unreadable_body_chirho: String,
    pub unavailable_heading_chirho: String,
    pub unavailable_body_chirho: String,
    pub reopen_label_chirho: String,
    pub back_label_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
}

impl WorkspaceEditorCopyChirho {
    pub fn conflict_attempted_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.conflict_attempted_template_chirho, version_chirho)
    }

    pub fn conflict_head_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.conflict_head_template_chirho, version_chirho)
    }
}

/// The first minute of a new customer (L.J. 2026-09-20: not "pick a trade, get an example page";
/// the shape of his Genesis intake instead): ONE form, and two ways to fill it - one thing at a time,
/// or everything at once - as views over the same mounted fields, an example in every box, and a
/// progress line. Switching never loses work because nothing is unmounted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceGuidedServiceExampleChirho {
    pub title_chirho: String,
    pub body_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceGuidedCopyChirho {
    pub choose_heading_chirho: String,
    pub choose_body_chirho: String,
    pub guided_action_chirho: String,
    pub all_action_chirho: String,
    pub progress_template_chirho: String,
    pub previous_label_chirho: String,
    pub next_label_chirho: String,
    /// Shown on the last guided step only while an earlier step still has a missing answer.
    pub still_needed_chirho: String,
    pub name_heading_chirho: String,
    pub name_intro_chirho: String,
    pub headline_heading_chirho: String,
    pub headline_intro_chirho: String,
    pub summary_heading_chirho: String,
    pub summary_intro_chirho: String,
    pub example_business_name_chirho: String,
    pub example_headline_chirho: String,
    pub example_summary_chirho: String,
    pub example_services_chirho: Vec<WorkspaceGuidedServiceExampleChirho>,
    pub example_contact_heading_chirho: String,
    pub example_contact_phone_chirho: String,
    pub example_contact_email_chirho: String,
    pub example_contact_details_chirho: String,
}

/// The guided view's steps: name, headline, introduction, services, contact.
pub const WORKSPACE_GUIDED_STEP_COUNT_CHIRHO: usize = 5;

impl WorkspaceGuidedCopyChirho {
    pub fn progress_chirho(&self, step_chirho: usize) -> String {
        self.progress_template_chirho
            .replace("{step_chirho}", &step_chirho.to_string())
            .replace(
                "{total_chirho}",
                &WORKSPACE_GUIDED_STEP_COUNT_CHIRHO.to_string(),
            )
    }

    /// The example for a service slot; slots past the examples reuse the last one. Askama hands a
    /// loop index over by reference, so the index is taken through `Borrow`.
    pub fn example_service_chirho(
        &self,
        index_chirho: impl std::borrow::Borrow<usize>,
    ) -> &WorkspaceGuidedServiceExampleChirho {
        self.example_services_chirho
            .get(*index_chirho.borrow())
            .or_else(|| self.example_services_chirho.last())
            .expect("the catalog lists at least one service example")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceExportCopyChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub saved_status_template_chirho: String,
    pub nothing_saved_status_chirho: String,
    pub unverifiable_status_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    pub version_label_chirho: String,
    pub panel_number_chirho: String,
    pub panel_heading_chirho: String,
    pub panel_intro_chirho: String,
    pub document_label_chirho: String,
    pub document_help_chirho: String,
    pub download_label_chirho: String,
    pub unavailable_heading_chirho: String,
    pub back_label_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
}

impl WorkspaceExportCopyChirho {
    pub fn saved_status_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.saved_status_template_chirho, version_chirho)
    }
}

/// Copy for the publishing status surface. Refusal sentences are keyed by the engine's fixed
/// refusal tokens (wire words carry the `-chirho` suffix, cairn branch claude-publication-intent-chirho);
/// an unknown token gets the generic sentence rather than an engine code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePublicationCopyChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    pub status_not_published_chirho: String,
    pub status_live_template_chirho: String,
    pub status_pending_chirho: String,
    pub status_unavailable_chirho: String,
    pub address_label_chirho: String,
    /// Under a name she only asked for and was refused: never "your address", never its path.
    pub address_tried_note_chirho: String,
    pub live_version_label_chirho: String,
    pub not_published_heading_chirho: String,
    pub not_published_body_chirho: String,
    pub pending_heading_chirho: String,
    pub pending_body_chirho: String,
    pub history_heading_chirho: String,
    pub history_intro_chirho: String,
    pub history_empty_chirho: String,
    pub kind_publish_chirho: String,
    pub kind_republish_chirho: String,
    pub kind_rollback_chirho: String,
    pub kind_unknown_chirho: String,
    pub outcome_applied_chirho: String,
    pub outcome_live_chirho: String,
    pub outcome_refused_chirho: String,
    pub outcome_pending_chirho: String,
    pub refusal_slug_reserved_chirho: String,
    pub refusal_slug_taken_chirho: String,
    pub refusal_not_your_app_chirho: String,
    pub refusal_unknown_prior_params_chirho: String,
    pub refusal_unknown_template_chirho: String,
    pub refusal_intent_invalid_chirho: String,
    pub refusal_params_missing_chirho: String,
    pub refusal_template_not_deployable_chirho: String,
    pub refusal_unknown_chirho: String,
    pub door_refused_heading_chirho: String,
    pub door_refusal_base_moved_chirho: String,
    pub door_refusal_version_stale_chirho: String,
    pub door_refusal_no_saved_version_chirho: String,
    pub door_refusal_not_a_document_chirho: String,
    pub door_refusal_request_invalid_chirho: String,
    pub retention_note_chirho: String,
    pub unavailable_heading_chirho: String,
    pub back_label_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
    pub form_number_chirho: String,
    pub form_heading_chirho: String,
    pub form_intro_chirho: String,
    /// Above the Publishing page when the address she chose was refused: what did not happen,
    /// and that the way forward is another name.
    pub address_refused_heading_chirho: String,
    pub address_refused_body_chirho: String,
    pub slug_label_chirho: String,
    pub slug_help_chirho: String,
    pub slug_title_chirho: String,
    pub consent_template_chirho: String,
    pub publish_button_template_chirho: String,
    pub rollback_button_template_chirho: String,
    pub republish_button_template_chirho: String,
    pub saved_version_template_chirho: String,
    pub saved_version_live_template_chirho: String,
    pub saved_version_pending_template_chirho: String,
    pub review_link_label_chirho: String,
    pub operator_publish_body_chirho: String,
    pub new_address_heading_chirho: String,
    pub republish_heading_chirho: String,
    /// Replaces the republish form when the saved version is known to be the live one.
    pub live_is_saved_template_chirho: String,
    pub pending_wait_chirho: String,
    pub editor_pointer_template_chirho: String,
    /// The editor's neutral pointer: it cannot see what is live, so it never offers to publish.
    pub editor_publishing_link_chirho: String,
    pub intent_page_title_chirho: String,
    pub intent_page_description_chirho: String,
    pub queued_status_chirho: String,
    pub queued_heading_chirho: String,
    pub queued_body_template_chirho: String,
    pub queued_rollback_body_chirho: String,
    pub rejected_status_chirho: String,
    pub rejected_heading_chirho: String,
    pub invalid_slug_notice_chirho: String,
    pub nothing_saved_notice_chirho: String,
    pub unknown_request_notice_chirho: String,
    pub invalid_rollback_notice_chirho: String,
    pub status_link_label_chirho: String,
}

impl WorkspacePublicationCopyChirho {
    pub fn consent_chirho(&self, version_chirho: u64, address_chirho: &str) -> String {
        replace_version_chirho(&self.consent_template_chirho, version_chirho)
            .replace("{address_chirho}", address_chirho)
    }

    pub fn publish_button_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.publish_button_template_chirho, version_chirho)
    }

    pub fn rollback_button_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.rollback_button_template_chirho, version_chirho)
    }

    pub fn republish_button_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.republish_button_template_chirho, version_chirho)
    }

    pub fn saved_version_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.saved_version_template_chirho, version_chirho)
    }

    /// The header line when the saved version is exactly what is live (claude2 2026-09-24: it
    /// used to say "ready to publish" beside a panel saying that version was live).
    pub fn saved_version_live_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.saved_version_live_template_chirho, version_chirho)
    }

    /// The header line while an approval of that saved version is queued.
    pub fn saved_version_pending_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.saved_version_pending_template_chirho, version_chirho)
    }

    pub fn editor_pointer_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.editor_pointer_template_chirho, version_chirho)
    }

    pub fn live_is_saved_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.live_is_saved_template_chirho, version_chirho)
    }

    pub fn queued_body_chirho(&self, version_chirho: u64, address_chirho: &str) -> String {
        replace_version_chirho(&self.queued_body_template_chirho, version_chirho)
            .replace("{address_chirho}", address_chirho)
    }

    pub fn live_status_chirho(&self, version_chirho: u64) -> String {
        replace_version_chirho(&self.status_live_template_chirho, version_chirho)
    }

    pub fn kind_label_chirho(&self, kind_chirho: &str) -> &str {
        match kind_chirho {
            "publish-chirho" => &self.kind_publish_chirho,
            "republish-chirho" => &self.kind_republish_chirho,
            "rollback-chirho" => &self.kind_rollback_chirho,
            _ => &self.kind_unknown_chirho,
        }
    }

    pub fn refusal_sentence_chirho(&self, code_chirho: &str) -> &str {
        match code_chirho {
            "slug-reserved-chirho" => &self.refusal_slug_reserved_chirho,
            "slug-taken-chirho" => &self.refusal_slug_taken_chirho,
            "not-your-app-chirho" => &self.refusal_not_your_app_chirho,
            "unknown-prior-params-chirho" => &self.refusal_unknown_prior_params_chirho,
            "unknown-template-chirho" => &self.refusal_unknown_template_chirho,
            "intent-invalid-chirho" => &self.refusal_intent_invalid_chirho,
            "params-missing-chirho" => &self.refusal_params_missing_chirho,
            "template-not-deployable-chirho" => &self.refusal_template_not_deployable_chirho,
            _ => &self.refusal_unknown_chirho,
        }
    }

    /// The door's typed refusal codes (CAIRN #21819), read with or without the wire suffix.
    pub fn door_refusal_sentence_chirho(&self, code_chirho: &str) -> &str {
        match code_chirho.trim_end_matches("-chirho") {
            "base-moved" => &self.door_refusal_base_moved_chirho,
            "version-stale" => &self.door_refusal_version_stale_chirho,
            "no-saved-version" => &self.door_refusal_no_saved_version_chirho,
            "saved-version-not-a-document" => &self.door_refusal_not_a_document_chirho,
            "request-invalid" => &self.door_refusal_request_invalid_chirho,
            // The door refuses a name that is another account's before any intent is queued
            // (cairn's ownership-aware contract, CAIRN #24153): the applier's own words for it.
            "slug-taken" => &self.refusal_slug_taken_chirho,
            _ => &self.refusal_unknown_chirho,
        }
    }
}

/// What the editor says about a refused document. A control is named by the label the form
/// itself shows (the editor copy), so the summary and the field can never disagree; only what
/// the form has no label for lives here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceValidationCopyChirho {
    /// "Service {slot_chirho} title": a problem in a service pair names which pair.
    pub service_title_label_template_chirho: String,
    pub service_body_label_template_chirho: String,
    pub schema_label_chirho: String,
    pub page_label_chirho: String,
    pub required_message_chirho: String,
    /// No service at all: said on the group, where "the missing information" would name nothing.
    pub services_required_message_chirho: String,
    /// A pair with only a title or only a description: both ways out, beside the empty half.
    pub service_pair_incomplete_message_chirho: String,
    pub contact_required_message_chirho: String,
    pub invalid_phone_message_chirho: String,
    pub invalid_email_message_chirho: String,
    /// The sentence for an over-long value with no control to take a limit from.
    pub too_many_characters_message_chirho: String,
    /// "Keep this to {limit_chirho} characters or fewer.", beside the control whose limit it is.
    pub too_many_characters_template_chirho: String,
    pub too_many_bytes_message_chirho: String,
    pub unsupported_control_message_chirho: String,
    pub unsupported_schema_message_chirho: String,
    pub document_too_large_message_chirho: String,
}

impl WorkspaceValidationCopyChirho {
    pub fn service_title_label_chirho(&self, slot_chirho: usize) -> String {
        replace_slot_chirho(&self.service_title_label_template_chirho, slot_chirho)
    }

    pub fn service_body_label_chirho(&self, slot_chirho: usize) -> String {
        replace_slot_chirho(&self.service_body_label_template_chirho, slot_chirho)
    }

    pub fn too_many_characters_chirho(&self, limit_chirho: usize) -> String {
        self.too_many_characters_template_chirho
            .replace("{limit_chirho}", &limit_chirho.to_string())
    }
}

/// The Account page: her password, a recovery code, closing the account. Field names and
/// destinations stay out of the catalog; the engine's roads for these acts land contract-first
/// (CAIRN #21830), so the held page names the operator path instead of a control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAccountCopyChirho {
    pub page_title_chirho: String,
    /// Capability-neutral: true whichever roads are bound; each bound section names its action.
    pub page_description_chirho: String,
    pub status_held_chirho: String,
    pub status_ready_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    pub held_heading_chirho: String,
    pub held_body_chirho: String,
    pub password_number_chirho: String,
    pub password_heading_chirho: String,
    pub password_intro_chirho: String,
    pub current_password_label_chirho: String,
    pub new_password_label_chirho: String,
    pub new_password_help_chirho: String,
    pub password_boundary_chirho: String,
    pub password_submit_chirho: String,
    pub refusal_heading_chirho: String,
    pub refusal_credential_mismatch_chirho: String,
    pub refusal_credential_too_weak_chirho: String,
    pub refusal_request_invalid_chirho: String,
    pub refusal_change_failed_chirho: String,
    pub refusal_consent_missing_chirho: String,
    pub refusal_publication_pending_chirho: String,
    pub refusal_close_failed_chirho: String,
    pub refusal_recovery_failed_chirho: String,
    pub recovery_number_chirho: String,
    pub recovery_heading_chirho: String,
    pub recovery_intro_chirho: String,
    pub recovery_boundary_chirho: String,
    pub recovery_password_label_chirho: String,
    pub recovery_submit_chirho: String,
    pub close_number_chirho: String,
    pub close_heading_chirho: String,
    pub close_intro_chirho: String,
    pub close_password_label_chirho: String,
    pub close_consent_label_chirho: String,
    pub close_boundary_chirho: String,
    pub close_submit_chirho: String,
    pub back_label_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
}

impl WorkspaceAccountCopyChirho {
    /// The account roads' refusal tokens that return to the Account page (CAIRN #21925, #22494:
    /// a spent guess budget signs her out to the login page instead, so it has no sentence
    /// here; R2/R3 tokens per the recovery workflow doc's contract-first section), read with or
    /// without the wire suffix; an unknown token falls to the generic sentence.
    pub fn refusal_sentence_chirho(&self, token_chirho: &str) -> &str {
        match token_chirho.trim_end_matches("-chirho") {
            "credential-mismatch" => &self.refusal_credential_mismatch_chirho,
            "credential-too-weak" => &self.refusal_credential_too_weak_chirho,
            "request-invalid" => &self.refusal_request_invalid_chirho,
            "consent-missing" => &self.refusal_consent_missing_chirho,
            "publication-pending" => &self.refusal_publication_pending_chirho,
            "close-account-failed" => &self.refusal_close_failed_chirho,
            "recovery-code-failed" => &self.refusal_recovery_failed_chirho,
            _ => &self.refusal_change_failed_chirho,
        }
    }
}

/// The first-draft brief and its outcomes. Field names, the card's wire and the helper's cost
/// stay out of the catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDraftCopyChirho {
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub status_brief_chirho: String,
    pub status_has_page_chirho: String,
    pub status_rejected_chirho: String,
    pub status_not_drafted_chirho: String,
    pub status_unavailable_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    pub held_heading_chirho: String,
    pub held_body_chirho: String,
    pub panel_number_chirho: String,
    pub panel_heading_chirho: String,
    pub panel_intro_chirho: String,
    pub business_name_label_chirho: String,
    pub business_name_help_chirho: String,
    pub about_label_chirho: String,
    pub about_help_chirho: String,
    pub audience_label_chirho: String,
    pub audience_help_chirho: String,
    pub tone_label_chirho: String,
    pub tone_warm_chirho: String,
    pub tone_plain_chirho: String,
    pub tone_confident_chirho: String,
    pub services_label_chirho: String,
    pub services_help_chirho: String,
    pub service_label_template_chirho: String,
    /// The phone and email pair: one of the two is required, so the rule lives on the group.
    pub contact_method_label_chirho: String,
    pub contact_method_help_chirho: String,
    pub contact_phone_label_chirho: String,
    pub contact_phone_help_chirho: String,
    pub contact_email_label_chirho: String,
    pub contact_details_label_chirho: String,
    pub contact_details_help_chirho: String,
    pub boundary_chirho: String,
    pub submit_chirho: String,
    pub has_page_heading_chirho: String,
    pub has_page_body_chirho: String,
    pub editor_link_chirho: String,
    pub brief_link_chirho: String,
    pub rejected_heading_chirho: String,
    /// Under the heading of a brief that came back: nothing sent, nothing charged, words kept.
    pub rejected_body_chirho: String,
    pub not_sent_page_title_chirho: String,
    pub notice_unreadable_chirho: String,
    pub notice_name_missing_chirho: String,
    pub notice_about_missing_chirho: String,
    pub notice_contact_missing_chirho: String,
    pub notice_too_long_chirho: String,
    pub not_drafted_heading_chirho: String,
    pub not_drafted_body_chirho: String,
    pub review_notice_chirho: String,
    pub dropped_contact_notice_chirho: String,
    pub refusal_over_ceiling_chirho: String,
    pub refusal_provider_unavailable_chirho: String,
    pub refusal_timeout_chirho: String,
    pub refusal_unparseable_chirho: String,
    pub refusal_insufficient_credit_chirho: String,
    pub refusal_unknown_chirho: String,
    pub unavailable_heading_chirho: String,
    pub unavailable_body_chirho: String,
    pub help_heading_chirho: String,
    pub help_body_chirho: String,
}

impl WorkspaceDraftCopyChirho {
    pub fn service_label_chirho(&self, slot_chirho: usize) -> String {
        replace_version_chirho(&self.service_label_template_chirho, slot_chirho as u64)
    }

    /// The card's refusal tokens (CAIRN #22285), read with or without the wire suffix.
    pub fn refusal_sentence_chirho(&self, code_chirho: &str) -> &str {
        match code_chirho.trim_end_matches("-chirho") {
            "over-ceiling" => &self.refusal_over_ceiling_chirho,
            "provider-unavailable" => &self.refusal_provider_unavailable_chirho,
            "timeout" => &self.refusal_timeout_chirho,
            "response-unparseable" => &self.refusal_unparseable_chirho,
            "insufficient-credit" => &self.refusal_insufficient_credit_chirho,
            _ => &self.refusal_unknown_chirho,
        }
    }
}

/// The browser caller-key carrier's four refusals. Each is pre-rendered as a hidden `:target` notice on every page
/// with an app-route write form, and revealed when the engine answers a refused form with a 303 to
/// `#refusal-<token>`. The contract:
/// `spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md`.
/// Three are definitely not run; the fourth's outcome is unknown, so it has its own heading.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCarrierCopyChirho {
    pub not_run_heading_chirho: String,
    pub unknown_heading_chirho: String,
    pub caller_key_invalid_chirho: String,
    pub caller_key_conflict_chirho: String,
    pub write_unavailable_chirho: String,
    pub write_outcome_unknown_chirho: String,
}

impl WorkspaceCarrierCopyChirho {
    /// The sentence for one of the carrier's four final tokens, or None for any other token.
    pub fn sentence_chirho(&self, token_chirho: &str) -> Option<&str> {
        match token_chirho {
            "caller-key-invalid-chirho" => Some(&self.caller_key_invalid_chirho),
            "caller-key-conflict-chirho" => Some(&self.caller_key_conflict_chirho),
            "write-unavailable-chirho" => Some(&self.write_unavailable_chirho),
            "write-outcome-unknown-chirho" => Some(&self.write_outcome_unknown_chirho),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub chrome_chirho: WorkspaceChromeCopyChirho,
    pub editor_chirho: WorkspaceEditorCopyChirho,
    pub guided_chirho: WorkspaceGuidedCopyChirho,
    pub import_chirho: WorkspaceImportCopyChirho,
    pub export_chirho: WorkspaceExportCopyChirho,
    pub publication_chirho: WorkspacePublicationCopyChirho,
    pub account_chirho: WorkspaceAccountCopyChirho,
    pub connections_chirho: WorkspaceConnectionsCopyChirho,
    pub figures_chirho: WorkspaceFiguresCopyChirho,
    pub draft_chirho: WorkspaceDraftCopyChirho,
    pub validation_chirho: WorkspaceValidationCopyChirho,
    pub carrier_chirho: WorkspaceCarrierCopyChirho,
}

impl WorkspaceCopyChirho {
    pub fn english_chirho() -> Self {
        Self::for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/workspace-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the embedded typed workspace catalog must remain valid")
    }

    pub fn site_chrome_chirho(
        &self,
        page_title_chirho: &str,
        page_description_chirho: &str,
        stylesheet_name_chirho: &str,
        brand_href_chirho: &str,
        nav_links_chirho: Vec<ChromeNavLinkChirho>,
    ) -> SiteChromeChirho {
        SiteChromeChirho {
            page_title_chirho: page_title_chirho.to_string(),
            page_description_chirho: page_description_chirho.to_string(),
            stylesheet_name_chirho: stylesheet_name_chirho.to_string(),
            preview_marker_chirho: self.chrome_chirho.private_marker_chirho.clone(),
            brand_chirho: self.chrome_chirho.brand_chirho.clone(),
            brand_href_chirho: brand_href_chirho.to_string(),
            nav_label_chirho: self.chrome_chirho.nav_label_chirho.clone(),
            nav_links_chirho,
            footer_note_chirho: Some(self.chrome_chirho.footer_note_chirho.clone()),
        }
    }
}

fn replace_slot_chirho(template_chirho: &str, slot_chirho: usize) -> String {
    template_chirho.replace("{slot_chirho}", &slot_chirho.to_string())
}

fn replace_version_chirho(template_chirho: &str, version_chirho: u64) -> String {
    template_chirho.replace("{version_chirho}", &version_chirho.to_string())
}
