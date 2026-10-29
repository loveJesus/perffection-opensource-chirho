// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The first-draft brief page and the pages around it (already has a page, not sent, not
//! drafted, unavailable). The reviewable draft itself is the existing editor in its imported
//! state. Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/first-draft-chirho.md`.

use crate::draft_brief_form_chirho::DraftBriefFormViewChirho;
use crate::first_draft_chirho::{
    DraftBriefErrorChirho, DraftBriefIssueChirho, DraftBriefWordsChirho,
    BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO, BRIEF_AUDIENCE_MAX_CHARACTERS_CHIRHO,
    BRIEF_SERVICE_MAX_CHARACTERS_CHIRHO, BRIEF_TONES_CHIRHO,
};
use crate::page_document_chirho::{
    BUSINESS_NAME_MAX_CHARACTERS_CHIRHO, CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
    CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO, CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
};
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
    WORKSPACE_HOME_HREF_CHIRHO,
};
use perffection_credits_chirho::credit_view_port_chirho::{
    CreditViewChirho, CreditViewUnavailableChirho,
};
use perffection_ui_chirho::{
    CreditsCopyChirho, SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho,
};

/// The brief page (GET) and the draft door (POST); both held until the served `llm` card runs.
pub const WORKSPACE_DRAFT_HREF_CHIRHO: &str = "/app-chirho/perffection-workspace/draft-chirho";
pub const WORKSPACE_DRAFT_ACTION_CHIRHO: &str =
    "/app-chirho/perffection-workspace/draft-page-chirho";

/// The draft door every brief form posts to: `None` until the served `llm` card runs, so the
/// brief page names the operator-free path instead. The brief page and the page that hands a
/// refused brief back both read it here, so a landing flips one switch.
pub fn deployed_brief_action_chirho() -> Option<&'static str> {
    None
}

/// What a served brief page reads for her credits today. No payer reaches a page node until Cairn's ledger port
/// exists (W3), so a served page answers exactly as the unconnected port would: unavailable, never zero.
pub const SERVED_CREDIT_VIEW_CHIRHO: CreditViewChirho =
    CreditViewChirho::UnavailableChirho(CreditViewUnavailableChirho::NotConnectedChirho);

