use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/applicationSubCategory
/// Subcategory of the application, e.g. 'Arcade Game'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ApplicationSubCategoryPropEnum {
    Text(Text),
    URL(URL),
}
