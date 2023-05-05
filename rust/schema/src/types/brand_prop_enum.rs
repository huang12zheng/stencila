use crate::prelude::*;

use super::brand::Brand;
use super::organization::Organization;


/// http://schema.org/brand
/// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum BrandPropEnum {
    Brand(Brand),
    Organization(Organization),
}
