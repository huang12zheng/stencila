// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::administrative_area::AdministrativeArea;
use super::contact_point::ContactPoint;
use super::country::Country;
use super::date::Date;
use super::demand::Demand;
use super::drug::Drug;
use super::educational_occupational_credential::EducationalOccupationalCredential;
use super::event::Event;
use super::grant::Grant;
use super::integer::Integer;
use super::interaction_counter::InteractionCounter;
use super::medical_condition::MedicalCondition;
use super::number::Number;
use super::occupation::Occupation;
use super::offer::Offer;
use super::offer_catalog::OfferCatalog;
use super::organization::Organization;
use super::person::Person;
use super::place::Place;
use super::quantitative_value::QuantitativeValue;
use super::text::Text;
use super::url::URL;
use super::address::address;
use super::alumni_of::alumniOf;
use super::brand::brand;
use super::colleague::colleague;
use super::funder::funder;
use super::gender::gender;
use super::height::height;
use super::home_location::homeLocation;
use super::identifier::identifier;
use super::image::image;
use super::job_title::jobTitle;
use super::knows_about::knowsAbout;
use super::knows_language::knowsLanguage;
use super::main_entity_of_page::mainEntityOfPage;
use super::member_of::memberOf;
use super::net_worth::netWorth;
use super::owns::owns;
use super::publishing_principles::publishingPrinciples;
use super::sponsor::sponsor;
use super::subject_of::subjectOf;
use super::suggested_gender::suggestedGender;
use super::work_location::workLocation;

