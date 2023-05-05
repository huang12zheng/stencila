use crate::prelude::*;

use super::product::Product;
use super::service::Service;


/// http://schema.org/isRelatedTo
/// A pointer to another, somehow related product (or multiple products).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IsRelatedToPropEnum {
    Product(Product),
    Service(Service),
}
