use crate::prelude::*;

use super::product::Product;
use super::text::Text;


/// http://schema.org/productSupported
/// The product or service this support contact point is related to (such as product support for a particular product line). This can be a specific product or product line (e.g. "iPhone") or a general category of products or services (e.g. "smartphones").
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ProductSupportedPropEnum {
    Product(Product),
    Text(Text),
}
