use crate::prelude::*;

use super::rental_car_reservation::RentalCarReservation;
use super::taxi_reservation::TaxiReservation;

/// When a taxi will pick up a passenger or a rental car can be picked up.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pickupTime {
    RentalCarReservation(RentalCarReservation),
    TaxiReservation(TaxiReservation),
}
