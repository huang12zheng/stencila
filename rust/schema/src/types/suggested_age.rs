use crate::prelude::*;

use super::people_audience::PeopleAudience;
use super::size_specification::SizeSpecification;

/// The age or age range for the intended audience or person, for example 3-12 months for infants, 1-5 years for toddlers.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum suggestedAge {
    PeopleAudience(PeopleAudience),
    SizeSpecification(SizeSpecification),
}
