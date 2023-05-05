// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/Intangible
/// * COMMENT:
/// A utility class that serves as the umbrella for a number of 'intangible' things such as quantities, structured values, etc.
/// * EXTEND FROM:
/// https://schema.org/Thing
/// * LOOK ALSO:
/// https://schema.org/ActionAccessSpecification, https://schema.org/AlignmentObject, https://schema.org/Audience, https://schema.org/BedDetails, https://schema.org/Brand, https://schema.org/BroadcastChannel, https://schema.org/BroadcastFrequencySpecification, https://schema.org/Class, https://schema.org/ComputerLanguage, https://schema.org/DataFeedItem, https://schema.org/DefinedTerm, https://schema.org/Demand, https://schema.org/DigitalDocumentPermission, https://schema.org/EducationalOccupationalProgram, https://schema.org/EnergyConsumptionDetails, https://schema.org/EntryPoint, https://schema.org/Enumeration, https://schema.org/FloorPlan, https://schema.org/GameServer, https://schema.org/GeospatialGeometry, https://schema.org/Grant, https://schema.org/HealthInsurancePlan, https://schema.org/HealthPlanCostSharingSpecification, https://schema.org/HealthPlanFormulary, https://schema.org/HealthPlanNetwork, https://schema.org/Invoice, https://schema.org/ItemList, https://schema.org/JobPosting, https://schema.org/Language, https://schema.org/ListItem, https://schema.org/MediaSubscription, https://schema.org/MenuItem, https://schema.org/MerchantReturnPolicy, https://schema.org/MerchantReturnPolicySeasonalOverride, https://schema.org/Observation, https://schema.org/Occupation, https://schema.org/OccupationalExperienceRequirements, https://schema.org/Offer, https://schema.org/Order, https://schema.org/OrderItem, https://schema.org/ParcelDelivery, https://schema.org/Permit, https://schema.org/ProductReturnPolicy, https://schema.org/ProgramMembership, https://schema.org/Property, https://schema.org/PropertyValueSpecification, https://schema.org/Quantity, https://schema.org/Rating, https://schema.org/Reservation, https://schema.org/Role, https://schema.org/Schedule, https://schema.org/Seat, https://schema.org/Series, https://schema.org/Service, https://schema.org/ServiceChannel, https://schema.org/SpeakableSpecification, https://schema.org/StatisticalPopulation, https://schema.org/StructuredValue, https://schema.org/Ticket, https://schema.org/Trip, https://schema.org/VirtualLocation
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Intangible {
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
}

impl Intangible {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
