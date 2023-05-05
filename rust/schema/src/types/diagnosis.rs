use crate::prelude::*;

use super::d_dx_element::DDxElement;
use super::patient::Patient;

/// One or more alternative conditions considered in the differential diagnosis process as output of a diagnosis process.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum diagnosis {
    DDxElement(DDxElement),
    Patient(Patient),
}
