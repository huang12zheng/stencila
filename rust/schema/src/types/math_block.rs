// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::execution_digest::ExecutionDigest;
use super::string::String;

/// A block of math, e.g an equation, to be treated as block content.
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct MathBlock {
    /// The type of this item
    pub r#type: MustBe!("MathBlock"),

    /// The identifier for this item
    pub id: Option<String>,

    /// The language used for the equation e.g tex, mathml, asciimath.
    pub math_language: String,

    /// The code of the equation in the `mathLanguage`.
    pub code: String,

    /// A digest of the `code` and `mathLanguage` used to avoid unnecessary transpilation to MathML
    pub compile_digest: Option<ExecutionDigest>,

    /// Errors that occurred when parsing the math equation.
    pub errors: Option<Vec<String>>,

    /// The MathML transpiled from the `code`
    pub mathml: Option<String>,

    /// A short label for the math block.
    pub label: Option<String>,
}

impl MathBlock {
    pub fn new(math_language: String, code: String) -> Self {
        Self {
            math_language,
            code,
            ..Default::default()
        }
    }
}
