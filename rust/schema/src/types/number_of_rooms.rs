use crate::prelude::*;

use super::accommodation::Accommodation;
use super::apartment::Apartment;
use super::floor_plan::FloorPlan;
use super::house::House;
use super::lodging_business::LodgingBusiness;
use super::single_family_residence::SingleFamilyResidence;
use super::suite::Suite;

/// The number of rooms (excluding bathrooms and closets) of the accommodation or lodging business. Typical unit code(s): ROM for room or C62 for no unit. The type of room can be put in the unitText property of the QuantitativeValue.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfRooms {
    Accommodation(Accommodation),
    Apartment(Apartment),
    FloorPlan(FloorPlan),
    House(House),
    LodgingBusiness(LodgingBusiness),
    SingleFamilyResidence(SingleFamilyResidence),
    Suite(Suite),
}
