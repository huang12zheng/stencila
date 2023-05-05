use crate::prelude::*;

use super::class::Class;
use super::enumeration::Enumeration;
use super::property::Property;


/// http://schema.org/supersededBy
/// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SupersededByPropEnum {
    Class(Class),
    Enumeration(Enumeration),
    Property(Property),
}
