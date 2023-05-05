use crate::prelude::*;

use super::boolean::Boolean;
use super::text::Text;
use super::url::URL;

/// Indicates whether a FoodEstablishment accepts reservations. Values can be Boolean, an URL at which reservations can be made or (for backwards compatibility) the strings <code>Yes</code> or <code>No</code>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum acceptsReservations {
    Boolean(Boolean),
    Text(Text),
    URL(URL),
}
