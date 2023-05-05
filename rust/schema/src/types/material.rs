use crate::prelude::*;

use super::product::Product;
use super::text::Text;
use super::url::URL;

/// A material that something is made from, e.g. leather, wool, cotton, paper.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum material {
    Product(Product),
    Text(Text),
    URL(URL),
}
