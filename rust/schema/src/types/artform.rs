use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// e.g. Painting, Drawing, Sculpture, Print, Photograph, Assemblage, Collage, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum artform {
    Text(Text),
    URL(URL),
}
