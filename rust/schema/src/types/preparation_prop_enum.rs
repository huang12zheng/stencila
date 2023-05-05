use crate::prelude::*;

use super::medical_entity::MedicalEntity;
use super::text::Text;


/// http://schema.org/preparation
/// Typical preparation that a patient must undergo before having the procedure performed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum PreparationPropEnum {
    MedicalEntity(MedicalEntity),
    Text(Text),
}
