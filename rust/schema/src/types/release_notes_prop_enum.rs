use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/releaseNotes
/// Description of what changed in this version.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ReleaseNotesPropEnum {
    Text(Text),
    URL(URL),
}
