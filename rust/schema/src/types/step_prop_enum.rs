use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::how_to_section::HowToSection;
use super::how_to_step::HowToStep;
use super::text::Text;


/// http://schema.org/step
/// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum StepPropEnum {
    CreativeWork(CreativeWork),
    HowToSection(HowToSection),
    HowToStep(HowToStep),
    Text(Text),
}
