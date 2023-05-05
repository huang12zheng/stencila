use crate::prelude::*;

use super::organization::Organization;
use super::person::Person;


/// http://schema.org/author
/// The author of this content or rating. Please note that author is special in that HTML 5 provides a special mechanism for indicating authorship via the rel tag. That is equivalent to this and may be used interchangeably.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum AuthorPropEnum {
    Organization(Organization),
    Person(Person),
}
