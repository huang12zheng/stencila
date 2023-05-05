use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/memoryRequirements
/// Minimum memory requirements.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MemoryRequirementsPropEnum {
    Text(Text),
    URL(URL),
}
