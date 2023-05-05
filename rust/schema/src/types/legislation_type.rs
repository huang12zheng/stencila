use crate::prelude::*;

use super::category_code::CategoryCode;
use super::text::Text;

/// The type of the legislation. Examples of values are "law", "act", "directive", "decree", "regulation", "statutory instrument", "loi organique", "règlement grand-ducal", etc., depending on the country.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum legislationType {
    CategoryCode(CategoryCode),
    Text(Text),
}
