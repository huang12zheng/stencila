use crate::prelude::*;

use super::merchant_return_policy::MerchantReturnPolicy;
use super::place::Place;
use super::product::Product;
use super::qualitative_value::QualitativeValue;
use super::quantitative_value::QuantitativeValue;

/// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum additionalProperty {
    MerchantReturnPolicy(MerchantReturnPolicy),
    Place(Place),
    Product(Product),
    QualitativeValue(QualitativeValue),
    QuantitativeValue(QuantitativeValue),
}
