use crate::prelude::*;

use super::lodging_business::LodgingBusiness;
use super::lodging_reservation::LodgingReservation;

/// The earliest someone may check into a lodging establishment.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum checkinTime {
    LodgingBusiness(LodgingBusiness),
    LodgingReservation(LodgingReservation),
}
