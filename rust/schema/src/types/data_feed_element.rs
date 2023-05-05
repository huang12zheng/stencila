use crate::prelude::*;

use super::data_feed_item::DataFeedItem;
use super::text::Text;
use super::thing::Thing;

/// An item within a data feed. Data feeds may have many elements.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum dataFeedElement {
    DataFeedItem(DataFeedItem),
    Text(Text),
    Thing(Thing),
}
