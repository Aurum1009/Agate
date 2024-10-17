use super::object::Object;
use dashu::{
    float::DBig,
    integer::{IBig, UBig},
};
use parking_lot::RwLock;
use std::{sync::Arc, thread::JoinHandle};

#[repr(u8)]
#[derive(Debug, Default)]
pub enum Value {
    Int(i64),
    ApInt(Box<IBig>),
    Uint(u64),
    ApUint(Box<UBig>),
    Float(f64),
    ApFloat(Box<DBig>),
    Object(Arc<RwLock<Object>>),
    Pointer(*const Value),
    MutPtr(*mut Value),
    Thread(Box<JoinHandle<Value>>),
    #[default]
    Null,
    Filler,
}
impl Value {
    /// This deeply clones the value. This will create a new instance of any reference type, and clones any value type.
    pub fn deep_clone(&self) -> Result<Value, &'static str> {
        use Value::*;
        Ok(match self {
            Int(i) => Int(*i),
            ApInt(i) => ApInt(i.clone()),
            Uint(u) => Uint(*u),
            ApUint(u) => ApUint(u.clone()),
            Float(f) => Float(*f),
            ApFloat(f) => ApFloat(f.clone()),
            Object(o) => Object(Arc::new(RwLock::new((*o).read().clone()))),
            Pointer(ptr) => {
                unsafe {
                    let r#ref = &**ptr;
                    return r#ref.deep_clone();
                }
            }
            MutPtr(ptr) => {
                unsafe {
                    let r#ref = &**ptr;
                    return r#ref.deep_clone();
                }
            }
            Thread(_) => return Err("Cannot clone a thread handle"),
            Null => Null,
            Filler => Filler,
        })
    }
    /// Weakly clones the given value. For any value types, this performs the exact same as `Value::deep_clone`, but for reference
    /// types this clones the reference, not the underlying value
    pub fn weak_clone(&self) -> Result<Value, &'static str> {
        use Value::*;

        Ok(match self {
            Int(i) => Int(*i),
            ApInt(i) => ApInt(i.clone()),
            Uint(u) => Uint(*u),
            ApUint(u) => ApUint(u.clone()),
            Float(f) => Float(*f),
            ApFloat(f) => ApFloat(f.clone()),
            Object(o) => Object(o.clone()),
            Thread(_) => return Err("Cannot clone a thread handle"),
            Pointer(ptr) => Pointer(*ptr),
            MutPtr(_) => return Err("Cannot weakly clone a mutable pointer (it would be unsafe)"),
            Null => Null,
            Filler => Filler,
        })
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        use Value::*;
        match (self, other) {
            (Int(i1), Int(i2)) => i1 == i2,
            (ApInt(i1), ApInt(i2)) => i1 == i2,
            (Uint(u1), Uint(u2)) => u1 == u2,
            (ApUint(u1), ApUint(u2)) => u1 == u2,
            (Float(f1), Float(f2)) => f1 == f2,
            (ApFloat(f1), ApFloat(f2)) => f1 == f2,
            (Object(o1), Object(o2)) => o1.read().eq(&*o2.read()),
            (Null, Null) => true,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! obj {
    () => {};
}

#[cfg(test)]
mod tests {
    use crate::value::Value;

    #[test]
    fn assert_size() {
        assert!(size_of::<Value>() <= 16);
    }
}
