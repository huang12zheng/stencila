// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::course::Course;
use super::date::Date;
use super::day_of_week::DayOfWeek;
use super::duration::Duration;
use super::educational_credential_awarded_prop_enum::EducationalCredentialAwardedPropEnum;
use super::educational_program_mode_prop_enum::EducationalProgramModePropEnum;
use super::end_date_prop_enum::EndDatePropEnum;
use super::financial_aid_eligible_prop_enum::FinancialAidEligiblePropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::integer::Integer;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::monetary_amount_distribution::MonetaryAmountDistribution;
use super::number::Number;
use super::number_of_credits_prop_enum::NumberOfCreditsPropEnum;
use super::occupational_category_prop_enum::OccupationalCategoryPropEnum;
use super::occupational_credential_awarded_prop_enum::OccupationalCredentialAwardedPropEnum;
use super::offers_prop_enum::OffersPropEnum;
use super::program_prerequisites_prop_enum::ProgramPrerequisitesPropEnum;
use super::program_type_prop_enum::ProgramTypePropEnum;
use super::provider_prop_enum::ProviderPropEnum;
use super::start_date_prop_enum::StartDatePropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::typical_credits_per_term_prop_enum::TypicalCreditsPerTermPropEnum;
use super::url::URL;

/// https://schema.org/WorkBasedProgram
/// * MOD OF:
/// https://pending.schema.org
/// * COMMENT:
/// A program with both an educational and employment component. Typically based at a workplace and structured around work-based learning, with the aim of instilling competencies related to an occupation. WorkBasedProgram is used to distinguish programs such as apprenticeships from school, college or other classroom based educational programs.
/// * EXTEND FROM:
/// https://schema.org/EducationalOccupationalProgram
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct WorkBasedProgram {
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

    /// The date at which the program stops collecting applications for the next enrollment cycle.
    pub application_deadline_prop_enum: Option<Date>,

    /// The date at which the program begins collecting applications for the next enrollment cycle.
    pub application_start_date_prop_enum: Option<Date>,

    /// The day of the week for which these opening hours are valid.
    pub day_of_week_prop_enum: Option<DayOfWeek>,

    /// A description of the qualification, award, certificate, diploma or other educational credential awarded as a consequence of successful completion of this course or program.
    pub educational_credential_awarded_prop_enum: Option<EducationalCredentialAwardedPropEnum>,

    /// Similar to courseMode, the medium or means of delivery of the program as a whole. The value may either be a text label (e.g. "online", "onsite" or "blended"; "synchronous" or "asynchronous"; "full-time" or "part-time") or a URL reference to a term from a controlled vocabulary (e.g. https://ceds.ed.gov/element/001311#Asynchronous ).
    pub educational_program_mode_prop_enum: Option<EducationalProgramModePropEnum>,

    /// The end date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub end_date_prop_enum: Option<EndDatePropEnum>,

    /// A financial aid type or program which students may use to pay for tuition or fees associated with the program.
    pub financial_aid_eligible_prop_enum: Option<FinancialAidEligiblePropEnum>,

    /// A course or class that is one of the learning opportunities that constitute an educational / occupational program. No information is implied about whether the course is mandatory or optional; no guarantee is implied about whether the course will be available to everyone on the program.
    pub has_course_prop_enum: Option<Course>,

    /// The maximum number of students who may be enrolled in the program.
    pub maximum_enrollment_prop_enum: Option<Integer>,

    /// The number of credits or units awarded by a Course or required to complete an EducationalOccupationalProgram.
    pub number_of_credits_prop_enum: Option<NumberOfCreditsPropEnum>,

    /// A category describing the job, preferably using a term from a taxonomy such as <a href="http://www.onetcenter.org/taxonomy.html">BLS O*NET-SOC</a>, <a href="https://www.ilo.org/public/english/bureau/stat/isco/isco08/">ISCO-08</a> or similar, with the property repeated for each applicable value. Ideally the taxonomy should be identified, and both the textual label and formal code for the category should be provided.<br/><br/>  Note: for historical reasons, any textual label and formal code provided as a literal may be assumed to be from O*NET-SOC.
    pub occupational_category_prop_enum: Option<OccupationalCategoryPropEnum>,

    /// A description of the qualification, award, certificate, diploma or other occupational credential awarded as a consequence of successful completion of this course or program.
    pub occupational_credential_awarded_prop_enum: Option<OccupationalCredentialAwardedPropEnum>,

    /// An offer to provide this item&#x2014;for example, an offer to sell a product, rent the DVD of a movie, perform a service, or give away tickets to an event. Use <a class="localLink" href="/businessFunction">businessFunction</a> to indicate the kind of transaction offered, i.e. sell, lease, etc. This property can also be used to describe a <a class="localLink" href="/Demand">Demand</a>. While this property is listed as expected on a number of common types, it can be used in others. In that case, using a second type, such as Product or a subtype of Product, can clarify the nature of the offer.
    pub offers_prop_enum: Option<OffersPropEnum>,

    /// Prerequisites for enrolling in the program.
    pub program_prerequisites_prop_enum: Option<ProgramPrerequisitesPropEnum>,

    /// The type of educational or occupational program. For example, classroom, internship, alternance, etc.
    pub program_type_prop_enum: Option<ProgramTypePropEnum>,

    /// The service provider, service operator, or service performer; the goods producer. Another party (a seller) may offer those services or goods on behalf of the provider. A provider may also serve as the seller.
    pub provider_prop_enum: Option<ProviderPropEnum>,

    /// The expected salary upon completing the training.
    pub salary_upon_completion_prop_enum: Option<MonetaryAmountDistribution>,

    /// The start date and time of the item (in <a href="http://en.wikipedia.org/wiki/ISO_8601">ISO 8601 date format</a>).
    pub start_date_prop_enum: Option<StartDatePropEnum>,

    /// The amount of time in a term as defined by the institution. A term is a length of time where students take one or more classes. Semesters and quarters are common units for term.
    pub term_duration_prop_enum: Option<Duration>,

    /// The number of times terms of study are offered per year. Semesters and quarters are common units for term. For example, if the student can only take 2 semesters for the program in one year, then termsPerYear should be 2.
    pub terms_per_year_prop_enum: Option<Number>,

    /// The time of day the program normally runs. For example, "evenings".
    pub time_of_day_prop_enum: Option<Text>,

    /// The expected length of time to complete the program if attending full-time.
    pub time_to_complete_prop_enum: Option<Duration>,

    /// The estimated salary earned while in the program.
    pub training_salary_prop_enum: Option<MonetaryAmountDistribution>,

    /// The number of credits or units a full-time student would be expected to take in 1 term however 'term' is defined by the institution.
    pub typical_credits_per_term_prop_enum: Option<TypicalCreditsPerTermPropEnum>,
}

impl WorkBasedProgram {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
