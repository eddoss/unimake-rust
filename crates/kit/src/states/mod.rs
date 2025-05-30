use rustpython_vm::class::PyClassImpl;
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

pub fn init(vm: &VirtualMachine) {
    State::make_class(&vm.ctx);
    cli::CommandInitializer::make_class(&vm.ctx);
}

pub fn setup(vm: &VirtualMachine) -> PyResult<()> {
    vm.builtins
        .set_attr("__unimake__", State::default().to_pyobject(vm), vm)
}
