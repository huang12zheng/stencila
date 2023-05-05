use crate::prelude::*;

use super::food_establishment::FoodEstablishment;
use super::place::Place;


/// http://schema.org/foodEstablishment
/// A sub property of location. The specific food establishment where the action occurred.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FoodEstablishmentPropEnum {
    FoodEstablishment(FoodEstablishment),
    Place(Place),
}
