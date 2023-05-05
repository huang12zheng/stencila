use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::product::Product;
use super::url::URL;


/// http://schema.org/isBasedOn
/// A resource from which this work is derived or from which it is a modification or adaption.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum IsBasedOnPropEnum {
    CreativeWork(CreativeWork),
    Product(Product),
    URL(URL),
}
