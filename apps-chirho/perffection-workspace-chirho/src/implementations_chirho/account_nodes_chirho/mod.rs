// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The Account page's nodes. `render_account_chirho` is the deployed, held page; the own-keys read
//! (`build_own_keys_read_chirho`, then the blessed read-store card at `own-keys-chirho/`, then
//! `render_account_keys_chirho`) is the bound page, staged only where the engine carries K3; and
//! `render_account_connect_chirho` is that page with the entry to the engine's connection form (K4), bound separately.

pub mod build_own_keys_read_chirho;
pub mod render_account_chirho;
pub mod render_account_connect_chirho;
pub mod render_account_keys_chirho;
