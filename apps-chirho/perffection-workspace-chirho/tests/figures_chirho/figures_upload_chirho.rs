// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures as a chosen file (gpt #26599, choice A): one strict grammar reads the review form's
//! multipart body into the same fields the pasted form carries. Real Chrome bodies captured by the
//! carrier probe (fixtures_chirho/) read as she sent them; every framing, header, field, bound and
//! source rule refuses by its code, never by truncating or choosing. Behaviour, never wording.

use perffection_ui_chirho::WorkspaceCopyChirho;
use perffection_workspace_chirho::figures_flow_chirho::figures_form_chirho::{
    classify_figures_save_chirho, read_figures_review_form_chirho, FiguresFormRefusalChirho,
    FiguresSaveDecisionChirho,
};
use perffection_workspace_chirho::figures_flow_chirho::figures_upload_chirho::parse_figures_upload_chirho;
use perffection_workspace_chirho::implementations_chirho::{
    build_figures_write_chirho::build_figures_write_chirho,
    review_figures_chirho::review_figures_chirho,
};

use FiguresFormRefusalChirho::{
    EmptyFileChirho, FileAndPasteChirho, FileNotTextChirho, TooLargeChirho, UnreadableChirho,
};

const CHROME_PASTE_ONLY_CHIRHO: &[u8] =
    include_bytes!("fixtures_chirho/chrome-paste-only-chirho.bin");
const CHROME_FILE_ONLY_CHIRHO: &[u8] =
    include_bytes!("fixtures_chirho/chrome-file-only-chirho.bin");
const CHROME_FILE_AND_PASTE_CHIRHO: &[u8] =
    include_bytes!("fixtures_chirho/chrome-file-and-paste-chirho.bin");
const BOUNDARY_CHIRHO: &str = "----WebKitFormBoundaryTestChirho0123";
const CSV_CHIRHO: &[u8] = b"month,kind,money_in,costs,jobs\r\n2026-01,Bread,1200.50,400,300\r\n";

/// One part as a browser frames it.
struct PartSpecChirho<'a> {
    name_chirho: &'a str,
    file_name_chirho: Option<&'a str>,
    content_chirho: &'a [u8],
}

fn part_chirho<'a>(name_chirho: &'a str, content_chirho: &'a [u8]) -> PartSpecChirho<'a> {
    PartSpecChirho {
        name_chirho,
        file_name_chirho: None,
        content_chirho,
    }
}

fn file_part_chirho<'a>(file_name_chirho: &'a str, content_chirho: &'a [u8]) -> PartSpecChirho<'a> {
    PartSpecChirho {
        name_chirho: "csv_file_chirho",
        file_name_chirho: Some(file_name_chirho),
        content_chirho,
    }
}

fn body_chirho(parts_chirho: &[PartSpecChirho]) -> Vec<u8> {
    let mut body_chirho = Vec::new();
    for part_spec_chirho in parts_chirho {
        body_chirho.extend(format!("--{BOUNDARY_CHIRHO}\r\n").as_bytes());
        let mut head_chirho = format!(
            "Content-Disposition: form-data; name=\"{}\"",
            part_spec_chirho.name_chirho
        );
        if let Some(file_name_chirho) = part_spec_chirho.file_name_chirho {
            head_chirho.push_str(&format!(
                "; filename=\"{file_name_chirho}\"\r\nContent-Type: text/csv"
            ));
        }
        body_chirho.extend(head_chirho.as_bytes());
        body_chirho.extend(b"\r\n\r\n");
        body_chirho.extend(part_spec_chirho.content_chirho);
        body_chirho.extend(b"\r\n");
    }
    body_chirho.extend(format!("--{BOUNDARY_CHIRHO}--\r\n").as_bytes());
    body_chirho
}

/// The figures form as Chrome sends it, in its own field order.
fn form_body_chirho(file_chirho: Option<(&str, &[u8])>, paste_chirho: &[u8]) -> Vec<u8> {
    let (file_name_chirho, file_bytes_chirho) = file_chirho.unwrap_or(("", b""));
    body_chirho(&[
        part_chirho("expected_version_chirho", b"0"),
        part_chirho("csv_chirho", paste_chirho),
        file_part_chirho(file_name_chirho, file_bytes_chirho),
        part_chirho("currency_chirho", b"USD"),
        part_chirho("note_chirho", b"from my books"),
    ])
}

