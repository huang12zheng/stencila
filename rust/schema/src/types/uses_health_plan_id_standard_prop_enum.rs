use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/usesHealthPlanIdStandard
/// The standard for interpreting the Plan ID. The preferred is "HIOS". See the Centers for Medicare &amp; Medicaid Services for more details.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum UsesHealthPlanIdStandardPropEnum {
    Text(Text),
    URL(URL),
}