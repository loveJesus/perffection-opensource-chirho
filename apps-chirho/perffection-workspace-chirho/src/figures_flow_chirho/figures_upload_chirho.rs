// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Her figures as a chosen CSV file: the review form posted as multipart/form-data, read by one strict
//! grammar into the same four fields the pasted form carries (gpt #26599, choice A).
//!
//! Workflow: `spec-chirho/workflows-chirho/invite-beta-chirho/her-figures-chirho.md`. The engine hands
//! this guest the request body alone, with no declared media type and so no boundary (measured in
//! `spec-chirho/evidence-chirho/agent-authoring-chirho/26-10-08-upload-carrier-measured-chirho/`). The
//! boundary is therefore read from the body's own first line. That is a product wire grammar inferred
//! from the bytes, NOT proof of the declared HTTP media type; once Cairn carries the declared type, a
//! mismatch refuses.
//!
//! All or nothing, bounded before anything is decoded:
//! - the whole body at most 200 KiB;
//! - it opens with `--`, a boundary of 1 to 70 RFC 2046 characters, then CRLF: no preamble;
//! - every later delimiter is CRLF `--` boundary, followed by CRLF (another part) or `--` (the close),
//!   and after the close only an optional CRLF. A boundary-looking sequence followed by anything else
//!   is broken framing, refused, never a place to cut her file short;
//! - each part: header lines `Content-Disposition` (once, `form-data` with `name` and, on the file
//!   part only, `filename`) and at most one `Content-Type`, which is never authority, then a blank
//!   line and the content;
//! - exactly the five fields, each once: the file, the paste box, currency, note and version. Besides
//!   them, at most one non-file part may carry the engine's caller key (FORM_CALLER_KEY_FIELD_CHIRHO),
//!   which is dropped unread. Workflow:
//!   `spec-chirho/workflows-chirho/platform-route-architecture-chirho/browser-caller-key-carrier-chirho.md`.
//!
//! A chosen file and a non-empty paste are refused before either is read. No file chosen is the
//! browser's empty part with an empty filename, so the paste is read as ever. A chosen file must be
//! 1 byte to 64 KiB of UTF-8; one leading byte-order mark, as spreadsheets write it, is not part of
//! her figures. The file name is display only.

use crate::figures_flow_chirho::figures_csv_chirho::FIGURES_CSV_MAX_BYTES_CHIRHO;
use crate::figures_flow_chirho::figures_form_chirho::{
    expected_version_from_chirho, FiguresFormChirho, FiguresFormRefusalChirho,
    FIGURES_FORM_MAX_BYTES_CHIRHO,
};
use crate::page_edit_request_chirho::FORM_CALLER_KEY_FIELD_CHIRHO;

/// The five parts, in the order the figures form sends them; the first is the file.
pub const FIGURES_UPLOAD_FIELDS_CHIRHO: [&str; 5] = [
    "csv_file_chirho",
    "csv_chirho",
    "currency_chirho",
    "note_chirho",
    "expected_version_chirho",
];
const BOUNDARY_MAX_CHARS_CHIRHO: usize = 70;
const PART_HEADERS_MAX_BYTES_CHIRHO: usize = 1024;
const SHORT_FIELD_MAX_BYTES_CHIRHO: usize = 1024;
const FILE_NAME_MAX_BYTES_CHIRHO: usize = 255;

struct PartChirho<'a> {
    name_chirho: &'a str,
    file_name_chirho: Option<&'a str>,
    content_chirho: &'a [u8],
}

