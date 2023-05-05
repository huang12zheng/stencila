// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::aggregate_rating::AggregateRating;
use super::annual_percentage_rate_prop_enum::AnnualPercentageRatePropEnum;
use super::area_served_prop_enum::AreaServedPropEnum;
use super::audience::Audience;
use super::brand_prop_enum::BrandPropEnum;
use super::broker_prop_enum::BrokerPropEnum;
use super::category_prop_enum::CategoryPropEnum;
use super::fees_and_commissions_specification_prop_enum::FeesAndCommissionsSpecificationPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::interest_rate_prop_enum::InterestRatePropEnum;
use super::is_related_to_prop_enum::IsRelatedToPropEnum;
use super::is_similar_to_prop_enum::IsSimilarToPropEnum;
use super::logo_prop_enum::LogoPropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::offer_catalog::OfferCatalog;
use super::offers_prop_enum::OffersPropEnum;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::provider_prop_enum::ProviderPropEnum;
use super::review::Review;
use super::service_channel::ServiceChannel;
use super::service_type_prop_enum::ServiceTypePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::terms_of_service_prop_enum::TermsOfServicePropEnum;
use super::text::Text;
use super::thing::Thing;
use super::url::URL;

/// https://schema.org/FinancialProduct
/// * COMMENT:
/// A product provided to consumers and businesses by financial institutions such as banks, insurance companies, brokerage firms, consumer finance companies, and investment companies which comprise the financial services industry.
/// * EXTEND FROM:
/// https://schema.org/Service
/// * LOOK ALSO:
/// https://schema.org/BankAccount, https://schema.org/CurrencyConversionService, https://schema.org/InvestmentOrDeposit, https://schema.org/LoanOrCredit, https://schema.org/PaymentCard, https://schema.org/PaymentService
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct FinancialProduct {
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

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating_prop_enum: Option<AggregateRating>,

    /// The geographic area where a service or offered item is provided.
    pub area_served_prop_enum: Option<AreaServedPropEnum>,

    /// An intended audience, i.e. a group for whom something was created.
    pub audience_prop_enum: Option<Audience>,

    /// A means of accessing the service (e.g. a phone bank, a web site, a location, etc.).
    pub available_channel_prop_enum: Option<ServiceChannel>,

    /// An award won by or for this item.
    pub award_prop_enum: Option<Text>,

    /// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
    pub brand_prop_enum: Option<BrandPropEnum>,

    /// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
    pub broker_prop_enum: Option<BrokerPropEnum>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category_prop_enum: Option<CategoryPropEnum>,

    /// Indicates an OfferCatalog listing for this Organization, Person, or Service.
    pub has_offer_catalog_prop_enum: Option<OfferCatalog>,

    /// The hours during which this service or contact is available.
    pub hours_available_prop_enum: Option<OpeningHoursSpecification>,

    /// A pointer to another, somehow related product (or multiple products).
    pub is_related_to_prop_enum: Option<IsRelatedToPropEnum>,

    /// A pointer to another, functionally similar product (or multiple products).
    pub is_similar_to_prop_enum: Option<IsSimilarToPropEnum>,

    /// An associated logo.
    pub logo_prop_enum: Option<LogoPropEnum>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers_prop_enum: Option<OffersPropEnum>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider_prop_enum: Option<ProviderPropEnum>,

    /// Indicates the mobility of a provided service (e.g. 'static', 'dynamic').
    pub provider_mobility_prop_enum: Option<Text>,

    /// A review of the item.
    pub review_prop_enum: Option<Review>,

    /// The tangible thing generated by the service, e.g. a passport, permit, etc.
    pub service_output_prop_enum: Option<Thing>,

    /// The type of service being offered, e.g. veterans' benefits, emergency relief, etc.
    pub service_type_prop_enum: Option<ServiceTypePropEnum>,

    /// A slogan or motto associated with the item.
    pub slogan_prop_enum: Option<Text>,

    /// Human-readable terms of service documentation.
    pub terms_of_service_prop_enum: Option<TermsOfServicePropEnum>,

    /// The annual rate that is charged for borrowing (or made by investing), expressed as a single percentage number that represents the actual yearly cost of funds over the term of a loan. This includes any fees or additional costs associated with the transaction.
    pub annual_percentage_rate_prop_enum: Option<AnnualPercentageRatePropEnum>,

    /// Description of fees, commissions, and other terms applied either to a class of financial product, or by a financial service organization.
    pub fees_and_commissions_specification_prop_enum: Option<FeesAndCommissionsSpecificationPropEnum>,

    /// The interest rate, charged or paid, applicable to the financial product. Note: This is different from the calculated annualPercentageRate.
    pub interest_rate_prop_enum: Option<InterestRatePropEnum>,
}

impl FinancialProduct {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
