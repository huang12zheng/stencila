use crate::prelude::*;

use super::comic_issue::ComicIssue;
use super::comic_story::ComicStory;
use super::visual_artwork::VisualArtwork;

/// The individual who adds lettering, including speech balloons and sound effects, to artwork.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum letterer {
    ComicIssue(ComicIssue),
    ComicStory(ComicStory),
    VisualArtwork(VisualArtwork),
}
