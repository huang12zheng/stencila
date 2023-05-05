use crate::prelude::*;

use super::product::Product;
use super::service::Service;


/// http://schema.org/isSimilarTo
/// A pointer to another, functionally similar product (or multiple products).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IsSimilarToPropEnum {
    Product(Product),
    Service(Service),
}
