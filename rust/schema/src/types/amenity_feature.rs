use crate::prelude::*;

use super::accommodation::Accommodation;
use super::floor_plan::FloorPlan;
use super::lodging_business::LodgingBusiness;
use super::place::Place;

/// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum amenityFeature {
    Accommodation(Accommodation),
    FloorPlan(FloorPlan),
    LodgingBusiness(LodgingBusiness),
    Place(Place),
}
