use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/artform
/// e.g. Painting, Drawing, Sculpture, Print, Photograph, Assemblage, Collage, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ArtformPropEnum {
    Text(Text),
    URL(URL),
}
