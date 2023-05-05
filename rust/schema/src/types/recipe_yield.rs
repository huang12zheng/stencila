use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;

/// The quantity produced by the recipe (for example, number of people served, number of servings, etc).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(untagged, crate = "common::serde")]

pub enum recipeYield {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
