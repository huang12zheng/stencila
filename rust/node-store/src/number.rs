use crate::prelude::*;

impl Read for f64 {
    fn load_f64(value: &f64) -> Result<Self> {
        Ok(*value)
    }

    fn load_none() -> Result<Self> {
        Ok(Self::default())
    }
}

impl Write for f64 {
    fn insert_prop(&self, store: &mut WriteStore, obj_id: &ObjId, prop: Prop) -> Result<()> {
        match prop {
            Prop::Map(key) => store.put(obj_id, key, *self)?,
            Prop::Seq(index) => store.insert(obj_id, index, *self)?,
        };
        Ok(())
    }

    fn put_prop(&self, store: &mut WriteStore, obj_id: &ObjId, prop: Prop) -> Result<()> {
        Ok(store.put(obj_id, prop, *self)?)
    }

    fn similarity<S: ReadStore>(&self, store: &S, obj: &ObjId, prop: Prop) -> Result<usize> {
        if let Some((Value::Scalar(scalar), ..)) = store.get(obj, prop)? {
            if let ScalarValue::F64(value) = *scalar {
                if value == *self {
                    return Ok(SIMILARITY_MAX);
                }
            }
        }
        Ok(0)
    }
}
