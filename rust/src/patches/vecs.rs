use super::prelude::*;
use similar::DiffOp;
use std::cmp::min;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// Implements patching for vectors
impl<Type: Patchable> Patchable for Vec<Type>
where
    Type: Clone + 'static,
{
    patchable_is_same!();

    fn is_equal(&self, other: &Self) -> Result<()> {
        if self.len() != other.len() {
            bail!(Error::NotEqual)
        }
        for index in 0..self.len() {
            self[index].is_equal(&other[index])?
        }
        Ok(())
    }

    fn make_hash<H: Hasher>(&self, state: &mut H) {
        for item in self {
            item.make_hash(state)
        }
    }

    patchable_diff!();

    /// Generate the difference between two vectors.
    ///
    /// If both vectors are zero length, will generate no operations.
    /// Otherwise, if either of the vectors are of zero length, will generate
    /// a `Replace` operation. Otherwise, will perform a Patience diff on the
    /// vectors.
    fn diff_same(&self, differ: &mut Differ, other: &Self) {
        if self.is_empty() && other.is_empty() {
            return;
        }

        if self.is_empty() && !other.is_empty() {
            return differ.append(vec![Operation::Add {
                address: Address::from(0),
                value: Box::new(other.clone()),
                length: other.len(),
            }]);
        }

        if !self.is_empty() && other.is_empty() {
            return differ.append(vec![Operation::Remove {
                address: Address::from(0),
                items: self.len(),
            }]);
        }

        let (me_ids, other_ids) = unique_items(self, other);
        let diff_ops =
            similar::capture_diff_slices(similar::Algorithm::Patience, &me_ids, &other_ids);

        let mut index = 0;
        let mut ops = Vec::new();
        let mut removes: HashMap<usize, (usize, usize)> = HashMap::new();
        for diff_op in diff_ops {
            match diff_op {
                DiffOp::Equal { len, .. } => index += len,
                DiffOp::Insert {
                    new_index, new_len, ..
                } => {
                    // Attempt to find a previous `Remove` operation, at the top level, with the same value,
                    // remove it, and add a `Move` here. Otherwise add a `Add`.
                    let mut matched = false;
                    let mut shift = 0i32;
                    let added_value = other[new_index..(new_index + new_len)].to_vec();
                    for prev in (0..ops.len()).rev() {
                        let op = &ops[prev];
                        match op {
                            Operation::Add {
                                address, length, ..
                            } => {
                                if address.len() == 1 {
                                    shift -= *length as i32;
                                }
                            }
                            Operation::Remove { address, items, .. } => {
                                if address.len() == 1 {
                                    shift += *items as i32;
                                    let remove_index = if let Slot::Index(remove_index) = address[0]
                                    {
                                        remove_index
                                    } else {
                                        panic!("Should be a index")
                                    };
                                    let removed = removes
                                        .get(&remove_index)
                                        .expect("To have an entry for all removes");
                                    let removed_value =
                                        self[removed.0..(removed.0 + removed.1)].to_vec();
                                    if added_value.is_equal(&removed_value).is_ok() {
                                        ops[prev] = Operation::Move {
                                            from: address.clone(),
                                            items: *items,
                                            to: Address::from((index as i32 + shift) as usize),
                                        };
                                        matched = true;
                                        break;
                                    }
                                }
                            }
                            Operation::Replace {
                                address,
                                items,
                                length,
                                ..
                            } => {
                                if address.len() == 1 {
                                    shift -= *length as i32 - *items as i32
                                }
                            }
                            _ => {}
                        }
                    }
                    if !matched {
                        ops.push(Operation::Add {
                            address: Address::from(index),
                            value: Box::new(added_value),
                            length: new_len,
                        })
                    }

                    index += new_len
                }
                DiffOp::Delete {
                    old_index, old_len, ..
                } => {
                    // Attempt to find a previous `Add` operations, at the top level, with the same value
                    // and replace it with a `Move` from here.
                    let mut matched = false;
                    let mut shift = 0i32;
                    let removed_value = self[old_index..(old_index + old_len)].to_vec();
                    for prev in (0..ops.len()).rev() {
                        let op = &ops[prev];
                        match op {
                            Operation::Add {
                                address,
                                value,
                                length,
                            } => {
                                if address.len() == 1 {
                                    shift -= *length as i32;
                                    let added_value = value
                                        .deref()
                                        .downcast_ref::<Self>()
                                        .expect("To be a Vec<Type>");
                                    if added_value.is_equal(&removed_value).is_ok() {
                                        ops[prev] = Operation::Move {
                                            from: Address::from((index as i32 + shift) as usize),
                                            items: old_len,
                                            to: address.clone(),
                                        };
                                        matched = true;
                                        break;
                                    }
                                }
                            }
                            Operation::Remove { address, items, .. } => {
                                if address.len() == 1 {
                                    shift += *items as i32
                                }
                            }
                            Operation::Replace {
                                address,
                                items,
                                length,
                                ..
                            } => {
                                if address.len() == 1 {
                                    shift -= *length as i32 - *items as i32
                                }
                            }
                            _ => {}
                        }
                    }
                    if !matched {
                        ops.push(Operation::Remove {
                            address: Address::from(index),
                            items: old_len,
                        });
                        removes.insert(index, (old_index, old_len));
                    }
                }
                DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    // Attempt to generate more fine-grained operations for each item instead of
                    // just replacing them all
                    let mut replace_ops = Vec::new();

                    // Diff each item for which there is an old and new item.
                    // Merge `Replace` operations together at this level, rather than have several
                    // replaces at the lower level
                    for item_index in 0usize..min(old_len, new_len) {
                        let mut differ = Differ::default();
                        differ.item(
                            index,
                            &self[old_index + item_index],
                            &other[new_index + item_index],
                        );
                        index += 1;

                        let mut item_ops = differ.patch;
                        // If there is only one operation...
                        if item_ops.len() == 1 {
                            // and its a `Replace`...
                            if let Some(Operation::Replace { address, .. }) = item_ops.get(0) {
                                // at the root of the item.
                                if address.len() == 1 {
                                    // Then, if the previous operation is a `Replace` at the root
                                    if let Some(Operation::Replace {
                                        address: last_address,
                                        items: last_items,
                                        value: last_value,
                                        length: last_length,
                                        ..
                                    }) = replace_ops.last_mut()
                                    {
                                        if last_address.len() == 1 {
                                            *last_items += 1;
                                            last_value
                                                .downcast_mut::<Vec<Type>>()
                                                .expect("To be a Vec<Type>")
                                                .push(other[new_index + item_index].clone());
                                            *last_length += 1;
                                            continue;
                                        }
                                    }

                                    // Otherwise, add it
                                    replace_ops.push(Operation::Replace {
                                        address: address.clone(),
                                        items: 1,
                                        value: Box::new(
                                            vec![other[new_index + item_index].clone()],
                                        ),
                                        length: 1,
                                    });
                                    continue;
                                }
                            }
                        }
                        // Otherwise append to replacement ops
                        replace_ops.append(&mut item_ops);
                    }

                    #[allow(clippy::comparison_chain)]
                    if new_len > old_len {
                        // Add remaining items
                        let length = new_len - old_len;
                        replace_ops.push(Operation::Add {
                            address: Address::from(index),
                            value: Box::new(
                                other[(new_index + old_len)..(new_index + new_len)].to_vec(),
                            ),
                            length,
                        });
                        index += length;
                    } else if new_len < old_len {
                        // If the last op was a `Replace` at level of the vector, them just add to
                        // the number of items. Otherwise, remove remaining items.
                        let mut remove = true;
                        if let Some(Operation::Replace { address, items, .. }) =
                            replace_ops.last_mut()
                        {
                            if address.len() == 1 {
                                *items = *items + old_len - new_len;
                                remove = false;
                            }
                        }
                        if remove {
                            replace_ops.push(Operation::Remove {
                                address: Address::from(index),
                                items: old_len - new_len,
                            });
                            removes.insert(index, (old_index, old_len));
                        }
                    }

                    ops.append(&mut replace_ops);
                }
            }
        }
        differ.append(ops);
    }

    fn apply_add(&mut self, address: &mut Address, value: &Box<dyn Any>) {
        if address.len() == 1 {
            if let Some(Slot::Index(index)) = address.pop_front() {
                let value = if let Some(value) = value.deref().downcast_ref::<Self>() {
                    value
                } else {
                    return invalid_value!();
                };
                *self = [&self[..index], value, &self[index..]].concat().to_vec();
            } else {
                invalid_address!(address)
            }
        } else if let Some(Slot::Index(index)) = address.pop_front() {
            if let Some(item) = self.get_mut(index) {
                item.apply_add(address, value);
            } else {
                invalid_index!(index)
            }
        } else {
            invalid_address!(address)
        }
    }

    fn apply_remove(&mut self, address: &mut Address, items: usize) {
        if address.len() == 1 {
            if let Some(Slot::Index(index)) = address.pop_front() {
                *self = [&self[..index], &self[(index + items)..]].concat().to_vec();
            } else {
                invalid_address!(address)
            }
        } else if let Some(Slot::Index(index)) = address.pop_front() {
            if let Some(item) = self.get_mut(index) {
                item.apply_remove(address, items);
            } else {
                invalid_index!(index)
            }
        } else {
            invalid_address!(address)
        }
    }

    fn apply_replace(&mut self, address: &mut Address, items: usize, value: &Box<dyn Any>) {
        if address.len() == 1 {
            let value = if let Some(value) = value.deref().downcast_ref::<Self>() {
                value
            } else {
                return invalid_value!();
            };
            if let Some(Slot::Index(index)) = address.pop_front() {
                *self = [&self[..index], value, &self[(index + items)..]]
                    .concat()
                    .to_vec();
            } else {
                invalid_address!(address)
            }
        } else if let Some(Slot::Index(index)) = address.pop_front() {
            if let Some(item) = self.get_mut(index) {
                item.apply_replace(address, items, value);
            } else {
                invalid_index!(index)
            }
        } else {
            invalid_address!(address)
        }
    }

    fn apply_move(&mut self, from: &mut Address, items: usize, to: &mut Address) {
        if from.len() == 1 {
            if let (Some(Slot::Index(from)), Some(Slot::Index(to))) =
                (from.pop_front(), to.pop_front())
            {
                *self = if from < to {
                    [
                        &self[..from],
                        &self[(from + items)..to],
                        &self[from..(from + items)],
                        &self[to..],
                    ]
                } else {
                    [
                        &self[..to],
                        &self[from..(from + items)],
                        &self[to..from],
                        &self[(from + items)..],
                    ]
                }
                .concat()
                .to_vec();
            } else {
                invalid_address!(from)
            }
        } else if let Some(Slot::Index(index)) = from.pop_front() {
            if let Some(item) = self.get_mut(index) {
                item.apply_move(from, items, to);
            } else {
                invalid_index!(index)
            }
        } else {
            invalid_address!(from)
        }
    }

    fn apply_transform(&mut self, address: &mut Address, from: &str, to: &str) {
        if address.len() == 1 {
            if let Some(Slot::Index(index)) = address.pop_front() {
                if let Some(item) = self.get_mut(index) {
                    item.apply_transform(address, from, to);
                } else {
                    invalid_index!(index)
                }
            }
        } else {
            invalid_address!(address)
        }
    }
}

