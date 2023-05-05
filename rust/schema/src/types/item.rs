use crate::prelude::*;

use super::data_feed_item::DataFeedItem;
use super::list_item::ListItem;

/// An entity represented by an entry in a list or data feed (e.g. an 'artist' in a list of 'artists').
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum item {
    DataFeedItem(DataFeedItem),
    ListItem(ListItem),
}
