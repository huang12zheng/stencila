use crate::prelude::*;

use super::property_value::PropertyValue;
use super::quantitative_value::QuantitativeValue;
use super::type_and_quantity_node::TypeAndQuantityNode;
use super::unit_price_specification::UnitPriceSpecification;

/// The unit of measurement given using the UN/CEFACT Common Code (3 characters) or a URL. Other codes than the UN/CEFACT Common Code may be used with a prefix followed by a colon.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum unitCode {
    PropertyValue(PropertyValue),
    QuantitativeValue(QuantitativeValue),
    TypeAndQuantityNode(TypeAndQuantityNode),
    UnitPriceSpecification(UnitPriceSpecification),
}
