use crate::prelude::*;

use super::bus_station::BusStation;
use super::bus_stop::BusStop;

/// The stop or station from which the bus departs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum departureBusStop {
    BusStation(BusStation),
    BusStop(BusStop),
}
