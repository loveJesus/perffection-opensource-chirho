// For God so loved the world, that He gave His only begotten Son, that whoever believes in Him should not perish but have eternal life. - John 3:16

use serde::{Deserialize, Serialize};

/// The raw HTTP request as it arrives (static pages ignore it; it types the surface boundary).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpRequestV1Chirho {
    #[serde(default)]
    pub path_chirho: String,
    /// Legacy optional input used by request-nonreflection tests. All live public renderers ignore
    /// it; a caller-supplied value grants no identity or authority. Workspace identity comes from
    /// Cairn's separate authenticated request context, never from this public display model.
    #[serde(default)]
    pub user_principal_chirho: Option<String>,
}
