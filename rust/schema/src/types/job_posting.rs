// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::administrative_area::AdministrativeArea;
use super::base_salary_prop_enum::BaseSalaryPropEnum;
use super::boolean::Boolean;
use super::contact_point::ContactPoint;
use super::date_posted_prop_enum::DatePostedPropEnum;
use super::education_requirements_prop_enum::EducationRequirementsPropEnum;
use super::estimated_salary_prop_enum::EstimatedSalaryPropEnum;
use super::experience_requirements_prop_enum::ExperienceRequirementsPropEnum;
use super::hiring_organization_prop_enum::HiringOrganizationPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::industry_prop_enum::IndustryPropEnum;
use super::integer::Integer;
use super::job_start_date_prop_enum::JobStartDatePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::occupation::Occupation;
use super::occupational_category_prop_enum::OccupationalCategoryPropEnum;
use super::organization::Organization;
use super::physical_requirement_prop_enum::PhysicalRequirementPropEnum;
use super::place::Place;
use super::qualifications_prop_enum::QualificationsPropEnum;
use super::security_clearance_requirement_prop_enum::SecurityClearanceRequirementPropEnum;
use super::sensory_requirement_prop_enum::SensoryRequirementPropEnum;
use super::skills_prop_enum::SkillsPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;
use super::valid_through_prop_enum::ValidThroughPropEnum;

