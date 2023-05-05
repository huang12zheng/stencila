use crate::prelude::*;

use super::boolean::Boolean;
use super::text::Text;


/// http://schema.org/petsAllowed
/// Indicates whether pets are allowed to enter the accommodation or lodging business. More detailed information can be put in a text value.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PetsAllowedPropEnum {
    Boolean(Boolean),
    Text(Text),
}
