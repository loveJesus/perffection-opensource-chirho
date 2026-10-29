// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)

//! Typed pieces shared by the FIRST LIGHT inner pages ("What's in it", Trust): an opening band with
//! its picture, a short point, the fold that keeps the technical proof out of an owner's way, and the
//! closing band. The partials in `templates_chirho/partials_chirho/fl_*_chirho.html` read these
//! through `model_chirho`, so every page that includes them carries the same field names.

use serde::{Deserialize, Serialize};

/// One short, checkable reason: a title and one sentence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointChirho {
    pub title_chirho: String,
    pub body_chirho: String,
}

/// The opening band of an inner page: words on the left, a generated picture melting in on the
/// right. The picture is decoration (the words carry the meaning), so it is rendered without alt
/// text and hidden from assistive technology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageHeroChirho {
    /// The first focusable thing on the page: a keyboard visitor's way past the navigation.
    pub skip_label_chirho: String,
    pub mark_chirho: String,
    /// One entry per rendered line; where a headline this large breaks is a typographic decision.
    pub headline_lines_chirho: Vec<String>,
    pub lead_chirho: String,
    /// Display-only asset names: the wide picture and its phone-width copy.
    pub image_chirho: String,
    pub image_small_chirho: String,
    pub primary_label_chirho: String,
    pub primary_href_chirho: String,
    pub secondary_label_chirho: String,
    pub secondary_href_chirho: String,
}

/// "For your web developer": the checks behind the plain-words claims, folded away so an owner is
/// not asked to read them, and present so nobody has to take the page's word for anything.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeveloperFoldChirho {
    pub summary_chirho: String,
    pub intro_chirho: String,
    pub points_chirho: Vec<String>,
}

/// The closing band: a plain statement and the one open door, sign-in for invitees, plus a way on
/// to the next page. Not a call to action: there is no open intake path yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageCloseChirho {
    pub mark_chirho: String,
    pub heading_chirho: String,
    pub body_chirho: String,
    pub signin_label_chirho: String,
    pub secondary_label_chirho: String,
    pub secondary_href_chirho: String,
}
