use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;
use super::url::URL;


/// http://schema.org/taxonRank
/// The taxonomic rank of this taxon given preferably as a URI from a controlled vocabulary – typically the ranks from TDWG TaxonRank ontology or equivalent Wikidata URIs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TaxonRankPropEnum {
    PropertyValue(PropertyValue),
    Text(Text),
    URL(URL),
}
