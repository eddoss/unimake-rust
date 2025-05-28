use crate::basic::Decorator;
use crate::states::cli::{Command, CommandInitializer};
use crate::states::Accessor;
use crate::states::Entrypoint;
use crate::{basic, states};
use rustpython::vm::{PyResult, VirtualMachine};
use rustpython_vm::builtins::{PyCode, PyDict, PyFunction, PyModule, PyTypeRef};
use rustpython_vm::convert::{ToPyObject, ToPyResult};
use rustpython_vm::function::{FuncArgs, PyMethodDef, PyMethodFlags};
use rustpython_vm::{AsObject, FromArgs, PyObjectRef, PyRef};
use std::sync::Arc;

//////////////////////////////////////////////////////////////////
// Command
//////////////////////////////////////////////////////////////////

fn get_call_arguments_and_annotations(
    callable: PyObjectRef,
    vm: &VirtualMachine,
) -> PyResult<()> {
    let func = if callable.payload_is::<PyFunction>() {
        callable
            .downcast::<PyFunction>()
            .expect("Failed to downcast callable to the `PyFunction`")
    } else {
        callable
            .get_attr("__call__", vm)?
            .downcast::<PyFunction>()
            .expect("Failed to downcast callable.__call__ to the `PyFunction`")
    };
    // Get the __code__ attribute (Python-style access)
    let code_obj = func.as_object().get_attr("__code__", vm)?;
    let code = code_obj.downcast_ref::<PyCode>()
        .ok_or_else(|| vm.new_type_error("Expected code object".to_string()))?;

    // Get argument names from code object
    let arg_names: Vec<String> = code.arg_names().kwonlyargs.iter()
        .map(|s| s.to_string())
        .collect();

    // Get annotations (handle missing __annotations__)
    let annotations = func
        .as_object()
        .get_attr("__annotations__", vm)
        .unwrap_or_else(|_| vm.ctx.new_dict().into());
    let annotations_dict = annotations.downcast::<PyDict>()
        .expect("Annotations should be a dict");
    for (_, v) in annotations_dict.clone() {
        let a = v.str(vm)?;
        vm.print((a,))?
    }

    println!("Args: {:?}", arg_names);
    vm.print(("Annotations:", annotations_dict.clone()))
}

#[derive(FromArgs, Debug, Clone)]
pub struct CmdArgs {
    #[pyarg(positional)]
    pub name: String,
}

impl Cmd {
    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let options = Arc::new(basic::args_to::<CmdArgs>(inputs.clone(), vm)?);
        let inner = vm.new_function(
            "umk::cli::cmd::inner",
            move |args: FuncArgs, vm: &VirtualMachine| -> PyResult {
                let func = args.args.get(0).unwrap().clone();
                get_call_arguments_and_annotations(func.clone(), vm)?;
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
        let options = Arc::new(basic::args_to::<OptArgs>(inputs.clone(), vm)?);
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
        let options = Arc::new(basic::args_to::<ArgArgs>(inputs.clone(), vm)?);
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

impl basic::Registerable for Arg {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        basic::register::decorator::<Self>(vm, module)
    }
}
