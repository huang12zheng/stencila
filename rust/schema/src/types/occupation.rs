// Generated file. Do not edit; see `schema-gen` crate.

use crate::prelude::*;

use super::action::Action;
use super::administrative_area::AdministrativeArea;
use super::education_requirements_prop_enum::EducationRequirementsPropEnum;
use super::estimated_salary_prop_enum::EstimatedSalaryPropEnum;
use super::experience_requirements_prop_enum::ExperienceRequirementsPropEnum;
use super::identifier_prop_enum::IdentifierPropEnum;
use super::image_prop_enum::ImagePropEnum;
use super::main_entity_of_page_prop_enum::MainEntityOfPagePropEnum;
use super::occupational_category_prop_enum::OccupationalCategoryPropEnum;
use super::qualifications_prop_enum::QualificationsPropEnum;
use super::skills_prop_enum::SkillsPropEnum;
use super::subject_of_prop_enum::SubjectOfPropEnum;
use super::text::Text;
use super::url::URL;

/// https://schema.org/Occupation
/// * COMMENT:
/// A profession, may involve prolonged training and/or a formal qualification.
/// * EXTEND FROM:
/// https://schema.org/Intangible
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Occupation {
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

    /// Educational background needed for the position or Occupation.
    pub education_requirements_prop_enum: Option<EducationRequirementsPropEnum>,

    /// An estimated salary for a job posting or occupation, based on a variety of variables including, but not limited to industry, job title, and location. Estimated salaries  are often computed by outside organizations rather than the hiring organization, who may not have committed to the estimated value.
    pub estimated_salary_prop_enum: Option<EstimatedSalaryPropEnum>,

    /// Description of skills and experience needed for the position or Occupation.
    pub experience_requirements_prop_enum: Option<ExperienceRequirementsPropEnum>,

    /// The region/country for which this occupational description is appropriate. Note that educational requirements and qualifications can vary between jurisdictions.
    pub occupation_location_prop_enum: Option<AdministrativeArea>,

    /// A category describing the job, preferably using a term from a taxonomy such as <a href="http://www.onetcenter.org/taxonomy.html">BLS O*NET-SOC</a>, <a href="https://www.ilo.org/public/english/bureau/stat/isco/isco08/">ISCO-08</a> or similar, with the property repeated for each applicable value. Ideally the taxonomy should be identified, and both the textual label and formal code for the category should be provided.<br/><br/>  Note: for historical reasons, any textual label and formal code provided as a literal may be assumed to be from O*NET-SOC.
    pub occupational_category_prop_enum: Option<OccupationalCategoryPropEnum>,

    /// Specific qualifications required for this role or Occupation.
    pub qualifications_prop_enum: Option<QualificationsPropEnum>,

    /// Responsibilities associated with this role or Occupation.
    pub responsibilities_prop_enum: Option<Text>,

    /// A statement of knowledge, skill, ability, task or any other assertion expressing a competency that is desired or required to fulfill this role or to work in this occupation.
    pub skills_prop_enum: Option<SkillsPropEnum>,
}

impl Occupation {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
