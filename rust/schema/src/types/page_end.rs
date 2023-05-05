use crate::prelude::*;

use super::article::Article;
use super::chapter::Chapter;
use super::publication_issue::PublicationIssue;
use super::publication_volume::PublicationVolume;

/// The page on which the work ends; for example "138" or "xvi".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pageEnd {
    Article(Article),
    Chapter(Chapter),
    PublicationIssue(PublicationIssue),
    PublicationVolume(PublicationVolume),
}
