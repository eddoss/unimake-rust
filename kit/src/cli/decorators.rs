use crate::basic;
use crate::cli::holder;
use rustpython::vm::{PyResult, VirtualMachine};
use rustpython_vm::builtins::PyModule;
use rustpython_vm::convert::ToPyObject;
use rustpython_vm::function::{FuncArgs, PyMethodDef, PyMethodFlags};
use rustpython_vm::{FromArgs, PyRef};

pub struct Cmd {}

impl basic::Registerable for Cmd {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        basic::register::function(vm, module, Self::NAME, &Self::METHOD)
    }
}

#[derive(FromArgs, Debug, Clone)]
pub struct CmdOptions {
    #[pyarg(positional)]
    pub name: String,

    #[pyarg(named, default = "None")]
    pub group: Option<String>,
}

impl Cmd {
    const NAME: &'static str = "cmd";
    const METHOD: PyMethodDef = PyMethodDef::new_const(Self::NAME, Self::decorate, PyMethodFlags::empty(), None);

    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let options = basic::args_to::<CmdOptions>(inputs.clone(), vm)?;
        let mut hold = holder::Hodler::default();
        hold.command = options.name;
        hold.group = options.group.unwrap_or_default();
        Ok(holder::Object::from(hold).to_pyobject(vm))
    }
}
