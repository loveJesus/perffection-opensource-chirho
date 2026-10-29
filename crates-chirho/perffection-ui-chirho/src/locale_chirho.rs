// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The deliberately small locale boundary for product-authored copy.
//!
//! English is the only reviewed release catalog. The RTL pseudolocale is an explicit test and
//! authoring aid; it is not a translated release and is never selected from an untrusted request.

use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum UiLocaleChirho {
    #[default]
    EnglishChirho,
    PseudolocaleRtlChirho,
}

impl UiLocaleChirho {
    pub const fn lang_chirho(self) -> &'static str {
        match self {
            Self::EnglishChirho => "en",
            Self::PseudolocaleRtlChirho => "ar-XB",
        }
    }

    pub const fn dir_chirho(self) -> &'static str {
        match self {
            Self::EnglishChirho => "ltr",
            Self::PseudolocaleRtlChirho => "rtl",
        }
    }
}

/// Copy names the new-password minimum through this placeholder, so the number has one source.
pub(crate) const MIN_CHARACTERS_PLACEHOLDER_CHIRHO: &str = "{min_characters_chirho}";

/// Copy names the sign-in wait through this placeholder, so the minutes have one source.
pub(crate) const WAIT_MINUTES_PLACEHOLDER_CHIRHO: &str = "{wait_minutes_chirho}";

pub(crate) fn load_catalog_chirho<CatalogChirho>(
    english_catalog_chirho: &str,
    locale_chirho: UiLocaleChirho,
) -> Result<CatalogChirho, serde_json::Error>
where
    CatalogChirho: DeserializeOwned,
{
    let filled_chirho = english_catalog_chirho
        .replace(
            MIN_CHARACTERS_PLACEHOLDER_CHIRHO,
            &crate::NEW_PASSWORD_MIN_CHARACTERS_CHIRHO.to_string(),
        )
        .replace(
            WAIT_MINUTES_PLACEHOLDER_CHIRHO,
            &crate::SIGN_IN_WAIT_MINUTES_CHIRHO.to_string(),
        );
    let english_catalog_chirho = filled_chirho.as_str();
    if locale_chirho == UiLocaleChirho::EnglishChirho {
        return serde_json::from_str(english_catalog_chirho);
    }

    let mut catalog_value_chirho: Value = serde_json::from_str(english_catalog_chirho)?;
    pseudolocalize_catalog_value_chirho(&mut catalog_value_chirho);
    if let Some(catalog_object_chirho) = catalog_value_chirho.as_object_mut() {
        catalog_object_chirho.insert(
            "lang_chirho".to_string(),
            Value::String(locale_chirho.lang_chirho().to_string()),
        );
        catalog_object_chirho.insert(
            "dir_chirho".to_string(),
            Value::String(locale_chirho.dir_chirho().to_string()),
        );
    }
    serde_json::from_value(catalog_value_chirho)
}

fn pseudolocalize_catalog_value_chirho(value_chirho: &mut Value) {
    match value_chirho {
        Value::String(text_chirho) => {
            *text_chirho = pseudolocalize_text_chirho(text_chirho);
        }
        Value::Array(items_chirho) => {
            for item_chirho in items_chirho {
                pseudolocalize_catalog_value_chirho(item_chirho);
            }
        }
        Value::Object(fields_chirho) => {
            for field_value_chirho in fields_chirho.values_mut() {
                pseudolocalize_catalog_value_chirho(field_value_chirho);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

/// Makes untranslated copy conspicuous and longer while preserving named interpolation tokens.
pub fn pseudolocalize_text_chirho(text_chirho: &str) -> String {
    let mut output_chirho = String::with_capacity(text_chirho.len() * 2 + 8);
    output_chirho.push_str("⟦!! ");
    let mut token_depth_chirho = 0_u8;

    for character_chirho in text_chirho.chars() {
        if character_chirho == '{' {
            token_depth_chirho = token_depth_chirho.saturating_add(1);
            output_chirho.push(character_chirho);
            continue;
        }
        if character_chirho == '}' && token_depth_chirho > 0 {
            token_depth_chirho -= 1;
            output_chirho.push(character_chirho);
            continue;
        }
        if token_depth_chirho > 0 {
            output_chirho.push(character_chirho);
            continue;
        }

        match character_chirho {
            'a' => output_chirho.push_str("áa"),
            'A' => output_chirho.push_str("ÁA"),
            'e' => output_chirho.push_str("ée"),
            'E' => output_chirho.push_str("ÉE"),
            'i' => output_chirho.push_str("íi"),
            'I' => output_chirho.push_str("ÍI"),
            'o' => output_chirho.push_str("óo"),
            'O' => output_chirho.push_str("ÓO"),
            'u' => output_chirho.push_str("úu"),
            'U' => output_chirho.push_str("ÚU"),
            _ => output_chirho.push(character_chirho),
        }
    }
    output_chirho.push_str(" !!⟧");
    output_chirho
}
