// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::boolean::Boolean;
use super::default_value_prop_enum::DefaultValuePropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::number::Number;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/PropertyValueSpecification
/// * COMMENT:
/// A Property value specification.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PropertyValueSpecification {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type_prop_enum: Option<URL>,

    /// An alias for the item.
    pub alternate_name_prop_enum: Option<Text>,

    /// A description of the item.
    pub description_prop_enum: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description_prop_enum: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier_prop_enum: Option<IdentifierPropEnum>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image_prop_enum: Option<ImagePropEnum>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page_prop_enum: Option<MainEntityOfPagePropEnum>,

    /// The name of the item.
    pub name_prop_enum: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action_prop_enum: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as_prop_enum: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of_prop_enum: Option<SubjectOfPropEnum>,

    /// URL of the item.
    pub url_prop_enum: Option<URL>,

    /// The default value of the input.  For properties that expect a literal, the default is a literal value, for properties that expect an object, it's an ID reference to one of the current values.
    pub default_value_prop_enum: Option<DefaultValuePropEnum>,

    /// The upper value of some characteristic or property.
    pub max_value_prop_enum: Option<Number>,

    /// The lower value of some characteristic or property.
    pub min_value_prop_enum: Option<Number>,

    /// Whether multiple values are allowed for the property.  Default is false.
    pub multiple_values_prop_enum: Option<Boolean>,

    /// Whether or not a property is mutable.  Default is false. Specifying this for a property that also has a value makes it act similar to a "hidden" input in an HTML form.
    pub readonly_value_prop_enum: Option<Boolean>,

    /// The stepValue attribute indicates the granularity that is expected (and required) of the value in a PropertyValueSpecification.
    pub step_value_prop_enum: Option<Number>,

    /// Specifies the allowed range for number of characters in a literal value.
    pub value_max_length_prop_enum: Option<Number>,

    /// Specifies the minimum allowed range for number of characters in a literal value.
    pub value_min_length_prop_enum: Option<Number>,

    /// Indicates the name of the PropertyValueSpecification to be used in URL templates and form encoding in a manner analogous to HTML's input@name.
    pub value_name_prop_enum: Option<Text>,

    /// Specifies a regular expression for testing literal values according to the HTML spec.
    pub value_pattern_prop_enum: Option<Text>,

    /// Whether the property must be filled in to complete the action.  Default is false.
    pub value_required_prop_enum: Option<Boolean>,
}

impl PropertyValueSpecification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
