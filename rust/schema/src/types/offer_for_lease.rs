// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::offer::Offer;
use super::thing::Thing;
use super::accepted_payment_method::acceptedPaymentMethod;
use super::advance_booking_requirement::advanceBookingRequirement;
use super::aggregate_rating::aggregateRating;
use super::area_served::areaServed;
use super::asin::asin;
use super::availability::availability;
use super::availability_ends::availabilityEnds;
use super::availability_starts::availabilityStarts;
use super::available_at_or_from::availableAtOrFrom;
use super::available_delivery_method::availableDeliveryMethod;
use super::business_function::businessFunction;
use super::category::category;
use super::delivery_lead_time::deliveryLeadTime;
use super::eligible_customer_type::eligibleCustomerType;
use super::eligible_duration::eligibleDuration;
use super::eligible_quantity::eligibleQuantity;
use super::eligible_region::eligibleRegion;
use super::eligible_transaction_volume::eligibleTransactionVolume;
use super::gtin::gtin;
use super::gtin_12::gtin12;
use super::gtin_13::gtin13;
use super::gtin_14::gtin14;
use super::gtin_8::gtin8;
use super::has_adult_consideration::hasAdultConsideration;
use super::has_measurement::hasMeasurement;
use super::has_merchant_return_policy::hasMerchantReturnPolicy;
use super::includes_object::includesObject;
use super::ineligible_region::ineligibleRegion;
use super::inventory_level::inventoryLevel;
use super::is_family_friendly::isFamilyFriendly;
use super::item_condition::itemCondition;
use super::item_offered::itemOffered;
use super::lease_length::leaseLength;
use super::mobile_url::mobileUrl;
use super::mpn::mpn;
use super::price::price;
use super::price_currency::priceCurrency;
use super::price_specification::priceSpecification;
use super::review::review;
use super::seller::seller;
use super::serial_number::serialNumber;
use super::sku::sku;
use super::valid_from::validFrom;
use super::valid_through::validThrough;
use super::warranty::warranty;

