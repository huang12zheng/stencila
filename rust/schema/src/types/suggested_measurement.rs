use crate::prelude::*;

use super::people_audience::PeopleAudience;
use super::size_specification::SizeSpecification;

/// A suggested range of body measurements for the intended audience or person, for example inseam between 32 and 34 inches or height between 170 and 190 cm. Typically found on a size chart for wearable products.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum suggestedMeasurement {
    PeopleAudience(PeopleAudience),
    SizeSpecification(SizeSpecification),
}
