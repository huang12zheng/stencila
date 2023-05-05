use crate::prelude::*;

use super::speakable_specification::SpeakableSpecification;
use super::web_page_element::WebPageElement;

/// A CSS selector, e.g. of a <a class="localLink" href="/SpeakableSpecification">SpeakableSpecification</a> or <a class="localLink" href="/WebPageElement">WebPageElement</a>. In the latter case, multiple matches within a page can constitute a single conceptual "Web page element".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum cssSelector {
    SpeakableSpecification(SpeakableSpecification),
    WebPageElement(WebPageElement),
}
