use crate::prelude::*;

use super::compound_price_specification::CompoundPriceSpecification;
use super::unit_price_specification::UnitPriceSpecification;

/// Defines the type of a price specified for an offered product, for example a list price, a (temporary) sale price or a manufacturer suggested retail price. If multiple prices are specified for an offer the <a class="localLink" href="/priceType">priceType</a> property can be used to identify the type of each such specified price. The value of priceType can be specified as a value from enumeration PriceTypeEnumeration or as a free form text string for price types that are not already predefined in PriceTypeEnumeration.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum priceType {
    CompoundPriceSpecification(CompoundPriceSpecification),
    UnitPriceSpecification(UnitPriceSpecification),
}
