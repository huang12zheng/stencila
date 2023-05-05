use crate::prelude::*;

use super::integer::Integer;
use super::text::Text;


/// http://schema.org/episodeNumber
/// Position of the episode within an ordered group of episodes.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum EpisodeNumberPropEnum {
    Integer(Integer),
    Text(Text),
}