fn replaced_chirho(body_chirho: &[u8], from_chirho: &str, to_chirho: &str) -> Vec<u8> {
    let text_chirho = String::from_utf8(body_chirho.to_vec()).unwrap();
    assert!(text_chirho.contains(from_chirho), "{from_chirho}");
    text_chirho.replacen(from_chirho, to_chirho, 1).into_bytes()
}

#[test]
fn real_chrome_uploads_read_as_she_sent_them_chirho() {
    let pasted_chirho = parse_figures_upload_chirho(CHROME_PASTE_ONLY_CHIRHO).unwrap();
    assert_eq!(
        (
            pasted_chirho.csv_chirho.as_str(),
            pasted_chirho.file_name_chirho.as_deref(),
            pasted_chirho.currency_chirho.as_str(),
            pasted_chirho.note_chirho.as_str(),
            pasted_chirho.expected_version_chirho
        ),
        (
            "month,kind,money_in,costs,jobs\r\n2026-03,Pies,77,,\r\n",
            None,
            "USD",
            "from my books",
            0
        )
    );
    let filed_chirho = parse_figures_upload_chirho(CHROME_FILE_ONLY_CHIRHO).unwrap();
    assert_eq!(
        filed_chirho.csv_chirho,
        "month,kind,money_in,costs,jobs\r\n2026-01,Café,1200.50,400,300\r\n2026-02,Café,1300,450,320\r\n"
    );
    assert_eq!(
        filed_chirho.file_name_chirho.as_deref(),
        Some("upload-figures.csv")
    );
    let kept_chirho = filed_chirho.import_chirho().kept_chirho.unwrap();
    assert_eq!(
        kept_chirho.document_chirho().kinds_chirho,
        vec!["Café".to_string()]
    );
    assert_eq!(
        parse_figures_upload_chirho(CHROME_FILE_AND_PASTE_CHIRHO),
        Err(FileAndPasteChirho)
    );
}

#[test]
fn the_framing_is_all_or_nothing_chirho() {
    let good_chirho = form_body_chirho(Some(("f.csv", CSV_CHIRHO)), b"");
    assert!(parse_figures_upload_chirho(&good_chirho).is_ok());
    let close_chirho = format!("--{BOUNDARY_CHIRHO}--\r\n");
    let unread_chirho = |label_chirho: &str, body_chirho: Vec<u8>| {
        assert_eq!(
            parse_figures_upload_chirho(&body_chirho),
            Err(UnreadableChirho),
            "{label_chirho}"
        );
    };
    unread_chirho(
        "no closing delimiter",
        good_chirho[..good_chirho.len() - close_chirho.len()].to_vec(),
    );
    unread_chirho(
        "data after the close",
        [good_chirho.as_slice(), b"more"].concat(),
    );
    unread_chirho(
        "a preamble",
        [b"hello\r\n".as_slice(), &good_chirho].concat(),
    );
    unread_chirho(
        "a delimiter running straight into the next header, with no CRLF",
        replaced_chirho(
            &good_chirho,
            &format!("\r\n--{BOUNDARY_CHIRHO}\r\nContent-Disposition: form-data; name=\"currency"),
            &format!("\r\n--{BOUNDARY_CHIRHO}Content-Disposition: form-data; name=\"currency"),
        ),
    );
    let long_chirho = "b".repeat(71);
    unread_chirho(
        "a 71-character boundary",
        good_chirho_with_boundary_chirho(&good_chirho, &long_chirho),
    );
    unread_chirho(
        "a boundary character RFC 2046 does not allow",
        good_chirho_with_boundary_chirho(&good_chirho, "bad@boundary"),
    );
    unread_chirho(
        "an empty boundary",
        good_chirho_with_boundary_chirho(&good_chirho, ""),
    );
    unread_chirho(
        "a boundary ending in a space",
        good_chirho_with_boundary_chirho(&good_chirho, "edge "),
    );
    assert!(
        parse_figures_upload_chirho(&good_chirho_with_boundary_chirho(
            &good_chirho,
            &"b".repeat(70)
        ))
        .is_ok()
    );
    // the close with no final CRLF is still a close
    assert!(parse_figures_upload_chirho(&good_chirho[..good_chirho.len() - 2]).is_ok());
}

