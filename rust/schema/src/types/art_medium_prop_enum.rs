use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/artMedium
/// The material used. (E.g. Oil, Watercolour, Acrylic, Linoprint, Marble, Cyanotype, Digital, Lithograph, DryPoint, Intaglio, Pastel, Woodcut, Pencil, Mixed Media, etc.)
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ArtMediumPropEnum {
    Text(Text),
    URL(URL),
}
