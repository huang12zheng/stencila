use crate::prelude::*;

use super::number::Number;
use super::quantitative_value::QuantitativeValue;

/// The total integer number of bedrooms in a some <a class="localLink" href="/Accommodation">Accommodation</a>, <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> or <a class="localLink" href="/FloorPlan">FloorPlan</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum numberOfBedrooms {
    Number(Number),
    QuantitativeValue(QuantitativeValue),
}
