use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/unitCode
/// The unit of measurement given using the UN/CEFACT Common Code (3 characters) or a URL. Other codes than the UN/CEFACT Common Code may be used with a prefix followed by a colon.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum UnitCodePropEnum {
    Text(Text),
    URL(URL),
}
