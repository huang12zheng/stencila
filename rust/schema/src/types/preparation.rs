use crate::prelude::*;

use super::medical_entity::MedicalEntity;
use super::text::Text;

/// Typical preparation that a patient must undergo before having the procedure performed.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum preparation {
    MedicalEntity(MedicalEntity),
    Text(Text),
}