pub fn parse_figures_upload_chirho(
    body_chirho: &[u8],
) -> Result<FiguresFormChirho, FiguresFormRefusalChirho> {
    use FiguresFormRefusalChirho::{
        EmptyFileChirho, FileAndPasteChirho, FileNotTextChirho, TooLargeChirho, UnreadableChirho,
    };
    if body_chirho.len() > FIGURES_FORM_MAX_BYTES_CHIRHO {
        return Err(TooLargeChirho);
    }
    let boundary_chirho = opening_boundary_chirho(body_chirho).ok_or(UnreadableChirho)?;
    let delimiter_chirho = [b"\r\n--".as_slice(), boundary_chirho].concat();
    let mut rest_chirho = &body_chirho[2 + boundary_chirho.len() + 2..];
    let mut fields_chirho: [Option<PartChirho>; 5] = Default::default();
    let mut caller_key_seen_chirho = false;
    loop {
        let end_chirho = find_chirho(rest_chirho, &delimiter_chirho).ok_or(UnreadableChirho)?;
        let part_chirho = read_part_chirho(&rest_chirho[..end_chirho])?;
        if part_chirho.name_chirho == FORM_CALLER_KEY_FIELD_CHIRHO {
            // The engine's caller key (the browser caller-key carrier) is at most once, never a file,
            // and bounded like a short field. It is then dropped unread, as the urlencoded reader
            // drops it.
            if caller_key_seen_chirho
                || part_chirho.file_name_chirho.is_some()
                || part_chirho.content_chirho.len() > SHORT_FIELD_MAX_BYTES_CHIRHO
            {
                return Err(UnreadableChirho);
            }
            caller_key_seen_chirho = true;
        } else {
            let index_chirho = FIGURES_UPLOAD_FIELDS_CHIRHO
                .iter()
                .position(|field_chirho| *field_chirho == part_chirho.name_chirho)
                .ok_or(UnreadableChirho)?;
            // the file part, and only the file part, carries a filename
            if (index_chirho == 0) != part_chirho.file_name_chirho.is_some() {
                return Err(UnreadableChirho);
            }
            if fields_chirho[index_chirho].replace(part_chirho).is_some() {
                return Err(UnreadableChirho);
            }
        }
        let after_chirho = &rest_chirho[end_chirho + delimiter_chirho.len()..];
        if let Some(tail_chirho) = after_chirho.strip_prefix(b"--") {
            if !(tail_chirho.is_empty() || tail_chirho == b"\r\n") {
                return Err(UnreadableChirho);
            }
            break;
        }
        rest_chirho = after_chirho.strip_prefix(b"\r\n").ok_or(UnreadableChirho)?;
    }
    let [Some(file_chirho), Some(paste_chirho), Some(currency_chirho), Some(note_chirho), Some(version_chirho)] =
        fields_chirho
    else {
        return Err(UnreadableChirho);
    };

    // Which source, before either is read: a chosen file and a non-empty paste are refused together.
    let file_name_chirho = file_chirho.file_name_chirho.unwrap_or_default();
    let file_chosen_chirho = !file_name_chirho.is_empty();
    if !file_chosen_chirho && !file_chirho.content_chirho.is_empty() {
        return Err(UnreadableChirho);
    }
    let paste_given_chirho = paste_chirho
        .content_chirho
        .iter()
        .any(|byte_chirho| !byte_chirho.is_ascii_whitespace());
    if file_chosen_chirho && paste_given_chirho {
        return Err(FileAndPasteChirho);
    }
    let csv_chirho = if file_chosen_chirho {
        if file_chirho.content_chirho.is_empty() {
            return Err(EmptyFileChirho);
        }
        if file_chirho.content_chirho.len() > FIGURES_CSV_MAX_BYTES_CHIRHO {
            return Err(TooLargeChirho);
        }
        let text_chirho =
            std::str::from_utf8(file_chirho.content_chirho).map_err(|_| FileNotTextChirho)?;
        text_chirho.strip_prefix('\u{feff}').unwrap_or(text_chirho)
    } else {
        if paste_chirho.content_chirho.len() > FIGURES_CSV_MAX_BYTES_CHIRHO {
            return Err(TooLargeChirho);
        }
        std::str::from_utf8(paste_chirho.content_chirho).map_err(|_| UnreadableChirho)?
    };
    Ok(FiguresFormChirho {
        csv_chirho: csv_chirho.to_string(),
        currency_chirho: short_field_chirho(&currency_chirho)?.trim().to_string(),
        note_chirho: short_field_chirho(&note_chirho)?.trim().to_string(),
        expected_version_chirho: expected_version_from_chirho(short_field_chirho(&version_chirho)?)
            .ok_or(UnreadableChirho)?,
        file_name_chirho: file_chosen_chirho.then(|| file_name_chirho.to_string()),
    })
}

