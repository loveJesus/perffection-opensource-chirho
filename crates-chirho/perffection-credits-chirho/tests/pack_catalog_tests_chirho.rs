// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The starting packs keep the money rule, not particular prices (L.J. may edit the list): every pack costs exactly its
//! credits in cents, so no credit exists that cash does not reserve; ids are distinct; and a pack is credited only at
//! its exact price.

use perffection_credits_chirho::pack_catalog_chirho::{
    pack_id_for_credits_chirho, PackCatalogChirho, PackRefusalChirho, CENTS_PER_CREDIT_CHIRHO,
    PACK_CURRENCY_CHIRHO, STARTING_PACK_CREDITS_CHIRHO,
};

#[test]
fn every_starting_pack_costs_exactly_its_credits_in_cents_chirho() {
    let catalog_chirho = PackCatalogChirho::starting_chirho();
    let packs_chirho: Vec<_> = catalog_chirho.packs_chirho().collect();
    assert_eq!(
        packs_chirho.len(),
        STARTING_PACK_CREDITS_CHIRHO.len(),
        "ids are distinct"
    );
    for (pack_id_chirho, pack_chirho) in packs_chirho {
        assert_eq!(
            pack_id_chirho,
            pack_id_for_credits_chirho(pack_chirho.credits_chirho)
        );
        assert_eq!(
            pack_chirho.amount_minor_chirho,
            pack_chirho.credits_chirho * CENTS_PER_CREDIT_CHIRHO
        );
        assert_eq!(pack_chirho.currency_chirho, PACK_CURRENCY_CHIRHO);
        assert!(pack_chirho.credits_chirho > 0);
    }
}

#[test]
fn a_starting_pack_credits_only_at_its_exact_price_chirho() {
    let catalog_chirho = PackCatalogChirho::starting_chirho();
    for credits_chirho in STARTING_PACK_CREDITS_CHIRHO {
        let pack_id_chirho = pack_id_for_credits_chirho(credits_chirho);
        let price_chirho = credits_chirho * CENTS_PER_CREDIT_CHIRHO;
        assert_eq!(
            catalog_chirho.credits_for_chirho(&pack_id_chirho, price_chirho, PACK_CURRENCY_CHIRHO),
            Ok(credits_chirho)
        );
        assert_eq!(
            catalog_chirho.credits_for_chirho(
                &pack_id_chirho,
                price_chirho - 1,
                PACK_CURRENCY_CHIRHO
            ),
            Err(PackRefusalChirho::PriceMismatchChirho)
        );
        assert_eq!(
            catalog_chirho.credits_for_chirho(&pack_id_chirho, price_chirho, "eur"),
            Err(PackRefusalChirho::PriceMismatchChirho)
        );
    }
    assert_eq!(
        catalog_chirho.credits_for_chirho("credits-999-chirho", 999, PACK_CURRENCY_CHIRHO),
        Err(PackRefusalChirho::UnknownPackChirho)
    );
}
