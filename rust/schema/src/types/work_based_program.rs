// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::educational_occupational_program::EducationalOccupationalProgram;
use super::thing::Thing;
use super::day_of_week::dayOfWeek;
use super::educational_credential_awarded::educationalCredentialAwarded;
use super::end_date::endDate;
use super::number_of_credits::numberOfCredits;
use super::occupational_category::occupationalCategory;
use super::occupational_credential_awarded::occupationalCredentialAwarded;
use super::offers::offers;
use super::provider::provider;
use super::start_date::startDate;
use super::training_salary::trainingSalary;

/// * MOD OF: https://pending.schema.org * COMMENT: A program with both an educational and employment component. Typically based at a workplace and structured around work-based learning, with the aim of instilling competencies related to an occupation. WorkBasedProgram is used to distinguish programs such as apprenticeships from school, college or other classroom based educational programs. * EXTEND FROM: https://schema.org/EducationalOccupationalProgram
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct WorkBasedProgram {
    

    /// Non-core optional fields
    #[serde(flatten)]
    pub options: Box<WorkBasedProgramOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize, Strip, Read, Write, ToHtml)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct WorkBasedProgramOptions {
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

    /// The date at which the program stops collecting applications for the next enrollment cycle.
    pub application_deadline: Option<EducationalOccupationalProgram>,

    /// The date at which the program begins collecting applications for the next enrollment cycle.
    pub application_start_date: Option<EducationalOccupationalProgram>,

    /// The day of the week for which these opening hours are valid.
    pub day_of_week: Option<dayOfWeek>,

    /// A description of the qualification, award, certificate, diploma or other educational credential awarded as a consequence of successful completion of this course or program.
    pub educational_credential_awarded: Option<educationalCredentialAwarded>,

    /// Similar to courseMode, the medium or means of delivery of the program as a whole. The value may either be a text label (e.g. "online", "onsite" or "blended"; "synchronous" or "asynchronous"; "full-time" or "part-time") or a URL reference to a term from a controlled vocabulary (e.g. https://ceds.ed.gov/element/001311#Asynchronous ).
    pub educational_program_mode: Option<EducationalOccupationalProgram>,

    /// The end date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub end_date: Option<endDate>,

    /// A financial aid type or program which students may use to pay for tuition or fees associated with the program.
    pub financial_aid_eligible: Option<EducationalOccupationalProgram>,

    /// A course or class that is one of the learning opportunities that constitute an educational / occupational program. No information is implied about whether the course is mandatory or optional; no guarantee is implied about whether the course will be available to everyone on the program.
    pub has_course: Option<EducationalOccupationalProgram>,

    /// The maximum number of students who may be enrolled in the program.
    pub maximum_enrollment: Option<EducationalOccupationalProgram>,

    /// The number of credits or units awarded by a Course or required to complete an EducationalOccupationalProgram.
    pub number_of_credits: Option<numberOfCredits>,

    /// A category describing the job, preferably using a term from a taxonomy such as <a href="http://www.onetcenter.org/taxonomy.html">BLS O*NET-SOC</a>, <a href="https://www.ilo.org/public/english/bureau/stat/isco/isco08/">ISCO-08</a> or similar, with the property repeated for each applicable value. Ideally the taxonomy should be identified, and both the textual label and formal code for the category should be provided.<br/><br/>  Note: for historical reasons, any textual label and formal code provided as a literal may be assumed to be from O*NET-SOC.
    pub occupational_category: Option<occupationalCategory>,

    /// A description of the qualification, award, certificate, diploma or other occupational credential awarded as a consequence of successful completion of this course or program.
    pub occupational_credential_awarded: Option<occupationalCredentialAwarded>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers: Option<offers>,

    /// Prerequisites for enrolling in the program.
    pub program_prerequisites: Option<EducationalOccupationalProgram>,

    /// The type of educational or occupational program. For example, classroom, internship, alternance, etc.
    pub program_type: Option<EducationalOccupationalProgram>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider: Option<provider>,

    /// The expected salary upon completing the training.
    pub salary_upon_completion: Option<EducationalOccupationalProgram>,

    /// The start date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub start_date: Option<startDate>,

    /// The amount of time in a term as defined by the institution. A term is a length of time where students take one or more classes. Semesters and quarters are common units for term.
    pub term_duration: Option<EducationalOccupationalProgram>,

    /// The number of times terms of study are offered per year. Semesters and quarters are common units for term. For example, if the student can only take 2 semesters for the program in one year, then termsPerYear should be 2.
    pub terms_per_year: Option<EducationalOccupationalProgram>,

    /// The time of day the program normally runs. For example, "evenings".
    pub time_of_day: Option<EducationalOccupationalProgram>,

    /// The expected length of time to complete the program if attending full-time.
    pub time_to_complete: Option<EducationalOccupationalProgram>,

    /// The estimated salary earned while in the program.
    pub training_salary: Option<trainingSalary>,

    /// The number of credits or units a full-time student would be expected to take in 1 term however 'term' is defined by the institution.
    pub typical_credits_per_term: Option<EducationalOccupationalProgram>,
}

impl WorkBasedProgram {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
