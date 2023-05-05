use crate::prelude::*;

use super::product_model::ProductModel;
use super::text::Text;


/// http://schema.org/model
/// The model of the product. Use with the URL of a ProductModel or a textual representation of the model identifier. The URL of the ProductModel can be from an external source. It is recommended to additionally provide strong product identifiers via the gtin8/gtin13/gtin14 and mpn properties.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ModelPropEnum {
    ProductModel(ProductModel),
    Text(Text),
}