/// A short field (currency, note, version): bounded before it is decoded, then UTF-8.
fn short_field_chirho<'body_chirho>(
    part_chirho: &PartChirho<'body_chirho>,
) -> Result<&'body_chirho str, FiguresFormRefusalChirho> {
    if part_chirho.content_chirho.len() > SHORT_FIELD_MAX_BYTES_CHIRHO {
        return Err(FiguresFormRefusalChirho::UnreadableChirho);
    }
    std::str::from_utf8(part_chirho.content_chirho)
        .map_err(|_| FiguresFormRefusalChirho::UnreadableChirho)
}

/// The boundary from the body's first line: `--`, 1 to 70 RFC 2046 characters (the last not a space),
/// then CRLF.
fn opening_boundary_chirho(body_chirho: &[u8]) -> Option<&[u8]> {
    let line_end_chirho = find_chirho(body_chirho, b"\r\n")?;
    let boundary_chirho = body_chirho[..line_end_chirho].strip_prefix(b"--")?;
    let allowed_chirho = |byte_chirho: &u8| {
        byte_chirho.is_ascii_alphanumeric() || b"'()+_,-./:=? ".contains(byte_chirho)
    };
    (!boundary_chirho.is_empty()
        && boundary_chirho.len() <= BOUNDARY_MAX_CHARS_CHIRHO
        && boundary_chirho.iter().all(allowed_chirho)
        && boundary_chirho.last() != Some(&b' '))
    .then_some(boundary_chirho)
}

/// One part: its header lines, a blank line, then everything up to the next delimiter as its content.
fn read_part_chirho(bytes_chirho: &[u8]) -> Result<PartChirho<'_>, FiguresFormRefusalChirho> {
    use FiguresFormRefusalChirho::UnreadableChirho;
    let head_end_chirho = find_chirho(bytes_chirho, b"\r\n\r\n").ok_or(UnreadableChirho)?;
    if head_end_chirho > PART_HEADERS_MAX_BYTES_CHIRHO {
        return Err(UnreadableChirho);
    }
    let head_chirho =
        std::str::from_utf8(&bytes_chirho[..head_end_chirho]).map_err(|_| UnreadableChirho)?;
    let token_chirho = |byte_chirho: u8| {
        byte_chirho.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte_chirho)
    };
    let mut disposition_chirho = None;
    let mut typed_chirho = false;
    for line_chirho in head_chirho.split("\r\n") {
        let (field_chirho, value_chirho) = line_chirho.split_once(':').ok_or(UnreadableChirho)?;
        if field_chirho.is_empty()
            || !field_chirho.bytes().all(token_chirho)
            || value_chirho
                .chars()
                .any(|char_chirho| char_chirho.is_control() && char_chirho != '\t')
        {
            return Err(UnreadableChirho);
        }
        let value_chirho = value_chirho.trim_matches([' ', '\t']);
        if field_chirho.eq_ignore_ascii_case("content-disposition") {
            if disposition_chirho.replace(value_chirho).is_some() {
                return Err(UnreadableChirho);
            }
        } else if field_chirho.eq_ignore_ascii_case("content-type") {
            // display-only information from her browser: never authority for what the part holds
            if std::mem::replace(&mut typed_chirho, true) {
                return Err(UnreadableChirho);
            }
        } else {
            return Err(UnreadableChirho);
        }
    }
    let (name_chirho, file_name_chirho) =
        disposition_params_chirho(disposition_chirho.ok_or(UnreadableChirho)?)?;
    Ok(PartChirho {
        name_chirho,
        file_name_chirho,
        content_chirho: &bytes_chirho[head_end_chirho + 4..],
    })
}

