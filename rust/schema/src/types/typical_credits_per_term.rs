use crate::prelude::*;

use super::integer::Integer;
use super::structured_value::StructuredValue;

/// The number of credits or units a full-time student would be expected to take in 1 term however 'term' is defined by the institution.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum typicalCreditsPerTerm {
    Integer(Integer),
    StructuredValue(StructuredValue),
}
