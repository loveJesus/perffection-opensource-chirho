// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! How the editor shows the validator's issues: a message beside the control it belongs to, and
//! a summary whose items link to that control.
//!
//! Nothing here decides validity. `PageDocumentV2Chirho::validate_chirho` reports every issue
//! with a field path; this module finds each issue's home in the editor form. An issue with no
//! home (the whole document, the format) stays in the summary, never dropped.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/page-document-lifecycle-chirho.md`.

use crate::field_view_chirho::{FieldGroupViewChirho, FieldViewChirho, FormNoticeChirho};
use crate::page_document_chirho::{
    PageDocumentValidationErrorChirho, PageValidationCodeChirho, PageValidationIssueChirho,
    BUSINESS_NAME_MAX_CHARACTERS_CHIRHO, CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
    CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO, CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
    CONTACT_PHONE_MAX_CHARACTERS_CHIRHO, HEADLINE_MAX_CHARACTERS_CHIRHO,
    PAGE_SERVICE_SLOT_COUNT_CHIRHO, SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
    SERVICE_TITLE_MAX_CHARACTERS_CHIRHO, SUMMARY_MAX_CHARACTERS_CHIRHO,
};
use perffection_ui_chirho::{WorkspaceCopyChirho, WorkspaceValidationCopyChirho};

/// The id of the message for "a phone number or an email"; the template renders it on the group.
pub const CONTACT_METHOD_MESSAGE_ID_CHIRHO: &str = "contact-method-error-chirho";
/// The id of the message for "at least one service".
pub const SERVICES_MESSAGE_ID_CHIRHO: &str = "services-error-chirho";

/// One editor control. Its DOM ids derive from one stem, so the template's literal ids and the
/// ids this module describes a control by cannot drift silently (the render tests resolve both).
#[expect(
    clippy::enum_variant_names,
    reason = "every identifier we name carries the Chirho suffix; private enums are the only ones clippy checks for it"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorControlChirho {
    BusinessNameChirho,
    HeadlineChirho,
    SummaryChirho,
    /// Zero-based slot.
    ServiceTitleChirho(usize),
    ServiceBodyChirho(usize),
    ContactHeadingChirho,
    ContactPhoneChirho,
    ContactEmailChirho,
    ContactDetailsChirho,
}

/// A rule over several controls: one message on the group, described into each member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorGroupChirho {
    ContactMethodChirho,
    ServicesChirho,
}

#[expect(
    clippy::enum_variant_names,
    reason = "every identifier we name carries the Chirho suffix; private enums are the only ones clippy checks for it"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorIssueHomeChirho {
    ControlChirho(EditorControlChirho),
    GroupChirho(EditorGroupChirho),
    /// The whole document or its format: no control to point at.
    PageChirho,
}

impl EditorControlChirho {
    fn stem_chirho(self) -> String {
        match self {
            Self::BusinessNameChirho => "business-name".to_string(),
            Self::HeadlineChirho => "headline".to_string(),
            Self::SummaryChirho => "summary".to_string(),
            Self::ServiceTitleChirho(slot_chirho) => format!("service-{}-title", slot_chirho + 1),
            Self::ServiceBodyChirho(slot_chirho) => format!("service-{}-body", slot_chirho + 1),
            Self::ContactHeadingChirho => "contact-heading".to_string(),
            Self::ContactPhoneChirho => "contact-phone".to_string(),
            Self::ContactEmailChirho => "contact-email".to_string(),
            Self::ContactDetailsChirho => "contact-details".to_string(),
        }
    }

    fn control_id_chirho(self) -> String {
        format!("{}-chirho", self.stem_chirho())
    }

    fn help_id_chirho(self) -> String {
        format!("{}-help-chirho", self.stem_chirho())
    }

    fn message_id_chirho(self) -> String {
        format!("{}-error-chirho", self.stem_chirho())
    }

    /// Whether the template renders help text under this control.
    fn has_help_chirho(self) -> bool {
        !matches!(
            self,
            Self::ServiceTitleChirho(_) | Self::ServiceBodyChirho(_) | Self::ContactHeadingChirho
        )
    }

