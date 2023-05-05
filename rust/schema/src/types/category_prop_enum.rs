use crate::prelude::*;

use super::category_code::CategoryCode;
use super::physical_activity_category::PhysicalActivityCategory;
use super::text::Text;
use super::thing::Thing;
use super::url::URL;


/// http://schema.org/category
/// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CategoryPropEnum {
    CategoryCode(CategoryCode),
    PhysicalActivityCategory(PhysicalActivityCategory),
    Text(Text),
    Thing(Thing),
    URL(URL),
}
