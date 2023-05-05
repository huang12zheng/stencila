use crate::prelude::*;

use super::category_code_set::CategoryCodeSet;
use super::url::URL;

/// A <a class="localLink" href="/CategoryCodeSet">CategoryCodeSet</a> that contains this category code.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum inCodeSet {
    CategoryCodeSet(CategoryCodeSet),
    URL(URL),
}
