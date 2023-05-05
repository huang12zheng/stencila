use crate::prelude::*;

use super::property_value::PropertyValue;
use super::quantitative_value::QuantitativeValue;
use super::type_and_quantity_node::TypeAndQuantityNode;
use super::unit_price_specification::UnitPriceSpecification;

/// A string or text indicating the unit of measurement. Useful if you cannot provide a standard unit code for <a href='unitCode'>unitCode</a>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum unitText {
    PropertyValue(PropertyValue),
    QuantitativeValue(QuantitativeValue),
    TypeAndQuantityNode(TypeAndQuantityNode),
    UnitPriceSpecification(UnitPriceSpecification),
}
