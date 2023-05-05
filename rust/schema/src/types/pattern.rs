use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::product::Product;

/// A pattern that something has, for example 'polka dot', 'striped', 'Canadian flag'. Values are typically expressed as text, although links to controlled value schemes are also supported.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pattern {
    CreativeWork(CreativeWork),
    Product(Product),
}
