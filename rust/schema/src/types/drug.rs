use crate::prelude::*;

use super::drug_class::DrugClass;
use super::medical_condition::MedicalCondition;
use super::patient::Patient;
use super::therapeutic_procedure::TherapeuticProcedure;

/// Specifying a drug or medicine used in a medication procedure.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum drug {
    DrugClass(DrugClass),
    MedicalCondition(MedicalCondition),
    Patient(Patient),
    TherapeuticProcedure(TherapeuticProcedure),
}
