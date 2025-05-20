use ahash::HashMapExt;
use rustpython::vm::{PyRef, VirtualMachine, builtins::PyModule, pymodule};
use rustpython_vm::class::PyClassImpl;
use rustpython_vm::stdlib::StdlibMap;
use std::borrow::Cow::Borrowed;

pub mod basic;
pub mod cli;
pub mod project;
pub mod state;

#[pymodule(name = "umk")]
mod umk {}

pub fn init(vm: &VirtualMachine) {
    state::Object::make_class(&vm.ctx);
    project::make(vm);
    cli::make(vm);
}

pub fn stdlib() -> StdlibMap {
    let mut result = StdlibMap::new();
    result.insert(Borrowed("umk"), Box::new(package));
    result
}

pub fn package(vm: &VirtualMachine) -> PyRef<PyModule> {
    let root = umk::make_module(vm);
    basic::register::submodule(vm, &root, project::make(vm));
    basic::register::submodule(vm, &root, cli::make(vm));
    root
}