    fn max_characters_chirho(self) -> usize {
        match self {
            Self::BusinessNameChirho => BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
            Self::HeadlineChirho => HEADLINE_MAX_CHARACTERS_CHIRHO,
            Self::SummaryChirho => SUMMARY_MAX_CHARACTERS_CHIRHO,
            Self::ServiceTitleChirho(_) => SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
            Self::ServiceBodyChirho(_) => SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
            Self::ContactHeadingChirho => CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
            Self::ContactPhoneChirho => CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
            Self::ContactEmailChirho => CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
            Self::ContactDetailsChirho => CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
        }
    }

    fn group_chirho(self) -> Option<EditorGroupChirho> {
        match self {
            Self::ContactPhoneChirho | Self::ContactEmailChirho => {
                Some(EditorGroupChirho::ContactMethodChirho)
            }
            // "At least one service" is asked of the first pair, the one the form requires.
            Self::ServiceTitleChirho(0) | Self::ServiceBodyChirho(0) => {
                Some(EditorGroupChirho::ServicesChirho)
            }
            _ => None,
        }
    }

    /// The label the form shows on the control, so the summary's link and the field agree. A
    /// service pair's controls are labelled "Title" and "Description" under their own heading,
    /// so the summary names the pair.
    fn label_chirho(self, copy_chirho: &WorkspaceCopyChirho) -> String {
        let editor_chirho = &copy_chirho.editor_chirho;
        match self {
            Self::BusinessNameChirho => editor_chirho.business_name_label_chirho.clone(),
            Self::HeadlineChirho => editor_chirho.headline_label_chirho.clone(),
            Self::SummaryChirho => editor_chirho.summary_label_chirho.clone(),
            Self::ServiceTitleChirho(slot_chirho) => copy_chirho
                .validation_chirho
                .service_title_label_chirho(slot_chirho + 1),
            Self::ServiceBodyChirho(slot_chirho) => copy_chirho
                .validation_chirho
                .service_body_label_chirho(slot_chirho + 1),
            Self::ContactHeadingChirho => editor_chirho.contact_heading_label_chirho.clone(),
            Self::ContactPhoneChirho => editor_chirho.contact_phone_label_chirho.clone(),
            Self::ContactEmailChirho => editor_chirho.contact_email_label_chirho.clone(),
            Self::ContactDetailsChirho => editor_chirho.contact_details_label_chirho.clone(),
        }
    }
}

impl EditorGroupChirho {
    fn message_id_chirho(self) -> &'static str {
        match self {
            Self::ContactMethodChirho => CONTACT_METHOD_MESSAGE_ID_CHIRHO,
            Self::ServicesChirho => SERVICES_MESSAGE_ID_CHIRHO,
        }
    }

    /// Where the summary sends her: the first control that can satisfy the rule.
    fn first_control_chirho(self) -> EditorControlChirho {
        match self {
            Self::ContactMethodChirho => EditorControlChirho::ContactPhoneChirho,
            Self::ServicesChirho => EditorControlChirho::ServiceTitleChirho(0),
        }
    }

    /// The group's own legend, as the form shows it.
    fn label_chirho(self, copy_chirho: &WorkspaceCopyChirho) -> String {
        match self {
            Self::ContactMethodChirho => copy_chirho
                .editor_chirho
                .contact_method_label_chirho
                .clone(),
            Self::ServicesChirho => copy_chirho.editor_chirho.services_heading_chirho.clone(),
        }
    }
}

