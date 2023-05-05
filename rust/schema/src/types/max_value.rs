use crate::prelude::*;

use super::monetary_amount::MonetaryAmount;
use super::property_value::PropertyValue;
use super::property_value_specification::PropertyValueSpecification;
use super::quantitative_value::QuantitativeValue;

/// The upper value of some characteristic or property.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum maxValue {
    MonetaryAmount(MonetaryAmount),
    PropertyValue(PropertyValue),
    PropertyValueSpecification(PropertyValueSpecification),
    QuantitativeValue(QuantitativeValue),
}
