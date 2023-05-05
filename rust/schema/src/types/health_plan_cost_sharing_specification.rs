// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;

/// * MOD OF: https://pending.schema.org * COMMENT: A description of costs to the patient under a given network or formulary. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct HealthPlanCostSharingSpecification {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<HealthPlanCostSharingSpecificationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct HealthPlanCostSharingSpecificationOptions {
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

    /// Whether the coinsurance applies before or after deductible, etc. TODO: Is this a closed set?
    pub health_plan_coinsurance_option: Option<HealthPlanCostSharingSpecification>,

    /// The rate of coinsurance expressed as a number between 0.0 and 1.0.
    pub health_plan_coinsurance_rate: Option<HealthPlanCostSharingSpecification>,

    /// The copay amount.
    pub health_plan_copay: Option<HealthPlanCostSharingSpecification>,

    /// Whether the copay is before or after deductible, etc. TODO: Is this a closed set?
    pub health_plan_copay_option: Option<HealthPlanCostSharingSpecification>,

    /// The category or type of pharmacy associated with this cost sharing.
    pub health_plan_pharmacy_category: Option<HealthPlanCostSharingSpecification>,
}

impl HealthPlanCostSharingSpecification {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
