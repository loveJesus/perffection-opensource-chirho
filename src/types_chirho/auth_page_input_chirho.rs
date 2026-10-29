// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Cairn auth-display bytes, never an HTTP request or credential body.
//! Workflow: spec-chirho/workflows-chirho/invite-beta-chirho/signup-chirho.md.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthPageKindChirho {
    ChallengeChirho,
    RefusalChirho,
    SuccessHandoffChirho,
    UsernameLoginChirho,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthPagePostureChirho {
    SignInChirho,
    RegistrationOpenChirho,
    RegistrationClosedChirho,
    RegistrationFullChirho,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPageChallengeChirho {
    pub question_chirho: String,
    pub token_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPageSuccessHandoffChirho {
    pub next_route_chirho: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPageFormChirho {
    pub action_route_chirho: String,
    pub field_names_chirho: Vec<String>,
}

/// Deny a credential, identity or raw request field rather than silently receiving sensitive data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthPageInputChirho {
    pub page_kind_chirho: AuthPageKindChirho,
    pub posture_chirho: AuthPagePostureChirho,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub challenge_chirho: Option<AuthPageChallengeChirho>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal_code_chirho: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success_handoff_chirho: Option<AuthPageSuccessHandoffChirho>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_chirho: Option<AuthPageFormChirho>,
}
