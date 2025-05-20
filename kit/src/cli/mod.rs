use crate::basic::{Deco, Registerer};
use rustpython::vm::{pymodule, PyRef, VirtualMachine};
use rustpython_vm::builtins::PyModule;

pub mod decorators;
mod holder;

#[pymodule(name = "cli")]
mod module {}

pub fn make(vm: &VirtualMachine) -> PyRef<PyModule> {
    let result = module::make_module(vm);
    result.register::<holder::Object>(vm);
    result.register::<decorators::Cmd>(vm);
    result
}
