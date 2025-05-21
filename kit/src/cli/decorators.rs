use crate::basic::Decorator;
use crate::states::Accessor;
use crate::states::Entrypoint;
use crate::states::cli::{Command, CommandInitializer};
use crate::{basic, states};
use rustpython::vm::{PyResult, VirtualMachine};
use rustpython_vm::builtins::{PyModule, PyTypeRef};
use rustpython_vm::convert::{ToPyObject, ToPyResult};
use rustpython_vm::function::{FuncArgs, PyMethodDef, PyMethodFlags};
use rustpython_vm::{FromArgs, PyObjectRef, PyRef};
use std::sync::Arc;

//////////////////////////////////////////////////////////////////
// Command
//////////////////////////////////////////////////////////////////

#[derive(FromArgs, Debug, Clone)]
pub struct CmdOptions {
    #[pyarg(positional)]
    pub name: String,
}

impl Cmd {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let options = Arc::new(basic::args_to::<CmdOptions>(inputs.clone(), vm)?);
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
pub struct OptOptions {
    #[pyarg(positional)]
    pub class: PyTypeRef,

    #[pyarg(positional)]
    pub name: String,

    #[pyarg(any, optional, default = "None")]
    pub short: Option<String>,

    #[pyarg(any, optional, default = "None")]
    pub default: Option<PyObjectRef>,
}

impl Opt {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let options = Arc::new(basic::args_to::<OptOptions>(inputs.clone(), vm)?);
        let inner = vm.new_function(
            "umk::cli::opt::inner",
            move |args: FuncArgs, vm: &VirtualMachine| -> PyResult {
                let func = args.args.get(0).unwrap().clone();
                let mut cmd = get_command_initializer(func.clone(), vm)?;
                cmd.options.insert(
                    options.name.clone(),
                    states::cli::Opt::new(
                        options.name.clone(),
                        options.short.clone().unwrap_or_default(),
                        options.class.clone(),
                        options.default.clone().unwrap_or(vm.ctx.none()),
                    ),
                );
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
        Err(_) => Ok(Command::default()),
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

impl basic::Registerable for Cmd {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        basic::register::decorator::<Self>(vm, module)
    }
}

impl basic::Registerable for Opt {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        basic::register::decorator::<Self>(vm, module)
    }
}
