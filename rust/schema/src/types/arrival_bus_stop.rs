use crate::prelude::*;

use super::bus_station::BusStation;
use super::bus_stop::BusStop;

/// The stop or station from which the bus arrives.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum arrivalBusStop {
    BusStation(BusStation),
    BusStop(BusStop),
}