/// An item used in the hash map for the `unique_items` function below
struct Item<'lt, Type>
where
    Type: Patchable,
{
    item: &'lt Type,
}

impl<'lt, Type> Hash for Item<'lt, Type>
where
    Type: Patchable,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.make_hash(state)
    }
}

impl<'lt, Type> PartialEq for Item<'lt, Type>
where
    Type: Patchable,
{
    fn eq(&self, other: &Self) -> bool {
        self.item.is_equal(other.item).is_ok()
    }
}

impl<'lt, Type> Eq for Item<'lt, Type> where Type: Patchable {}

/// Generate unique integer ids for items across two vectors using the
/// the `make_hash` trait property.
fn unique_items<Type: Patchable>(a: &[Type], b: &[Type]) -> (Vec<u32>, Vec<u32>) {
    let mut map = HashMap::new();
    let mut id = 0;
    let mut a_ids = Vec::new();
    let mut b_ids = Vec::new();

    for item in a {
        let id = match map.entry(Item { item }) {
            Entry::Occupied(occupied) => *occupied.get(),
            Entry::Vacant(vacant) => {
                id += 1;
                *vacant.insert(id)
            }
        };
        a_ids.push(id);
    }

    for item in b {
        let id = match map.entry(Item { item }) {
            Entry::Occupied(occupied) => *occupied.get(),
            Entry::Vacant(vacant) => {
                id += 1;
                *vacant.insert(id)
            }
        };
        b_ids.push(id);
    }

    (a_ids, b_ids)
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_json, assert_json_eq,
        patches::{apply_new, diff, equal},
    };
    use pretty_assertions::assert_eq;
    use stencila_schema::{Emphasis, InlineContent, Integer, Strong};

    // Test patches that operate on atomic items (integers) with no
    // pass though.
    #[test]
    fn basic() {
        let empty: Vec<Integer> = vec![];
        let a: Vec<Integer> = vec![1];
        let b: Vec<Integer> = vec![1, 2];

        assert!(equal(&empty, &empty));
        assert!(equal(&a, &a));
        assert!(equal(&b, &b));

        assert!(!equal(&empty, &a));
        assert!(!equal(&empty, &b));
        assert!(!equal(&a, &b));

        // Add / replace all

        assert_json!(diff(&empty, &empty), []);

        let patch = diff(&empty, &b);
        assert_json!(
            patch,
            [{ "type": "Add", "address": [0], "value": [1, 2], "length": 2 }]
        );
        assert_eq!(apply_new(&empty, &patch), b);

        let patch = diff(&b, &empty);
        assert_json!(
            patch,
            [{ "type": "Remove", "address": [0], "items": 2 }]
        );
        assert_eq!(apply_new(&b, &patch), empty);

        // Add

        let a: Vec<Integer> = vec![1];
        let b: Vec<Integer> = vec![1, 2];
        let patch = diff(&a, &b);
        assert_json!(
            patch,
            [{ "type": "Add", "address": [1], "value": [2], "length": 1 }]
        );
        assert_eq!(apply_new(&a, &patch), b);

        // Remove

        let a: Vec<Integer> = vec![1, 2];
        let b: Vec<Integer> = vec![1];
        let patch = diff(&a, &b);
        assert_json!(
            patch,
            [{ "type": "Remove", "address": [1], "items": 1 }]
        );
        assert_eq!(apply_new(&a, &patch), b);

        // Replace

        let a: Vec<Integer> = vec![1, 2];
        let b: Vec<Integer> = vec![3, 4];
        let patch = diff(&a, &b);
        assert_json!(
            patch,
            [{ "type": "Replace", "address": [0], "items": 2, "value": [3, 4], "length": 2 }]
        );
        assert_eq!(apply_new(&a, &patch), b);

        // Move

        let a: Vec<Integer> = vec![1, 3];
        let b: Vec<Integer> = vec![3, 1];
        let patch = diff(&a, &b);
        assert_json!(
            patch, [
                { "type": "Move", "from": [1], "items": 1, "to": [0] }
            ]
        );
        assert_eq!(apply_new(&a, &patch), b);

        let a: Vec<Integer> = vec![1, 2, 3, 4];
        let b: Vec<Integer> = vec![2, 3, 1, 4];
        let patch = diff(&a, &b);
        assert_json!(
            patch, [
                { "type": "Move", "from": [0], "items": 1, "to": [3] }
            ]
        );
        assert_eq!(apply_new(&a, &patch), b);
    }

    // Test patches that operate on compound items (strings) to check that
    // fine grained operations are generated for each item and passed through on apply.
    #[test]
    fn item_ops() {
        // Add

        let a = vec!["a".to_string()];
        let b = vec!["ab".to_string()];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Add", "address": [0, 1], "value": "b", "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);

        // Remove

        let a = vec!["ab".to_string()];
        let b = vec!["a".to_string()];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Remove", "address": [0, 1], "items": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);

        // Replace

        let a = vec!["a".to_string()];
        let b = vec!["b".to_string()];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Replace", "address": [0, 0], "items": 1, "value": "b", "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);

        // Transform

        let a = vec![InlineContent::Emphasis(Emphasis {
            content: vec![InlineContent::String("word".to_string())],
            ..Default::default()
        })];
        let b = vec![InlineContent::Strong(Strong {
            content: vec![InlineContent::String("word".to_string())],
            ..Default::default()
        })];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Transform", "address": [0], "from": "Emphasis", "to": "Strong" },
        ]);
        assert_json_eq!(apply_new(&a, &patch), b);
    }

    // As above, but with an extra `Add` or `Remove` as needed.
    #[test]
    fn item_ops_plus() {
        let a = vec!["a".to_string()];
        let b = vec!["ab".to_string(), "c".to_string()];

        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Add", "address": [0, 1], "value": "b", "length": 1 },
            { "type": "Add", "address": [1], "value": ["c"], "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);

        let patch = diff(&b, &a);
        assert_json!(patch, [
            { "type": "Remove", "address": [0, 1], "items": 1 },
            { "type": "Remove", "address": [1], "items": 1 },
        ]);
        assert_eq!(apply_new(&b, &patch), a);
    }

    // Regression tests of minimal failing cases found using property testing
    // and elsewhere.

    #[test]
    fn regression_1() {
        let a = vec![7, 0, 4, 1];
        let b = vec![4, 7, 1, 0, 1];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Move", "from": [2], "items": 1, "to": [0] },
            { "type": "Add", "address": [2], "value": [1], "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_2() {
        let a = vec![0, 6, 2, 4, 2];
        let b = vec![2, 2, 4];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Remove", "address": [0], "items": 2 },
            { "type": "Move", "from": [2], "items": 1, "to": [1] },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_3() {
        let a = vec!["".to_string(), "".to_string()];
        let b = vec![
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
        ];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Add", "address": [0], "value": ["a", "a", "a"], "length": 3 },
            { "type": "Add", "address": [4, 0], "value": "a", "length": 1 },
            { "type": "Add", "address": [5], "value": ["a"], "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_4() {
        let a = vec![6, 1, 1, 1];
        let b = vec![2, 2, 0];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Replace", "address": [0], "items": 4, "value": [2, 2, 0], "length": 3 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_5() {
        let a = vec!["c".to_string(), "".to_string(), "d".to_string()];
        let b = vec!["cd".to_string(), "a".to_string(), "".to_string()];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Add", "address": [0, 1], "value": "d", "length": 1 },
            { "type": "Add", "address": [1], "value": ["a"], "length": 1 },
            { "type": "Remove", "address": [3], "items": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_6() {
        let a = vec!["".to_string(), "a".to_string(), "".to_string()];
        let b = vec![
            "b".to_string(),
            "".to_string(),
            "".to_string(),
            "b".to_string(),
        ];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Add", "address": [0], "value": ["b"], "length": 1 },
            { "type": "Remove", "address": [2], "items": 1 },
            { "type": "Add", "address": [3], "value": ["b"], "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_7() {
        let a = vec![1, 7, 3];
        let b = vec![7, 3, 1];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Move", "from": [0], "items": 1, "to": [3] },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_8() {
        let a = vec![3, 0, 7];
        let b = vec![0, 1, 7, 3];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Move", "from": [0], "items": 1, "to": [3] },
            { "type": "Add", "address": [1], "value": [1], "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }

    #[test]
    fn regression_9() {
        let a = vec![
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "d".to_string(),
        ];
        let b = vec!["a".to_string(), "d".to_string(), "".to_string()];
        let patch = diff(&a, &b);
        assert_json!(patch, [
            { "type": "Add", "address": [0, 0], "value": "a", "length": 1 },
            { "type": "Remove", "address": [1], "items": 2 },
            { "type": "Add", "address": [2], "value": [""], "length": 1 },
        ]);
        assert_eq!(apply_new(&a, &patch), b);
    }
}