fn issue_home_chirho(field_path_chirho: &str) -> EditorIssueHomeChirho {
    use EditorControlChirho as ControlChirho;
    use EditorIssueHomeChirho as HomeChirho;
    match field_path_chirho {
        "business_name_chirho" => HomeChirho::ControlChirho(ControlChirho::BusinessNameChirho),
        "headline_chirho" => HomeChirho::ControlChirho(ControlChirho::HeadlineChirho),
        "summary_chirho" => HomeChirho::ControlChirho(ControlChirho::SummaryChirho),
        "contact_heading_chirho" => HomeChirho::ControlChirho(ControlChirho::ContactHeadingChirho),
        "contact_phone_chirho" => HomeChirho::ControlChirho(ControlChirho::ContactPhoneChirho),
        "contact_email_chirho" => HomeChirho::ControlChirho(ControlChirho::ContactEmailChirho),
        "contact_details_chirho" => HomeChirho::ControlChirho(ControlChirho::ContactDetailsChirho),
        "contact_chirho" => HomeChirho::GroupChirho(EditorGroupChirho::ContactMethodChirho),
        "services_chirho" => HomeChirho::GroupChirho(EditorGroupChirho::ServicesChirho),
        other_chirho => service_control_chirho(other_chirho)
            .map_or(HomeChirho::PageChirho, HomeChirho::ControlChirho),
    }
}

