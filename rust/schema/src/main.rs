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

    extend(
        &mut dentist,
        vec![Thing {
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

fn object<T>(value: &T) -> Map<String, Value>
where
    T: Serialize,
{
    serde_json::from_value(json!(value)).unwrap_or_else(|_| unreachable!())
}

pub fn extend<T, I>(origin: &mut T, values: I) -> Result<()>
where
    T: Serialize + for<'de> common::serde::Deserialize<'de> + TypeGroup,
    I: IntoIterator,
    I::Item: Serialize,
{
    let origin_map = object(origin);
    let value_maps: Vec<(String, Map<String, Value>)> = values
        .into_iter()
        .map(|value| (T::check_type(&value), object(&value)))
        .collect();

    // 1. Check keys
    let mut origin_keys: HashSet<_> = origin_map.keys().collect();

    let err_output: Vec<String> = value_maps
        .iter()
        .filter_map(|(type_name, value)| {
            let value_keys: HashSet<_> = value.keys().collect();
            let duplicate_keys = origin_keys
                .intersection(&value_keys)
                .collect::<HashSet<_>>();

            let result = if !duplicate_keys.is_empty() {
                // Duplicate keys found:

                Some(format!("{type_name} with {:?}", duplicate_keys))
            } else {
                None
            };
            origin_keys.extend(value_keys);
            result
        })
        .collect();

    if !err_output.is_empty() {
        common::eyre::bail!("Duplicate keys found: {:?}", err_output);
    }

    // 2. Merge values
    let mut origin_map = origin_map;
    origin_map.extend(value_maps.into_iter().flat_map(|(_, m)| m.into_iter()));

    *origin = serde_json::from_value(Value::Object(origin_map))?;
    Ok(())
}
