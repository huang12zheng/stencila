use crate::prelude::*;

use super::boolean::Boolean;
use super::text::Text;
use super::url::URL;


/// http://schema.org/acceptsReservations
/// Indicates whether a FoodEstablishment accepts reservations. Values can be Boolean, an URL at which reservations can be made or (for backwards compatibility) the strings <code>Yes</code> or <code>No</code>.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AcceptsReservationsPropEnum {
    Boolean(Boolean),
    Text(Text),
    URL(URL),
}
