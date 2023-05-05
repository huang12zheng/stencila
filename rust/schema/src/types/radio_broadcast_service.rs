// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::aggregate_rating::AggregateRating;
use super::audience::Audience;
use super::broadcast_channel::BroadcastChannel;
use super::broadcast_service::BroadcastService;
use super::offer_catalog::OfferCatalog;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::organization::Organization;
use super::review::Review;
use super::service_channel::ServiceChannel;
use super::text::Text;
use super::thing::Thing;
use super::url::URL;
use super::area_served::areaServed;
use super::brand::brand;
use super::broadcast_frequency::broadcastFrequency;
use super::broker::broker;
use super::category::category;
use super::identifier::identifier;
use super::image::image;
use super::in_language::inLanguage;
use super::is_related_to::isRelatedTo;
use super::is_similar_to::isSimilarTo;
use super::logo::logo;
use super::main_entity_of_page::mainEntityOfPage;
use super::offers::offers;
use super::provider::provider;
use super::service_type::serviceType;
use super::subject_of::subjectOf;
use super::terms_of_service::termsOfService;

/// * MOD OF: https://pending.schema.org * COMMENT: A delivery service through which radio content is provided via broadcast over the air or online. * EXTEND FROM: https://schema.org/BroadcastService
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct RadioBroadcastService {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<RadioBroadcastServiceOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct RadioBroadcastServiceOptions {
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

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: Option<AggregateRating>,

    /// The geographic area where a service or offered item is provided.
    pub area_served: Option<areaServed>,

    /// An intended audience, i.e. a group for whom something was created.
    pub audience: Option<Audience>,

    /// A means of accessing the service (e.g. a phone bank, a web site, a location, etc.).
    pub available_channel: Option<ServiceChannel>,

    /// An award won by or for this item.
    pub award: Option<Text>,

    /// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
    pub brand: Option<brand>,

    /// An entity that arranges for an exchange between a buyer and a seller.  In most cases a broker never acquires or releases ownership of a product or service involved in an exchange.  If it is not clear whether an entity is a broker, seller, or buyer, the latter two terms are preferred.
    pub broker: Option<broker>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category: Option<category>,

    /// Indicates an OfferCatalog listing for this Organization, Person, or Service.
    pub has_offer_catalog: Option<OfferCatalog>,

    /// The hours during which this service or contact is available.
    pub hours_available: Option<OpeningHoursSpecification>,

    /// A pointer to another, somehow related product (or multiple products).
    pub is_related_to: Option<isRelatedTo>,

    /// A pointer to another, functionally similar product (or multiple products).
    pub is_similar_to: Option<isSimilarTo>,

    /// An associated logo.
    pub logo: Option<logo>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers: Option<offers>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// Indicates the mobility of a provided service (e.g. 'static', 'dynamic').
    pub provider_mobility: Option<Text>,

    /// A review of the item.
    pub review: Option<Review>,

    /// The tangible thing generated by the service, e.g. a passport, permit, etc.
    pub service_output: Option<Thing>,

    /// The type of service being offered, e.g. veterans' benefits, emergency relief, etc.
    pub service_type: Option<serviceType>,

    /// A slogan or motto associated with the item.
    pub slogan: Option<Text>,

    /// Human-readable terms of service documentation.
    pub terms_of_service: Option<termsOfService>,

    /// The media network(s) whose content is broadcast on this station.
    pub broadcast_affiliate_of: Option<Organization>,

    /// The name displayed in the channel guide. For many US affiliates, it is the network name.
    pub broadcast_display_name: Option<Text>,

    /// The frequency used for over-the-air broadcasts. Numeric values or simple ranges, e.g. 87-99. In addition a shortcut idiom is supported for frequences of AM and FM radio channels, e.g. "87 FM".
    pub broadcast_frequency: Option<broadcastFrequency>,

    /// The timezone in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 format</a> for which the service bases its broadcasts.
    pub broadcast_timezone: Option<Text>,

    /// The organization owning or operating the broadcast service.
    pub broadcaster: Option<Organization>,

    /// A <a href="https://en.wikipedia.org/wiki/Call_sign">callsign</a>, as used in broadcasting and radio communications to identify people, radio and TV stations, or vehicles.
    pub call_sign: Option<Text>,

    /// A broadcast channel of a broadcast service.
    pub has_broadcast_channel: Option<BroadcastChannel>,

    /// The language of the content or performance or used in an action. Please use one of the language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>. See also <a class="localLink" href="/availableLanguage">availableLanguage</a>.
    pub in_language: Option<inLanguage>,

    /// A broadcast service to which the broadcast service may belong to such as regional variations of a national channel.
    pub parent_service: Option<BroadcastService>,

    /// The type of screening or video broadcast used (e.g. IMAX, 3D, SD, HD, etc.).
    pub video_format: Option<Text>,
}

impl RadioBroadcastService {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
