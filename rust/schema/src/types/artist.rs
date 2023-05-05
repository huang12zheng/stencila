use crate::prelude::*;

use super::comic_issue::ComicIssue;
use super::comic_story::ComicStory;
use super::visual_artwork::VisualArtwork;

/// The primary artist for a work         in a medium other than pencils or digital line art--for example, if the         primary artwork is done in watercolors or digital paints.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum artist {
    ComicIssue(ComicIssue),
    ComicStory(ComicStory),
    VisualArtwork(VisualArtwork),
}