/// `services_chirho[1].body_chirho` → the second pair's description; a slot the form does not
/// render has no control to point at.
fn service_control_chirho(field_path_chirho: &str) -> Option<EditorControlChirho> {
    let (slot_text_chirho, member_chirho) = field_path_chirho
        .strip_prefix("services_chirho[")?
        .split_once("].")?;
    let slot_chirho = slot_text_chirho.parse::<usize>().ok()?;
    if slot_chirho >= PAGE_SERVICE_SLOT_COUNT_CHIRHO {
        return None;
    }
    match member_chirho {
        "title_chirho" => Some(EditorControlChirho::ServiceTitleChirho(slot_chirho)),
        "body_chirho" => Some(EditorControlChirho::ServiceBodyChirho(slot_chirho)),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorServiceFieldsChirho {
    pub(crate) title_chirho: FieldViewChirho,
    pub(crate) body_chirho: FieldViewChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorFieldsChirho {
    pub(crate) business_name_chirho: FieldViewChirho,
    pub(crate) headline_chirho: FieldViewChirho,
    pub(crate) summary_chirho: FieldViewChirho,
    /// One entry per service slot the form renders, in slot order.
    pub(crate) services_chirho: Vec<EditorServiceFieldsChirho>,
    pub(crate) services_group_chirho: FieldGroupViewChirho,
    pub(crate) contact_heading_chirho: FieldViewChirho,
    pub(crate) contact_method_group_chirho: FieldGroupViewChirho,
    pub(crate) contact_phone_chirho: FieldViewChirho,
    pub(crate) contact_email_chirho: FieldViewChirho,
    pub(crate) contact_details_chirho: FieldViewChirho,
}

impl EditorFieldsChirho {
    fn clean_chirho() -> Self {
        use EditorControlChirho as ControlChirho;
        let field_chirho = |control_chirho: EditorControlChirho| {
            FieldViewChirho::clean_chirho(control_chirho.message_id_chirho())
        };
        Self {
            business_name_chirho: field_chirho(ControlChirho::BusinessNameChirho),
            headline_chirho: field_chirho(ControlChirho::HeadlineChirho),
            summary_chirho: field_chirho(ControlChirho::SummaryChirho),
            services_chirho: (0..PAGE_SERVICE_SLOT_COUNT_CHIRHO)
                .map(|slot_chirho| EditorServiceFieldsChirho {
                    title_chirho: field_chirho(ControlChirho::ServiceTitleChirho(slot_chirho)),
                    body_chirho: field_chirho(ControlChirho::ServiceBodyChirho(slot_chirho)),
                })
                .collect(),
            services_group_chirho: FieldGroupViewChirho::clean_chirho(
                EditorGroupChirho::ServicesChirho.message_id_chirho(),
            ),
            contact_heading_chirho: field_chirho(ControlChirho::ContactHeadingChirho),
            contact_method_group_chirho: FieldGroupViewChirho::clean_chirho(
                EditorGroupChirho::ContactMethodChirho.message_id_chirho(),
            ),
            contact_phone_chirho: field_chirho(ControlChirho::ContactPhoneChirho),
            contact_email_chirho: field_chirho(ControlChirho::ContactEmailChirho),
            contact_details_chirho: field_chirho(ControlChirho::ContactDetailsChirho),
        }
    }

    /// The form with nothing to report: each control described by its help text alone.
    pub(crate) fn without_issues_chirho() -> Self {
        let mut fields_chirho = Self::clean_chirho();
        fields_chirho.settle_chirho();
        fields_chirho
    }

    fn field_mut_chirho(&mut self, control_chirho: EditorControlChirho) -> &mut FieldViewChirho {
        use EditorControlChirho as ControlChirho;
        match control_chirho {
            ControlChirho::BusinessNameChirho => &mut self.business_name_chirho,
            ControlChirho::HeadlineChirho => &mut self.headline_chirho,
            ControlChirho::SummaryChirho => &mut self.summary_chirho,
            ControlChirho::ServiceTitleChirho(slot_chirho) => {
                &mut self.services_chirho[slot_chirho].title_chirho
            }
            ControlChirho::ServiceBodyChirho(slot_chirho) => {
                &mut self.services_chirho[slot_chirho].body_chirho
            }
            ControlChirho::ContactHeadingChirho => &mut self.contact_heading_chirho,
            ControlChirho::ContactPhoneChirho => &mut self.contact_phone_chirho,
            ControlChirho::ContactEmailChirho => &mut self.contact_email_chirho,
            ControlChirho::ContactDetailsChirho => &mut self.contact_details_chirho,
        }
    }

    fn group_chirho(&self, group_chirho: EditorGroupChirho) -> &FieldGroupViewChirho {
        match group_chirho {
            EditorGroupChirho::ContactMethodChirho => &self.contact_method_group_chirho,
            EditorGroupChirho::ServicesChirho => &self.services_group_chirho,
        }
    }

    fn group_mut_chirho(&mut self, group_chirho: EditorGroupChirho) -> &mut FieldGroupViewChirho {
        match group_chirho {
            EditorGroupChirho::ContactMethodChirho => &mut self.contact_method_group_chirho,
            EditorGroupChirho::ServicesChirho => &mut self.services_group_chirho,
        }
    }

    fn controls_chirho() -> impl Iterator<Item = EditorControlChirho> {
        use EditorControlChirho as ControlChirho;
        [
            ControlChirho::BusinessNameChirho,
            ControlChirho::HeadlineChirho,
            ControlChirho::SummaryChirho,
            ControlChirho::ContactHeadingChirho,
            ControlChirho::ContactPhoneChirho,
            ControlChirho::ContactEmailChirho,
            ControlChirho::ContactDetailsChirho,
        ]
        .into_iter()
        .chain((0..PAGE_SERVICE_SLOT_COUNT_CHIRHO).flat_map(|slot_chirho| {
            [
                ControlChirho::ServiceTitleChirho(slot_chirho),
                ControlChirho::ServiceBodyChirho(slot_chirho),
            ]
        }))
    }

    /// After every message is placed: what describes each control, and whether it is invalid.
    fn settle_chirho(&mut self) {
        for control_chirho in Self::controls_chirho() {
            let group_message_id_chirho = control_chirho
                .group_chirho()
                .and_then(|group_chirho| self.group_chirho(group_chirho).live_message_id_chirho());
            let help_id_chirho = control_chirho
                .has_help_chirho()
                .then(|| control_chirho.help_id_chirho());
            self.field_mut_chirho(control_chirho)
                .settle_chirho(help_id_chirho, group_message_id_chirho);
        }
    }
}

/// Every issue twice over: once in the summary, once beside its control or group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorValidationViewChirho {
    pub(crate) notices_chirho: Vec<FormNoticeChirho>,
    pub(crate) fields_chirho: EditorFieldsChirho,
}

impl EditorValidationViewChirho {
    pub(crate) fn from_error_chirho(
        validation_error_chirho: Option<&PageDocumentValidationErrorChirho>,
        copy_chirho: &WorkspaceCopyChirho,
    ) -> Self {
        let mut fields_chirho = EditorFieldsChirho::clean_chirho();
        let issues_chirho: &[PageValidationIssueChirho] = validation_error_chirho
            .map_or(&[], |error_chirho| error_chirho.issues_chirho.as_slice());
        let mut notices_chirho = Vec::with_capacity(issues_chirho.len());
        for issue_chirho in issues_chirho {
            let home_chirho = issue_home_chirho(&issue_chirho.field_path_chirho);
            let message_chirho = message_chirho(
                issue_chirho.code_chirho,
                home_chirho,
                &copy_chirho.validation_chirho,
            );
            let (field_label_chirho, control_id_chirho) = match home_chirho {
                EditorIssueHomeChirho::ControlChirho(control_chirho) => {
                    fields_chirho
                        .field_mut_chirho(control_chirho)
                        .messages_chirho
                        .push(message_chirho.clone());
                    (
                        control_chirho.label_chirho(copy_chirho),
                        Some(control_chirho.control_id_chirho()),
                    )
                }
                EditorIssueHomeChirho::GroupChirho(group_chirho) => {
                    fields_chirho
                        .group_mut_chirho(group_chirho)
                        .messages_chirho
                        .push(message_chirho.clone());
                    (
                        group_chirho.label_chirho(copy_chirho),
                        Some(group_chirho.first_control_chirho().control_id_chirho()),
                    )
                }
                EditorIssueHomeChirho::PageChirho => (
                    page_label_chirho(
                        &issue_chirho.field_path_chirho,
                        &copy_chirho.validation_chirho,
                    ),
                    None,
                ),
            };
            notices_chirho.push(FormNoticeChirho {
                field_label_chirho,
                message_chirho,
                control_id_chirho,
            });
        }
        fields_chirho.settle_chirho();
        Self {
            notices_chirho,
            fields_chirho,
        }
    }
}

fn page_label_chirho(
    field_path_chirho: &str,
    copy_chirho: &WorkspaceValidationCopyChirho,
) -> String {
    if field_path_chirho == "schema_version_chirho" {
        copy_chirho.schema_label_chirho.clone()
    } else {
        copy_chirho.page_label_chirho.clone()
    }
}

fn message_chirho(
    code_chirho: PageValidationCodeChirho,
    home_chirho: EditorIssueHomeChirho,
    copy_chirho: &WorkspaceValidationCopyChirho,
) -> String {
    match code_chirho {
        // "Required" means three different things to her, depending on where it lands.
        PageValidationCodeChirho::RequiredChirho => match home_chirho {
            EditorIssueHomeChirho::GroupChirho(EditorGroupChirho::ServicesChirho) => {
                copy_chirho.services_required_message_chirho.clone()
            }
            EditorIssueHomeChirho::ControlChirho(
                EditorControlChirho::ServiceTitleChirho(_)
                | EditorControlChirho::ServiceBodyChirho(_),
            ) => copy_chirho.service_pair_incomplete_message_chirho.clone(),
            _ => copy_chirho.required_message_chirho.clone(),
        },
        PageValidationCodeChirho::ContactMethodRequiredChirho => {
            copy_chirho.contact_required_message_chirho.clone()
        }
        PageValidationCodeChirho::InvalidPhoneChirho => {
            copy_chirho.invalid_phone_message_chirho.clone()
        }
        PageValidationCodeChirho::InvalidEmailChirho => {
            copy_chirho.invalid_email_message_chirho.clone()
        }
        // The limit is the control's; an over-long value with no control keeps the plain sentence.
        PageValidationCodeChirho::TooManyCharactersChirho => match home_chirho {
            EditorIssueHomeChirho::ControlChirho(control_chirho) => {
                copy_chirho.too_many_characters_chirho(control_chirho.max_characters_chirho())
            }
            _ => copy_chirho.too_many_characters_message_chirho.clone(),
        },
        PageValidationCodeChirho::TooManyBytesChirho => {
            copy_chirho.too_many_bytes_message_chirho.clone()
        }
        PageValidationCodeChirho::UnsupportedControlCharacterChirho => {
            copy_chirho.unsupported_control_message_chirho.clone()
        }
        PageValidationCodeChirho::UnsupportedSchemaChirho => {
            copy_chirho.unsupported_schema_message_chirho.clone()
        }
        PageValidationCodeChirho::DocumentTooLargeChirho => {
            copy_chirho.document_too_large_message_chirho.clone()
        }
    }
}
