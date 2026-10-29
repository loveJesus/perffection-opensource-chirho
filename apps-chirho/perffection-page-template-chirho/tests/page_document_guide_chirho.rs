// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! The description both drafting writers receive stays tied to the validator's limits and to
//! itself. Numbers and keys are asserted; the instruction's wording is not.

use perffection_page_template_chirho::page_document_chirho::{
    PageDocumentV2Chirho, PageServiceV1Chirho, PAGE_SERVICE_SLOT_COUNT_CHIRHO,
};
use perffection_page_template_chirho::page_document_guide_chirho::{
    draft_answer_instruction_chirho, own_agent_guide_chirho, DRAFT_ANSWER_KEYS_CHIRHO,
    DRAFT_ANSWER_SERVICE_COUNT_CHIRHO, OWN_AGENT_SAMPLE_ANSWER_CHIRHO,
};

/// A document whose every written field sits exactly at the limit the guide states for it.
fn document_at_the_stated_limits_chirho() -> PageDocumentV2Chirho {
    let limit_chirho = |key_chirho: &str| {
        DRAFT_ANSWER_KEYS_CHIRHO
            .iter()
            .find(|entry_chirho| entry_chirho.key_chirho == key_chirho)
            .unwrap_or_else(|| panic!("the guide has no key {key_chirho}"))
            .max_characters_chirho
    };
    let fill_chirho = |key_chirho: &str| "a".repeat(limit_chirho(key_chirho));
    PageDocumentV2Chirho {
        business_name_chirho: fill_chirho("business_name"),
        headline_chirho: fill_chirho("headline"),
        summary_chirho: fill_chirho("summary"),
        services_chirho: std::array::from_fn(|_| PageServiceV1Chirho {
            title_chirho: fill_chirho("services[].title"),
            body_chirho: fill_chirho("services[].body"),
        }),
        contact_heading_chirho: fill_chirho("contact_heading"),
        contact_phone_chirho: "+1 555 0100".to_string(),
        contact_email_chirho: String::new(),
        contact_details_chirho: fill_chirho("contact_details"),
        ..PageDocumentV2Chirho::default()
    }
}

#[test]
fn a_document_at_every_stated_limit_is_valid_and_one_character_more_is_not_chirho() {
    let document_chirho = document_at_the_stated_limits_chirho();
    assert!(
        document_chirho.validate_chirho().is_ok(),
        "the guide states a limit the validator refuses: {:?}",
        document_chirho.validate_chirho().err()
    );
    let mut over_chirho = document_chirho;
    over_chirho.headline_chirho.push('a');
    assert!(
        over_chirho.validate_chirho().is_err(),
        "the guide states a headline limit looser than the validator's"
    );
}

#[test]
fn every_limit_in_the_table_is_said_in_the_instruction_chirho() {
    let instruction_chirho = draft_answer_instruction_chirho(None);
    for entry_chirho in DRAFT_ANSWER_KEYS_CHIRHO
        .iter()
        .filter(|entry_chirho| !entry_chirho.copied_exactly_chirho)
    {
        assert!(
            instruction_chirho.contains(&entry_chirho.max_characters_chirho.to_string()),
            "the instruction never states the limit of {}",
            entry_chirho.key_chirho
        );
    }
    let top_level_key_chirho = |key_chirho: &str| {
        key_chirho
            .split("[]")
            .next()
            .unwrap_or(key_chirho)
            .to_string()
    };
    for entry_chirho in &DRAFT_ANSWER_KEYS_CHIRHO {
        assert!(
            instruction_chirho.contains(&top_level_key_chirho(entry_chirho.key_chirho)),
            "the instruction never names {}",
            entry_chirho.key_chirho
        );
    }
    assert_eq!(
        DRAFT_ANSWER_SERVICE_COUNT_CHIRHO,
        PAGE_SERVICE_SLOT_COUNT_CHIRHO
    );
}

#[test]
fn the_tone_is_the_platform_briefs_alone_chirho() {
    let with_tone_chirho = draft_answer_instruction_chirho(Some("plain and direct"));
    let without_chirho = draft_answer_instruction_chirho(None);
    assert!(with_tone_chirho.starts_with(&without_chirho));
    assert!(with_tone_chirho.ends_with("plain and direct."));
    assert_ne!(with_tone_chirho, without_chirho);
}

#[test]
fn her_own_agent_reads_the_same_instruction_the_platform_card_reads_chirho() {
    let guide_chirho = own_agent_guide_chirho();
    assert!(
        guide_chirho.ends_with(&draft_answer_instruction_chirho(None)),
        "the two roads must share one instruction"
    );
}

#[test]
fn the_sample_answer_uses_only_the_keys_the_table_names_and_stays_inside_their_limits_chirho() {
    let sample_chirho: serde_json::Value =
        serde_json::from_str(OWN_AGENT_SAMPLE_ANSWER_CHIRHO).expect("the sample answer is JSON");
    let object_chirho = sample_chirho
        .as_object()
        .expect("the sample answer is one object");
    let limit_chirho = |key_chirho: &str| {
        DRAFT_ANSWER_KEYS_CHIRHO
            .iter()
            .find(|entry_chirho| entry_chirho.key_chirho == key_chirho)
            .unwrap_or_else(|| {
                panic!("the sample uses a key the table does not name: {key_chirho}")
            })
            .max_characters_chirho
    };
    for (key_chirho, value_chirho) in object_chirho {
        if key_chirho == "services" {
            let services_chirho = value_chirho.as_array().expect("services is an array");
            assert!(
                !services_chirho.is_empty()
                    && services_chirho.len() <= DRAFT_ANSWER_SERVICE_COUNT_CHIRHO
            );
            for service_chirho in services_chirho {
                for (part_chirho, text_chirho) in
                    service_chirho.as_object().expect("a service is an object")
                {
                    let text_chirho = text_chirho.as_str().expect("service parts are text");
                    assert!(
                        text_chirho.chars().count()
                            <= limit_chirho(&format!("services[].{part_chirho}"))
                    );
                }
            }
        } else {
            let text_chirho = value_chirho.as_str().expect("answer values are text");
            assert!(text_chirho.chars().count() <= limit_chirho(key_chirho));
        }
    }
}
