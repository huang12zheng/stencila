use crate::prelude::*;

use super::boolean::Boolean;
use super::text::Text;

/// Indicates whether pets are allowed to enter the accommodation or lodging business. More detailed information can be put in a text value.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum petsAllowed {
    Boolean(Boolean),
    Text(Text),
}
