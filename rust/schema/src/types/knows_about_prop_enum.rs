use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;
use super::url::URL;


/// http://schema.org/knowsAbout
/// Of a <a class="localLink" href="/Person">Person</a>, and less typically of an <a class="localLink" href="/Organization">Organization</a>, to indicate a topic that is known about - suggesting possible expertise but not implying it. We do not distinguish skill levels here, or relate this to educational content, events, objectives or <a class="localLink" href="/JobPosting">JobPosting</a> descriptions.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum KnowsAboutPropEnum {
    Text(Text),
    Thing(Thing),
    URL(URL),
}
