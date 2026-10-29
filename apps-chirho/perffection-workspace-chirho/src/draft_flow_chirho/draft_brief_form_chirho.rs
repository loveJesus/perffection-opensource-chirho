// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The first-draft brief form as the template renders it: blank on the way in, and around her own
//! words when the page cannot use the brief yet. `read_draft_brief_chirho` decides what is wrong;
//! this module decides where each problem is shown, the way `editor_validation_chirho` does for
//! the editor. Her words go back only into the form she is looking at.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/first-draft-chirho.md`.

use crate::field_view_chirho::{FieldGroupViewChirho, FieldViewChirho, FormNoticeChirho};
use crate::first_draft_chirho::{
    DraftBriefFieldChirho, DraftBriefIssueChirho, DraftBriefWordsChirho,
    BRIEF_SERVICE_COUNT_CHIRHO, BRIEF_TONES_CHIRHO,
};
use perffection_ui_chirho::WorkspaceCopyChirho;

/// The id of the message for "a phone number or an email" on the brief.
pub const BRIEF_CONTACT_METHOD_MESSAGE_ID_CHIRHO: &str = "draft-contact-method-error-chirho";

impl DraftBriefFieldChirho {
    fn stem_chirho(self) -> String {
        match self {
            Self::BusinessNameChirho => "draft-business-name".to_string(),
            Self::AboutChirho => "draft-about".to_string(),
            Self::AudienceChirho => "draft-audience".to_string(),
            Self::ServiceChirho(slot_chirho) => format!("draft-service-{slot_chirho}"),
            Self::ContactPhoneChirho => "draft-contact-phone".to_string(),
            Self::ContactEmailChirho => "draft-contact-email".to_string(),
            Self::ContactDetailsChirho => "draft-contact-details".to_string(),
        }
    }

    fn control_id_chirho(self) -> String {
        format!("{}-chirho", self.stem_chirho())
    }

    /// Whether the template renders help text under this control.
    fn has_help_chirho(self) -> bool {
        !matches!(self, Self::ServiceChirho(_) | Self::ContactEmailChirho)
    }

    fn in_contact_pair_chirho(self) -> bool {
        matches!(self, Self::ContactPhoneChirho | Self::ContactEmailChirho)
    }

