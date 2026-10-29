// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The page document described for a writer that is not a person.
//!
//! Two writers draft a customer's page: the platform's drafting card (paid from credits) and the
//! customer's own AI agent (her own tool, no provider call of ours). Both are given THIS
//! description of the answer we can read, stated from the validator's own limits, so the two
//! roads cannot drift from each other or from the validator. The answer is untrusted either way:
//! the workspace maps it through one bounded mapper and she reviews it before anything is saved.
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/first-draft-chirho.md`.

use crate::page_document_chirho::{
    BUSINESS_NAME_MAX_CHARACTERS_CHIRHO, CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
    CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO, CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
    CONTACT_PHONE_MAX_CHARACTERS_CHIRHO, HEADLINE_MAX_CHARACTERS_CHIRHO,
    PAGE_SERVICE_SLOT_COUNT_CHIRHO, SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
    SERVICE_TITLE_MAX_CHARACTERS_CHIRHO, SUMMARY_MAX_CHARACTERS_CHIRHO,
};

/// One key of the answer a drafting writer returns, for a page that shows the shape as a table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DraftAnswerKeyChirho {
    /// The key as the writer spells it; keys inside a service are `services[].title` style.
    pub key_chirho: &'static str,
    /// Greatest number of characters kept.
    pub max_characters_chirho: usize,
    /// The owner's own value: copied exactly or left empty, never rewritten by a writer.
    pub copied_exactly_chirho: bool,
}

const fn written_chirho(key_chirho: &'static str, max_chirho: usize) -> DraftAnswerKeyChirho {
    DraftAnswerKeyChirho {
        key_chirho,
        max_characters_chirho: max_chirho,
        copied_exactly_chirho: false,
    }
}

const fn copied_chirho(key_chirho: &'static str, max_chirho: usize) -> DraftAnswerKeyChirho {
    DraftAnswerKeyChirho {
        key_chirho,
        max_characters_chirho: max_chirho,
        copied_exactly_chirho: true,
    }
}

/// Every key of the answer with the validator's limit for it.
pub const DRAFT_ANSWER_KEYS_CHIRHO: [DraftAnswerKeyChirho; 9] = [
    written_chirho("business_name", BUSINESS_NAME_MAX_CHARACTERS_CHIRHO),
    written_chirho("headline", HEADLINE_MAX_CHARACTERS_CHIRHO),
    written_chirho("summary", SUMMARY_MAX_CHARACTERS_CHIRHO),
    written_chirho("services[].title", SERVICE_TITLE_MAX_CHARACTERS_CHIRHO),
    written_chirho("services[].body", SERVICE_BODY_MAX_CHARACTERS_CHIRHO),
    written_chirho("contact_heading", CONTACT_HEADING_MAX_CHARACTERS_CHIRHO),
    copied_chirho("contact_phone", CONTACT_PHONE_MAX_CHARACTERS_CHIRHO),
    copied_chirho("contact_email", CONTACT_EMAIL_MAX_CHARACTERS_CHIRHO),
    written_chirho("contact_details", CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO),
];

/// How many services an answer may list.
pub const DRAFT_ANSWER_SERVICE_COUNT_CHIRHO: usize = PAGE_SERVICE_SLOT_COUNT_CHIRHO;

/// The instruction both writers receive. `tone_words_chirho` is the platform brief's chosen
/// tone; an agent working with the owner directly hears the tone from her, so it passes `None`.
pub fn draft_answer_instruction_chirho(tone_words_chirho: Option<&str>) -> String {
    let mut instruction_chirho = format!(
        "You draft the first version of a small local business's one-page website from the owner's own words. \
Answer with ONE JSON object and nothing else, no markdown, no code fence, with exactly these keys: \
business_name (at most {name} characters), headline (at most {headline}), summary (at most {summary}), \
services (an array of up to {services} objects with title at most {title} and body at most {body}), \
contact_heading (at most {contact_heading}), contact_phone (copied EXACTLY as given, or empty), \
contact_email (copied EXACTLY as given, or empty), contact_details (at most {details}). \
Plain text only: no HTML, no links, no emoji, no claims the owner did not make, no prices unless given. \
Write in the language the owner wrote in.",
        name = BUSINESS_NAME_MAX_CHARACTERS_CHIRHO,
        headline = HEADLINE_MAX_CHARACTERS_CHIRHO,
        summary = SUMMARY_MAX_CHARACTERS_CHIRHO,
        services = PAGE_SERVICE_SLOT_COUNT_CHIRHO,
        title = SERVICE_TITLE_MAX_CHARACTERS_CHIRHO,
        body = SERVICE_BODY_MAX_CHARACTERS_CHIRHO,
        contact_heading = CONTACT_HEADING_MAX_CHARACTERS_CHIRHO,
        details = CONTACT_DETAILS_MAX_CHARACTERS_CHIRHO,
    );
    if let Some(tone_words_chirho) = tone_words_chirho {
        instruction_chirho.push_str(" Tone: ");
        instruction_chirho.push_str(tone_words_chirho);
        instruction_chirho.push('.');
    }
    instruction_chirho
}

/// What a customer's own agent is told before the instruction: who it is working for, what it
/// must never invent, and what happens to its answer. Generic: it carries nothing of a customer.
pub const OWN_AGENT_PREFACE_CHIRHO: &str = "You are helping the owner of a small local business \
write their one-page website on Perffection. Work from what the owner tells you. If you need \
something you do not know, ask the owner: never invent a phone number, an email address, an \
address, opening hours, a price, a review, an award or any claim. The owner will paste your \
answer into Perffection, which checks it against these limits and shows it to the owner to \
review and correct; nothing is saved or published until the owner does that. A page is complete \
only with a business name, a headline, a summary, a contact heading, at least one service that \
has both a title and a body, and a phone number or an email address; contact details are \
optional. If the owner has not given you one of the required parts, ask for it.";

/// Said before the sample answer wherever a guide shows it.
pub const OWN_AGENT_SAMPLE_PREFACE_CHIRHO: &str = "An example of a complete answer, for a \
business that does not exist. Follow its shape, never its content.";

/// A complete example of the answer, for a guide to show beside the instruction: it maps to a
/// document the validator accepts (tested where the mapper lives). The business is fictional, the
/// phone number is from the range reserved for fiction and the email address is at a domain
/// reserved for examples, so nothing in it can be mistaken for, or reach, a real one.
pub const OWN_AGENT_SAMPLE_ANSWER_CHIRHO: &str = r#"{
  "business_name": "Example Corner Bakery",
  "headline": "Bread and pastries baked before sunrise",
  "summary": "A small neighbourhood bakery. We bake in small batches every morning and sell out most days, so regulars call ahead.",
  "services": [
    {
      "title": "Daily bread",
      "body": "Sourdough, wholemeal and seeded loaves, out of the oven by seven."
    },
    {
      "title": "Celebration cakes",
      "body": "Made to order with a week's notice. Tell us the occasion and how many you are feeding."
    }
  ],
  "contact_heading": "Call ahead or come by",
  "contact_phone": "555-0142",
  "contact_email": "hello@example.com",
  "contact_details": "Open Tuesday to Saturday, 7 am to 2 pm."
}"#;

/// Said before a customer's current page when her agent is asked to revise it rather than start
/// over. The page itself is the customer's and is added only inside her signed-in workspace.
pub const OWN_AGENT_REVISION_PREFACE_CHIRHO: &str = "The owner's current page follows as the \
same JSON object. Keep what the owner does not ask you to change, revise the rest as the owner \
asks, and answer with the whole object again.";

/// The generic guide for a customer's own agent: the preface, then the same instruction the
/// platform's drafting card receives.
pub fn own_agent_guide_chirho() -> String {
    format!(
        "{OWN_AGENT_PREFACE_CHIRHO}\n\n{}",
        draft_answer_instruction_chirho(None)
    )
}
