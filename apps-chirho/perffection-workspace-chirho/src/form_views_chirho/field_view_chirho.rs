// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! What every workspace form renders beside a control: the ids that describe it and the
//! messages it carries. The editor and the first-draft brief each decide where an issue belongs;
//! the shape they hand the templates (`partials_chirho/field_state_chirho.html`) is this one.

/// One control's state as the template renders it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldViewChirho {
    pub(crate) message_id_chirho: String,
    pub(crate) messages_chirho: Vec<String>,
    /// Space-separated ids of what describes the control: its help text, its group's message, its
    /// own message. Every id names an element the template renders; empty renders no attribute.
    pub(crate) described_by_chirho: String,
    pub(crate) invalid_chirho: bool,
}

impl FieldViewChirho {
    pub(crate) fn clean_chirho(message_id_chirho: String) -> Self {
        Self {
            message_id_chirho,
            messages_chirho: Vec::new(),
            described_by_chirho: String::new(),
            invalid_chirho: false,
        }
    }

    /// After every message is placed: what describes the control, and whether it is invalid.
    /// `help_id_chirho` is the help text the template renders under it, if any;
    /// `group_message_id_chirho` is its group's message when that group's rule is broken.
    pub(crate) fn settle_chirho(
        &mut self,
        help_id_chirho: Option<String>,
        group_message_id_chirho: Option<&str>,
    ) {
        let has_own_message_chirho = !self.messages_chirho.is_empty();
        let mut described_by_chirho = Vec::with_capacity(3);
        if let Some(help_id_chirho) = help_id_chirho {
            described_by_chirho.push(help_id_chirho);
        }
        if let Some(message_id_chirho) = group_message_id_chirho {
            described_by_chirho.push(message_id_chirho.to_string());
        }
        if has_own_message_chirho {
            described_by_chirho.push(self.message_id_chirho.clone());
        }
        self.described_by_chirho = described_by_chirho.join(" ");
        self.invalid_chirho = has_own_message_chirho || group_message_id_chirho.is_some();
    }
}

/// A rule over several controls ("a phone number or an email"): its message is rendered once on
/// the group and described into each member control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldGroupViewChirho {
    pub(crate) message_id_chirho: &'static str,
    pub(crate) messages_chirho: Vec<String>,
}

impl FieldGroupViewChirho {
    pub(crate) fn clean_chirho(message_id_chirho: &'static str) -> Self {
        Self {
            message_id_chirho,
            messages_chirho: Vec::new(),
        }
    }

    pub(crate) fn is_invalid_chirho(&self) -> bool {
        !self.messages_chirho.is_empty()
    }

    /// The id a member control is described by, only while the rule is broken.
    pub(crate) fn live_message_id_chirho(&self) -> Option<&'static str> {
        self.is_invalid_chirho().then_some(self.message_id_chirho)
    }
}

/// One line of the summary at the top of a refused form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormNoticeChirho {
    pub(crate) field_label_chirho: String,
    pub(crate) message_chirho: String,
    /// The control the line links to; `None` for an issue with no control to point at.
    pub(crate) control_id_chirho: Option<String>,
}
