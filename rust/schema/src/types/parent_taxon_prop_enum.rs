use crate::prelude::*;

use super::taxon::Taxon;
use super::text::Text;
use super::url::URL;


/// http://schema.org/parentTaxon
/// Closest parent taxon of the taxon in question.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ParentTaxonPropEnum {
    Taxon(Taxon),
    Text(Text),
    URL(URL),
}
