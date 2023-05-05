// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::text::Text;
use super::url::URL;
use super::identifier::identifier;
use super::image::image;
use super::main_entity_of_page::mainEntityOfPage;
use super::subject_of::subjectOf;
use super::superseded_by::supersededBy;

/// * COMMENT: Lists or enumerations—for example, a list of cuisines or music genres, etc. * EXTEND FROM: https://schema.org/Intangible * LOOK ALSO: https://schema.org/AdultOrientedEnumeration, https://schema.org/BoardingPolicyType, https://schema.org/BookFormatType, https://schema.org/BusinessEntityType, https://schema.org/BusinessFunction, https://schema.org/CarUsageType, https://schema.org/ContactPointOption, https://schema.org/DayOfWeek, https://schema.org/DeliveryMethod, https://schema.org/DigitalDocumentPermissionType, https://schema.org/DigitalPlatformEnumeration, https://schema.org/EnergyEfficiencyEnumeration, https://schema.org/EventAttendanceModeEnumeration, https://schema.org/GameAvailabilityEnumeration, https://schema.org/GamePlayMode, https://schema.org/GenderType, https://schema.org/GovernmentBenefitsType, https://schema.org/HealthAspectEnumeration, https://schema.org/ItemAvailability, https://schema.org/ItemListOrderType, https://schema.org/LegalValueLevel, https://schema.org/MapCategoryType, https://schema.org/MeasurementTypeEnumeration, https://schema.org/MediaManipulationRatingEnumeration, https://schema.org/MedicalEnumeration, https://schema.org/MerchantReturnEnumeration, https://schema.org/MusicAlbumProductionType, https://schema.org/MusicAlbumReleaseType, https://schema.org/MusicReleaseFormatType, https://schema.org/NonprofitType, https://schema.org/OfferItemCondition, https://schema.org/PaymentMethod, https://schema.org/PhysicalActivityCategory, https://schema.org/PriceComponentTypeEnumeration, https://schema.org/PriceTypeEnumeration, https://schema.org/ProductReturnEnumeration, https://schema.org/QualitativeValue, https://schema.org/RefundTypeEnumeration, https://schema.org/RestrictedDiet, https://schema.org/ReturnFeesEnumeration, https://schema.org/ReturnLabelSourceEnumeration, https://schema.org/ReturnMethodEnumeration, https://schema.org/RsvpResponseType, https://schema.org/SizeGroupEnumeration, https://schema.org/SizeSystemEnumeration, https://schema.org/Specialty, https://schema.org/StatusEnumeration, https://schema.org/WarrantyScope
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Enumeration {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<EnumerationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct EnumerationOptions {
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

    /// Relates a term (i.e. a property, class or enumeration) to one that supersedes it.
    pub superseded_by: Option<supersededBy>,
}

impl Enumeration {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
