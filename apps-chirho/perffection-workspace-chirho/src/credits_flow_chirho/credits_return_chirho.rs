// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The two pages Stripe's hosted Checkout returns her to.
//!
//! Workflow: `spec-chirho/workflows-chirho/selling-chirho/credit-purchase-chirho.md`.
//!
//! A return page proves nothing: only the verified Stripe event credits her. So these pages read no ledger and carry
//! no balance. The paid one says her credits appear once the payment is confirmed, and both point to the Credits
//! page, which shows whatever the ledger has posted.

use crate::credits_flow_chirho::credits_page_chirho::WORKSPACE_CREDITS_HREF_CHIRHO;
use crate::workspace_page_chirho::{
    workspace_chrome_chirho, WorkspaceNavPageChirho, WorkspaceNavRoadsChirho,
};
use perffection_ui_chirho::{
    CreditsCopyChirho, SiteChromeChirho, UiLocaleChirho, WorkspaceCopyChirho,
};

/// Where Checkout returns her after a payment (held, like the Credits page).
pub const WORKSPACE_CREDITS_PAID_HREF_CHIRHO: &str =
    "/app-chirho/perffection-workspace/credits-paid-chirho";
/// Where Checkout returns her when she cancels (held).
pub const WORKSPACE_CREDITS_CANCELLED_HREF_CHIRHO: &str =
    "/app-chirho/perffection-workspace/credits-cancelled-chirho";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreditsReturnKindChirho {
    PaidChirho,
    CancelledChirho,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreditsReturnV1Chirho {
    pub(crate) chrome_chirho: SiteChromeChirho,
    pub(crate) copy_chirho: CreditsCopyChirho,
    pub(crate) heading_chirho: String,
    pub(crate) body_chirho: String,
    pub(crate) credits_href_chirho: &'static str,
}

impl CreditsReturnV1Chirho {
    pub fn for_kind_chirho(
        kind_chirho: CreditsReturnKindChirho,
        locale_chirho: UiLocaleChirho,
    ) -> Self {
        let copy_chirho = CreditsCopyChirho::for_locale_chirho(locale_chirho);
        let (page_title_chirho, heading_chirho, body_chirho) = match kind_chirho {
            CreditsReturnKindChirho::PaidChirho => (
                &copy_chirho.paid_page_title_chirho,
                &copy_chirho.paid_heading_chirho,
                &copy_chirho.paid_body_chirho,
            ),
            CreditsReturnKindChirho::CancelledChirho => (
                &copy_chirho.cancelled_page_title_chirho,
                &copy_chirho.cancelled_heading_chirho,
                &copy_chirho.cancelled_body_chirho,
            ),
        };
        let chrome_chirho = workspace_chrome_chirho(
            &WorkspaceCopyChirho::for_locale_chirho(locale_chirho),
            page_title_chirho,
            &copy_chirho.page_description_chirho,
            WorkspaceNavPageChirho::CreditsChirho,
            WorkspaceNavRoadsChirho::deployed_chirho(),
        );
        Self {
            chrome_chirho,
            heading_chirho: heading_chirho.clone(),
            body_chirho: body_chirho.clone(),
            copy_chirho,
            credits_href_chirho: WORKSPACE_CREDITS_HREF_CHIRHO,
        }
    }
}
