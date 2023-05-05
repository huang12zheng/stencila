use crate::prelude::*;

use super::apartment::Apartment;
use super::hotel_room::HotelRoom;
use super::single_family_residence::SingleFamilyResidence;
use super::suite::Suite;

/// The allowed total occupancy for the accommodation in persons (including infants etc). For individual accommodations, this is not necessarily the legal maximum but defines the permitted usage as per the contractual agreement (e.g. a double room used by a single person). Typical unit code(s): C62 for person
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum occupancy {
    Apartment(Apartment),
    HotelRoom(HotelRoom),
    SingleFamilyResidence(SingleFamilyResidence),
    Suite(Suite),
}
