use crate::prelude::*;

use super::lodging_business::LodgingBusiness;
use super::lodging_reservation::LodgingReservation;

/// The latest someone may check out of a lodging establishment.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum checkoutTime {
    LodgingBusiness(LodgingBusiness),
    LodgingReservation(LodgingReservation),
}
