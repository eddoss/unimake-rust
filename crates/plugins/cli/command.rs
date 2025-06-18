use crate::{argument, consts, option};
use builder_pattern::Builder;
use const_format::concatcp;
use rustpython_vm::builtins::{PyBool, PyFloat, PyInt, PyModule, PyStr, PyTypeRef};
use rustpython_vm::class::StaticType;
use rustpython_vm::common::lock::PyRwLock;
use rustpython_vm::convert::{ToPyObject, ToPyResult};
use rustpython_vm::function::{FuncArgs, PyMethodDef, PyMethodFlags};
use rustpython_vm::types::{Constructor, DefaultConstructor, Initializer};
use rustpython_vm::{FromArgs, PyObjectRef, PyPayload, PyRef, PyResult, VirtualMachine, pyclass};
use sdk::py;
use sdk::py::Function;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

//////////////////////////////////////////////////////////////////
// Access
//////////////////////////////////////////////////////////////////

type Container = HashMap<String, Pythonic>;

fn set(cmd: Pythonic, vm: &VirtualMachine) -> PyResult<()> {
    let mut container = list(vm)?;
    if container.contains_key(&cmd.name) {
        let msg = format!("CLI command already exists: {}", cmd.name);
        return Err(vm.new_value_error(msg));
    } else {
        container.insert(cmd.name.clone(), cmd);
    }
    py::set(vm, consts::GLOBAL_KEY, container)
}

fn list(vm: &VirtualMachine) -> PyResult<Container> {
    match py::get::<Container>(vm, consts::GLOBAL_KEY)? {
        None => Ok(Default::default()),
        Some(v) => Ok(v.deref().clone()),
    }
}

//////////////////////////////////////////////////////////////////
// Decorator
//////////////////////////////////////////////////////////////////

#[derive(FromArgs, Debug, Clone, Builder)]
pub struct DecoArgs {
    #[pyarg(positional)]
    pub name: String,

    #[pyarg(any, optional, default = "None")]
    pub help: Option<String>,
}

pub struct Decorator {}

impl Function for Decorator {
    const NAME: &'static str = consts::DECORATOR_CMD;
    const METHOD: PyMethodDef =
        PyMethodDef::new_const(Self::NAME, Self::decorate, PyMethodFlags::empty(), None);
}

impl Decorator {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let inputs = Arc::new(py::args_to::<DecoArgs>(inputs.clone(), vm)?);
        let inner = vm.new_function(
            consts::DECORATOR_CMD_INNER,
            move |args: FuncArgs, vm: &VirtualMachine| -> PyResult {
                let func = args.args.get(0).unwrap().clone();
                let mut cmd = Pythonic::get(&func, vm)?;
                cmd.name = inputs.name.clone();
                cmd.help = inputs.help.clone();
                cmd.function = Some(func.clone());
                set(cmd, vm)?;
                Pythonic::del(&func, vm)?;
                func.to_pyresult(vm)
            },
        );
        inner.to_pyobject(vm).to_pyresult(vm)
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
    pub name: String,
    pub options: HashMap<String, option::Cacheable>,
    pub arguments: HashMap<String, argument::Cacheable>,
    pub help: Option<String>,
}

//////////////////////////////////////////////////////////////////
// Bootstrapper
//////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Pythonic {
    pub name: String,
    pub options: HashMap<String, option::Pythonic>,
    pub arguments: HashMap<String, argument::Pythonic>,
    pub function: Option<PyObjectRef>,
    pub help: Option<String>,
    pub types: Vec<PyTypeRef>,
}

impl Default for Pythonic {
    fn default() -> Self {
        Self {
            name: Default::default(),
            options: Default::default(),
            arguments: Default::default(),
            function: Default::default(),
            help: Default::default(),
            types: vec![
                PyStr::create_static_type(),
                PyInt::create_static_type(),
                PyFloat::create_static_type(),
                PyBool::create_static_type(),
            ],
        }
    }
}

impl Pythonic {
    pub fn get(func: &PyObjectRef, vm: &VirtualMachine) -> PyResult<Pythonic> {
        match func.get_attr(consts::FUNC_ATTR, vm) {
            Ok(v) => match v.downcast::<Binding>() {
                Ok(v) => Ok(v.inner.read().clone()),
                Err(_) => Err(vm.new_runtime_error("Failed to cast CLI command info".to_string())),
            },
            Err(_) => Ok(Default::default()),
        }
    }

    pub fn set(func: &PyObjectRef, value: Pythonic, vm: &VirtualMachine) -> PyResult<()> {
        func.set_attr(consts::FUNC_ATTR, Binding::from(value).to_pyobject(vm), vm)
    }

    pub fn del(func: &PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
        func.del_attr(consts::FUNC_ATTR, vm)
    }
}

impl Pythonic {
    pub fn option(&mut self, value: option::Pythonic, vm: &VirtualMachine) -> PyResult<()> {
        if self.options.contains_key(&value.name) {
            let msg = format!("CLI command option already exists: '{}'", value.name);
            return Err(vm.new_value_error(msg));
        }
        let allowed = self
            .types
            .iter()
            .find(|&x| py::is_same_type(vm, x, &value.class))
            .is_some();
        if !allowed {
            let msg = format!(
                "CLI command option has unsupported type '{}'",
                value.class.name()
            );
            return Err(vm.new_type_error(msg));
        }
        self.options.insert(value.name.clone(), value);
        Ok(())
    }

    pub fn argument(&mut self, value: argument::Pythonic, vm: &VirtualMachine) -> PyResult<()> {
        if self.arguments.contains_key(&value.name) {
            let msg = format!("CLI command argument already exists: '{}'", value.name);
            return Err(vm.new_value_error(msg));
        }
        let allowed = self
            .types
            .iter()
            .find(|&x| py::is_same_type(vm, x, &value.class))
            .is_some();
        if !allowed {
            let msg = format!(
                "CLI command argument has unsupported type '{}'",
                value.class.name()
            );
            return Err(vm.new_type_error(msg));
        }
        self.arguments.insert(value.name.clone(), value);
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////
// Bootstrapper binding
//////////////////////////////////////////////////////////////////

#[pyclass(module = false, name = "Command")]
#[derive(Debug, PyPayload)]
pub struct Binding {
    inner: PyRwLock<Pythonic>,
}

impl py::Registerable for Binding {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::class::<Self>(vm, module)
    }
}

impl Default for Binding {
    fn default() -> Self {
        Self {
            inner: PyRwLock::new(Default::default()),
        }
    }
}

impl DefaultConstructor for Binding {}

impl From<Pythonic> for Binding {
    fn from(value: Pythonic) -> Self {
        Self {
            inner: PyRwLock::new(value),
        }
    }
}

impl Initializer for Binding {
    type Args = ();

    fn init(zelf: PyRef<Self>, _: Self::Args, _: &VirtualMachine) -> PyResult<()> {
        *zelf.inner.write() = Default::default();
        Ok(())
    }
}

#[pyclass(with(Constructor, Initializer))]
impl Binding {}