fn good_chirho_with_boundary_chirho(body_chirho: &[u8], boundary_chirho: &str) -> Vec<u8> {
    String::from_utf8(body_chirho.to_vec())
        .unwrap()
        .replace(BOUNDARY_CHIRHO, boundary_chirho)
        .into_bytes()
}

#[test]
fn each_part_header_is_read_strictly_chirho() {
    let good_chirho = form_body_chirho(Some(("f.csv", CSV_CHIRHO)), b"");
    let currency_head_chirho = "Content-Disposition: form-data; name=\"currency_chirho\"";
    for (label_chirho, replacement_chirho) in [
        (
            "an unknown header",
            format!("{currency_head_chirho}\r\nContent-Length: 3"),
        ),
        (
            "a line without a colon",
            format!("{currency_head_chirho}\r\nnot a header"),
        ),
        (
            "a header name with a space",
            "Content Disposition: form-data; name=\"currency_chirho\"".to_string(),
        ),
        (
            "two dispositions",
            format!("{currency_head_chirho}\r\n{currency_head_chirho}"),
        ),
        (
            "two content types",
            format!(
                "{currency_head_chirho}\r\nContent-Type: text/plain\r\nContent-Type: text/plain"
            ),
        ),
        (
            "not form-data",
            "Content-Disposition: attachment; name=\"currency_chirho\"".to_string(),
        ),
        (
            "an unknown parameter",
            format!("{currency_head_chirho}; size=\"3\""),
        ),
        (
            "an extended filename",
            format!("{currency_head_chirho}; filename*=\"x\""),
        ),
        (
            "an unquoted name",
            "Content-Disposition: form-data; name=currency_chirho".to_string(),
        ),
        (
            "a control character",
            format!("{currency_head_chirho}\u{1}"),
        ),
        (
            "a header block over 1 KiB",
            format!(
                "{currency_head_chirho}\r\nContent-Type: {}",
                "x".repeat(1100)
            ),
        ),
    ] {
        assert_eq!(
            parse_figures_upload_chirho(&replaced_chirho(
                &good_chirho,
                currency_head_chirho,
                &replacement_chirho
            )),
            Err(UnreadableChirho),
            "{label_chirho}"
        );
    }
    // one declared type is allowed, and is never authority for what the part holds
    let typed_chirho = replaced_chirho(
        &good_chirho,
        currency_head_chirho,
        &format!("{currency_head_chirho}\r\nContent-Type: image/png"),
    );
    assert_eq!(
        parse_figures_upload_chirho(&typed_chirho)
            .unwrap()
            .currency_chirho,
        "USD"
    );
}

#[test]
fn exactly_the_five_fields_each_once_chirho() {
    let parts_chirho = |extra_chirho: Option<PartSpecChirho<'static>>, skip_chirho: &str| {
        let mut list_chirho = vec![
            part_chirho("expected_version_chirho", b"0"),
            part_chirho("csv_chirho", b""),
            file_part_chirho("f.csv", CSV_CHIRHO),
            part_chirho("currency_chirho", b"USD"),
            part_chirho("note_chirho", b"n"),
        ];
        list_chirho.retain(|part_spec_chirho| part_spec_chirho.name_chirho != skip_chirho);
        list_chirho.extend(extra_chirho);
        body_chirho(&list_chirho)
    };
    assert!(parse_figures_upload_chirho(&parts_chirho(None, "")).is_ok());
    for (label_chirho, body_chirho_bytes) in [
        (
            "an unknown field",
            parts_chirho(Some(part_chirho("extra_chirho", b"1")), ""),
        ),
        (
            "a repeated field",
            parts_chirho(Some(part_chirho("currency_chirho", b"EUR")), ""),
        ),
        ("a missing field", parts_chirho(None, "note_chirho")),
        (
            "a filename on a text field",
            parts_chirho(
                Some(PartSpecChirho {
                    name_chirho: "note_chirho",
                    file_name_chirho: Some("n.txt"),
                    content_chirho: b"n",
                }),
                "note_chirho",
            ),
        ),
        (
            "the file part without a filename",
            parts_chirho(
                Some(part_chirho("csv_file_chirho", CSV_CHIRHO)),
                "csv_file_chirho",
            ),
        ),
        (
            "no file chosen, yet bytes in its part",
            parts_chirho(Some(file_part_chirho("", CSV_CHIRHO)), "csv_file_chirho"),
        ),
    ] {
        assert_eq!(
            parse_figures_upload_chirho(&body_chirho_bytes),
            Err(UnreadableChirho),
            "{label_chirho}"
        );
    }
}

