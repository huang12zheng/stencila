use crate::prelude::*;

use super::government_benefits_type::GovernmentBenefitsType;
use super::text::Text;


/// http://schema.org/serviceType
/// The type of service being offered, e.g. veterans' benefits, emergency relief, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum ServiceTypePropEnum {
    GovernmentBenefitsType(GovernmentBenefitsType),
    Text(Text),
}