    /// The label the brief form shows on the control.
    fn label_chirho(self, copy_chirho: &WorkspaceCopyChirho) -> String {
        let draft_chirho = &copy_chirho.draft_chirho;
        match self {
            Self::BusinessNameChirho => draft_chirho.business_name_label_chirho.clone(),
            Self::AboutChirho => draft_chirho.about_label_chirho.clone(),
            Self::AudienceChirho => draft_chirho.audience_label_chirho.clone(),
            Self::ServiceChirho(slot_chirho) => draft_chirho.service_label_chirho(slot_chirho),
            Self::ContactPhoneChirho => draft_chirho.contact_phone_label_chirho.clone(),
            Self::ContactEmailChirho => draft_chirho.contact_email_label_chirho.clone(),
            Self::ContactDetailsChirho => draft_chirho.contact_details_label_chirho.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftBriefServiceFieldChirho {
    /// One-based, as the form names the slot.
    pub(crate) index_chirho: usize,
    pub(crate) label_chirho: String,
    pub(crate) value_chirho: String,
    pub(crate) field_chirho: FieldViewChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftBriefFormViewChirho {
    pub(crate) words_chirho: DraftBriefWordsChirho,
    /// Every problem once: the summary at the top of a brief that was not sent.
    pub(crate) notices_chirho: Vec<FormNoticeChirho>,
    pub(crate) business_name_chirho: FieldViewChirho,
    pub(crate) about_chirho: FieldViewChirho,
    pub(crate) audience_chirho: FieldViewChirho,
    pub(crate) services_chirho: Vec<DraftBriefServiceFieldChirho>,
    pub(crate) contact_method_group_chirho: FieldGroupViewChirho,
    pub(crate) contact_phone_chirho: FieldViewChirho,
    pub(crate) contact_email_chirho: FieldViewChirho,
    pub(crate) contact_details_chirho: FieldViewChirho,
}

impl DraftBriefFormViewChirho {
    /// The form on the way in: nothing typed, nothing to report.
    pub(crate) fn blank_chirho(copy_chirho: &WorkspaceCopyChirho) -> Self {
        let words_chirho = DraftBriefWordsChirho {
            tone_chirho: BRIEF_TONES_CHIRHO[0],
            services_chirho: vec![String::new(); BRIEF_SERVICE_COUNT_CHIRHO],
            ..DraftBriefWordsChirho::default()
        };
        Self::refused_chirho(words_chirho, &[], copy_chirho)
    }

    /// The form around her own words, each problem in the summary and beside its control.
    pub(crate) fn refused_chirho(
        words_chirho: DraftBriefWordsChirho,
        issues_chirho: &[DraftBriefIssueChirho],
        copy_chirho: &WorkspaceCopyChirho,
    ) -> Self {
        use DraftBriefFieldChirho as FieldChirho;
        let clean_chirho = |field_chirho: FieldChirho| {
            FieldViewChirho::clean_chirho(format!("{}-error-chirho", field_chirho.stem_chirho()))
        };
        let mut form_chirho = Self {
            notices_chirho: Vec::with_capacity(issues_chirho.len()),
            business_name_chirho: clean_chirho(FieldChirho::BusinessNameChirho),
            about_chirho: clean_chirho(FieldChirho::AboutChirho),
            audience_chirho: clean_chirho(FieldChirho::AudienceChirho),
            services_chirho: (1..=BRIEF_SERVICE_COUNT_CHIRHO)
                .map(|slot_chirho| DraftBriefServiceFieldChirho {
                    index_chirho: slot_chirho,
                    label_chirho: copy_chirho.draft_chirho.service_label_chirho(slot_chirho),
                    value_chirho: words_chirho
                        .services_chirho
                        .get(slot_chirho - 1)
                        .cloned()
                        .unwrap_or_default(),
                    field_chirho: clean_chirho(FieldChirho::ServiceChirho(slot_chirho)),
                })
                .collect(),
            contact_method_group_chirho: FieldGroupViewChirho::clean_chirho(
                BRIEF_CONTACT_METHOD_MESSAGE_ID_CHIRHO,
            ),
            contact_phone_chirho: clean_chirho(FieldChirho::ContactPhoneChirho),
            contact_email_chirho: clean_chirho(FieldChirho::ContactEmailChirho),
            contact_details_chirho: clean_chirho(FieldChirho::ContactDetailsChirho),
            words_chirho,
        };
        let draft_copy_chirho = &copy_chirho.draft_chirho;
        for issue_chirho in issues_chirho {
            let (field_chirho, message_chirho) = match *issue_chirho {
                DraftBriefIssueChirho::NameMissingChirho => (
                    Some(FieldChirho::BusinessNameChirho),
                    draft_copy_chirho.notice_name_missing_chirho.clone(),
                ),
                DraftBriefIssueChirho::AboutMissingChirho => (
                    Some(FieldChirho::AboutChirho),
                    draft_copy_chirho.notice_about_missing_chirho.clone(),
                ),
                DraftBriefIssueChirho::ContactMissingChirho => (
                    None,
                    draft_copy_chirho.notice_contact_missing_chirho.clone(),
                ),
                DraftBriefIssueChirho::TooLongChirho {
                    field_chirho,
                    max_characters_chirho,
                } => (
                    Some(field_chirho),
                    copy_chirho
                        .validation_chirho
                        .too_many_characters_chirho(max_characters_chirho),
                ),
            };
            let (field_label_chirho, control_chirho) = match field_chirho {
                Some(field_chirho) => {
                    form_chirho
                        .field_mut_chirho(field_chirho)
                        .messages_chirho
                        .push(message_chirho.clone());
                    (field_chirho.label_chirho(copy_chirho), field_chirho)
                }
                // The pair's rule is said on the group; the summary sends her to the phone.
                None => {
                    form_chirho
                        .contact_method_group_chirho
                        .messages_chirho
                        .push(message_chirho.clone());
                    (
                        draft_copy_chirho.contact_method_label_chirho.clone(),
                        FieldChirho::ContactPhoneChirho,
                    )
                }
            };
            form_chirho.notices_chirho.push(FormNoticeChirho {
                field_label_chirho,
                message_chirho,
                control_id_chirho: Some(control_chirho.control_id_chirho()),
            });
        }
        form_chirho.settle_chirho();
        form_chirho
    }

    fn field_mut_chirho(&mut self, field_chirho: DraftBriefFieldChirho) -> &mut FieldViewChirho {
        use DraftBriefFieldChirho as FieldChirho;
        match field_chirho {
            FieldChirho::BusinessNameChirho => &mut self.business_name_chirho,
            FieldChirho::AboutChirho => &mut self.about_chirho,
            FieldChirho::AudienceChirho => &mut self.audience_chirho,
            FieldChirho::ServiceChirho(slot_chirho) => {
                &mut self.services_chirho[slot_chirho - 1].field_chirho
            }
            FieldChirho::ContactPhoneChirho => &mut self.contact_phone_chirho,
            FieldChirho::ContactEmailChirho => &mut self.contact_email_chirho,
            FieldChirho::ContactDetailsChirho => &mut self.contact_details_chirho,
        }
    }

    fn settle_chirho(&mut self) {
        use DraftBriefFieldChirho as FieldChirho;
        let group_message_id_chirho = self.contact_method_group_chirho.live_message_id_chirho();
        let fields_chirho = [
            FieldChirho::BusinessNameChirho,
            FieldChirho::AboutChirho,
            FieldChirho::AudienceChirho,
            FieldChirho::ContactPhoneChirho,
            FieldChirho::ContactEmailChirho,
            FieldChirho::ContactDetailsChirho,
        ]
        .into_iter()
        .chain((1..=BRIEF_SERVICE_COUNT_CHIRHO).map(FieldChirho::ServiceChirho));
        for field_chirho in fields_chirho {
            let help_id_chirho = field_chirho
                .has_help_chirho()
                .then(|| format!("{}-help-chirho", field_chirho.stem_chirho()));
            let group_chirho =
                group_message_id_chirho.filter(|_| field_chirho.in_contact_pair_chirho());
            self.field_mut_chirho(field_chirho)
                .settle_chirho(help_id_chirho, group_chirho);
        }
    }
}
