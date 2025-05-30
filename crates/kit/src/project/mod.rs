use rustpython::vm::{PyRef, VirtualMachine, pymodule};
use rustpython_vm::builtins::PyModule;

pub mod contributor;
pub mod decorators;
pub mod info;

use crate::basic::Registerer;
pub use contributor::Contributor;
pub use info::Info;

#[pymodule(name = "project")]
mod module {}

pub fn init(vm: &VirtualMachine) -> PyRef<PyModule> {
    let result = module::make_module(vm);
    result.register::<Contributor>(vm);
    result.register::<Info>(vm);
    result.register::<decorators::Init>(vm);
    result
}
