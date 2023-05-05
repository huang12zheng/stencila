use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/ticketToken
/// Reference to an asset (e.g., Barcode, QR code image or PDF) usable for entrance.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum TicketTokenPropEnum {
    Text(Text),
    URL(URL),
}
