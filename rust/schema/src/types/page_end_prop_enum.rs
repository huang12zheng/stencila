use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/pageEnd
/// The page on which the work ends; for example "138" or "xvi".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PageEndPropEnum {
    Integer(Integer),
    Text(Text),
}
