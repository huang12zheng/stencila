use crate::prelude::*;

use super::url::URL;
use super::web_content::WebContent;


/// http://schema.org/newsUpdatesAndGuidelines
/// Indicates a page with news updates and guidelines. This could often be (but is not required to be) the main page containing <a class="localLink" href="/SpecialAnnouncement">SpecialAnnouncement</a> markup on a site.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum NewsUpdatesAndGuidelinesPropEnum {
    URL(URL),
    WebContent(WebContent),
}
