use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::data_feed_item::DataFeedItem;

/// The date on which the CreativeWork was created or the item was added to a DataFeed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum dateCreated {
    CreativeWork(CreativeWork),
    DataFeedItem(DataFeedItem),
}
