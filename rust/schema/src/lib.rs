mod prelude;


#[rustfmt::skip]
mod types;
pub use types::*;

use std::collections::HashMap;
use std::{any::Any, collections::HashSet};

use common::eyre::Result;
use common::{
    itertools::Itertools,
    serde::Serialize,
    serde_json::{self, json, to_value, Map, Value},
};
fn object<T>(value: &T) -> Map<String, Value>
where
    T: Serialize,
{
    serde_json::from_value(json!(value)).unwrap_or_else(|_| unreachable!())
}

pub(crate) fn extend<T, I>(origin: &mut T, values: I) -> Result<()>
where
    T: Serialize + for<'de> common::serde::Deserialize<'de>,
    I: IntoIterator,
    I::Item: Serialize,
{
    let origin_map = object(origin);
    let value_maps: Vec<(String, Map<String, Value>)> = values
        .into_iter()
        .map(|value| (check_type(&value), object(&value)))
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
pub fn check_type<T>(_: &T) -> String {
    std::any::type_name::<T>()
        .split("::")
        .last()
        .expect("Invalid type name")
        .to_string()
}
