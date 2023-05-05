use crate::prelude::*;

use super::clip::Clip;
use super::creative_work_season::CreativeWorkSeason;
use super::episode::Episode;

/// The series to which this episode or season belongs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum partOfSeries {
    Clip(Clip),
    CreativeWorkSeason(CreativeWorkSeason),
    Episode(Episode),
}
