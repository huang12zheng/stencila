use crate::prelude::*;

use super::text::Text;
use super::thing::Thing;


/// http://schema.org/defaultValue
/// The default value of the input.  For properties that expect a literal, the default is a literal value, for properties that expect an object, it's an ID reference to one of the current values.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DefaultValuePropEnum {
    Text(Text),
    Thing(Thing),
}
