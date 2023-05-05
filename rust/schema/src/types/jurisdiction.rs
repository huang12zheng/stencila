use crate::prelude::*;

use super::government_service::GovernmentService;
use super::legislation::Legislation;

/// Indicates a legal jurisdiction, e.g. of some legislation, or where some government service is based.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum jurisdiction {
    GovernmentService(GovernmentService),
    Legislation(Legislation),
}
