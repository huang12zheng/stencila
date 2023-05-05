use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/pageStart
/// The page on which the work starts; for example "135" or "xiii".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PageStartPropEnum {
    Integer(Integer),
    Text(Text),
}
