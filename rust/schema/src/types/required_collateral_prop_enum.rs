use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;


/// http://schema.org/requiredCollateral
/// Assets required to secure loan or credit repayments. It may take form of third party pledge, goods, financial instruments (cash, securities, etc.)
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RequiredCollateralPropEnum {
    Text(Text),
    Thing(Thing),
}
