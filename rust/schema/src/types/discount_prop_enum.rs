use crate::prelude::*;

use super::number::Number;
use super::text::Text;


/// http://schema.org/discount
/// Any discount applied (to an Order).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DiscountPropEnum {
    Number(Number),
    Text(Text),
}