/// The browser caller-key carrier (the contract:
/// spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md)
/// allows one more part, the engine's caller key, anywhere in the body. It is never a file, is no longer
/// than a short field, and is dropped unread, so her figures read the same.
#[test]
fn the_engines_caller_key_part_is_dropped_unread_at_most_once_chirho() {
    use perffection_workspace_chirho::page_edit_request_chirho::FORM_CALLER_KEY_FIELD_CHIRHO;
    let five_chirho = || {
        vec![
            part_chirho("expected_version_chirho", b"0"),
            part_chirho("csv_chirho", b""),
            file_part_chirho("f.csv", CSV_CHIRHO),
            part_chirho("currency_chirho", b"USD"),
            part_chirho("note_chirho", b"from my books"),
        ]
    };
    let plain_chirho = parse_figures_upload_chirho(&body_chirho(&five_chirho()));
    assert!(plain_chirho.is_ok());
    let longest_chirho = vec![b'k'; 1024];
    for value_chirho in [
        b"".as_slice(),
        b"Mx3kQ9-zR_7vT2nB8wY4pL6dF1gH5jK0aS3eU9iO2c".as_slice(),
        longest_chirho.as_slice(),
    ] {
        for at_chirho in [0, 2, 5] {
            let mut parts_chirho = five_chirho();
            parts_chirho.insert(
                at_chirho,
                part_chirho(FORM_CALLER_KEY_FIELD_CHIRHO, value_chirho),
            );
            assert_eq!(
                parse_figures_upload_chirho(&body_chirho(&parts_chirho)),
                plain_chirho,
                "a {}-byte key at part {at_chirho}",
                value_chirho.len()
            );
        }
    }
    let too_long_chirho = vec![b'k'; 1025];
    for (label_chirho, extra_chirho) in [
        (
            "a second key",
            vec![
                part_chirho(FORM_CALLER_KEY_FIELD_CHIRHO, b"a"),
                part_chirho(FORM_CALLER_KEY_FIELD_CHIRHO, b"b"),
            ],
        ),
        (
            "a key sent as a file",
            vec![PartSpecChirho {
                name_chirho: FORM_CALLER_KEY_FIELD_CHIRHO,
                file_name_chirho: Some("k.txt"),
                content_chirho: b"k",
            }],
        ),
        (
            "a key past a short field's bound",
            vec![part_chirho(FORM_CALLER_KEY_FIELD_CHIRHO, &too_long_chirho)],
        ),
    ] {
        let mut parts_chirho = five_chirho();
        parts_chirho.extend(extra_chirho);
        assert_eq!(
            parse_figures_upload_chirho(&body_chirho(&parts_chirho)),
            Err(UnreadableChirho),
            "{label_chirho}"
        );
    }
}

