use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::text::Text;


/// http://schema.org/citation
/// A citation or reference to another creative work, such as another publication, web page, scholarly article, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CitationPropEnum {
    CreativeWork(CreativeWork),
    Text(Text),
}
