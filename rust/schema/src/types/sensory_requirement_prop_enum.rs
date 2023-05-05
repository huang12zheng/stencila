use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::text::Text;
use super::url::URL;


/// http://schema.org/sensoryRequirement
/// A description of any sensory requirements and levels necessary to function on the job, including hearing and vision. Defined terms such as those in O*net may be used, but note that there is no way to specify the level of ability as well as its nature when using a defined term.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum SensoryRequirementPropEnum {
    DefinedTerm(DefinedTerm),
    Text(Text),
    URL(URL),
}
