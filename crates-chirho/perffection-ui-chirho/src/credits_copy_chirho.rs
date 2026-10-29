// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Credits page's display copy, and the two return pages after Stripe's hosted Checkout
//! (`spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md`). Numbers are filled in at render time
//! through `{count_chirho}`, `{credits_chirho}` and `{price_chirho}`, in whole units only. The words are a draft for
//! L.J. to settle.

use crate::{locale_chirho::load_catalog_chirho, UiLocaleChirho};
use serde::{Deserialize, Serialize};

const COUNT_PLACEHOLDER_CHIRHO: &str = "{count_chirho}";
const CREDITS_PLACEHOLDER_CHIRHO: &str = "{credits_chirho}";
const PRICE_PLACEHOLDER_CHIRHO: &str = "{price_chirho}";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreditsCopyChirho {
    pub lang_chirho: String,
    pub dir_chirho: String,
    pub page_title_chirho: String,
    pub page_description_chirho: String,
    pub eyebrow_chirho: String,
    pub heading_chirho: String,
    pub lede_chirho: String,
    /// The panels' numbers, the way the Account page numbers its own.
    pub balance_number_chirho: String,
    pub balance_heading_chirho: String,
    pub balance_one_template_chirho: String,
    pub balance_many_template_chirho: String,
    /// Shown whenever the ledger's answer is unavailable. It is never a balance.
    pub unavailable_heading_chirho: String,
    pub unavailable_body_chirho: String,
    pub packs_number_chirho: String,
    pub packs_heading_chirho: String,
    pub packs_intro_chirho: String,
    pub pack_template_chirho: String,
    /// Shown in place of any purchase control while purchases are held.
    pub purchases_held_chirho: String,
    pub history_number_chirho: String,
    pub history_heading_chirho: String,
    pub history_empty_chirho: String,
    pub history_unavailable_chirho: String,
    pub history_more_template_chirho: String,
    pub entry_purchase_chirho: String,
    pub entry_grant_chirho: String,
    pub entry_spend_chirho: String,
    pub entry_return_chirho: String,
    pub entry_in_template_chirho: String,
    pub entry_out_template_chirho: String,
    pub back_label_chirho: String,
    /// The return pages prove nothing: the paid one never says credits arrived, only that they appear once confirmed.
    pub paid_page_title_chirho: String,
    pub paid_heading_chirho: String,
    pub paid_body_chirho: String,
    pub cancelled_page_title_chirho: String,
    pub cancelled_heading_chirho: String,
    pub cancelled_body_chirho: String,
    pub return_link_chirho: String,
}

impl CreditsCopyChirho {
    pub fn english_chirho() -> Self {
        Self::for_locale_chirho(UiLocaleChirho::EnglishChirho)
    }

    pub fn for_locale_chirho(locale_chirho: UiLocaleChirho) -> Self {
        load_catalog_chirho(
            include_str!("../locales_chirho/credits-chirho/en-chirho.json"),
            locale_chirho,
        )
        .expect("the embedded typed credits catalog must remain valid")
    }

    /// Her balance in words: "1 credit", "1,250 credits".
    pub fn balance_chirho(&self, credits_chirho: u64) -> String {
        let template_chirho = if credits_chirho == 1 {
            &self.balance_one_template_chirho
        } else {
            &self.balance_many_template_chirho
        };
        template_chirho.replace(
            COUNT_PLACEHOLDER_CHIRHO,
            &group_thousands_chirho(credits_chirho),
        )
    }

    /// One pack: its credits and its price in US dollars.
    pub fn pack_chirho(&self, credits_chirho: u64, amount_cents_chirho: u64) -> String {
        self.pack_template_chirho
            .replace(
                CREDITS_PLACEHOLDER_CHIRHO,
                &group_thousands_chirho(credits_chirho),
            )
            .replace(
                PRICE_PLACEHOLDER_CHIRHO,
                &dollars_from_cents_chirho(amount_cents_chirho),
            )
    }

    /// The note under a history that holds only her latest entries.
    pub fn history_more_chirho(&self, shown_chirho: usize) -> String {
        self.history_more_template_chirho
            .replace(COUNT_PLACEHOLDER_CHIRHO, &shown_chirho.to_string())
    }

    /// One entry's amount, signed by which way it moved her credits.
    pub fn entry_amount_chirho(&self, adds_chirho: bool, credits_chirho: u64) -> String {
        let template_chirho = if adds_chirho {
            &self.entry_in_template_chirho
        } else {
            &self.entry_out_template_chirho
        };
        template_chirho.replace(
            COUNT_PLACEHOLDER_CHIRHO,
            &group_thousands_chirho(credits_chirho),
        )
    }
}

/// A whole number grouped by thousands, the English way: 1,250.
fn group_thousands_chirho(value_chirho: u64) -> String {
    let digits_chirho = value_chirho.to_string();
    let mut grouped_chirho = String::with_capacity(digits_chirho.len() + digits_chirho.len() / 3);
    for (index_chirho, digit_chirho) in digits_chirho.chars().enumerate() {
        if index_chirho > 0 && (digits_chirho.len() - index_chirho).is_multiple_of(3) {
            grouped_chirho.push(',');
        }
        grouped_chirho.push(digit_chirho);
    }
    grouped_chirho
}

/// US dollars from whole cents, with no floating point: $10, or $10.50 when there are cents.
fn dollars_from_cents_chirho(cents_chirho: u64) -> String {
    let dollars_chirho = group_thousands_chirho(cents_chirho / 100);
    match cents_chirho % 100 {
        0 => format!("${dollars_chirho}"),
        rest_chirho => format!("${dollars_chirho}.{rest_chirho:02}"),
    }
}
