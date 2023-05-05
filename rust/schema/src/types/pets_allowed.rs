use crate::prelude::*;

use super::accommodation::Accommodation;
use super::apartment_complex::ApartmentComplex;
use super::floor_plan::FloorPlan;
use super::lodging_business::LodgingBusiness;

/// Indicates whether pets are allowed to enter the accommodation or lodging business. More detailed information can be put in a text value.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum petsAllowed {
    Accommodation(Accommodation),
    ApartmentComplex(ApartmentComplex),
    FloorPlan(FloorPlan),
    LodgingBusiness(LodgingBusiness),
}
