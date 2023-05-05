use crate::prelude::*;

use super::media_object::MediaObject;
use super::url::URL;
use super::web_page::WebPage;

/// Link to the page containing an original version of the content, or directly to an online copy of the original <a class="localLink" href="/MediaObject">MediaObject</a> content, e.g. video file.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum originalMediaLink {
    MediaObject(MediaObject),
    URL(URL),
    WebPage(WebPage),
}
