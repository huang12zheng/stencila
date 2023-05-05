use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/artEdition
/// The number of copies when multiple copies of a piece of artwork are produced - e.g. for a limited edition of 20 prints, 'artEdition' refers to the total number of copies (in this example "20").
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ArtEditionPropEnum {
    Integer(Integer),
    Text(Text),
}
