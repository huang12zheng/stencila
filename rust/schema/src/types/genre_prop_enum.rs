use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/genre
/// Genre of the creative work, broadcast channel or group.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum GenrePropEnum {
    Text(Text),
    URL(URL),
}
