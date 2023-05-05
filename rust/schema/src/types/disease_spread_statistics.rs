use crate::prelude::*;

use super::dataset::Dataset;
use super::observation::Observation;
use super::url::URL;
use super::web_content::WebContent;

/// Statistical information about the spread of a disease, either as <a class="localLink" href="/WebContent">WebContent</a>, or   described directly as a <a class="localLink" href="/Dataset">Dataset</a>, or the specific <a class="localLink" href="/Observation">Observation</a>s in the dataset. When a <a class="localLink" href="/WebContent">WebContent</a> URL is   provided, the page indicated might also contain more such markup.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum diseaseSpreadStatistics {
    Dataset(Dataset),
    Observation(Observation),
    URL(URL),
    WebContent(WebContent),
}
