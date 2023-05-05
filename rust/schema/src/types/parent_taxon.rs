use crate::prelude::*;

use super::taxon::Taxon;
use super::text::Text;
use super::url::URL;

/// Closest parent taxon of the taxon in question.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum parentTaxon {
    Taxon(Taxon),
    Text(Text),
    URL(URL),
}