/// * COMMENT: A patient is any person recipient of health care services. * EXTEND FROM: https://schema.org/MedicalAudience, https://schema.org/Person
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Patient {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<PatientOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct PatientOptions {
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

    /// The target group associated with a given audience (e.g. veterans, car owners, musicians, etc.).
    pub audience_type: Option<Text>,

    /// The geographic area associated with the audience.
    pub geographic_area: Option<AdministrativeArea>,

    /// Specifying the health condition(s) of a patient, medical study, or other target audience.
    pub health_condition: Option<MedicalCondition>,

    /// Audiences defined by a person's gender.
    pub required_gender: Option<Text>,

    /// Audiences defined by a person's maximum age.
    pub required_max_age: Option<Integer>,

    /// Audiences defined by a person's minimum age.
    pub required_min_age: Option<Integer>,

    /// The age or age range for the intended audience or person, for example 3-12 months for infants, 1-5 years for toddlers.
    pub suggested_age: Option<QuantitativeValue>,

    /// The suggested gender of the intended person or audience, for example "male", "female", or "unisex".
    pub suggested_gender: Option<suggestedGender>,

    /// Maximum recommended age in years for the audience or user.
    pub suggested_max_age: Option<Number>,

    /// A suggested range of body measurements for the intended audience or person, for example inseam between 32 and 34 inches or height between 170 and 190 cm. Typically found on a size chart for wearable products.
    pub suggested_measurement: Option<QuantitativeValue>,

    /// Minimum recommended age in years for the audience or user.
    pub suggested_min_age: Option<Number>,

    /// An additional name for a Person, can be used for a middle name.
    pub additional_name: Option<Text>,

    /// Physical address of the item.
    pub address: Option<address>,

    /// An organization that this person is affiliated with. For example, a school/university, a club, or a team.
    pub affiliation: Option<Organization>,

    /// An organization that the person is an alumni of.
    pub alumni_of: Option<alumniOf>,

    /// An award won by or for this item.
    pub award: Option<Text>,

    /// Date of birth.
    pub birth_date: Option<Date>,

    /// The place where the person was born.
    pub birth_place: Option<Place>,

    /// The brand(s) associated with a product or service, or the brand(s) maintained by an organization or business person.
    pub brand: Option<brand>,

    /// A <a href="https://en.wikipedia.org/wiki/Call_sign">callsign</a>, as used in broadcasting and radio communications to identify people, radio and TV stations, or vehicles.
    pub call_sign: Option<Text>,

    /// A child of the person.
    pub children: Option<Person>,

    /// A colleague of the person.
    pub colleague: Option<colleague>,

    /// A contact point for a person or organization.
    pub contact_point: Option<ContactPoint>,

    /// Date of death.
    pub death_date: Option<Date>,

    /// The place where the person died.
    pub death_place: Option<Place>,

    /// The Dun &amp; Bradstreet DUNS number for identifying an organization or business person.
    pub duns: Option<Text>,

    /// Email address.
    pub email: Option<Text>,

    /// Family name. In the U.S., the last name of a Person.
    pub family_name: Option<Text>,

    /// The fax number.
    pub fax_number: Option<Text>,

    /// The most generic uni-directional social relation.
    pub follows: Option<Person>,

    /// A person or organization that supports (sponsors) something through some kind of financial contribution.
    pub funder: Option<funder>,

    /// A <a class="localLink" href="/Grant">Grant</a> that directly or indirectly provide funding or sponsorship for this item. See also <a class="localLink" href="/ownershipFundingInfo">ownershipFundingInfo</a>.
    pub funding: Option<Grant>,

    /// Gender of something, typically a <a class="localLink" href="/Person">Person</a>, but possibly also fictional characters, animals, etc. While http://schema.org/Male and http://schema.org/Female may be used, text strings are also acceptable for people who do not identify as a binary gender. The <a class="localLink" href="/gender">gender</a> property can also be used in an extended sense to cover e.g. the gender of sports teams. As with the gender of individuals, we do not try to enumerate all possibilities. A mixed-gender <a class="localLink" href="/SportsTeam">SportsTeam</a> can be indicated with a text value of "Mixed".
    pub gender: Option<gender>,

    /// Given name. In the U.S., the first name of a Person.
    pub given_name: Option<Text>,

    /// The <a href="http://www.gs1.org/gln">Global Location Number</a> (GLN, sometimes also referred to as International Location Number or ILN) of the respective organization, person, or place. The GLN is a 13-digit number used to identify parties and physical locations.
    pub global_location_number: Option<Text>,

    /// A credential awarded to the Person or Organization.
    pub has_credential: Option<EducationalOccupationalCredential>,

    /// The Person's occupation. For past professions, use Role for expressing dates.
    pub has_occupation: Option<Occupation>,

    /// Indicates an OfferCatalog listing for this Organization, Person, or Service.
    pub has_offer_catalog: Option<OfferCatalog>,

    /// Points-of-Sales operated by the organization or person.
    pub has_pos: Option<Place>,

    /// The height of the item.
    pub height: Option<height>,

    /// A contact location for a person's residence.
    pub home_location: Option<homeLocation>,

    /// An honorific prefix preceding a Person's name such as Dr/Mrs/Mr.
    pub honorific_prefix: Option<Text>,

    /// An honorific suffix following a Person's name such as M.D./PhD/MSCSW.
    pub honorific_suffix: Option<Text>,

    /// The number of interactions for the CreativeWork using the WebSite or SoftwareApplication. The most specific child type of InteractionCounter should be used.
    pub interaction_statistic: Option<InteractionCounter>,

    /// The International Standard of Industrial Classification of All Economic Activities (ISIC), Revision 4 code for a particular organization, business person, or place.
    pub isic_v4: Option<Text>,

    /// The job title of the person (for example, Financial Manager).
    pub job_title: Option<jobTitle>,

    /// The most generic bi-directional social/work relation.
    pub knows: Option<Person>,

    /// Of a <a class="localLink" href="/Person">Person</a>, and less typically of an <a class="localLink" href="/Organization">Organization</a>, to indicate a topic that is known about - suggesting possible expertise but not implying it. We do not distinguish skill levels here, or relate this to educational content, events, objectives or <a class="localLink" href="/JobPosting">JobPosting</a> descriptions.
    pub knows_about: Option<knowsAbout>,

    /// Of a <a class="localLink" href="/Person">Person</a>, and less typically of an <a class="localLink" href="/Organization">Organization</a>, to indicate a known language. We do not distinguish skill levels or reading/writing/speaking/signing here. Use language codes from the <a href="http://tools.ietf.org/html/bcp47">IETF BCP 47 standard</a>.
    pub knows_language: Option<knowsLanguage>,

    /// A pointer to products or services offered by the organization or person.
    pub makes_offer: Option<Offer>,

    /// An Organization (or ProgramMembership) to which this Person or Organization belongs.
    pub member_of: Option<memberOf>,

    /// The North American Industry Classification System (NAICS) code for a particular organization or business person.
    pub naics: Option<Text>,

    /// Nationality of the person.
    pub nationality: Option<Country>,

    /// The total financial value of the person as calculated by subtracting assets from liabilities.
    pub net_worth: Option<netWorth>,

    /// Products owned by the organization or person.
    pub owns: Option<owns>,

    /// A parent of this person.
    pub parent: Option<Person>,

    /// Event that this person is a performer or participant in.
    pub performer_in: Option<Event>,

    /// The publishingPrinciples property indicates (typically via <a class="localLink" href="/URL">URL</a>) a document describing the editorial principles of an <a class="localLink" href="/Organization">Organization</a> (or individual, e.g. a <a class="localLink" href="/Person">Person</a> writing a blog) that relate to their activities as a publisher, e.g. ethics or diversity policies. When applied to a <a class="localLink" href="/CreativeWork">CreativeWork</a> (e.g. <a class="localLink" href="/NewsArticle">NewsArticle</a>) the principles are those of the party primarily responsible for the creation of the <a class="localLink" href="/CreativeWork">CreativeWork</a>.<br/><br/>  While such policies are most typically expressed in natural language, sometimes related information (e.g. indicating a <a class="localLink" href="/funder">funder</a>) can be expressed using schema.org terminology.
    pub publishing_principles: Option<publishingPrinciples>,

    /// The most generic familial relation.
    pub related_to: Option<Person>,

    /// A pointer to products or services sought by the organization or person (demand).
    pub seeks: Option<Demand>,

    /// A sibling of the person.
    pub sibling: Option<Person>,

    /// A person or organization that supports a thing through a pledge, promise, or financial contribution. E.g. a sponsor of a Medical Study or a corporate sponsor of an event.
    pub sponsor: Option<sponsor>,

    /// The person's spouse.
    pub spouse: Option<Person>,

    /// The Tax / Fiscal ID of the organization or person, e.g. the TIN in the US or the CIF/NIF in Spain.
    pub tax_id: Option<Text>,

    /// The telephone number.
    pub telephone: Option<Text>,

    /// The Value-added Tax ID of the organization or person.
    pub vat_id: Option<Text>,

    /// The weight of the product or person.
    pub weight: Option<QuantitativeValue>,

    /// A contact location for a person's place of work.
    pub work_location: Option<workLocation>,

    /// Organizations that the person works for.
    pub works_for: Option<Organization>,

    /// One or more alternative conditions considered in the differential diagnosis process as output of a diagnosis process.
    pub diagnosis: Option<MedicalCondition>,

    /// Specifying a drug or medicine used in a medication procedure.
    pub drug: Option<Drug>,
}

impl Patient {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