#[test]
fn the_chosen_source_is_bounded_before_it_is_read_and_must_be_text_chirho() {
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", b"")), b"")),
        Err(EmptyFileChirho)
    );
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", b"month,\xff\xfe")), b"")),
        Err(FileNotTextChirho)
    );
    // over 64 KiB is refused before decoding: these bytes are not even UTF-8
    let over_chirho = vec![0xffu8; 64 * 1024 + 1];
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", &over_chirho)), b"")),
        Err(TooLargeChirho)
    );
    let at_bound_chirho = vec![b'a'; 64 * 1024];
    assert!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", &at_bound_chirho)), b""))
            .is_ok()
    );
    let long_paste_chirho = vec![b'a'; 64 * 1024 + 1];
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(None, &long_paste_chirho)),
        Err(TooLargeChirho)
    );
    // the whole body is bounded first, however it is framed
    let huge_chirho = [
        format!("--{BOUNDARY_CHIRHO}\r\n").into_bytes(),
        vec![b'x'; 200 * 1024],
    ]
    .concat();
    assert_eq!(
        parse_figures_upload_chirho(&huge_chirho),
        Err(TooLargeChirho)
    );
    // one byte-order mark, as spreadsheets write it, is not part of her figures; a second one is
    let marked_chirho = [b"\xef\xbb\xbf".as_slice(), CSV_CHIRHO].concat();
    assert!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", &marked_chirho)), b""))
            .unwrap()
            .csv_chirho
            .starts_with("month,")
    );
    let twice_chirho = [b"\xef\xbb\xbf\xef\xbb\xbf".as_slice(), CSV_CHIRHO].concat();
    assert!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", &twice_chirho)), b""))
            .unwrap()
            .csv_chirho
            .starts_with('\u{feff}')
    );
    // short fields are bounded too, and the version is read as in the pasted form
    let long_note_chirho = body_chirho(&[
        part_chirho("expected_version_chirho", b"0"),
        part_chirho("csv_chirho", b""),
        file_part_chirho("f.csv", CSV_CHIRHO),
        part_chirho("currency_chirho", b"USD"),
        part_chirho("note_chirho", &[b'n'; 1025]),
    ]);
    assert_eq!(
        parse_figures_upload_chirho(&long_note_chirho),
        Err(UnreadableChirho)
    );
    let bad_version_chirho = replaced_chirho(
        &form_body_chirho(Some(("f.csv", CSV_CHIRHO)), b""),
        "name=\"expected_version_chirho\"\r\n\r\n0",
        "name=\"expected_version_chirho\"\r\n\r\n1x",
    );
    assert_eq!(
        parse_figures_upload_chirho(&bad_version_chirho),
        Err(UnreadableChirho)
    );
}

#[test]
fn a_file_and_a_paste_are_refused_before_either_is_read_chirho() {
    let paste_chirho = b"month,kind,money_in,costs,jobs\r\n2026-03,Pies,77,,\r\n";
    // even when the file could never be read, the pair is what is refused
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(
            Some(("f.csv", b"\xff\xfe")),
            paste_chirho
        )),
        Err(FileAndPasteChirho)
    );
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(
            Some(("f.csv", &vec![b'a'; 64 * 1024 + 1])),
            paste_chirho
        )),
        Err(FileAndPasteChirho)
    );
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", b"")), paste_chirho)),
        Err(FileAndPasteChirho)
    );
    // a box holding only whitespace holds no figures: the file is read
    let filed_chirho = parse_figures_upload_chirho(&form_body_chirho(
        Some(("f.csv", CSV_CHIRHO)),
        b"\r\n  \r\n",
    ))
    .unwrap();
    assert_eq!(filed_chirho.file_name_chirho.as_deref(), Some("f.csv"));
}

