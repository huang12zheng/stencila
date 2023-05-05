// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::contact_point::ContactPoint;
use super::health_plan_formulary::HealthPlanFormulary;
use super::health_plan_network::HealthPlanNetwork;
use super::text::Text;
use super::url::URL;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;
use super::uses_health_plan_id_standard::usesHealthPlanIdStandard;

/// * MOD OF: https://pending.schema.org * COMMENT: A US-style health insurance plan, including PPOs, EPOs, and HMOs. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct HealthInsurancePlan {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<HealthInsurancePlanOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct HealthInsurancePlanOptions {
    /// An additional type for the item, typically used for adding more specific types from external vocabularies in microdata syntax. This is a relationship between something and a class that the thing is in. In RDFa syntax, it is better to use the native RDFa syntax - the 'typeof' attribute - for multiple types. Schema.org tools may have only weaker understanding of extra types, in particular those defined externally.
    pub additional_type: Option<URL>,

    /// An alias for the item.
    pub alternate_name: Option<Text>,

    /// A description of the item.
    pub description: Option<Text>,

    /// A sub property of description. A short description of the item used to disambiguate from other, similar items. Information from other properties (in particular, name) may be necessary for the description to be useful for disambiguation.
    pub disambiguating_description: Option<Text>,

    /// The identifier property represents any kind of identifier for any kind of <a class="localLink" href="/Thing">Thing</a>, such as ISBNs, GTIN codes, UUIDs etc. Schema.org provides dedicated properties for representing many of these, either as textual strings or as URL (URI) links. See <a href="/docs/datamodel.html#identifierBg">background notes</a> for more details.
    pub identifier: Option<identifier>,

    /// An image of the item. This can be a <a class="localLink" href="/URL">URL</a> or a fully described <a class="localLink" href="/ImageObject">ImageObject</a>.
    pub image: Option<image>,

    /// Indicates a page (or other CreativeWork) for which this thing is the main entity being described. See <a href="/docs/datamodel.html#mainEntityBackground">background notes</a> for details.
    pub main_entity_of_page: Option<mainEntityOfPage>,

    /// The name of the item.
    pub name: Option<Text>,

    /// Indicates a potential Action, which describes an idealized action in which this thing would play an 'object' role.
    pub potential_action: Option<Action>,

    /// URL of a reference Web page that unambiguously indicates the item's identity. E.g. the URL of the item's Wikipedia page, Wikidata entry, or official website.
    pub same_as: Option<URL>,

    /// A CreativeWork or Event about this Thing.
    pub subject_of: Option<subjectOf>,

    /// URL of the item.
    pub url: Option<URL>,

    /// The URL that goes directly to the summary of benefits and coverage for the specific standard plan or plan variation.
    pub benefits_summary_url: Option<URL>,

    /// A contact point for a person or organization.
    pub contact_point: Option<ContactPoint>,

    /// TODO.
    pub health_plan_drug_option: Option<Text>,

    /// The tier(s) of drugs offered by this formulary or insurance plan.
    pub health_plan_drug_tier: Option<Text>,

    /// The 14-character, HIOS-generated Plan ID number. (Plan IDs must be unique, even across different markets.)
    pub health_plan_id: Option<Text>,

    /// The URL that goes directly to the plan brochure for the specific standard plan or plan variation.
    pub health_plan_marketing_url: Option<URL>,

    /// Formularies covered by this plan.
    pub includes_health_plan_formulary: Option<HealthPlanFormulary>,

    /// Networks covered by this plan.
    pub includes_health_plan_network: Option<HealthPlanNetwork>,

    /// The standard for interpreting the Plan ID. The preferred is "HIOS". See the Centers for Medicare &amp; Medicaid Services for more details.
    pub uses_health_plan_id_standard: Option<usesHealthPlanIdStandard>,
}

impl HealthInsurancePlan {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
