// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

use perffection_workspace_chirho::page_edit_request_chirho::{
    parse_page_edit_form_chirho, parse_page_restore_form_chirho, PAGE_EDIT_FORM_MAX_BYTES_CHIRHO,
    PAGE_RESTORE_FORM_MAX_BYTES_CHIRHO,
};

fn valid_form_chirho(expected_version_chirho: u64) -> Vec<u8> {
    format!(
        "schema_version_chirho=2&expected_version_chirho={expected_version_chirho}&business_name_chirho=North+Street+Repairs&headline_chirho=Clear+help+when+work+cannot+wait.&summary_chirho=Local+repair+service+for+homes+and+small+shops.&service_1_title_chirho=Same-week+repairs&service_1_body_chirho=Practical+repair+work+with+plain+estimates.&service_2_title_chirho=&service_2_body_chirho=&service_3_title_chirho=&service_3_body_chirho=&contact_heading_chirho=Tell+us+what+needs+attention.&contact_phone_chirho=555-0124&contact_email_chirho=&contact_details_chirho=During+business+hours."
    )
    .into_bytes()
}

#[test]
fn exact_browser_form_decodes_normalizes_and_maps_zero_to_first_write_chirho() {
    let request_chirho = parse_page_edit_form_chirho(&valid_form_chirho(0)).unwrap();

    assert_eq!(request_chirho.expected_current_version_chirho(), None);
    assert_eq!(
        request_chirho.document_chirho().business_name_chirho(),
        "North Street Repairs"
    );
    assert_eq!(request_chirho.document_chirho().services_chirho().len(), 1);
    assert_eq!(
        request_chirho.document_chirho().contact_details_chirho(),
        "During business hours."
    );
    assert_eq!(
        request_chirho
            .document_chirho()
            .contact_phone_chirho()
            .expect("the phone action should validate")
            .href_chirho(),
        "tel:5550124"
    );
}

#[test]
fn existing_document_requires_the_exact_optimistic_version_chirho() {
    let request_chirho = parse_page_edit_form_chirho(&valid_form_chirho(17)).unwrap();
    assert_eq!(request_chirho.expected_current_version_chirho(), Some(17));
}

#[test]
fn one_typed_contact_is_required_while_hours_and_service_area_remain_optional_chirho() {
    let email_only_chirho = String::from_utf8(valid_form_chirho(1))
        .unwrap()
        .replace("contact_phone_chirho=555-0124", "contact_phone_chirho=")
        .replace(
            "contact_email_chirho=",
            "contact_email_chirho=hello%40north-street.example",
        )
        .replace(
            "contact_details_chirho=During+business+hours.",
            "contact_details_chirho=",
        );
    let request_chirho = parse_page_edit_form_chirho(email_only_chirho.as_bytes()).unwrap();
    assert!(request_chirho
        .document_chirho()
        .contact_phone_chirho()
        .is_none());
    assert_eq!(
        request_chirho
            .document_chirho()
            .contact_email_chirho()
            .expect("the email-only form should validate")
            .href_chirho(),
        "mailto:hello@north-street.example"
    );
    assert!(request_chirho
        .document_chirho()
        .contact_details_chirho()
        .is_empty());

    let no_contact_chirho = String::from_utf8(valid_form_chirho(1))
        .unwrap()
        .replace("contact_phone_chirho=555-0124", "contact_phone_chirho=");
    assert_eq!(
        parse_page_edit_form_chirho(no_contact_chirho.as_bytes())
            .unwrap_err()
            .code_chirho(),
        "invalid-page-document-chirho"
    );
}

#[test]
fn malformed_duplicate_unknown_and_missing_fields_refuse_chirho() {
    let malformed_chirho = valid_form_chirho(1)
        .into_iter()
        .chain(b"&headline_chirho=duplicate".iter().copied())
        .collect::<Vec<_>>();
    assert_eq!(
        parse_page_edit_form_chirho(&malformed_chirho)
            .unwrap_err()
            .code_chirho(),
        "duplicate-field-chirho"
    );

    let unknown_chirho = valid_form_chirho(1)
        .into_iter()
        .chain(b"&viewer_chirho=someone".iter().copied())
        .collect::<Vec<_>>();
    assert_eq!(
        parse_page_edit_form_chirho(&unknown_chirho)
            .unwrap_err()
            .code_chirho(),
        "unexpected-field-chirho"
    );

    let missing_chirho = String::from_utf8(valid_form_chirho(1))
        .unwrap()
        .replace("&contact_details_chirho=During+business+hours.", "");
    assert_eq!(
        parse_page_edit_form_chirho(missing_chirho.as_bytes())
            .unwrap_err()
            .code_chirho(),
        "missing-field-chirho"
    );

    let missing_typed_contact_chirho = String::from_utf8(valid_form_chirho(1))
        .unwrap()
        .replace("&contact_phone_chirho=555-0124", "");
    assert_eq!(
        parse_page_edit_form_chirho(missing_typed_contact_chirho.as_bytes())
            .unwrap_err()
            .code_chirho(),
        "missing-field-chirho"
    );
}

