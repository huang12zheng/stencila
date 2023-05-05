use crate::prelude::*;

use super::clip::Clip;
use super::episode::Episode;

/// The season to which this episode belongs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum partOfSeason {
    Clip(Clip),
    Episode(Episode),
}
