use crate::prelude::*;

use super::category_code::CategoryCode;
use super::text::Text;


/// http://schema.org/legislationType
/// The type of the legislation. Examples of values are "law", "act", "directive", "decree", "regulation", "statutory instrument", "loi organique", "règlement grand-ducal", etc., depending on the country.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum LegislationTypePropEnum {
    CategoryCode(CategoryCode),
    Text(Text),
}
