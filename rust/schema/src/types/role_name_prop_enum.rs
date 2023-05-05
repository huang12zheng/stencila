use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/roleName
/// A role played, performed or filled by a person or organization. For example, the team of creators for a comic book might fill the roles named 'inker', 'penciller', and 'letterer'; or an athlete in a SportsTeam might play in the position named 'Quarterback'.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RoleNamePropEnum {
    Text(Text),
    URL(URL),
}
