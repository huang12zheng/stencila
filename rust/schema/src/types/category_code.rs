// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::defined_term::DefinedTerm;
use super::thing::Thing;
use super::code_value::codeValue;

/// * MOD OF: https://pending.schema.org * COMMENT: A Category Code. * EXTEND FROM: https://schema.org/DefinedTerm * LOOK ALSO: https://schema.org/MedicalCode
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct CategoryCode {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<CategoryCodeOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct CategoryCodeOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<Thing>,

    /// An alias for the item.
    pub alternate_name: Option<Thing>,

    /// A description of the item.
    pub description: Option<Thing>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Thing>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<Thing>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<Thing>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<Thing>,

    /// The name of the item.
    pub name: Option<Thing>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Thing>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<Thing>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<Thing>,

    /// URL of the item.
    pub url: Option<Thing>,

    /// A <a class="localLink" href="/DefinedTermSet">DefinedTermSet</a> that contains this term.
    pub in_defined_term_set: Option<DefinedTerm>,

    /// A code that identifies this <a class="localLink" href="/DefinedTerm">DefinedTerm</a> within a <a class="localLink" href="/DefinedTermSet">DefinedTermSet</a>
    pub term_code: Option<DefinedTerm>,

    /// A short textual code that uniquely identifies the value.
    pub code_value: Option<codeValue>,

    /// A <a class="localLink" href="/CategoryCodeSet">CategoryCodeSet</a> that contains this category code.
    pub in_code_set: Option<CategoryCode>,
}

impl CategoryCode {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
