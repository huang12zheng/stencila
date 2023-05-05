use crate::prelude::*;

use super::url::URL;
use super::web_content::WebContent;

/// Information about travel bans, e.g. in the context of a pandemic.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum travelBans {
    URL(URL),
    WebContent(WebContent),
}
