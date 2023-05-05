use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::product::Product;
use super::url::URL;

/// A resource from which this work is derived or from which it is a modification or adaption.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum isBasedOn {
    CreativeWork(CreativeWork),
    Product(Product),
    URL(URL),
}
