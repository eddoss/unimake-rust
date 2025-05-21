use rustpython_vm::convert::ToPyObject;
use rustpython_vm::{PyResult, VirtualMachine};

pub mod cli;
mod object;
mod project;
mod vm;

pub use object::Details;
pub use object::State;
pub use project::Project;
pub use vm::Accessor;
pub use vm::Entrypoint;

pub fn init(vm: &VirtualMachine) -> PyResult<()> {
    vm.builtins
        .set_attr("__unimake__", State::default().to_pyobject(vm), vm)
}
