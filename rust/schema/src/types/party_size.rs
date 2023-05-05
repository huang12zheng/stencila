use crate::prelude::*;

use super::food_establishment_reservation::FoodEstablishmentReservation;
use super::taxi_reservation::TaxiReservation;

/// Number of people the reservation should accommodate.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum partySize {
    FoodEstablishmentReservation(FoodEstablishmentReservation),
    TaxiReservation(TaxiReservation),
}