/// * MOD OF: https://pending.schema.org * COMMENT: An <a class="localLink" href="/OfferForLease">OfferForLease</a> in Schema.org represents an <a class="localLink" href="/Offer">Offer</a> to lease out something, i.e. an <a class="localLink" href="/Offer">Offer</a> whose   <a class="localLink" href="/businessFunction">businessFunction</a> is <a href="http://purl.org/goodrelations/v1#LeaseOut.">lease out</a>. See <a href="https://en.wikipedia.org/wiki/GoodRelations">Good Relations</a> for   background on the underlying concepts. * EXTEND FROM: https://schema.org/Offer
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct OfferForLease {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<OfferForLeaseOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct OfferForLeaseOptions {
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

    /// The payment method(s) accepted by seller for this offer.
    pub accepted_payment_method: Option<acceptedPaymentMethod>,

    /// An additional offer that can only be obtained in combination with the first base offer (e.g. supplements and extensions that are available for a surcharge).
    pub add_on: Option<Offer>,

    /// The amount of time that is required between accepting the offer and the actual usage of the resource or service.
    pub advance_booking_requirement: Option<advanceBookingRequirement>,

    /// The overall rating, based on a collection of reviews or ratings, of the item.
    pub aggregate_rating: Option<aggregateRating>,

    /// The geographic area where a service or offered item is provided.
    pub area_served: Option<areaServed>,

    /// An Amazon Standard Identification Number (ASIN) is a 10-character alphanumeric unique identifier assigned by Amazon.com and its partners for product identification within the Amazon organization (summary from <a href="https://en.wikipedia.org/wiki/Amazon_Standard_Identification_Number">Wikipedia</a>'s article).<br/><br/>  Note also that this is a definition for how to include ASINs in Schema.org data, and not a definition of ASINs in general - see documentation from Amazon for authoritative details. ASINs are most commonly encoded as text strings, but the [asin] property supports URL/URI as potential values too.
    pub asin: Option<asin>,

    /// The availability of this item&#x2014;for example In stock, Out of stock, Pre-order, etc.
    pub availability: Option<availability>,

    /// The end of the availability of the product or service included in the offer.
    pub availability_ends: Option<availabilityEnds>,

    /// The beginning of the availability of the product or service included in the offer.
    pub availability_starts: Option<availabilityStarts>,

    /// The place(s) from which the offer can be obtained (e.g. store locations).
    pub available_at_or_from: Option<availableAtOrFrom>,

    /// The delivery method(s) available for this offer.
    pub available_delivery_method: Option<availableDeliveryMethod>,

    /// The business function (e.g. sell, lease, repair, dispose) of the offer or component of a bundle (TypeAndQuantityNode). The default is http://purl.org/goodrelations/v1#Sell.
    pub business_function: Option<businessFunction>,

    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category: Option<category>,

    /// A URL template (RFC 6570) for a checkout page for an offer. This approach allows merchants to specify a URL for online checkout of the offered product, by interpolating parameters such as the logged in user ID, product ID, quantity, discount code etc. Parameter naming and standardization are not specified here.
    pub checkout_page_url_template: Option<Offer>,

    /// The typical delay between the receipt of the order and the goods either leaving the warehouse or being prepared for pickup, in case the delivery method is on site pickup.
    pub delivery_lead_time: Option<deliveryLeadTime>,

    /// The type(s) of customers for which the given offer is valid.
    pub eligible_customer_type: Option<eligibleCustomerType>,

    /// The duration for which the given offer is valid.
    pub eligible_duration: Option<eligibleDuration>,

    /// The interval and unit of measurement of ordering quantities for which the offer or price specification is valid. This allows e.g. specifying that a certain freight charge is valid only for a certain quantity.
    pub eligible_quantity: Option<eligibleQuantity>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is valid.<br/><br/>  See also <a class="localLink" href="/ineligibleRegion">ineligibleRegion</a>.
    pub eligible_region: Option<eligibleRegion>,

    /// The transaction volume, in a monetary unit, for which the offer or price specification is valid, e.g. for indicating a minimal purchasing volume, to express free shipping above a certain order volume, or to limit the acceptance of credit cards to purchases to a certain minimal amount.
    pub eligible_transaction_volume: Option<eligibleTransactionVolume>,

    /// A Global Trade Item Number (<a href="https://www.gs1.org/standards/id-keys/gtin">GTIN</a>). GTINs identify trade items, including products and services, using numeric identification codes.<br/><br/>  The GS1 <a href="https://www.gs1.org/standards/Digital-Link/">digital link specifications</a> express GTINs as URLs (URIs, IRIs, etc.). Details including regular expression examples can be found in, Section 6 of the GS1 URI Syntax specification; see also <a href="https://github.com/schemaorg/schemaorg/issues/3156#issuecomment-1209522809">schema.org tracking issue</a> for schema.org-specific discussion. A correct <a class="localLink" href="/gtin">gtin</a> value should be a valid GTIN, which means that it should be an all-numeric string of either 8, 12, 13 or 14 digits, or a "GS1 Digital Link" URL based on such a string. The numeric component should also have a <a href="https://www.gs1.org/services/check-digit-calculator">valid GS1 check digit</a> and meet the other rules for valid GTINs. See also <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1's GTIN Summary</a> and <a href="https://en.wikipedia.org/wiki/Global_Trade_Item_Number">Wikipedia</a> for more details. Left-padding of the gtin values is not required or encouraged. The <a class="localLink" href="/gtin">gtin</a> property generalizes the earlier <a class="localLink" href="/gtin8">gtin8</a>, <a class="localLink" href="/gtin12">gtin12</a>, <a class="localLink" href="/gtin13">gtin13</a>, and <a class="localLink" href="/gtin14">gtin14</a> properties.<br/><br/>  Note also that this is a definition for how to include GTINs in Schema.org data, and not a definition of GTINs in general - see the GS1 documentation for authoritative details.
    pub gtin: Option<gtin>,

    /// The GTIN-12 code of the product, or the product to which the offer refers. The GTIN-12 is the 12-digit GS1 Identification Key composed of a U.P.C. Company Prefix, Item Reference, and Check Digit used to identify trade items. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_12: Option<gtin12>,

    /// The GTIN-13 code of the product, or the product to which the offer refers. This is equivalent to 13-digit ISBN codes and EAN UCC-13. Former 12-digit UPC codes can be converted into a GTIN-13 code by simply adding a preceding zero. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_13: Option<gtin13>,

    /// The GTIN-14 code of the product, or the product to which the offer refers. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_14: Option<gtin14>,

    /// The GTIN-8 code of the product, or the product to which the offer refers. This code is also known as EAN/UCC-8 or 8-digit EAN. See <a href="http://www.gs1.org/barcodes/technical/idkeys/gtin">GS1 GTIN Summary</a> for more details.
    pub gtin_8: Option<gtin8>,

    /// Used to tag an item to be intended or suitable for consumption or use by adults only.
    pub has_adult_consideration: Option<hasAdultConsideration>,

    /// A product measurement, for example the inseam of pants, the wheel size of a bicycle, or the gauge of a screw. Usually an exact measurement, but can also be a range of measurements for adjustable products, for example belts and ski bindings.
    pub has_measurement: Option<hasMeasurement>,

    /// Specifies a MerchantReturnPolicy that may be applicable.
    pub has_merchant_return_policy: Option<hasMerchantReturnPolicy>,

    /// This links to a node or nodes indicating the exact quantity of the products included in  an <a class="localLink" href="/Offer">Offer</a> or <a class="localLink" href="/ProductCollection">ProductCollection</a>.
    pub includes_object: Option<includesObject>,

    /// The ISO 3166-1 (ISO 3166-1 alpha-2) or ISO 3166-2 code, the place, or the GeoShape for the geo-political region(s) for which the offer or delivery charge specification is not valid, e.g. a region where the transaction is not allowed.<br/><br/>  See also <a class="localLink" href="/eligibleRegion">eligibleRegion</a>.
    pub ineligible_region: Option<ineligibleRegion>,

    /// The current approximate inventory level for the item or items.
    pub inventory_level: Option<inventoryLevel>,

    /// Indicates whether this content is family friendly.
    pub is_family_friendly: Option<isFamilyFriendly>,

    /// A predefined value from OfferItemCondition specifying the condition of the product or service, or the products or services included in the offer. Also used for product return policies to specify the condition of products accepted for returns.
    pub item_condition: Option<itemCondition>,

    /// An item being offered (or demanded). The transactional nature of the offer or demand is documented using <a class="localLink" href="/businessFunction">businessFunction</a>, e.g. sell, lease etc. While several common expected types are listed explicitly in this definition, others can be used. Using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub item_offered: Option<itemOffered>,

    /// Length of the lease for some <a class="localLink" href="/Accommodation">Accommodation</a>, either particular to some <a class="localLink" href="/Offer">Offer</a> or in some cases intrinsic to the property.
    pub lease_length: Option<leaseLength>,

    /// The <a class="localLink" href="/mobileUrl">mobileUrl</a> property is provided for specific situations in which data consumers need to determine whether one of several provided URLs is a dedicated 'mobile site'.<br/><br/>  To discourage over-use, and reflecting intial usecases, the property is expected only on <a class="localLink" href="/Product">Product</a> and <a class="localLink" href="/Offer">Offer</a>, rather than <a class="localLink" href="/Thing">Thing</a>. The general trend in web technology is towards <a href="https://en.wikipedia.org/wiki/Responsive_web_design">responsive design</a> in which content can be flexibly adapted to a wide range of browsing environments. Pages and sites referenced with the long-established <a class="localLink" href="/url">url</a> property should ideally also be usable on a wide variety of devices, including mobile phones. In most cases, it would be pointless and counter productive to attempt to update all <a class="localLink" href="/url">url</a> markup to use <a class="localLink" href="/mobileUrl">mobileUrl</a> for more mobile-oriented pages. The property is intended for the case when items (primarily <a class="localLink" href="/Product">Product</a> and <a class="localLink" href="/Offer">Offer</a>) have extra URLs hosted on an additional "mobile site" alongside the main one. It should not be taken as an endorsement of this publication style.
    pub mobile_url: Option<mobileUrl>,

    /// The Manufacturer Part Number (MPN) of the product, or the product to which the offer refers.
    pub mpn: Option<mpn>,

    /// A pointer to the organization or person making the offer.
    pub offered_by: Option<Offer>,

    /// The offer price of a product, or of a price component when attached to PriceSpecification and its subtypes.<br/><br/>  Usage guidelines:<br/><br/>  <ul> <li>Use the <a class="localLink" href="/priceCurrency">priceCurrency</a> property (with standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR") instead of including <a href="http://en.wikipedia.org/wiki/Dollar_sign#Currencies_that_use_the_dollar_or_peso_sign">ambiguous symbols</a> such as '$' in the value.</li> <li>Use '.' (Unicode 'FULL STOP' (U+002E)) rather than ',' to indicate a decimal point. Avoid using these symbols as a readability separator.</li> <li>Note that both <a href="http://www.w3.org/TR/xhtml-rdfa-primer/#using-the-content-attribute">RDFa</a> and Microdata syntax allow the use of a "content=" attribute for publishing simple machine-readable values alongside more human-friendly formatting.</li> <li>Use values from 0123456789 (Unicode 'DIGIT ZERO' (U+0030) to 'DIGIT NINE' (U+0039)) rather than superficially similar Unicode symbols.</li> </ul>
    pub price: Option<price>,

    /// The currency of the price, or a price component when attached to <a class="localLink" href="/PriceSpecification">PriceSpecification</a> and its subtypes.<br/><br/>  Use standard formats: <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217 currency format</a>, e.g. "USD"; <a href="https://en.wikipedia.org/wiki/List_of_cryptocurrencies">Ticker symbol</a> for cryptocurrencies, e.g. "BTC"; well known names for <a href="https://en.wikipedia.org/wiki/Local_exchange_trading_system">Local Exchange Trading Systems</a> (LETS) and other currency types, e.g. "Ithaca HOUR".
    pub price_currency: Option<priceCurrency>,

    /// One or more detailed price specifications, indicating the unit price and delivery or payment charges.
    pub price_specification: Option<priceSpecification>,

    /// The date after which the price is no longer available.
    pub price_valid_until: Option<Offer>,

    /// A review of the item.
    pub review: Option<review>,

    /// An entity which offers (sells / leases / lends / loans) the services / goods.  A seller may also be a provider.
    pub seller: Option<seller>,

    /// The serial number or any alphanumeric identifier of a particular product. When attached to an offer, it is a shortcut for the serial number of the product included in the offer.
    pub serial_number: Option<serialNumber>,

    /// Indicates information about the shipping policies and options associated with an <a class="localLink" href="/Offer">Offer</a>.
    pub shipping_details: Option<Offer>,

    /// The Stock Keeping Unit (SKU), i.e. a merchant-specific identifier for a product or service, or the product to which the offer refers.
    pub sku: Option<sku>,

    /// The date when the item becomes valid.
    pub valid_from: Option<validFrom>,

    /// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
    pub valid_through: Option<validThrough>,

    /// The warranty promise(s) included in the offer.
    pub warranty: Option<warranty>,
}

impl OfferForLease {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
