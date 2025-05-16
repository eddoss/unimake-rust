use ahash::HashMapExt;
use rustpython::vm::{builtins::PyModule, pymodule, PyRef, VirtualMachine};
use rustpython_vm::convert::ToPyObject;
use rustpython_vm::stdlib::StdlibMap;
use std::borrow::Cow::Borrowed;

pub mod cli;
pub mod project;
pub mod state;
pub mod utils;

#[pymodule(name = "umk")]
mod umk {}

pub fn init(vm: &VirtualMachine) {
    project::module(vm);
    state::module(vm);
    cli::module(vm);
}

pub fn stdlib() -> StdlibMap {
    let mut result = StdlibMap::new();
    result.insert(Borrowed("umk"), Box::new(package));
    result
}

pub fn package(vm: &VirtualMachine) -> PyRef<PyModule> {
    let root = umk::make_module(vm);
    {
        let module = state::module(vm);
        root.dict()
            .set_item(module.name.unwrap(), module.to_pyobject(vm), vm)
            .expect("Failed to insert 'state' to framework");
    }
    {
        let module = project::module(vm);
        root.dict()
            .set_item(module.name.unwrap(), module.to_pyobject(vm), vm)
            .expect("Failed to insert 'project' to framework");
    }
    {
        let module = cli::module(vm);
        root.dict()
            .set_item(module.name.unwrap(), module.to_pyobject(vm), vm)
            .expect("Failed to insert 'cli' to framework");
    }
    root
}
