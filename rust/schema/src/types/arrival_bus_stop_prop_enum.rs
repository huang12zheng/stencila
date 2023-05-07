use crate::prelude::*;

use super::bus_station::BusStation;
use super::bus_stop::BusStop;


/// http://schema.org/arrivalBusStop
/// The stop or station from which the bus arrives.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ArrivalBusStopPropEnum {
    BusStation(BusStation),
    BusStop(BusStop),
}