// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::accommodation::Accommodation;
use super::place::Place;
use super::thing::Thing;
use super::accommodation_floor_plan::accommodationFloorPlan;
use super::additional_property::additionalProperty;
use super::address::address;
use super::aggregate_rating::aggregateRating;
use super::amenity_feature::amenityFeature;
use super::event::event;
use super::fax_number::faxNumber;
use super::floor_size::floorSize;
use super::geo_contains::geoContains;
use super::geo_covered_by::geoCoveredBy;
use super::geo_covers::geoCovers;
use super::geo_crosses::geoCrosses;
use super::geo_disjoint::geoDisjoint;
use super::geo_equals::geoEquals;
use super::geo_intersects::geoIntersects;
use super::geo_overlaps::geoOverlaps;
use super::geo_touches::geoTouches;
use super::geo_within::geoWithin;
use super::global_location_number::globalLocationNumber;
use super::is_accessible_for_free::isAccessibleForFree;
use super::isic_v4::isicV4;
use super::keywords::keywords;
use super::latitude::latitude;
use super::lease_length::leaseLength;
use super::logo::logo;
use super::longitude::longitude;
use super::maximum_attendee_capacity::maximumAttendeeCapacity;
use super::number_of_bathrooms_total::numberOfBathroomsTotal;
use super::number_of_bedrooms::numberOfBedrooms;
use super::number_of_full_bathrooms::numberOfFullBathrooms;
use super::number_of_partial_bathrooms::numberOfPartialBathrooms;
use super::number_of_rooms::numberOfRooms;
use super::occupancy::occupancy;
use super::pets_allowed::petsAllowed;
use super::review::review;
use super::slogan::slogan;
use super::telephone::telephone;
use super::tour_booking_page::tourBookingPage;

