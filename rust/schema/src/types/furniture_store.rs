// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::actionable_feedback_policy_prop_enum::ActionableFeedbackPolicyPropEnum;
use super::address_prop_enum::AddressPropEnum;
use super::aggregate_rating::AggregateRating;
use super::area_served_prop_enum::AreaServedPropEnum;
use super::boolean::Boolean;
use super::brand_prop_enum::BrandPropEnum;
use super::contact_point::ContactPoint;
use super::corrections_policy_prop_enum::CorrectionsPolicyPropEnum;
use super::date::Date;
use super::demand::Demand;
use super::diversity_policy_prop_enum::DiversityPolicyPropEnum;
use super::diversity_staffing_report_prop_enum::DiversityStaffingReportPropEnum;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::ethics_policy_prop_enum::EthicsPolicyPropEnum;
use super::event::Event;
use super::funder_prop_enum::FunderPropEnum;
use super::geo_contains_prop_enum::GeoContainsPropEnum;
use super::geo_covered_by_prop_enum::GeoCoveredByPropEnum;
use super::geo_covers_prop_enum::GeoCoversPropEnum;
use super::geo_crosses_prop_enum::GeoCrossesPropEnum;
use super::geo_disjoint_prop_enum::GeoDisjointPropEnum;
use super::geo_equals_prop_enum::GeoEqualsPropEnum;
use super::geo_intersects_prop_enum::GeoIntersectsPropEnum;
use super::geo_overlaps_prop_enum::GeoOverlapsPropEnum;
use super::geo_prop_enum::GeoPropEnum;
use super::geo_touches_prop_enum::GeoTouchesPropEnum;
use super::geo_within_prop_enum::GeoWithinPropEnum;
use super::grant::Grant;
use super::has_map_prop_enum::HasMapPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::integer::Integer;
use super::interaction_counter::InteractionCounter;
use super::keywords_prop_enum::KeywordsPropEnum;
use super::knows_about_prop_enum::KnowsAboutPropEnum;
use super::knows_language_prop_enum::KnowsLanguagePropEnum;
use super::latitude_prop_enum::LatitudePropEnum;
use super::location_feature_specification::LocationFeatureSpecification;
use super::location_prop_enum::LocationPropEnum;
use super::logo_prop_enum::LogoPropEnum;
use super::longitude_prop_enum::LongitudePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::member_of_prop_enum::MemberOfPropEnum;
use super::member_prop_enum::MemberPropEnum;
use super::merchant_return_policy::MerchantReturnPolicy;
use super::nonprofit_type::NonprofitType;
use super::offer::Offer;
use super::offer_catalog::OfferCatalog;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::organization::Organization;
use super::ownership_funding_info_prop_enum::OwnershipFundingInfoPropEnum;
use super::owns_prop_enum::OwnsPropEnum;
use super::person::Person;
use super::photo_prop_enum::PhotoPropEnum;
use super::place::Place;
use super::property_value::PropertyValue;
use super::publishing_principles_prop_enum::PublishingPrinciplesPropEnum;
use super::quantitative_value::QuantitativeValue;
use super::review::Review;
use super::sponsor_prop_enum::SponsorPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;
use super::unnamed_sources_policy_prop_enum::UnnamedSourcesPolicyPropEnum;

