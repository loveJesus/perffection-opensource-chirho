// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The credit packs: what each costs and how many credits it grants. One credit is one US cent, fully cash-backed
//! (`docs-chirho/business-chirho/pricing-chirho.md`), so a pack's price IS its credits in cents and no pack can grant a
//! credit that no cash reserves: there are no bonus credits. A paid session must match its pack exactly; a different
//! amount or currency is refused rather than credited by guess.
//! Workflow: spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md

use std::collections::BTreeMap;

/// One credit is one US cent (pricing-chirho.md: "1 credit = 1¢ (USD), fully cash-backed").
pub const CENTS_PER_CREDIT_CHIRHO: u64 = 1;
pub const PACK_CURRENCY_CHIRHO: &str = "usd";
/// The starting packs: $10, $25 and $50. L.J., 2026-10-04: "packs choose something sensible for us to start we can edit
/// it a bit later". Editing means changing this list; each id names its credits, so a changed pack is a new id and a
/// past purchase never reads as a different pack.
pub const STARTING_PACK_CREDITS_CHIRHO: [u64; 3] = [1_000, 2_500, 5_000];

/// A pack's id from its credits: `credits-1000-chirho`.
pub fn pack_id_for_credits_chirho(credits_chirho: u64) -> String {
    format!("credits-{credits_chirho}-chirho")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditPackChirho {
    pub amount_minor_chirho: u64,
    pub currency_chirho: String,
    pub credits_chirho: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackCatalogChirho {
    packs_chirho: BTreeMap<String, CreditPackChirho>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackRefusalChirho {
    UnknownPackChirho,
    /// The session's amount or currency is not the pack's price.
    PriceMismatchChirho,
}

impl PackCatalogChirho {
    /// The starting catalog: every pack priced at exactly its credits in cents.
    pub fn starting_chirho() -> Self {
        STARTING_PACK_CREDITS_CHIRHO.iter().fold(
            Self::default(),
            |catalog_chirho, credits_chirho| {
                catalog_chirho.with_pack_chirho(
                    &pack_id_for_credits_chirho(*credits_chirho),
                    CreditPackChirho {
                        amount_minor_chirho: credits_chirho * CENTS_PER_CREDIT_CHIRHO,
                        currency_chirho: PACK_CURRENCY_CHIRHO.to_string(),
                        credits_chirho: *credits_chirho,
                    },
                )
            },
        )
    }

    /// The packs by id, for the Credits page and the checkout.
    pub fn packs_chirho(&self) -> impl Iterator<Item = (&str, &CreditPackChirho)> {
        self.packs_chirho
            .iter()
            .map(|(pack_id_chirho, pack_chirho)| (pack_id_chirho.as_str(), pack_chirho))
    }

    pub fn with_pack_chirho(mut self, pack_id_chirho: &str, pack_chirho: CreditPackChirho) -> Self {
        self.packs_chirho
            .insert(pack_id_chirho.to_string(), pack_chirho);
        self
    }

    /// The pack's credits, if the paid amount and currency are exactly its price.
    pub fn credits_for_chirho(
        &self,
        pack_id_chirho: &str,
        amount_minor_chirho: u64,
        currency_chirho: &str,
    ) -> Result<u64, PackRefusalChirho> {
        let pack_chirho = self
            .packs_chirho
            .get(pack_id_chirho)
            .ok_or(PackRefusalChirho::UnknownPackChirho)?;
        if pack_chirho.amount_minor_chirho != amount_minor_chirho
            || pack_chirho.currency_chirho != currency_chirho
        {
            return Err(PackRefusalChirho::PriceMismatchChirho);
        }
        Ok(pack_chirho.credits_chirho)
    }
}
