use crate::py::Registerer;
use ahash::HashMapExt;
use rustpython::vm::stdlib::StdlibMap;
use rustpython::vm::PyResult;
use rustpython::vm::{builtins::PyModule, pymodule, PyRef, VirtualMachine};
use std::borrow::Cow::Borrowed;

pub mod py;
pub mod cli;
pub mod project;
pub mod states;

#[pymodule(name = "umk")]
mod module {}

pub fn init(vm: &VirtualMachine) -> PyResult<()> {
    states::init(vm);
    project::init(vm);
    cli::init(vm);
    Ok(())
}

pub fn stdlib() -> StdlibMap {
    let package = |vm: &VirtualMachine| -> PyRef<PyModule> {
        let framework = module::make_module(vm);
        py::register::submodule(vm, &framework, project::init(vm));
        py::register::submodule(vm, &framework, cli::init(vm));
        framework
    };

    let mut result = StdlibMap::new();
    result.insert(Borrowed("umk"), Box::new(package));
    result
}
