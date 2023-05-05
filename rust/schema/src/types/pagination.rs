use crate::prelude::*;

use super::article::Article;
use super::chapter::Chapter;
use super::publication_issue::PublicationIssue;
use super::publication_volume::PublicationVolume;

/// Any description of pages that is not separated into pageStart and pageEnd; for example, "1-6, 9, 55" or "10-12, 46-49".
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum pagination {
    Article(Article),
    Chapter(Chapter),
    PublicationIssue(PublicationIssue),
    PublicationVolume(PublicationVolume),
}
