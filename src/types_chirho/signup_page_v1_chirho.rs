// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Typed public copy model for the sign-up page.
//!
//! The *page* renders no form: account creation runs through the engine's generic
//! `/auth-chirho/register-chirho` (open math-captcha human check, then an HttpOnly cookie).
//!
//! The *route*, however, is mounted `get(..).post(..)` like every app route, so anyone can POST a
//! body to it whether or not we render a form. Under the `Plain` input policy that body is
//! content-addressed into CAS **as plaintext**, permanently — verified empirically. That is exactly
//! what `app-verify`'s PII warning was pointing at, and it was NOT a false positive.
//!
//! So this route declares `private-blob-ref` with an app subject (`app-chirho.toml`). An anonymous
//! GET still renders; an anonymous POST body is no longer persisted as plaintext. The pure render
//! ignores the input either way.

use crate::types_chirho::site_chrome_chirho::{ChromeNavLinkChirho, SiteChromeChirho};
use serde::{Deserialize, Serialize};

/// Engine auth contract — do not rename.
pub const REGISTER_HREF_CHIRHO: &str = "/auth-chirho/register-chirho";

const FOOTER_NOTE_CHIRHO: &str = "perffection · built on cairn · to God be the glory";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignupPageV1Chirho {
    pub chrome_chirho: SiteChromeChirho,
    pub heading_chirho: String,
    pub intro_chirho: String,
    pub primary_cta_chirho: String,
    pub primary_cta_href_chirho: String,
    pub human_check_note_chirho: String,
    pub alt_prompt_chirho: String,
    pub alt_link_label_chirho: String,
    pub alt_link_href_chirho: String,
}

impl SignupPageV1Chirho {
    /// The reviewed public copy for the sign-up page.
    pub fn default_public_chirho() -> Self {
        Self {
            chrome_chirho: SiteChromeChirho {
                page_title_chirho: "Sign up — perffection".to_string(),
                page_description_chirho: "Public sign-up is closed in this release candidate."
                    .to_string(),
                stylesheet_name_chirho: "account-chirho.css".to_string(),
                preview_marker_chirho: "Development preview".to_string(),
                brand_chirho: "perffection".to_string(),
                brand_href_chirho: "/app-chirho/perffection/".to_string(),
                nav_label_chirho: "Account".to_string(),
                nav_links_chirho: vec![ChromeNavLinkChirho::link_chirho(
                    "Log in",
                    "/app-chirho/perffection/login-chirho",
                )],
                footer_note_chirho: Some(FOOTER_NOTE_CHIRHO.to_string()),
            },
            heading_chirho: "Create your account".to_string(),
            intro_chirho: "Build and host your software with AI — free to begin.".to_string(),
            primary_cta_chirho: "Continue to secure sign-up".to_string(),
            primary_cta_href_chirho: REGISTER_HREF_CHIRHO.to_string(),
            human_check_note_chirho: "A quick human check keeps sign-up safe.".to_string(),
            alt_prompt_chirho: "Already have an account?".to_string(),
            alt_link_label_chirho: "Log in".to_string(),
            alt_link_href_chirho: "/app-chirho/perffection/login-chirho".to_string(),
        }
    }
}