/// * COMMENT: Residence type: Single-family home. * EXTEND FROM: https://schema.org/House
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct SingleFamilyResidence {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<SingleFamilyResidenceOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct SingleFamilyResidenceOptions {
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

    /// A property-value pair representing an additional characteristic of the entity, e.g. a product feature or another characteristic for which there is no matching property in schema.org.<br/><br/>  Note: Publishers should be aware that applications designed to use specific schema.org properties (e.g. http://schema.org/width, http://schema.org/color, http://schema.org/gtin13, ...) will typically expect such data to be provided using those properties, rather than using the generic property/value mechanism.
    pub additional_property: Option<additionalProperty>,

    /// Physical address of the item.
    pub address: Option<address>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: Option<aggregateRating>,

    /// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
    pub amenity_feature: Option<amenityFeature>,

    /// A short textual code (also called "store code") that uniquely identifies a place of business. The code is typically assigned by the parentOrganization and used in structured URLs.<br/><br/>  For example, in the URL http://www.starbucks.co.uk/store-locator/etc/detail/3047 the code "3047" is a branchCode for a particular branch.
    pub branch_code: Option<Place>,

    /// The basic containment relation between a place and one that contains it.
    pub contained_in_place: Option<Place>,

    /// The basic containment relation between a place and another that it contains.
    pub contains_place: Option<Place>,

    /// Upcoming or past event associated with this place, organization, or action.
    pub event: Option<event>,

    /// The fax number.
    pub fax_number: Option<faxNumber>,

    /// The geo coordinates of the place.
    pub geo: Option<Place>,

    /// Represents a relationship between two geometries (or the places they represent), relating a containing geometry to a contained geometry. "a contains b iff no points of b lie in the exterior of a, and at least one point of the interior of b lies in the interior of a". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_contains: Option<geoContains>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that covers it. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_covered_by: Option<geoCoveredBy>,

    /// Represents a relationship between two geometries (or the places they represent), relating a covering geometry to a covered geometry. "Every point of b is a point of (the interior or boundary of) a". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_covers: Option<geoCovers>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that crosses it: "a crosses b: they have some but not all interior points in common, and the dimension of the intersection is less than that of at least one of them". As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_crosses: Option<geoCrosses>,

    /// Represents spatial relations in which two geometries (or the places they represent) are topologically disjoint: "they have no point in common. They form a set of disconnected geometries." (A symmetric relationship, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.)
    pub geo_disjoint: Option<geoDisjoint>,

    /// Represents spatial relations in which two geometries (or the places they represent) are topologically equal, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>. "Two geometries are topologically equal if their interiors intersect and no part of the interior or boundary of one geometry intersects the exterior of the other" (a symmetric relationship).
    pub geo_equals: Option<geoEquals>,

    /// Represents spatial relations in which two geometries (or the places they represent) have at least one point in common. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_intersects: Option<geoIntersects>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to another that geospatially overlaps it, i.e. they have some but not all points in common. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_overlaps: Option<geoOverlaps>,

    /// Represents spatial relations in which two geometries (or the places they represent) touch: "they have at least one boundary point in common, but no interior points." (A symmetric relationship, as defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.)
    pub geo_touches: Option<geoTouches>,

    /// Represents a relationship between two geometries (or the places they represent), relating a geometry to one that contains it, i.e. it is inside (i.e. within) its interior. As defined in <a href="https://en.wikipedia.org/wiki/DE-9IM">DE-9IM</a>.
    pub geo_within: Option<geoWithin>,

    /// The <a href="http://www.gs1.org/gln">Global Location Number</a> (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
    pub global_location_number: Option<globalLocationNumber>,

    /// Indicates whether some facility (e.g. <a class="localLink" href="/FoodEstablishment">FoodEstablishment</a>, <a class="localLink" href="/CovidTestingFacility">CovidTestingFacility</a>) offers a service that can be used by driving through in a car. In the case of <a class="localLink" href="/CovidTestingFacility">CovidTestingFacility</a> such facilities could potentially help with social distancing from other potentially-infected users.
    pub has_drive_through_service: Option<Place>,

    /// A URL to a map of the place.
    pub has_map: Option<Place>,

    /// A flag to signal that the item, event, or place is accessible for free.
    pub is_accessible_for_free: Option<isAccessibleForFree>,

    /// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
    pub isic_v4: Option<isicV4>,

    /// Keywords or tags used to describe some item. Multiple textual entries in a keywords list are typically delimited by commas, or by repeating the property.
    pub keywords: Option<keywords>,

    /// The latitude of a location. For example <code>37.42242</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
    pub latitude: Option<latitude>,

    /// An associated logo.
    pub logo: Option<logo>,

    /// The longitude of a location. For example <code>-122.08585</code> (<a href="https://en.wikipedia.org/wiki/World_Geodetic_System">WGS 84</a>).
    pub longitude: Option<longitude>,

    /// The total number of individuals that may attend an event or venue.
    pub maximum_attendee_capacity: Option<maximumAttendeeCapacity>,

    /// The opening hours of a certain place.
    pub opening_hours_specification: Option<Place>,

    /// A photograph of this place.
    pub photo: Option<Place>,

    /// A flag to signal that the <a class="localLink" href="/Place">Place</a> is open to public visitors.  If this property is omitted there is no assumed default boolean value
    pub public_access: Option<Place>,

    /// A review of the item.
    pub review: Option<review>,

    /// A slogan or motto associated with the item.
    pub slogan: Option<slogan>,

    /// Indicates whether it is allowed to smoke in the place, e.g. in the restaurant, hotel or hotel room.
    pub smoking_allowed: Option<Place>,

    /// The special opening hours of a certain place.<br/><br/>  Use this to explicitly override general opening hours brought in scope by <a class="localLink" href="/openingHoursSpecification">openingHoursSpecification</a> or <a class="localLink" href="/openingHours">openingHours</a>.
    pub special_opening_hours_specification: Option<Place>,

    /// The telephone number.
    pub telephone: Option<telephone>,

    /// A page providing information on how to book a tour of some <a class="localLink" href="/Place">Place</a>, such as an <a class="localLink" href="/Accommodation">Accommodation</a> or <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> in a real estate setting, as well as other kinds of tours as appropriate.
    pub tour_booking_page: Option<tourBookingPage>,

    /// Category of an <a class="localLink" href="/Accommodation">Accommodation</a>, following real estate conventions, e.g. RESO (see <a href="https://ddwiki.reso.org/display/DDW17/PropertySubType+Field">PropertySubType</a>, and <a href="https://ddwiki.reso.org/display/DDW17/PropertyType+Field">PropertyType</a> fields  for suggested values).
    pub accommodation_category: Option<Accommodation>,

    /// A floorplan of some <a class="localLink" href="/Accommodation">Accommodation</a>.
    pub accommodation_floor_plan: Option<accommodationFloorPlan>,

    /// The floor level for an <a class="localLink" href="/Accommodation">Accommodation</a> in a multi-storey building. Since counting   systems <a href="https://en.wikipedia.org/wiki/Storey#Consecutive_number_floor_designations">vary internationally</a>, the local system should be used where possible.
    pub floor_level: Option<Accommodation>,

    /// The size of the accommodation, e.g. in square meter or squarefoot. Typical unit code(s): MTK for square meter, FTK for square foot, or YDK for square yard
    pub floor_size: Option<floorSize>,

    /// Length of the lease for some <a class="localLink" href="/Accommodation">Accommodation</a>, either particular to some <a class="localLink" href="/Offer">Offer</a> or in some cases intrinsic to the property.
    pub lease_length: Option<leaseLength>,

    /// The total integer number of bathrooms in some <a class="localLink" href="/Accommodation">Accommodation</a>, following real estate conventions as <a href="https://ddwiki.reso.org/display/DDW17/BathroomsTotalInteger+Field">documented in RESO</a>: "The simple sum of the number of bathrooms. For example for a property with two Full Bathrooms and one Half Bathroom, the Bathrooms Total Integer will be 3.". See also <a class="localLink" href="/numberOfRooms">numberOfRooms</a>.
    pub number_of_bathrooms_total: Option<numberOfBathroomsTotal>,

    /// The total integer number of bedrooms in a some <a class="localLink" href="/Accommodation">Accommodation</a>, <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> or <a class="localLink" href="/FloorPlan">FloorPlan</a>.
    pub number_of_bedrooms: Option<numberOfBedrooms>,

    /// Number of full bathrooms - The total number of full and ¾ bathrooms in an <a class="localLink" href="/Accommodation">Accommodation</a>. This corresponds to the <a href="https://ddwiki.reso.org/display/DDW17/BathroomsFull+Field">BathroomsFull field in RESO</a>.
    pub number_of_full_bathrooms: Option<numberOfFullBathrooms>,

    /// Number of partial bathrooms - The total number of half and ¼ bathrooms in an <a class="localLink" href="/Accommodation">Accommodation</a>. This corresponds to the <a href="https://ddwiki.reso.org/display/DDW17/BathroomsPartial+Field">BathroomsPartial field in RESO</a>.
    pub number_of_partial_bathrooms: Option<numberOfPartialBathrooms>,

    /// The number of rooms (excluding bathrooms and closets) of the accommodation or lodging business. Typical unit code(s): ROM for room or C62 for no unit. The type of room can be put in the unitText property of the QuantitativeValue.
    pub number_of_rooms: Option<numberOfRooms>,

    /// Indications regarding the permitted usage of the accommodation.
    pub permitted_usage: Option<Accommodation>,

    /// Indicates whether pets are allowed to enter the accommodation or lodging business. More detailed information can be put in a text value.
    pub pets_allowed: Option<petsAllowed>,

    /// The year an <a class="localLink" href="/Accommodation">Accommodation</a> was constructed. This corresponds to the <a href="https://ddwiki.reso.org/display/DDW17/YearBuilt+Field">YearBuilt field in RESO</a>.
    pub year_built: Option<Accommodation>,

    /// The allowed total occupancy for the accommodation in persons (including infants etc). For individual accommodations, this is not necessarily the legal maximum but defines the permitted usage as per the contractual agreement (e.g. a double room used by a single person). Typical unit code(s): C62 for person
    pub occupancy: Option<occupancy>,
}

impl SingleFamilyResidence {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
