use crate::prelude::*;

use super::offer::Offer;
use super::product::Product;
use super::size_specification::SizeSpecification;

/// A product measurement, for example the inseam of pants, the wheel size of a bicycle, or the gauge of a screw. Usually an exact measurement, but can also be a range of measurements for adjustable products, for example belts and ski bindings.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum hasMeasurement {
    Offer(Offer),
    Product(Product),
    SizeSpecification(SizeSpecification),
}
