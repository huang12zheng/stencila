use crate::prelude::*;

use super::text::Text;
use super::url::URL;


/// http://schema.org/featureList
/// Features or modules provided by this application (and possibly required by other applications).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum FeatureListPropEnum {
    Text(Text),
    URL(URL),
}
