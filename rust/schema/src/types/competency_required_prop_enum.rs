use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;
use super::url::URL;


/// http://schema.org/competencyRequired
/// Knowledge, skill, ability or personal attribute that must be demonstrated by a person or other entity in order to do something such as earn an Educational Occupational Credential or understand a LearningResource.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum CompetencyRequiredPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
    URL(URL),
}
