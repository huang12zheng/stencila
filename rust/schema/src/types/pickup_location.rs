use crate::prelude::*;

use super::rental_car_reservation::RentalCarReservation;
use super::taxi_reservation::TaxiReservation;

/// Where a taxi will pick up a passenger or a rental car can be picked up.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pickupLocation {
    RentalCarReservation(RentalCarReservation),
    TaxiReservation(TaxiReservation),
}
