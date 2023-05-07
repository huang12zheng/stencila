use std::collections::HashMap;
use std::{any::Any, collections::HashSet};

use common::eyre::Result;
use common::{
    itertools::Itertools,
    serde::Serialize,
    serde_json::{self, json, to_value, Map, Value},
};
use schema::*;

pub fn main() {
    let mut dentist: Dentist = Dentist {
        category: Some(CategoryPropEnum::CategoryCode("122".to_string())),
        target_collection: Some(Thing {}),
        item: Some(Thing {}),
        about: Some(Thing {}),
        ..Default::default()
    };

    // println!("{}", serde_json::to_string(&dentist).unwrap());
    // let v = serde_json::json!(dentist);
    // println!("{}", serde_json::json!(dentist));
    let a = json!(dentist);
    // a.as_object_mut().unwrap().extend(iter)
    schema::Dentist::extend(
        // extend(
        &mut dentist,
        vec![LocalBusiness {
            ..Default::default()
        }],
    )
    .unwrap();
    dbg!(dentist);
    dbg!(std::any::type_name::<LocalBusiness>());
    dbg!(std::any::type_name::<Dentist>());
    // // ,"item":{"text":"1"},"about":{"text":"1"}
    // let dentist: Dentist = common::serde_json::from_str(
    //     r#"{"category":"CategoryCode"}"#,
    // )
    // .unwrap();
    // println!("{}", serde_json::to_string(&dentist).unwrap());
    // let local: LocalBusiness = dentist.clone().into();
    // println!("{local:?}");
    // let local: MedicalBusiness = dentist.clone().into();
    // println!("{local:?}");
    // let local: MedicalOrganization = dentist.clone().into();
    // println!("{local:?}");
}