#[test]
fn invalid_numbers_encoding_document_and_size_refuse_chirho() {
    let invalid_version_chirho = String::from_utf8(valid_form_chirho(1)).unwrap().replace(
        "expected_version_chirho=1",
        "expected_version_chirho=not-a-number",
    );
    assert_eq!(
        parse_page_edit_form_chirho(invalid_version_chirho.as_bytes())
            .unwrap_err()
            .code_chirho(),
        "invalid-expected-version-chirho"
    );

    let invalid_encoding_chirho = String::from_utf8(valid_form_chirho(1))
        .unwrap()
        .replace("North+Street", "North%ZZStreet");
    assert_eq!(
        parse_page_edit_form_chirho(invalid_encoding_chirho.as_bytes())
            .unwrap_err()
            .code_chirho(),
        "malformed-form-encoding-chirho"
    );

    let invalid_document_chirho = String::from_utf8(valid_form_chirho(1)).unwrap().replace(
        "business_name_chirho=North+Street+Repairs",
        "business_name_chirho=",
    );
    assert_eq!(
        parse_page_edit_form_chirho(invalid_document_chirho.as_bytes())
            .unwrap_err()
            .code_chirho(),
        "invalid-page-document-chirho"
    );

    assert_eq!(
        parse_page_edit_form_chirho(&vec![b'x'; PAGE_EDIT_FORM_MAX_BYTES_CHIRHO + 1])
            .unwrap_err()
            .code_chirho(),
        "body-too-large-chirho"
    );
}

#[test]
fn exact_restore_form_carries_only_two_optimistic_versions_chirho() {
    let request_chirho =
        parse_page_restore_form_chirho(b"expected_version_chirho=7&restore_from_version_chirho=3")
            .unwrap();

    assert_eq!(request_chirho.expected_current_version_chirho(), 7);
    assert_eq!(request_chirho.restore_from_version_chirho(), 3);
}

#[test]
fn malformed_stale_and_oversized_restore_forms_refuse_chirho() {
    for (body_chirho, expected_code_chirho) in [
        (
            b"expected_version_chirho=7&restore_from_version_chirho=7".as_slice(),
            "invalid-restore-version-chirho",
        ),
        (
            b"expected_version_chirho=0&restore_from_version_chirho=1".as_slice(),
            "invalid-expected-version-chirho",
        ),
        (
            b"expected_version_chirho=7&restore_from_version_chirho=3&viewer_chirho=other"
                .as_slice(),
            "unexpected-field-chirho",
        ),
        (
            b"expected_version_chirho=7&restore_from_version_chirho=3&restore_from_version_chirho=2"
                .as_slice(),
            "duplicate-field-chirho",
        ),
    ] {
        assert_eq!(
            parse_page_restore_form_chirho(body_chirho)
                .unwrap_err()
                .code_chirho(),
            expected_code_chirho
        );
    }

    assert_eq!(
        parse_page_restore_form_chirho(&vec![b'x'; PAGE_RESTORE_FORM_MAX_BYTES_CHIRHO + 1])
            .unwrap_err()
            .code_chirho(),
        "body-too-large-chirho"
    );
}

/// The engine's caller key, through the browser caller-key carrier: every app-route form carries it at
/// most once, and every form reader drops it unread. The contract:
/// spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md
mod caller_key_carrier_chirho {
    use super::valid_form_chirho;
    use perffection_workspace_chirho::page_edit_request_chirho::{
        parse_page_edit_form_chirho, parse_page_restore_form_chirho, FORM_CALLER_KEY_FIELD_CHIRHO,
        FORM_CALLER_KEY_PLACEHOLDER_CHIRHO,
    };

