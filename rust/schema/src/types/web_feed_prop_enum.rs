use crate::prelude::*;

use super::data_feed::DataFeed;
use super::url::URL;


/// http://schema.org/webFeed
/// The URL for a feed, e.g. associated with a podcast series, blog, or series of date-stamped updates. This is usually RSS or Atom.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum WebFeedPropEnum {
    DataFeed(DataFeed),
    URL(URL),
}
