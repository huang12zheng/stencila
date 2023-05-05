use crate::prelude::*;

use super::class::Class;
use super::enumeration::Enumeration;
use super::property::Property;

/// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum supersededBy {
    Class(Class),
    Enumeration(Enumeration),
    Property(Property),
}
