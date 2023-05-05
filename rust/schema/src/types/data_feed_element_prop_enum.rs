use crate::prelude::*;

use super::data_feed_item::DataFeedItem;
use super::text::Text;
use super::thing::Thing;


/// http://schema.org/dataFeedElement
/// An item within a data feed. Data feeds may have many elements.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum DataFeedElementPropEnum {
    DataFeedItem(DataFeedItem),
    Text(Text),
    Thing(Thing),
}
