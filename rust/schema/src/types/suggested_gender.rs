use crate::prelude::*;

use super::people_audience::PeopleAudience;
use super::size_specification::SizeSpecification;

/// The suggested gender of the intended person or audience, for example "male", "female", or "unisex".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum suggestedGender {
    PeopleAudience(PeopleAudience),
    SizeSpecification(SizeSpecification),
}