/// `form-data; name="..."` and, optionally, `; filename="..."`: each once, nothing else. Quoted
/// values are taken as sent (browsers percent-encode a quote, CR and LF in them).
fn disposition_params_chirho(
    value_chirho: &str,
) -> Result<(&str, Option<&str>), FiguresFormRefusalChirho> {
    use FiguresFormRefusalChirho::UnreadableChirho;
    let kind_end_chirho = value_chirho.find(';').unwrap_or(value_chirho.len());
    if !value_chirho[..kind_end_chirho]
        .trim_end_matches([' ', '\t'])
        .eq_ignore_ascii_case("form-data")
    {
        return Err(UnreadableChirho);
    }
    let mut cursor_chirho = &value_chirho[kind_end_chirho..];
    let (mut name_chirho, mut file_name_chirho) = (None, None);
    loop {
        cursor_chirho = cursor_chirho.trim_start_matches([' ', '\t']);
        if cursor_chirho.is_empty() {
            break;
        }
        cursor_chirho = cursor_chirho
            .strip_prefix(';')
            .ok_or(UnreadableChirho)?
            .trim_start_matches([' ', '\t']);
        let (key_chirho, after_chirho) = cursor_chirho.split_once('=').ok_or(UnreadableChirho)?;
        let (quoted_chirho, after_chirho) = after_chirho
            .strip_prefix('"')
            .and_then(|inner_chirho| inner_chirho.split_once('"'))
            .ok_or(UnreadableChirho)?;
        let slot_chirho = if key_chirho.eq_ignore_ascii_case("name") {
            &mut name_chirho
        } else if key_chirho.eq_ignore_ascii_case("filename") {
            if quoted_chirho.len() > FILE_NAME_MAX_BYTES_CHIRHO {
                return Err(UnreadableChirho);
            }
            &mut file_name_chirho
        } else {
            return Err(UnreadableChirho);
        };
        if slot_chirho.replace(quoted_chirho).is_some() {
            return Err(UnreadableChirho);
        }
        cursor_chirho = after_chirho;
    }
    Ok((name_chirho.ok_or(UnreadableChirho)?, file_name_chirho))
}

/// The first place `needle` occurs in `haystack`, in linear time (Knuth-Morris-Pratt), so even a body
/// built to defeat a naive search costs the guest no more than one pass.
fn find_chirho(haystack_chirho: &[u8], needle_chirho: &[u8]) -> Option<usize> {
    if needle_chirho.is_empty() || needle_chirho.len() > haystack_chirho.len() {
        return None;
    }
    let mut fallback_chirho = vec![0usize; needle_chirho.len()];
    let mut matched_chirho = 0;
    for index_chirho in 1..needle_chirho.len() {
        while matched_chirho > 0 && needle_chirho[index_chirho] != needle_chirho[matched_chirho] {
            matched_chirho = fallback_chirho[matched_chirho - 1];
        }
        if needle_chirho[index_chirho] == needle_chirho[matched_chirho] {
            matched_chirho += 1;
        }
        fallback_chirho[index_chirho] = matched_chirho;
    }
    matched_chirho = 0;
    for (index_chirho, byte_chirho) in haystack_chirho.iter().enumerate() {
        while matched_chirho > 0 && *byte_chirho != needle_chirho[matched_chirho] {
            matched_chirho = fallback_chirho[matched_chirho - 1];
        }
        if *byte_chirho == needle_chirho[matched_chirho] {
            matched_chirho += 1;
            if matched_chirho == needle_chirho.len() {
                return Some(index_chirho + 1 - needle_chirho.len());
            }
        }
    }
    None
}
