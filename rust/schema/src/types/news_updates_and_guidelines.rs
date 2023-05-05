use crate::prelude::*;

use super::url::URL;
use super::web_content::WebContent;

/// Indicates a page with news updates and guidelines. This could often be (but is not required to be) the main page containing <a class="localLink" href="/SpecialAnnouncement">SpecialAnnouncement</a> markup on a site.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum newsUpdatesAndGuidelines {
    URL(URL),
    WebContent(WebContent),
}
