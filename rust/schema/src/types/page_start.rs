use crate::prelude::*;

use super::article::Article;
use super::chapter::Chapter;
use super::publication_issue::PublicationIssue;
use super::publication_volume::PublicationVolume;

/// The page on which the work starts; for example "135" or "xiii".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pageStart {
    Article(Article),
    Chapter(Chapter),
    PublicationIssue(PublicationIssue),
    PublicationVolume(PublicationVolume),
}
