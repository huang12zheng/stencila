use crate::prelude::*;

use super::data_feed::DataFeed;
use super::url::URL;

/// The URL for a feed, e.g. associated with a podcast series, blog, or series of date-stamped updates. This is usually RSS or Atom.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum webFeed {
    DataFeed(DataFeed),
    URL(URL),
}
