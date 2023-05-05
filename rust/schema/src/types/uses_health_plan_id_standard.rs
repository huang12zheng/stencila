use crate::prelude::*;

use super::text::Text;
use super::url::URL;

/// The standard for interpreting the Plan ID. The preferred is "HIOS". See the Centers for Medicare &amp; Medicaid Services for more details.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum usesHealthPlanIdStandard {
    Text(Text),
    URL(URL),
}
