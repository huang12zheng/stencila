use crate::prelude::*;

use super::accommodation::Accommodation;
use super::floor_plan::FloorPlan;

/// Number of partial bathrooms - The total number of half and ¼ bathrooms in an <a class="localLink" href="/Accommodation">Accommodation</a>. This corresponds to the <a href="https://ddwiki.reso.org/display/DDW17/BathroomsPartial+Field">BathroomsPartial field in RESO</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfPartialBathrooms {
    Accommodation(Accommodation),
    FloorPlan(FloorPlan),
}
