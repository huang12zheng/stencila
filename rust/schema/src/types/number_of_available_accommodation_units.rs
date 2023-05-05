use crate::prelude::*;

use super::apartment_complex::ApartmentComplex;
use super::floor_plan::FloorPlan;

/// Indicates the number of available accommodation units in an <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>, or the number of accommodation units for a specific <a class="localLink" href="/FloorPlan">FloorPlan</a> (within its specific <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>). See also <a class="localLink" href="/numberOfAccommodationUnits">numberOfAccommodationUnits</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfAvailableAccommodationUnits {
    ApartmentComplex(ApartmentComplex),
    FloorPlan(FloorPlan),
}
