use crate::prelude::*;

use super::food_establishment::FoodEstablishment;
use super::lodging_business::LodgingBusiness;

/// An official rating for a lodging business or food establishment, e.g. from national associations or standards bodies. Use the author property to indicate the rating organization, e.g. as an Organization with name such as (e.g. HOTREC, DEHOGA, WHR, or Hotelstars).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum starRating {
    FoodEstablishment(FoodEstablishment),
    LodgingBusiness(LodgingBusiness),
}
