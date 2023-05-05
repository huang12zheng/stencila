use crate::prelude::*;

use super::product::Product;
use super::service::Service;


/// http://schema.org/typeOfGood
/// The product that this structured value is referring to.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TypeOfGoodPropEnum {
    Product(Product),
    Service(Service),
}