/// https://schema.org/JobPosting
/// * COMMENT:
/// A listing that describes a job opening in a certain organization.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct JobPosting {
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

    /// The location(s) applicants can apply from. This is usually used for telecommuting jobs where the applicant does not need to be in a physical office. Note: This should not be used for citizenship or work visa requirements.
    pub applicant_location_requirements_prop_enum: Option<AdministrativeArea>,

    /// Contact details for further information relevant to this job posting.
    pub application_contact_prop_enum: Option<ContactPoint>,

    /// The base salary of the job or of an employee in an EmployeeRole.
    pub base_salary_prop_enum: Option<BaseSalaryPropEnum>,

    /// Publication date of an online listing.
    pub date_posted_prop_enum: Option<DatePostedPropEnum>,

    /// Indicates whether an <a class="localLink" href="/url">url</a> that is associated with a <a class="localLink" href="/JobPosting">JobPosting</a> enables direct application for the job, via the posting website. A job posting is considered to have directApply of <a class="localLink" href="/True">True</a> if an application process for the specified job can be directly initiated via the url(s) given (noting that e.g. multiple internet domains might nevertheless be involved at an implementation level). A value of <a class="localLink" href="/False">False</a> is appropriate if there is no clear path to applying directly online for the specified job, navigating directly from the JobPosting url(s) supplied.
    pub direct_apply_prop_enum: Option<Boolean>,

    /// Educational background needed for the position or Occupation.
    pub education_requirements_prop_enum: Option<EducationRequirementsPropEnum>,

    /// The legal requirements such as citizenship, visa and other documentation required for an applicant to this job.
    pub eligibility_to_work_requirement_prop_enum: Option<Text>,

    /// A description of the employer, career opportunities and work environment for this position.
    pub employer_overview_prop_enum: Option<Text>,

    /// Type of employment (e.g. full-time, part-time, contract, temporary, seasonal, internship).
    pub employment_type_prop_enum: Option<Text>,

    /// Indicates the department, unit and/or facility where the employee reports and/or in which the job is to be performed.
    pub employment_unit_prop_enum: Option<Organization>,

    /// An estimated salary for a job posting or occupation, based on a variety of variables including, but not limited to industry, job title, and location. Estimated salaries  are often computed by outside organizations rather than the hiring organization, who may not have committed to the estimated value.
    pub estimated_salary_prop_enum: Option<EstimatedSalaryPropEnum>,

    /// Indicates whether a <a class="localLink" href="/JobPosting">JobPosting</a> will accept experience (as indicated by <a class="localLink" href="/OccupationalExperienceRequirements">OccupationalExperienceRequirements</a>) in place of its formal educational qualifications (as indicated by <a class="localLink" href="/educationRequirements">educationRequirements</a>). If true, indicates that satisfying one of these requirements is sufficient.
    pub experience_in_place_of_education_prop_enum: Option<Boolean>,

    /// Description of skills and experience needed for the position or Occupation.
    pub experience_requirements_prop_enum: Option<ExperienceRequirementsPropEnum>,

    /// Organization or Person offering the job position.
    pub hiring_organization_prop_enum: Option<HiringOrganizationPropEnum>,

    /// Description of bonus and commission compensation aspects of the job.
    pub incentive_compensation_prop_enum: Option<Text>,

    /// The industry associated with the job position.
    pub industry_prop_enum: Option<IndustryPropEnum>,

    /// Description of benefits associated with the job.
    pub job_benefits_prop_enum: Option<Text>,

    /// An indicator as to whether a position is available for an immediate start.
    pub job_immediate_start_prop_enum: Option<Boolean>,

    /// A (typically single) geographic location associated with the job position.
    pub job_location_prop_enum: Option<Place>,

    /// A description of the job location (e.g. TELECOMMUTE for telecommute jobs).
    pub job_location_type_prop_enum: Option<Text>,

    /// The date on which a successful applicant for this job would be expected to start work. Choose a specific date in the future or use the jobImmediateStart property to indicate the position is to be filled as soon as possible.
    pub job_start_date_prop_enum: Option<JobStartDatePropEnum>,

    /// A category describing the job, preferably using a term from a taxonomy such as <a href="http://www.onetcenter.org/taxonomy.html">BLS O*NET-SOC</a>, <a href="https://www.ilo.org/public/english/bureau/stat/isco/isco08/">ISCO-08</a> or similar, with the property repeated for each applicable value. Ideally the taxonomy should be identified, and both the textual label and formal code for the category should be provided.<br/><br/>  Note: for historical reasons, any textual label and formal code provided as a literal may be assumed to be from O*NET-SOC.
    pub occupational_category_prop_enum: Option<OccupationalCategoryPropEnum>,

    /// A description of the types of physical activity associated with the job. Defined terms such as those in O*net may be used, but note that there is no way to specify the level of ability as well as its nature when using a defined term.
    pub physical_requirement_prop_enum: Option<PhysicalRequirementPropEnum>,

    /// Specific qualifications required for this role or Occupation.
    pub qualifications_prop_enum: Option<QualificationsPropEnum>,

    /// The Occupation for the JobPosting.
    pub relevant_occupation_prop_enum: Option<Occupation>,

    /// Responsibilities associated with this role or Occupation.
    pub responsibilities_prop_enum: Option<Text>,

    /// The currency (coded using <a href="http://en.wikipedia.org/wiki/ISO_4217">ISO 4217</a>) used for the main salary information in this job posting or for this employee.
    pub salary_currency_prop_enum: Option<Text>,

    /// A description of any security clearance requirements of the job.
    pub security_clearance_requirement_prop_enum: Option<SecurityClearanceRequirementPropEnum>,

    /// A description of any sensory requirements and levels necessary to function on the job, including hearing and vision. Defined terms such as those in O*net may be used, but note that there is no way to specify the level of ability as well as its nature when using a defined term.
    pub sensory_requirement_prop_enum: Option<SensoryRequirementPropEnum>,

    /// A statement of knowledge, skill, ability, task or any other assertion expressing a competency that is desired or required to fulfill this role or to work in this occupation.
    pub skills_prop_enum: Option<SkillsPropEnum>,

    /// Any special commitments associated with this job posting. Valid entries include VeteranCommit, MilitarySpouseCommit, etc.
    pub special_commitments_prop_enum: Option<Text>,

    /// The title of the job.
    pub title_prop_enum: Option<Text>,

    /// The number of positions open for this job posting. Use a positive integer. Do not use if the number of positions is unclear or not known.
    pub total_job_openings_prop_enum: Option<Integer>,

    /// The date after when the item is not valid. For example the end of an offer, salary period, or a period of opening hours.
    pub valid_through_prop_enum: Option<ValidThroughPropEnum>,

    /// The typical working hours for this job (e.g. 1st shift, night shift, 8am-5pm).
    pub work_hours_prop_enum: Option<Text>,
}

impl JobPosting {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
