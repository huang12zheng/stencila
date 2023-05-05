use crate::prelude::*;

use super::url::URL;
use super::web_page::WebPage;


/// http://schema.org/archivedAt
/// Indicates a page or other link involved in archival of a <a class="localLink" href="/CreativeWork">CreativeWork</a>. In the case of <a class="localLink" href="/MediaReview">MediaReview</a>, the items in a <a class="localLink" href="/MediaReviewItem">MediaReviewItem</a> may often become inaccessible, but be archived by archival, journalistic, activist, or law enforcement organizations. In such cases, the referenced page may not directly publish the content.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ArchivedAtPropEnum {
    URL(URL),
    WebPage(WebPage),
}
