use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/artworkSurface
/// The supporting materials for the artwork, e.g. Canvas, Paper, Wood, Board, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ArtworkSurfacePropEnum {
    Text(Text),
    URL(URL),
}