#[test]
fn a_boundary_looking_sequence_never_cuts_her_file_short_chirho() {
    let cut_chirho = [
        CSV_CHIRHO,
        format!("\r\n--{BOUNDARY_CHIRHO}X2026-02,Bread,5,,\r\n").as_bytes(),
    ]
    .concat();
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", &cut_chirho)), b"")),
        Err(UnreadableChirho)
    );
    let framed_chirho = [
        CSV_CHIRHO,
        format!("\r\n--{BOUNDARY_CHIRHO}\r\nnot a header\r\n\r\nx").as_bytes(),
    ]
    .concat();
    assert_eq!(
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", &framed_chirho)), b"")),
        Err(UnreadableChirho)
    );
    // A file whose bytes carry a delimiter-looking line and a field the form left out: read laxly,
    // her file would end early and the smuggled note would be taken. Read strictly, it is refused.
    let smuggling_chirho = [
        CSV_CHIRHO,
        format!(
            "\r\n--{BOUNDARY_CHIRHO}Content-Disposition: form-data; name=\"note_chirho\"\r\n\r\nsmuggled"
        )
        .as_bytes(),
    ]
    .concat();
    let without_note_chirho = body_chirho(&[
        part_chirho("expected_version_chirho", b"0"),
        part_chirho("csv_chirho", b""),
        file_part_chirho("f.csv", &smuggling_chirho),
        part_chirho("currency_chirho", b"USD"),
    ]);
    assert_eq!(
        parse_figures_upload_chirho(&without_note_chirho),
        Err(UnreadableChirho)
    );
    // control: the boundary's text inside a line, not at a line start, is just her text
    let inline_chirho = [
        CSV_CHIRHO.strip_suffix(b"\r\n").unwrap(),
        format!(" --{BOUNDARY_CHIRHO}\r\n").as_bytes(),
    ]
    .concat();
    let kept_chirho =
        parse_figures_upload_chirho(&form_body_chirho(Some(("f.csv", &inline_chirho)), b""))
            .unwrap();
    assert!(kept_chirho
        .csv_chirho
        .ends_with(&format!(" --{BOUNDARY_CHIRHO}\r\n")));
}

#[test]
fn the_review_reads_an_upload_into_the_same_fields_the_paste_carries_chirho() {
    let copy_chirho = WorkspaceCopyChirho::english_chirho();
    let review_chirho = String::from_utf8(review_figures_chirho(CHROME_FILE_ONLY_CHIRHO)).unwrap();
    assert!(review_chirho.contains("<th scope=\"row\">Café</th>"));
    assert!(review_chirho.contains(&copy_chirho.figures_chirho.from_file_label_chirho));
    assert!(review_chirho.contains("<strong>upload-figures.csv</strong>"));
    // the Save posts the reviewed CSV text as the pasted form does: no file, no file name
    let save_form_chirho = &review_chirho[review_chirho.find("save-figures-chirho").unwrap()..];
    assert!(save_form_chirho.contains("name=\"csv_chirho\" value=\"month,kind,money_in,costs,jobs"));
    assert!(!save_form_chirho.contains("csv_file_chirho"));
    assert!(!save_form_chirho.contains("upload-figures.csv"));
    // her file name is text, never markup
    let marked_up_chirho =
        review_figures_chirho(&form_body_chirho(Some(("<b>x</b>.csv", CSV_CHIRHO)), b""));
    let marked_up_chirho = String::from_utf8(marked_up_chirho).unwrap();
    assert!(!marked_up_chirho.contains("<b>x</b>"));
    // a refused upload is the review with its reason and no Save
    let refused_chirho =
        String::from_utf8(review_figures_chirho(CHROME_FILE_AND_PASTE_CHIRHO)).unwrap();
    assert!(refused_chirho.contains(&copy_chirho.figures_chirho.refusal_file_and_paste_chirho));
    assert!(refused_chirho.contains("data-refusal-chirho=\"file-and-paste-chirho\""));
    assert!(!refused_chirho.contains("save-figures-chirho\""));
    // the pasted form still reads as ever
    let pasted_chirho = format!(
        "csv_chirho={}&currency_chirho=USD&note_chirho=n&expected_version_chirho=0",
        "month%2Ckind%2Cmoney_in%2Ccosts%2Cjobs%0A2026-01%2CBread%2C5%2C%2C%0A"
    );
    assert!(read_figures_review_form_chirho(pasted_chirho.as_bytes()).is_ok());
}

#[test]
fn the_save_reads_only_the_pasted_form_chirho() {
    // the review's Save button posts the pasted form; an upload posted to the save writes nothing
    assert_eq!(
        classify_figures_save_chirho(CHROME_FILE_ONLY_CHIRHO),
        FiguresSaveDecisionChirho::UnreadableChirho
    );
    assert_eq!(
        build_figures_write_chirho(CHROME_FILE_ONLY_CHIRHO),
        cairn_node_sdk_chirho::skipped_bytes_chirho()
    );
}
