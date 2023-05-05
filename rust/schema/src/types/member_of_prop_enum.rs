use crate::prelude::*;

use super::organization::Organization;
use super::program_membership::ProgramMembership;


/// http://schema.org/memberOf
/// An Organization (or ProgramMembership) to which this Person or Organization belongs.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum MemberOfPropEnum {
    Organization(Organization),
    ProgramMembership(ProgramMembership),
}
