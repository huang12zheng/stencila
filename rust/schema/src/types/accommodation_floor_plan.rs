use crate::prelude::*;

use super::accommodation::Accommodation;
use super::residence::Residence;

/// A floorplan of some <a class="localLink" href="/Accommodation">Accommodation</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum accommodationFloorPlan {
    Accommodation(Accommodation),
    Residence(Residence),
}
