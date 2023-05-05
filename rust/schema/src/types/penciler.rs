use crate::prelude::*;

use super::comic_issue::ComicIssue;
use super::comic_story::ComicStory;
use super::visual_artwork::VisualArtwork;

/// The individual who draws the primary narrative artwork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum penciler {
    ComicIssue(ComicIssue),
    ComicStory(ComicStory),
    VisualArtwork(VisualArtwork),
}
