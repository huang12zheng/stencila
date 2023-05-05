use crate::prelude::*;

use super::category_code_set::CategoryCodeSet;
use super::url::URL;


/// http://schema.org/inCodeSet
/// A <a class="localLink" href="/CategoryCodeSet">CategoryCodeSet</a> that contains this category code.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum InCodeSetPropEnum {
    CategoryCodeSet(CategoryCodeSet),
    URL(URL),
}
