use crate::py::Decorator;
use crate::states::VmExtension;
use crate::states::VmExtensionAccess;
use crate::states::cli::{Command, CommandInitializer};
use crate::{py, states};
use rustpython::vm::builtins::{PyModule, PyTypeRef};
use rustpython::vm::convert::{ToPyObject, ToPyResult};
use rustpython::vm::function::{FuncArgs, PyMethodDef, PyMethodFlags};
use rustpython::vm::{FromArgs, PyObjectRef, PyRef};
use rustpython::vm::{PyResult, VirtualMachine};
use std::sync::Arc;

//////////////////////////////////////////////////////////////////
// Command
//////////////////////////////////////////////////////////////////

#[derive(FromArgs, Debug, Clone)]
pub struct CmdArgs {
    #[pyarg(positional)]
    pub name: String,
}

impl Cmd {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let options = Arc::new(py::args_to::<CmdArgs>(inputs.clone(), vm)?);
        let inner = vm.new_function(
            "umk::cli::cmd::inner",
            move |args: FuncArgs, vm: &VirtualMachine| -> PyResult {
                let func = args.args.get(0).unwrap().clone();
                let mut cmd = get_command_initializer(func.clone(), vm)?;
                cmd.name = options.name.clone();
                cmd.function = Some(func.clone());
                vm.umk().update(|s| {
                    if s.cli.commands.contains_key(&options.name) {
                        return Err(vm.new_value_error(format!(
                            "Command `{}` already exists",
                            options.name
                        )));
                    }
                    s.cli.commands.insert(options.name.clone(), cmd);
                    Ok(())
                })?;
                Ok(func.clone())
            },
        );
        inner.to_pyobject(vm).to_pyresult(vm)
    }
}

//////////////////////////////////////////////////////////////////
// Option
//////////////////////////////////////////////////////////////////

#[derive(FromArgs, Debug)]
pub struct OptArgs {
    #[pyarg(positional)]
    pub class: PyTypeRef,

    #[pyarg(positional)]
    pub name: String,

    #[pyarg(any, optional, default = "None")]
    pub short: Option<String>,

    #[pyarg(any, optional, default = "None")]
    pub default: Option<PyObjectRef>,

    #[pyarg(any, optional, default = "None")]
    pub help: Option<String>,

    #[pyarg(any, optional, default = "None")]
    pub var: Option<String>,

    #[pyarg(any, optional, default = "None")]
    pub required: Option<bool>,
}

impl OptArgs {
    fn get_short(&self) -> Option<char> {
        match &self.short {
            None => None,
            Some(v) => v.chars().next(),
        }
    }
}

impl Opt {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let options = Arc::new(py::args_to::<OptArgs>(inputs.clone(), vm)?);
        // TODO validate short name, long name and var
        let inner = vm.new_function(
            "umk::cli::opt::inner",
            move |args: FuncArgs, vm: &VirtualMachine| -> PyResult {
                let func = args.args.get(0).unwrap().clone();
                let mut cmd = get_command_initializer(func.clone(), vm)?;
                cmd.add_option(
                    vm,
                    options.name.clone(),
                    options.get_short(),
                    options.class.clone(),
                    options.var.clone(),
                    options.default.clone().unwrap_or(vm.ctx.none()),
                    options.help.clone(),
                    options.required.unwrap_or(false),
                )?;
                set_command_initializer(&func, cmd, vm)?;
                func.to_pyresult(vm)
            },
        );
        Ok(inner.to_pyobject(vm))
    }
}

//////////////////////////////////////////////////////////////////
// Argument
//////////////////////////////////////////////////////////////////

#[derive(FromArgs, Debug)]
pub struct ArgArgs {
    #[pyarg(positional)]
    pub class: PyTypeRef,

    #[pyarg(positional)]
    pub name: String,

    #[pyarg(any, optional, default = "None")]
    pub default: Option<PyObjectRef>,

    #[pyarg(any, optional, default = "None")]
    pub help: Option<String>,

    #[pyarg(any, optional, default = "None")]
    pub var: Option<String>,
}

impl Arg {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let options = Arc::new(py::args_to::<ArgArgs>(inputs.clone(), vm)?);
        let inner = vm.new_function(
            "umk::cli::arg::inner",
            move |args: FuncArgs, vm: &VirtualMachine| -> PyResult {
                let func = args.args.get(0).unwrap().clone();
                let mut cmd = get_command_initializer(func.clone(), vm)?;
                cmd.add_argument(
                    vm,
                    options.name.clone(),
                    options.class.clone(),
                    options.var.clone(),
                    options.default.clone().unwrap_or(vm.ctx.none()),
                    options.help.clone(),
                )?;
                set_command_initializer(&func, cmd, vm)?;
                func.to_pyresult(vm)
            },
        );
        Ok(inner.to_pyobject(vm))
    }
}

//////////////////////////////////////////////////////////////////
// Utils
//////////////////////////////////////////////////////////////////

fn get_command_initializer(func: PyObjectRef, vm: &VirtualMachine) -> PyResult<Command> {
    match func.get_attr("__umk_cli_command__", vm) {
        Ok(v) => match v.downcast::<CommandInitializer>() {
            Ok(v) => Ok(v.details.read().clone()),
            Err(_) => Err(vm.new_runtime_error(
                "Failed to cast UMK command initializer in CLI option builder".to_string(),
            )),
        },
        Err(_) => Ok(vm.umk().clone()?.cli.cmd("".to_string())),
    }
}

fn set_command_initializer(
    func: &PyObjectRef,
    value: Command,
    vm: &VirtualMachine,
) -> PyResult<()> {
    func.set_attr(
        "__umk_cli_command__",
        states::cli::CommandInitializer::from(value).to_pyobject(vm),
        vm,
    )
}

//////////////////////////////////////////////////////////////////
// Decorator basics
//////////////////////////////////////////////////////////////////

pub struct Cmd {}
impl Decorator for Cmd {
    const NAME: &'static str = "cmd";
    const METHOD: PyMethodDef =
        PyMethodDef::new_const(Self::NAME, Self::decorate, PyMethodFlags::empty(), None);
}

pub struct Opt {}
impl Decorator for Opt {
    const NAME: &'static str = "opt";
    const METHOD: PyMethodDef =
        PyMethodDef::new_const(Self::NAME, Self::decorate, PyMethodFlags::empty(), None);
}

pub struct Arg {}
impl Decorator for Arg {
    const NAME: &'static str = "arg";
    const METHOD: PyMethodDef =
        PyMethodDef::new_const(Self::NAME, Self::decorate, PyMethodFlags::empty(), None);
}

impl py::Registerable for Cmd {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::decorator::<Self>(vm, module)
    }
}

impl py::Registerable for Opt {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::decorator::<Self>(vm, module)
    }
}

impl py::Registerable for Arg {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::decorator::<Self>(vm, module)
    }
}
