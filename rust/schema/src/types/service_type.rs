use crate::prelude::*;

use super::government_benefits_type::GovernmentBenefitsType;
use super::text::Text;

/// The type of service being offered, e.g. veterans' benefits, emergency relief, etc.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum serviceType {
    GovernmentBenefitsType(GovernmentBenefitsType),
    Text(Text),
}
