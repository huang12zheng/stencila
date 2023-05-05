use crate::prelude::*;

use super::category_code::CategoryCode;
use super::medical_code::MedicalCode;

/// A short textual code that uniquely identifies the value.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum codeValue {
    CategoryCode(CategoryCode),
    MedicalCode(MedicalCode),
}
