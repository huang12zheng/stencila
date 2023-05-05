use crate::prelude::*;

use super::boolean::Boolean;
use super::number::Number;


/// http://schema.org/cashBack
/// A cardholder benefit that pays the cardholder a small percentage of their net expenditures.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CashBackPropEnum {
    Boolean(Boolean),
    Number(Number),
}
