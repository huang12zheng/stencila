use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::product::Product;

/// A material that something is made from, e.g. leather, wool, cotton, paper.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum material {
    CreativeWork(CreativeWork),
    Product(Product),
}