/// https://schema.org/FurnitureStore
/// * COMMENT:
/// A furniture store.
/// * EXTEND FROM:
/// https://schema.org/Store
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct FurnitureStore {
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

    /// For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> or other news-related <a class="localLink" href="/Organization">Organization</a>, a statement about public engagement activities (for news media, the newsroom’s), including involving the public - digitally or otherwise -- in coverage decisions, reporting and activities after publication.
    pub actionable_feedback_policy_prop_enum: Option<ActionableFeedbackPolicyPropEnum>,

    /// Physical address of the item.
    pub address_prop_enum: Option<AddressPropEnum>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating_prop_enum: Option<AggregateRating>,

    /// Alumni of an organization.
    pub alumni_prop_enum: Option<Person>,

    /// The geographic area where a service or offered item is provided.
    pub area_served_prop_enum: Option<AreaServedPropEnum>,

    /// An award won by or for this item.
    pub award_prop_enum: Option<Text>,

    /// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
    pub brand_prop_enum: Option<BrandPropEnum>,

    /// A contact point for a person or organization.
    pub contact_point_prop_enum: Option<ContactPoint>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (e.g. <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement describing (in news media, the newsroom’s) disclosure and correction policy for errors.
    pub corrections_policy_prop_enum: Option<CorrectionsPolicyPropEnum>,

    /// A relationship between an organization and a department of that organization, also described as an organization (allowing different urls, logos, opening hours). For example: a store with a pharmacy, or a bakery with a cafe.
    pub department_prop_enum: Option<Organization>,

    /// The date that this organization was dissolved.
    pub dissolution_date_prop_enum: Option<Date>,

    /// Statement on diversity policy by an <a class="localLink" href="/Organization">Organization</a> e.g. a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>. For a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, a statement describing the newsroom’s diversity policy on both staffing and sources, typically providing staffing data.
    pub diversity_policy_prop_enum: Option<DiversityPolicyPropEnum>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (often but not necessarily a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a report on staffing diversity issues. In a news context this might be for example ASNE or RTDNA (US) reports, or self-reported.
    pub diversity_staffing_report_prop_enum: Option<DiversityStaffingReportPropEnum>,

    /// The Dun &amp; Bradstreet DUNS number for identifying an organization or business person.
    pub duns_prop_enum: Option<Text>,

    /// Email address.
    pub email_prop_enum: Option<Text>,

    /// Someone working for this organization.
    pub employee_prop_enum: Option<Person>,

    /// Statement about ethics policy, e.g. of a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a> regarding journalistic and publishing practices, or of a <a class="localLink" href="/Restaurant">Restaurant</a>, a page describing food source policies. In the case of a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>, an ethicsPolicy is typically a statement describing the personal, organizational, and corporate standards of behavior expected by the organization.
    pub ethics_policy_prop_enum: Option<EthicsPolicyPropEnum>,

    /// Upcoming or past event associated with this place, organization, or action.
    pub event_prop_enum: Option<Event>,

    /// The fax number.
    pub fax_number_prop_enum: Option<Text>,

    /// A person who founded this organization.
    pub founder_prop_enum: Option<Person>,

    /// The date that this organization was founded.
    pub founding_date_prop_enum: Option<Date>,

    /// The place where the Organization was founded.
    pub founding_location_prop_enum: Option<Place>,

    /// A person or organization that supports (sponsors) something through some kind of financial contribution.
    pub funder_prop_enum: Option<FunderPropEnum>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding_prop_enum: Option<Grant>,

    /// The <a href="http://www.gs1.org/gln">Global Location Number</a> (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
    pub global_location_number_prop_enum: Option<Text>,

    /// A credential awarded to the Person or Organization.
    pub has_credential_prop_enum: Option<EducationalOccupationalCredential>,

    /// Specifies a MerchantReturnPolicy that may be applicable.
    pub has_merchant_return_policy_prop_enum: Option<MerchantReturnPolicy>,

    /// Indicates an OfferCatalog listing for this Organization, Person, or Service.
    pub has_offer_catalog_prop_enum: Option<OfferCatalog>,

    /// Points-of-Sales operated by the organization or person.
    pub has_pos_prop_enum: Option<Place>,

    /// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
    pub interaction_statistic_prop_enum: Option<InteractionCounter>,

    /// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
    pub isic_v4_prop_enum: Option<Text>,

    /// An organization identifier as defined in ISO 6523(-1). Note that many existing organization identifiers such as <a href="http://schema.org/leiCode">leiCode</a>, <a href="http://schema.org/duns">duns</a> and <a href="http://schema.org/vatID">vatID</a> can be expressed as an ISO 6523 identifier by setting the ICD part of the ISO 6523 identifier accordingly.
    pub iso_6523_code_prop_enum: Option<Text>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords_prop_enum: Option<KeywordsPropEnum>,

    /// Of a <a class="localLink" href="/Person">Person</a>, and less typically of an <a class="localLink" href="/Organization">Organization</a>, to indicate a topic that is known about - suggesting possible expertise but not implying it. We do not distinguish skill levels here, or relate this to educational content, events, objectives or <a class="localLink" href="/JobPosting">JobPosting</a> descriptions.
    pub knows_about_prop_enum: Option<KnowsAboutPropEnum>,

    /// Of a <a class="localLink" href="/Person">Person</a>, and less typically of an <a class="localLink" href="/Organization">Organization</a>, to indicate a known language. We do not distinguish skill levels or reading/writing/speaking/signing here. Use language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>.
    pub knows_language_prop_enum: Option<KnowsLanguagePropEnum>,

    /// The official name of the organization, e.g. the registered company name.
    pub legal_name_prop_enum: Option<Text>,

    /// An organization identifier that uniquely identifies a legal entity as defined in ISO 17442.
    pub lei_code_prop_enum: Option<Text>,

    /// The location of, for example, where an event is happening, where an organization is located, or where an action takes place.
    pub location_prop_enum: Option<LocationPropEnum>,

    /// An associated logo.
    pub logo_prop_enum: Option<LogoPropEnum>,

    /// A pointer to products or services offered by the organization or person.
    pub makes_offer_prop_enum: Option<Offer>,

    /// A member of an Organization or a ProgramMembership. Organizations can be members of organizations; ProgramMembership is typically for individuals.
    pub member_prop_enum: Option<MemberPropEnum>,

    /// An Organization (or ProgramMembership) to which this Person or Organization belongs.
    pub member_of_prop_enum: Option<MemberOfPropEnum>,

    /// The North American Industry Classification System (NAICS) code for a particular organization or business person.
    pub naics_prop_enum: Option<Text>,

    /// nonprofitStatus indicates the legal status of a non-profit organization in its primary place of business.
    pub nonprofit_status_prop_enum: Option<NonprofitType>,

    /// The number of employees in an organization, e.g. business.
    pub number_of_employees_prop_enum: Option<QuantitativeValue>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (often but not necessarily a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a description of organizational ownership structure; funding and grants. In a news/media setting, this is with particular reference to editorial independence.   Note that the <a class="localLink" href="/funder">funder</a> is also available and can be used to make basic funder information machine-readable.
    pub ownership_funding_info_prop_enum: Option<OwnershipFundingInfoPropEnum>,

    /// Products owned by the organization or person.
    pub owns_prop_enum: Option<OwnsPropEnum>,

    /// The larger organization that this organization is a <a class="localLink" href="/subOrganization">subOrganization</a> of, if any.
    pub parent_organization_prop_enum: Option<Organization>,

    /// The publishingPrinciples property indicates (typically via <a class="localLink" href="/URL">URL</a>) a document describing the editorial principles of an <a class="localLink" href="/Organization">Organization</a> (or individual, e.g. a <a class="localLink" href="/Person">Person</a> writing a blog) that relate to their activities as a publisher, e.g. ethics or diversity policies. When applied to a <a class="localLink" href="/CreativeWork">CreativeWork</a> (e.g. <a class="localLink" href="/NewsArticle">NewsArticle</a>) the principles are those of the party primarily responsible for the creation of the <a class="localLink" href="/CreativeWork">CreativeWork</a>.<br/><br/>  While such policies are most typically expressed in natural language, sometimes related information (e.g. indicating a <a class="localLink" href="/funder">funder</a>) can be expressed using schema.org terminology.
    pub publishing_principles_prop_enum: Option<PublishingPrinciplesPropEnum>,

    /// A review of the item.
    pub review_prop_enum: Option<Review>,

    /// A pointer to products or services sought by the organization or person (demand).
    pub seeks_prop_enum: Option<Demand>,

    /// A slogan or motto associated with the item.
    pub slogan_prop_enum: Option<Text>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor_prop_enum: Option<SponsorPropEnum>,

    /// A relationship between two organizations where the first includes the second, e.g., as a subsidiary. See also: the more specific 'department' property.
    pub sub_organization_prop_enum: Option<Organization>,

    /// The Tax / Fiscal ID of the organization or person, e.g. the TIN in the US or the CIF/NIF in Spain.
    pub tax_id_prop_enum: Option<Text>,

    /// The telephone number.
    pub telephone_prop_enum: Option<Text>,

    /// For an <a class="localLink" href="/Organization">Organization</a> (typically a <a class="localLink" href="/NewsMediaOrganization">NewsMediaOrganization</a>), a statement about policy on use of unnamed sources and the decision process required.
    pub unnamed_sources_policy_prop_enum: Option<UnnamedSourcesPolicyPropEnum>,

    /// The Value-added Tax ID of the organization or person.
    pub vat_id_prop_enum: Option<Text>,

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property_prop_enum: Option<PropertyValue>,

    /// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
    pub amenity_feature_prop_enum: Option<LocationFeatureSpecification>,

    /// A short textual code (also called "store code") that uniquely identifies a place of business. The code is typically assigned by the parentOrganization and used in structured URLs.<br/><br/>  For example, in the URL http://www.starbucks.co.uk/store-locator/etc/detail/3047 the code "3047" is a branchCode for a particular branch.
    pub branch_code_prop_enum: Option<Text>,

    /// The basic containment relation between a place and one that contains it.
    pub contained_in_place_prop_enum: Option<Place>,

    /// The basic containment relation between a place and another that it contains.
    pub contains_place_prop_enum: Option<Place>,

    /// The geo coordinates of the place.
    pub geo_prop_enum: Option<GeoPropEnum>,

    /// Represents a relationship between two geometries (or the places they represent), relating a containing geometry to a contained geometry. "a contains b iff no points of b lie in the exterior of a, and at least one point of the interior of b lies in the interior of a". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_contains_prop_enum: Option<GeoContainsPropEnum>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that covers it. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_covered_by_prop_enum: Option<GeoCoveredByPropEnum>,

    /// Represents a relationship between two geometries (or the places they represent), relating a covering geometry to a covered geometry. "Every point of b is a point of (the interior or boundary of) a". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_covers_prop_enum: Option<GeoCoversPropEnum>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that crosses it: "a crosses b: they have some but not all interior points in common, and the dimension of the intersection is less than that of at least one of them". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_crosses_prop_enum: Option<GeoCrossesPropEnum>,

    /// Represents spatial relations in which two geometries (or the places they represent) are topologically disjoint: "they have no point in common. They form a set of disconnected geometries." (A symmetric relationship, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.)
    pub geo_disjoint_prop_enum: Option<GeoDisjointPropEnum>,

    /// Represents spatial relations in which two geometries (or the places they represent) are topologically equal, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>. "Two geometries are topologically equal if their interiors intersect and no part of the interior or boundary of one geometry intersects the exterior of the other" (a symmetric relationship).
    pub geo_equals_prop_enum: Option<GeoEqualsPropEnum>,

    /// Represents spatial relations in which two geometries (or the places they represent) have at least one point in common. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_intersects_prop_enum: Option<GeoIntersectsPropEnum>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that geospatially overlaps it, i.e. they have some but not all points in common. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_overlaps_prop_enum: Option<GeoOverlapsPropEnum>,

    /// Represents spatial relations in which two geometries (or the places they represent) touch: "they have at least one boundary point in common, but no interior points." (A symmetric relationship, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.)
    pub geo_touches_prop_enum: Option<GeoTouchesPropEnum>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to one that contains it, i.e. it is inside (i.e. within) its interior. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_within_prop_enum: Option<GeoWithinPropEnum>,

    /// Indicates whether some facility (e.g. <a class="localLink" href="/FoodEstablishment">FoodEstablishment</a>, <a class="localLink" href="/CovidTestingFacility">CovidTestingFacility</a>) offers a service that can be used by driving through in a car. In the case of <a class="localLink" href="/CovidTestingFacility">CovidTestingFacility</a> such facilities could potentially help with social distancing from other potentially-infected users.
    pub has_drive_through_service_prop_enum: Option<Boolean>,

    /// A URL to a map of the place.
    pub has_map_prop_enum: Option<HasMapPropEnum>,

    /// A flag to signal that the item, event, or place is accessible for free.
    pub is_accessible_for_free_prop_enum: Option<Boolean>,

    /// The latitude of a location. For example <code>37.42242</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
    pub latitude_prop_enum: Option<LatitudePropEnum>,

    /// The longitude of a location. For example <code>-122.08585</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
    pub longitude_prop_enum: Option<LongitudePropEnum>,

    /// The total number of individuals that may attend an event or venue.
    pub maximum_attendee_capacity_prop_enum: Option<Integer>,

    /// The opening hours of a certain place.
    pub opening_hours_specification_prop_enum: Option<OpeningHoursSpecification>,

    /// A photograph of this place.
    pub photo_prop_enum: Option<PhotoPropEnum>,

    /// A flag to signal that the <a class="localLink" href="/Place">Place</a> is open to public visitors.  If this property is omitted there is no assumed default boolean value
    pub public_access_prop_enum: Option<Boolean>,

    /// Indicates whether it is allowed to smoke in the place, e.g. in the restaurant, hotel or hotel room.
    pub smoking_allowed_prop_enum: Option<Boolean>,

    /// The special opening hours of a certain place.<br/><br/>  Use this to explicitly override general opening hours brought in scope by <a class="localLink" href="/openingHoursSpecification">openingHoursSpecification</a> or <a class="localLink" href="/openingHours">openingHours</a>.
    pub special_opening_hours_specification_prop_enum: Option<OpeningHoursSpecification>,

    /// A page providing information on how to book a tour of some <a class="localLink" href="/Place">Place</a>, such as an <a class="localLink" href="/Accommodation">Accommodation</a> or <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> in a real estate setting, as well as other kinds of tours as appropriate.
    pub tour_booking_page_prop_enum: Option<URL>,

    /// The currency accepted.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
    pub currencies_accepted_prop_enum: Option<Text>,

    /// The general opening hours for a business. Opening hours can be specified as a weekly time range, starting with days, then times per day. Multiple days can be listed with commas ',' separating each day. Day or time ranges are specified using a hyphen '-'.<br/><br/>  <ul> <li>Days are specified using the following two-letter combinations: <code>Mo</code>, <code>Tu</code>, <code>We</code>, <code>Th</code>, <code>Fr</code>, <code>Sa</code>, <code>Su</code>.</li> <li>Times are specified using 24:00 format. For example, 3pm is specified as <code>15:00</code>, 10am as <code>10:00</code>. </li> <li>Here is an example: <code>&lt;time itemprop="openingHours" datetime=&quot;Tu,Th 16:00-20:00&quot;&gt;Tuesdays and Thursdays 4-8pm&lt;/time&gt;</code>.</li> <li>If a business is open 7 days a week, then it can be specified as <code>&lt;time itemprop=&quot;openingHours&quot; datetime=&quot;Mo-Su&quot;&gt;Monday through Sunday, all day&lt;/time&gt;</code>.</li> </ul>
    pub opening_hours_prop_enum: Option<Text>,

    /// Cash, Credit Card, Cryptocurrency, Local Exchange Tradings System, etc.
    pub payment_accepted_prop_enum: Option<Text>,

    /// The price range of the business, for example <code>$$$</code>.
    pub price_range_prop_enum: Option<Text>,
}

impl FurnitureStore {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
