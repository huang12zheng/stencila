use crate::prelude::*;

use super::creative_work::CreativeWork;
use super::how_to_section::HowToSection;
use super::how_to_step::HowToStep;
use super::text::Text;

/// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum step {
    CreativeWork(CreativeWork),
    HowToSection(HowToSection),
    HowToStep(HowToStep),
    Text(Text),
}
