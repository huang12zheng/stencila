use crate::prelude::*;

use super::property_value::PropertyValue;
use super::text::Text;
use super::url::URL;

/// The taxonomic rank of this taxon given preferably as a URI from a controlled vocabulary – typically the ranks from TDWG TaxonRank ontology or equivalent Wikidata URIs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum taxonRank {
    PropertyValue(PropertyValue),
    Text(Text),
    URL(URL),
}
