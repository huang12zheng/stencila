use crate::prelude::*;

use super::accommodation::Accommodation;
use super::floor_plan::FloorPlan;

/// The size of the accommodation, e.g. in square meter or squarefoot. Typical unit code(s): MTK for square meter, FTK for square foot, or YDK for square yard
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum floorSize {
    Accommodation(Accommodation),
    FloorPlan(FloorPlan),
}
