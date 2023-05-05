use crate::prelude::*;

use super::taxon::Taxon;
use super::text::Text;
use super::url::URL;

/// Closest child taxa of the taxon in question.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum childTaxon {
    Taxon(Taxon),
    Text(Text),
    URL(URL),
}
