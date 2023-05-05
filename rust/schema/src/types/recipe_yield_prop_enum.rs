use crate::prelude::*;

use super::quantitative_value::QuantitativeValue;
use super::text::Text;


/// http://schema.org/recipeYield
/// The quantity produced by the recipe (for example, number of people served, number of servings, etc).
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[serde(untagged, crate = "common::serde")]

#[allow(non_camel_case_types)]
pub enum RecipeYieldPropEnum {
    QuantitativeValue(QuantitativeValue),
    Text(Text),
}
