pub mod register;
mod signature;
mod utils;
mod validate;

pub use register::Registerable;
pub use register::Registerer;
use rustpython::vm::function::PyMethodDef;
pub use signature::*;
pub use utils::*;
pub use validate::*;

pub trait Decorator: 'static {
    const NAME: &'static str;
    const METHOD: PyMethodDef;
}
