use crate::prelude::*;

use super::apartment_complex::ApartmentComplex;
use super::floor_plan::FloorPlan;

/// Indicates the total (available plus unavailable) number of accommodation units in an <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>, or the number of accommodation units for a specific <a class="localLink" href="/FloorPlan">FloorPlan</a> (within its specific <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>). See also <a class="localLink" href="/numberOfAvailableAccommodationUnits">numberOfAvailableAccommodationUnits</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfAccommodationUnits {
    ApartmentComplex(ApartmentComplex),
    FloorPlan(FloorPlan),
}
