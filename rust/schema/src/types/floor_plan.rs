// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::thing::Thing;
use super::amenity_feature::amenityFeature;
use super::floor_size::floorSize;
use super::number_of_accommodation_units::numberOfAccommodationUnits;
use super::number_of_available_accommodation_units::numberOfAvailableAccommodationUnits;
use super::number_of_bathrooms_total::numberOfBathroomsTotal;
use super::number_of_bedrooms::numberOfBedrooms;
use super::number_of_full_bathrooms::numberOfFullBathrooms;
use super::number_of_partial_bathrooms::numberOfPartialBathrooms;
use super::number_of_rooms::numberOfRooms;
use super::pets_allowed::petsAllowed;

/// * MOD OF: https://pending.schema.org * COMMENT: A FloorPlan is an explicit representation of a collection of similar accommodations, allowing the provision of common information (room counts, sizes, layout diagrams) and offers for rental or sale. In typical use, some <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> has an <a class="localLink" href="/accommodationFloorPlan">accommodationFloorPlan</a> which is a <a class="localLink" href="/FloorPlan">FloorPlan</a>.  A FloorPlan is always in the context of a particular place, either a larger <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a> or a single <a class="localLink" href="/Apartment">Apartment</a>. The visual/spatial aspects of a floor plan (i.e. room layout, <a href="https://en.wikipedia.org/wiki/Floor_plan">see wikipedia</a>) can be indicated using <a class="localLink" href="/image">image</a>. * EXTEND FROM: https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct FloorPlan {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<FloorPlanOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct FloorPlanOptions {
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

    /// An amenity feature (e.g. a characteristic or service) of the Accommodation. This generic property does not make a statement about whether the feature is included in an offer for the main accommodation or available at extra costs.
    pub amenity_feature: Option<amenityFeature>,

    /// The size of the accommodation, e.g. in square meter or squarefoot. Typical unit code(s): MTK for square meter, FTK for square foot, or YDK for square yard
    pub floor_size: Option<floorSize>,

    /// Indicates some accommodation that this floor plan describes.
    pub is_plan_for_apartment: Option<FloorPlan>,

    /// A schematic image showing the floorplan layout.
    pub layout_image: Option<FloorPlan>,

    /// Indicates the total (available plus unavailable) number of accommodation units in an <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>, or the number of accommodation units for a specific <a class="localLink" href="/FloorPlan">FloorPlan</a> (within its specific <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>). See also <a class="localLink" href="/numberOfAvailableAccommodationUnits">numberOfAvailableAccommodationUnits</a>.
    pub number_of_accommodation_units: Option<numberOfAccommodationUnits>,

    /// Indicates the number of available accommodation units in an <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>, or the number of accommodation units for a specific <a class="localLink" href="/FloorPlan">FloorPlan</a> (within its specific <a class="localLink" href="/ApartmentComplex">ApartmentComplex</a>). See also <a class="localLink" href="/numberOfAccommodationUnits">numberOfAccommodationUnits</a>.
    pub number_of_available_accommodation_units: Option<numberOfAvailableAccommodationUnits>,

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

    /// Indicates whether pets are allowed to enter the accommodation or lodging business. More detailed information can be put in a text value.
    pub pets_allowed: Option<petsAllowed>,
}

impl FloorPlan {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