/// What she can draft with, beside a live brief form, in the Credits page's own words (one read of
/// `perffection_credits_chirho::credit_view_port_chirho`). An unavailable read is a sentence, never a number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftCreditsChirho {
    KnownChirho {
        label_chirho: String,
        balance_chirho: String,
    },
    UnavailableChirho {
        sentence_chirho: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftToneViewChirho {
    pub(crate) value_chirho: &'static str,
    pub(crate) label_chirho: String,
    /// The tone she chose, kept when the brief comes back to her.
    pub(crate) selected_chirho: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftPageKindChirho {
    /// The brief form; renders only with the draft door bound.
    BriefChirho,
    /// A saved version exists: point at the editor rather than draft over it.
    HasPageChirho { version_chirho: u64 },
    /// The brief could not be read at all, or the draft door is not bound: a customer sentence
    /// and nothing of hers. A readable brief the page cannot use yet is `BriefChirho` again,
    /// around her own words (`refused_brief_chirho`).
    RejectedChirho { notice_chirho: String },
    /// The helper answered, but not with a page.
    NotDraftedChirho,
    /// The card refused (a fixed token → a catalog sentence); nothing was charged or saved.
    RefusedChirho { notice_chirho: String },
    /// Her state could not be verified.
    UnavailableChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftPageV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: WorkspaceCopyChirho,
    pub(crate) status_chirho: String,
    pub(crate) kind_chirho: DraftPageKindChirho,
    /// The draft door; `None` until the served `llm` card is on the pin.
    pub(crate) brief_action_chirho: Option<String>,
    pub(crate) editor_href_chirho: String,
    pub(crate) brief_href_chirho: String,
    pub(crate) tones_chirho: Vec<DraftToneViewChirho>,
    /// The brief form: blank on the way in, around her words when it was not sent.
    pub(crate) form_chirho: DraftBriefFormViewChirho,
    /// What she can draft with; only beside a live brief form (`with_credits_chirho`), `None` everywhere else.
    pub(crate) credits_chirho: Option<DraftCreditsChirho>,
    pub(crate) business_name_max_chirho: usize,
    pub(crate) about_max_chirho: usize,
    pub(crate) audience_max_chirho: usize,
    pub(crate) service_max_chirho: usize,
    pub(crate) contact_phone_max_chirho: usize,
    pub(crate) contact_email_max_chirho: usize,
    pub(crate) contact_details_max_chirho: usize,
}

impl DraftPageV1Chirho {
    pub fn brief_chirho(brief_action_chirho: Option<&str>) -> Self {
        Self::for_locale_chirho(
            DraftPageKindChirho::BriefChirho,
            brief_action_chirho,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn has_page_chirho(version_chirho: u64) -> Self {
        Self::for_locale_chirho(
            DraftPageKindChirho::HasPageChirho { version_chirho },
            None,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn rejected_chirho(error_chirho: DraftBriefErrorChirho) -> Self {
        let copy_chirho = WorkspaceCopyChirho::english_chirho();
        let notice_chirho = match error_chirho {
            DraftBriefErrorChirho::UnreadableChirho => {
                copy_chirho.draft_chirho.notice_unreadable_chirho.clone()
            }
            DraftBriefErrorChirho::NameMissingChirho => {
                copy_chirho.draft_chirho.notice_name_missing_chirho.clone()
            }
            DraftBriefErrorChirho::AboutMissingChirho => {
                copy_chirho.draft_chirho.notice_about_missing_chirho.clone()
            }
            DraftBriefErrorChirho::ContactMissingChirho => copy_chirho
                .draft_chirho
                .notice_contact_missing_chirho
                .clone(),
            DraftBriefErrorChirho::TooLongChirho => {
                copy_chirho.draft_chirho.notice_too_long_chirho.clone()
            }
        };
        Self::for_locale_chirho(
            DraftPageKindChirho::RejectedChirho { notice_chirho },
            None,
            UiLocaleChirho::EnglishChirho,
        )
    }

    /// A readable brief the page cannot use yet: the form again, around her own words, with
    /// every problem in the summary and beside its control. Without a bound draft door there is
    /// no form to hand back, so the first problem is said as a sentence instead.
    pub fn refused_brief_chirho(
        words_chirho: DraftBriefWordsChirho,
        issues_chirho: &[DraftBriefIssueChirho],
        brief_action_chirho: Option<&str>,
    ) -> Self {
        let Some(brief_action_chirho) = brief_action_chirho else {
            return Self::rejected_chirho(match issues_chirho.first() {
                Some(DraftBriefIssueChirho::NameMissingChirho) => {
                    DraftBriefErrorChirho::NameMissingChirho
                }
                Some(DraftBriefIssueChirho::AboutMissingChirho) => {
                    DraftBriefErrorChirho::AboutMissingChirho
                }
                Some(DraftBriefIssueChirho::ContactMissingChirho) => {
                    DraftBriefErrorChirho::ContactMissingChirho
                }
                Some(DraftBriefIssueChirho::TooLongChirho { .. }) | None => {
                    DraftBriefErrorChirho::TooLongChirho
                }
            });
        };
        let mut model_chirho = Self::for_locale_chirho(
            DraftPageKindChirho::BriefChirho,
            Some(brief_action_chirho),
            UiLocaleChirho::EnglishChirho,
        );
        model_chirho.form_chirho = DraftBriefFormViewChirho::refused_chirho(
            words_chirho,
            issues_chirho,
            &model_chirho.copy_chirho,
        );
        for tone_chirho in &mut model_chirho.tones_chirho {
            tone_chirho.selected_chirho =
                tone_chirho.value_chirho == model_chirho.form_chirho.words_chirho.tone_chirho;
        }
        model_chirho.status_chirho = model_chirho
            .copy_chirho
            .draft_chirho
            .status_rejected_chirho
            .clone();
        model_chirho.chrome_chirho.page_title_chirho = model_chirho
            .copy_chirho
            .draft_chirho
            .not_sent_page_title_chirho
            .clone();
        model_chirho
    }

    pub fn not_drafted_chirho() -> Self {
        Self::for_locale_chirho(
            DraftPageKindChirho::NotDraftedChirho,
            None,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn refused_chirho(code_chirho: &str) -> Self {
        let copy_chirho = WorkspaceCopyChirho::english_chirho();
        let notice_chirho = copy_chirho
            .draft_chirho
            .refusal_sentence_chirho(code_chirho)
            .to_string();
        Self::for_locale_chirho(
            DraftPageKindChirho::RefusedChirho { notice_chirho },
            None,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn unavailable_chirho() -> Self {
        Self::for_locale_chirho(
            DraftPageKindChirho::UnavailableChirho,
            None,
            UiLocaleChirho::EnglishChirho,
        )
    }

    pub fn for_locale_chirho(
        kind_chirho: DraftPageKindChirho,
        brief_action_chirho: Option<&str>,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = WorkspaceCopyChirho::for_locale_chirho(locale_chirho);
        let draft_copy_chirho = &copy_chirho.draft_chirho;
        let status_chirho = match &kind_chirho {
            DraftPageKindChirho::BriefChirho => draft_copy_chirho.status_brief_chirho.clone(),
            DraftPageKindChirho::HasPageChirho { .. } => {
                draft_copy_chirho.status_has_page_chirho.clone()
            }
            DraftPageKindChirho::RejectedChirho { .. } => {
                draft_copy_chirho.status_rejected_chirho.clone()
            }
            DraftPageKindChirho::NotDraftedChirho | DraftPageKindChirho::RefusedChirho { .. } => {
                draft_copy_chirho.status_not_drafted_chirho.clone()
            }
            DraftPageKindChirho::UnavailableChirho => {
                draft_copy_chirho.status_unavailable_chirho.clone()
            }
        };
        let tones_chirho = vec![
            DraftToneViewChirho {
                value_chirho: BRIEF_TONES_CHIRHO[0],
                selected_chirho: true,
                label_chirho: draft_copy_chirho.tone_warm_chirho.clone(),
            },
            DraftToneViewChirho {
                value_chirho: BRIEF_TONES_CHIRHO[1],
                selected_chirho: false,
                label_chirho: draft_copy_chirho.tone_plain_chirho.clone(),
            },
            DraftToneViewChirho {
                value_chirho: BRIEF_TONES_CHIRHO[2],
                selected_chirho: false,
                label_chirho: draft_copy_chirho.tone_confident_chirho.clone(),
            },
        ];
        let form_chirho = DraftBriefFormViewChirho::blank_chirho(&copy_chirho);
        let chrome_chirho = workspace_chrome_chirho(
            &copy_chirho,
            &draft_copy_chirho.page_title_chirho,
            &draft_copy_chirho.page_description_chirho,
            WorkspaceNavPageChirho::DraftChirho,
            WorkspaceNavRoadsChirho {
                draft_chirho: true,
                ..WorkspaceNavRoadsChirho::deployed_chirho()
            },
        );
        Self {
            chrome_chirho,
            copy_chirho,
            status_chirho,
            kind_chirho,
            brief_action_chirho: brief_action_chirho.map(str::to_string),
            editor_href_chirho: WORKSPACE_HOME_HREF_CHIRHO.to_string(),
            brief_href_chirho: WORKSPACE_DRAFT_HREF_CHIRHO.to_string(),
            tones_chirho,
            form_chirho,
            credits_chirho: None,
            business_name_max_chirho: BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
            about_max_chirho: BRIEF_ABOUT_MAX_CHARACTERS_CHIRHO,
            audience_max_chirho: BRIEF_AUDIENCE_MAX_CHARACTERS_CHIRHO,
            service_max_chirho: BRIEF_SERVICE_MAX_CHARACTERS_CHIRHO,
            contact_phone_max_chirho: CONTACT_PHONE_MAX_CHARACTERS_CHIRHO,
            contact_email_max_chirho: CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO,
            contact_details_max_chirho: CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
        }
    }

    pub fn is_brief_chirho(&self) -> bool {
        matches!(self.kind_chirho, DraftPageKindChirho::BriefChirho)
    }

    /// The credit line beside a live brief form, from one read of the credit view port. A page with no form to send
    /// keeps none: there is nothing to draft with there.
    pub fn with_credits_chirho(
        mut self,
        view_chirho: &CreditViewChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        if !self.is_brief_chirho() || self.brief_action_chirho.is_none() {
            return self;
        }
        let copy_chirho = CreditsCopyChirho::for_locale_chirho(locale_chirho);
        self.credits_chirho = Some(match view_chirho {
            CreditViewChirho::KnownChirho(statement_chirho) => DraftCreditsChirho::KnownChirho {
                label_chirho: copy_chirho.balance_heading_chirho.clone(),
                balance_chirho: copy_chirho
                    .balance_chirho(statement_chirho.available_chirho().whole_chirho()),
            },
            CreditViewChirho::UnavailableChirho(_) => DraftCreditsChirho::UnavailableChirho {
                sentence_chirho: copy_chirho.unavailable_heading_chirho,
            },
        });
        self
    }
}
