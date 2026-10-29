// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Shared, presentation-only product UI contracts for Perffection.
//!
//! This crate owns display copy, locale metadata and the source-level chrome model. It deliberately
//! owns no auth destination, workspace binding, credential, viewer-state value or effect authority.

mod agent_guide_copy_chirho;
mod auth_copy_chirho;
mod connections_copy_chirho;
mod credits_copy_chirho;
mod figures_copy_chirho;
mod help_copy_chirho;
mod locale_chirho;
mod login_copy_chirho;
mod page_copy_chirho;
mod public_site_copy_chirho;
mod setup_copy_chirho;
mod site_chrome_chirho;
mod workspace_copy_chirho;

pub use agent_guide_copy_chirho::AgentGuideCopyChirho;
pub use auth_copy_chirho::{AuthCopyChirho, AuthNoticeCopyChirho};
pub use connections_copy_chirho::WorkspaceConnectionsCopyChirho;
pub use credits_copy_chirho::CreditsCopyChirho;
pub use figures_copy_chirho::WorkspaceFiguresCopyChirho;
pub use help_copy_chirho::{HelpCopyChirho, HelpTopicCopyChirho};
pub use locale_chirho::{pseudolocalize_text_chirho, UiLocaleChirho};
pub use login_copy_chirho::LoginCopyChirho;
pub use page_copy_chirho::PageCopyChirho;
pub use public_site_copy_chirho::PublicSiteCopyChirho;
pub use setup_copy_chirho::SetupCopyChirho;
pub use site_chrome_chirho::{
    ChromeNavLinkChirho, SiteChromeChirho, LOVEJESUS_SOFTWARE_URL_CHIRHO,
};
pub use workspace_copy_chirho::{
    WorkspaceChromeCopyChirho, WorkspaceCopyChirho, WorkspaceEditorCopyChirho,
    WorkspaceExportCopyChirho, WorkspaceGuidedCopyChirho, WorkspaceGuidedServiceExampleChirho,
    WorkspaceImportCopyChirho, WorkspacePublicationCopyChirho, WorkspaceValidationCopyChirho,
    WORKSPACE_GUIDED_STEP_COUNT_CHIRHO,
};

/// The minimum length every new-password field asks for in the browser (the Account page's change and the Set-up page's
/// first password), and the number copy names through `{min_characters_chirho}`. L.J., 2026-10-04: "yes do the password
/// thing" (15 characters, counted as characters; NIST SP 800-63B-4 for a password that is the only factor). Presentation
/// only: the engine's floor (`credential_meets_floor_chirho`) is the authority and refuses shorter ones.
pub const NEW_PASSWORD_MIN_CHARACTERS_CHIRHO: usize = 15;

/// How long a refused sign-in is asked to wait: the engine's one `Retry-After` on every sign-in rate refusal and on the
/// Set-up page's recover road, its 15-minute failure window (Cairn `AUTH_RATE_LIMIT_RETRY_AFTER_SECS_CHIRHO` at the R16
/// engine `a07696ca`). The browser reaches these notices through a fragment redirect and cannot read the header, so
/// copy names the wait through `{wait_minutes_chirho}`. L.J., 2026-10-26: "yes the time is important". Presentation
/// only: the engine's window is the authority, so the two change together.
pub const SIGN_IN_WAIT_SECONDS_CHIRHO: u64 = 900;

/// The same wait in whole minutes, the number copy names.
pub const SIGN_IN_WAIT_MINUTES_CHIRHO: u64 = SIGN_IN_WAIT_SECONDS_CHIRHO / 60;
