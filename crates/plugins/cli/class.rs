use rustpython_vm::builtins::{PyBool, PyFloat, PyInt, PyStr, PyTypeRef};
use rustpython_vm::class::StaticType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Class {
    String,
    Integer,
    Float,
    Boolean,
    Custom,
}

impl From<PyTypeRef> for Class {
    fn from(value: PyTypeRef) -> Self {
        if value.fast_issubclass(PyStr::static_type()) {
            Class::String
        } else if value.fast_issubclass(PyInt::static_type()) {
            Class::Integer
        } else if value.fast_issubclass(PyFloat::static_type()) {
            Class::Float
        } else if value.fast_issubclass(PyBool::static_type()) {
            Class::Boolean
        } else {
            Class::Custom
        }
    }
}
