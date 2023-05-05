use crate::prelude::*;

use super::taxon::Taxon;
use super::text::Text;
use super::url::URL;


/// http://schema.org/childTaxon
/// Closest child taxa of the taxon in question.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ChildTaxonPropEnum {
    Taxon(Taxon),
    Text(Text),
    URL(URL),
}
