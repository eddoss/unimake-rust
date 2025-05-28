use ahash::HashMapExt;
use rustpython::vm::{builtins::PyModule, pymodule, PyRef, VirtualMachine};
use rustpython::InterpreterConfig;
use rustpython_vm::class::PyClassImpl;
use rustpython_vm::stdlib::StdlibMap;
use rustpython_vm::{Interpreter, PyResult};
use std::borrow::Cow::Borrowed;

pub mod basic;
pub mod cli;
pub mod project;
pub mod states;

#[pymodule(name = "umk")]
mod umk {}

pub fn init(vm: &VirtualMachine) {
    states::init(vm);
    project::init(vm);
    cli::init(vm);
}

pub fn stdlib() -> StdlibMap {
    let mut result = StdlibMap::new();
    result.insert(Borrowed("umk"), Box::new(package));
    result
}

pub fn package(vm: &VirtualMachine) -> PyRef<PyModule> {
    let root = umk::make_module(vm);
    basic::register::submodule(vm, &root, project::init(vm));
    basic::register::submodule(vm, &root, cli::init(vm));
    root
}

pub fn interpreter() -> Interpreter {
    let result = InterpreterConfig::new()
        .init_stdlib()
        .init_hook(Box::new(|vm| vm.add_native_modules(stdlib())))
        .interpreter();
    result.exec(|vm| {
        init(vm);
        states::setup(vm)
    });
    result
}

pub trait Executable {
    fn exec<F, R>(&self, f: F)
    where
        F: FnOnce(&VirtualMachine) -> PyResult<R>;
}

impl Executable for Interpreter {
    fn exec<F, R>(&self, f: F)
    where
        F: FnOnce(&VirtualMachine) -> PyResult<R>,
    {
        match self.enter(|vm| f(vm)) {
            Ok(_) => {}
            Err(e) => {
                self.enter(|vm| vm.print_exception(e));
                std::process::exit(1);
            }
        };
    }
}
