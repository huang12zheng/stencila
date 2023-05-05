use crate::prelude::*;

use super::product::Product;
use super::text::Text;
use super::url::URL;


/// http://schema.org/material
/// A material that something is made from, e.g. leather, wool, cotton, paper.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MaterialPropEnum {
    Product(Product),
    Text(Text),
    URL(URL),
}
