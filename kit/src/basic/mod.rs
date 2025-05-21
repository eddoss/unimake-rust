pub mod register;
mod utils;
mod validate;

pub use register::Registerable;
pub use register::Registerer;
use rustpython::vm::function::PyMethodDef;
pub use utils::*;
pub use validate::*;

pub trait Decorator: 'static {
    const NAME: &'static str;
    const METHOD: PyMethodDef;
}
