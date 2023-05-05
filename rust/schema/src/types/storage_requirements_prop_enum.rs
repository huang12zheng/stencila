use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/storageRequirements
/// Storage requirements (free space required).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum StorageRequirementsPropEnum {
    Text(Text),
    URL(URL),
}
