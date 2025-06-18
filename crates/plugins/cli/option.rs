use crate::class::Class;
use crate::{command, consts};
use builder_pattern::Builder;
use const_format::concatcp;
use rustpython_vm::builtins::{PyModule, PyTypeRef};
use rustpython_vm::convert::{ToPyObject, ToPyResult};
use rustpython_vm::function::{FuncArgs, PyMethodDef, PyMethodFlags};
use rustpython_vm::{FromArgs, PyObjectRef, PyRef, PyResult, VirtualMachine};
use sdk::py;
use sdk::py::Function;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;

//////////////////////////////////////////////////////////////////
// Decorator
//////////////////////////////////////////////////////////////////

#[derive(FromArgs, Debug, Builder, Clone)]
pub struct Pythonic {
    #[pyarg(positional)]
    pub class: PyTypeRef,

    #[pyarg(positional)]
    #[into]
    pub name: String,

    #[pyarg(any, optional, default = "None")]
    pub short: Option<String>,

    #[pyarg(any, optional, default = "None")]
    pub fault: Option<PyObjectRef>,

    #[pyarg(any, optional, default = "None")]
    pub help: Option<String>,

    #[pyarg(any, optional, default = "None", name = "var")]
    pub variable: Option<String>,

    #[pyarg(any, optional, default = "false")]
    pub required: bool,
}

pub struct Decorator {}

impl Function for Decorator {
    const NAME: &'static str = consts::DECORATOR_OPT;
    const METHOD: PyMethodDef =
        PyMethodDef::new_const(Self::NAME, Self::decorate, PyMethodFlags::empty(), None);
}

impl Decorator {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let inputs = Arc::new(py::args_to::<Pythonic>(inputs.clone(), vm)?);
        // TODO validate short name, long name and var
        let inner = vm.new_function(
            concatcp!("cli::", Decorator::NAME),
            move |args: FuncArgs, vm: &VirtualMachine| -> PyResult {
                let func = args.args.get(0).unwrap().clone();
                let mut cmd = command::Pythonic::get(&func, vm)?;
                cmd.option(inputs.deref().clone(), vm)?;
                command::Pythonic::set(&func, cmd, vm)?;
                func.to_pyresult(vm)
            },
        );
        Ok(inner.to_pyobject(vm))
    }
}

impl py::Registerable for Decorator {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::function::<Self>(vm, module)
    }
}

//////////////////////////////////////////////////////////////////
// Cache
//////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cacheable {
    pub class: Class,
    pub name: String,
    pub short: Option<char>,
    pub help: Option<String>,
    pub variable: Option<String>,
    pub required: bool,
}

impl From<Pythonic> for Cacheable {
    fn from(value: Pythonic) -> Self {
        let mut result = Self {
            class: Class::from(value.class.clone()),
            name: value.name.clone(),
            short: None,
            help: value.help.clone(),
            variable: value.variable.clone(),
            required: value.required,
        };
        if let Some(short) = value.short {
            result.short = short.chars().next();
        }
        result
    }
}
