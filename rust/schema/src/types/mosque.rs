// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::address_prop_enum::AddressPropEnum;
use super::aggregate_rating::AggregateRating;
use super::boolean::Boolean;
use super::event::Event;
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
use super::has_map_prop_enum::HasMapPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::integer::Integer;
use super::keywords_prop_enum::KeywordsPropEnum;
use super::latitude_prop_enum::LatitudePropEnum;
use super::location_feature_specification::LocationFeatureSpecification;
use super::logo_prop_enum::LogoPropEnum;
use super::longitude_prop_enum::LongitudePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::opening_hours_specification::OpeningHoursSpecification;
use super::photo_prop_enum::PhotoPropEnum;
use super::place::Place;
use super::property_value::PropertyValue;
use super::review::Review;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/Mosque
/// * COMMENT:
/// A mosque.
/// * EXTEND FROM:
/// https://schema.org/PlaceOfWorship
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Mosque {
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

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property_prop_enum: Option<PropertyValue>,

    /// Physical address of the item.
    pub address_prop_enum: Option<AddressPropEnum>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating_prop_enum: Option<AggregateRating>,

    /// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
    pub amenity_feature_prop_enum: Option<LocationFeatureSpecification>,

    /// A short textual code (also called "store code") that uniquely identifies a place of business. The code is typically assigned by the parentOrganization and used in structured URLs.<br/><br/>  For example, in the URL http://www.starbucks.co.uk/store-locator/etc/detail/3047 the code "3047" is a branchCode for a particular branch.
    pub branch_code_prop_enum: Option<Text>,

    /// The basic containment relation between a place and one that contains it.
    pub contained_in_place_prop_enum: Option<Place>,

    /// The basic containment relation between a place and another that it contains.
    pub contains_place_prop_enum: Option<Place>,

    /// Upcoming or past event associated with this place, organization, or action.
    pub event_prop_enum: Option<Event>,

    /// The fax number.
    pub fax_number_prop_enum: Option<Text>,

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

    /// The <a href="http://www.gs1.org/gln">Global Location Number</a> (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
    pub global_location_number_prop_enum: Option<Text>,

    /// Indicates whether some facility (e.g. <a class="localLink" href="/FoodEstablishment">FoodEstablishment</a>, <a class="localLink" href="/CovidTestingFacility">CovidTestingFacility</a>) offers a service that can be used by driving through in a car. In the case of <a class="localLink" href="/CovidTestingFacility">CovidTestingFacility</a> such facilities could potentially help with social distancing from other potentially-infected users.
    pub has_drive_through_service_prop_enum: Option<Boolean>,

    /// A URL to a map of the place.
    pub has_map_prop_enum: Option<HasMapPropEnum>,

    /// A flag to signal that the item, event, or place is accessible for free.
    pub is_accessible_for_free_prop_enum: Option<Boolean>,

    /// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
    pub isic_v4_prop_enum: Option<Text>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords_prop_enum: Option<KeywordsPropEnum>,

    /// The latitude of a location. For example <code>37.42242</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
    pub latitude_prop_enum: Option<LatitudePropEnum>,

    /// An associated logo.
    pub logo_prop_enum: Option<LogoPropEnum>,

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

    /// A review of the item.
    pub review_prop_enum: Option<Review>,

    /// A slogan or motto associated with the item.
    pub slogan_prop_enum: Option<Text>,

    /// Indicates whether it is allowed to smoke in the place, e.g. in the restaurant, hotel or hotel room.
    pub smoking_allowed_prop_enum: Option<Boolean>,

    /// The special opening hours of a certain place.<br/><br/>  Use this to explicitly override general opening hours brought in scope by <a class="localLink" href="/openingHoursSpecification">openingHoursSpecification</a> or <a class="localLink" href="/openingHours">openingHours</a>.
    pub special_opening_hours_specification_prop_enum: Option<OpeningHoursSpecification>,

    /// The telephone number.
    pub telephone_prop_enum: Option<Text>,

    /// A page providing information on how to book a tour of some <a class="localLink" href="/Place">Place</a>, such as an <a class="localLink" href="/Accommodation">Accommodation</a> or <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> in a real estate setting, as well as other kinds of tours as appropriate.
    pub tour_booking_page_prop_enum: Option<URL>,

    /// The general opening hours for a business. Opening hours can be specified as a weekly time range, starting with days, then times per day. Multiple days can be listed with commas ',' separating each day. Day or time ranges are specified using a hyphen '-'.<br/><br/>  <ul> <li>Days are specified using the following two-letter combinations: <code>Mo</code>, <code>Tu</code>, <code>We</code>, <code>Th</code>, <code>Fr</code>, <code>Sa</code>, <code>Su</code>.</li> <li>Times are specified using 24:00 format. For example, 3pm is specified as <code>15:00</code>, 10am as <code>10:00</code>. </li> <li>Here is an example: <code>&lt;time itemprop="openingHours" datetime=&quot;Tu,Th 16:00-20:00&quot;&gt;Tuesdays and Thursdays 4-8pm&lt;/time&gt;</code>.</li> <li>If a business is open 7 days a week, then it can be specified as <code>&lt;time itemprop=&quot;openingHours&quot; datetime=&quot;Mo-Su&quot;&gt;Monday through Sunday, all day&lt;/time&gt;</code>.</li> </ul>
    pub opening_hours_prop_enum: Option<Text>,
}

impl Mosque {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
