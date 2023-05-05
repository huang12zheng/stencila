use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/applicationCategory
/// Type of software application, e.g. 'Game, Multimedia'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ApplicationCategoryPropEnum {
    Text(Text),
    URL(URL),
}
