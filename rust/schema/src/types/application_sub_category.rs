use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Subcategory of the application, e.g. 'Arcade Game'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum applicationSubCategory {
    Text(Text),
    URL(URL),
}
