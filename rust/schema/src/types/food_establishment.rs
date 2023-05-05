use crate::prelude::*;

use super::food_establishment::FoodEstablishment;
use super::place::Place;

/// A sub property of location. The specific food establishment where the action occurred.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum foodEstablishment {
    FoodEstablishment(FoodEstablishment),
    Place(Place),
}
