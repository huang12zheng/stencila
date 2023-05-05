use crate::prelude::*;

use super::blog::Blog;
use super::creative_work_series::CreativeWorkSeries;
use super::dataset::Dataset;
use super::web_site::WebSite;

/// The International Standard Serial Number (ISSN) that identifies this serial publication. You can repeat this property to identify different formats of, or the linking ISSN (ISSN-L) for, this serial publication.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum issn {
    Blog(Blog),
    CreativeWorkSeries(CreativeWorkSeries),
    Dataset(Dataset),
    WebSite(WebSite),
}
