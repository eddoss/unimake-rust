use crate::basic::Registerer;
use rustpython::vm::{pymodule, PyRef, VirtualMachine};
use rustpython_vm::builtins::PyModule;

pub mod decorators;

#[pymodule(name = "cli")]
mod module {}

pub fn init(vm: &VirtualMachine) -> PyRef<PyModule> {
    let result = module::make_module(vm);
    result.register::<decorators::Cmd>(vm);
    result.register::<decorators::Opt>(vm);
    result.register::<decorators::Arg>(vm);
    result
}
