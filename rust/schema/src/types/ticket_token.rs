use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// Reference to an asset (e.g., Barcode, QR code image or PDF) usable for entrance.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum ticketToken {
    Text(Text),
    URL(URL),
}
