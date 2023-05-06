// Generated file. Do not edit; see `schema-gen` crate.

use similar::DiffableStr;

use crate::prelude::*;

use super::category_prop_enum::CategoryPropEnum;
use super::local_business::LocalBusiness;
use super::medical_business::MedicalBusiness;
use super::medical_organization::MedicalOrganization;
use super::thing::Thing;

/// https://schema.org/Dentist
/// * COMMENT:
/// A dentist.
/// * EXTEND FROM:
/// https://schema.org/LocalBusiness, https://schema.org/MedicalBusiness, https://schema.org/MedicalOrganization
#[skip_serializing_none]
#[derive(Debug, Defaults, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", crate = "common::serde")]
pub struct Dentist {
    /// A category for the item. Greater signs or slashes can be used to informally indicate a category hierarchy.
    pub category: Option<CategoryPropEnum>,

    /// A sub property of object. The collection target of the action.
    pub target_collection: Option<Thing>,

    /// An entity represented by an entry in a list or data feed (e.g. an 'artist' in a list of 'artists').
    pub item: Option<Thing>,

    /// The subject matter of the content.
    pub about: Option<Thing>,
}
impl Dentist {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Into<LocalBusiness> for Dentist {
    fn into(self) -> LocalBusiness {
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }
}

impl Into<MedicalBusiness> for Dentist {
    fn into(self) -> MedicalBusiness {
        let a = serde_json::json!(self);
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }
}

impl Into<MedicalOrganization> for Dentist {
    fn into(self) -> MedicalOrganization {
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }
}
impl Dentist {
    pub fn into_local_business(self) -> LocalBusiness {
        self.into()
    }

    pub fn into_medical_business(self) -> MedicalBusiness {
        self.into()
    }

    pub fn into_medical_organization(self) -> MedicalOrganization {
        self.into()
    }
}
// [LocalBusiness,MedicalBusiness,MedicalOrganization]
pub trait TypeGroup {
    fn check_type<T>(_: &T) -> String;
}
impl TypeGroup for Dentist {
    fn check_type<T>(_: &T) -> String {
        let type_name = &std::any::type_name::<T>()
            .split("::")
            .last()
            .expect("Invalid type name");
        match *type_name {
            "LocalBusiness" | "MedicalBusiness" | "MedicalOrganization" => type_name.to_string(),
            _ => panic!("{type_name} is not in Dentist type group"),
        }
    }
}
