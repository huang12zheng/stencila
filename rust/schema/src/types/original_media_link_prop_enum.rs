use crate::prelude::*;

use super::media_object::MediaObject;
use super::url::URL;
use super::web_page::WebPage;


/// http://schema.org/originalMediaLink
/// Link to the page containing an original version of the content, or directly to an online copy of the original <a class="localLink" href="/MediaObject">MediaObject</a> content, e.g. video file.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum OriginalMediaLinkPropEnum {
    MediaObject(MediaObject),
    URL(URL),
    WebPage(WebPage),
}
