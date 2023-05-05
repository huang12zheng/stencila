use crate::prelude::*;

use super::audience::Audience;
use super::text::Text;

/// Attraction suitable for type(s) of tourist. E.g. children, visitors from a particular country, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum touristType {
    Audience(Audience),
    Text(Text),
}