    const RESTORE_CHIRHO: &[u8] = b"expected_version_chirho=7&restore_from_version_chirho=3";

    /// The field the carrier adds, first or last in the body. Values: empty (a local root's
    /// placeholder), an engine-shaped key, the longest the engine admits (128 bytes), and an encoded
    /// value that only looks like one of her fields.
    fn keyed_chirho(body_chirho: &[u8], value_chirho: &str, first_chirho: bool) -> Vec<u8> {
        let field_chirho = format!("{FORM_CALLER_KEY_FIELD_CHIRHO}={value_chirho}");
        let body_text_chirho = String::from_utf8(body_chirho.to_vec()).unwrap();
        if first_chirho {
            format!("{field_chirho}&{body_text_chirho}").into_bytes()
        } else {
            format!("{body_text_chirho}&{field_chirho}").into_bytes()
        }
    }

    fn values_chirho() -> [String; 4] {
        [
            String::new(),
            "Mx3kQ9-zR_7vT2nB8wY4pL6dF1gH5jK0aS3eU9iO2c".to_string(),
            "k".repeat(128),
            "expected_version_chirho%3D99".to_string(),
        ]
    }

    #[test]
    fn edit_and_restore_read_her_fields_the_same_with_the_key_anywhere_chirho() {
        let edit_chirho = parse_page_edit_form_chirho(&valid_form_chirho(4)).unwrap();
        let restore_chirho = parse_page_restore_form_chirho(RESTORE_CHIRHO).unwrap();
        for value_chirho in values_chirho() {
            for first_chirho in [true, false] {
                let keyed_edit_chirho =
                    keyed_chirho(&valid_form_chirho(4), &value_chirho, first_chirho);
                assert_eq!(
                    parse_page_edit_form_chirho(&keyed_edit_chirho).unwrap(),
                    edit_chirho,
                    "edit with key {value_chirho:?}"
                );
                let keyed_restore_chirho =
                    keyed_chirho(RESTORE_CHIRHO, &value_chirho, first_chirho);
                assert_eq!(
                    parse_page_restore_form_chirho(&keyed_restore_chirho).unwrap(),
                    restore_chirho,
                    "restore with key {value_chirho:?}"
                );
            }
        }
        // Each keyed parse equals this one, so the encoded look-alike stayed the key's value: her
        // expected version is 4 in all of them, never 99.
        assert_eq!(edit_chirho.expected_current_version_chirho(), Some(4));
    }

    #[test]
    fn a_second_key_is_a_duplicate_and_any_other_unknown_field_still_refuses_chirho() {
        let twice_chirho = keyed_chirho(&keyed_chirho(RESTORE_CHIRHO, "a", true), "b", false);
        assert_eq!(
            parse_page_restore_form_chirho(&twice_chirho)
                .unwrap_err()
                .code_chirho(),
            "duplicate-field-chirho"
        );
        let edit_twice_chirho =
            keyed_chirho(&keyed_chirho(&valid_form_chirho(4), "", true), "", false);
        assert_eq!(
            parse_page_edit_form_chirho(&edit_twice_chirho)
                .unwrap_err()
                .code_chirho(),
            "duplicate-field-chirho"
        );
        // a near name is not the carrier's field: it is unknown, as before
        let near_chirho = format!(
            "{}&{FORM_CALLER_KEY_FIELD_CHIRHO}x=1",
            String::from_utf8(RESTORE_CHIRHO.to_vec()).unwrap()
        );
        assert_eq!(
            parse_page_restore_form_chirho(near_chirho.as_bytes())
                .unwrap_err()
                .code_chirho(),
            "unexpected-field-chirho"
        );
        // the key alone is not a form: her fields are still required
        let alone_chirho = format!("{FORM_CALLER_KEY_FIELD_CHIRHO}=k");
        assert_eq!(
            parse_page_restore_form_chirho(alone_chirho.as_bytes())
                .unwrap_err()
                .code_chirho(),
            "missing-field-chirho"
        );
    }

    #[test]
    fn the_placeholder_is_the_carrier_field_with_an_empty_value_chirho() {
        assert_eq!(
            FORM_CALLER_KEY_PLACEHOLDER_CHIRHO,
            format!(r#"<input type="hidden" name="{FORM_CALLER_KEY_FIELD_CHIRHO}" value="">"#)
        );
    }
}
