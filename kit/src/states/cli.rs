use crate::basic;
use ahash::HashMap;
use rustpython::vm::PyObjectRef;
use rustpython_vm::builtins::{PyModule, PyTypeRef};
use rustpython_vm::common::lock::PyRwLock;
use rustpython_vm::compiler::parser::ast::located::Arguments;
use rustpython_vm::types::{Constructor, DefaultConstructor, Initializer};
use rustpython_vm::{PyPayload, PyRef, PyResult, VirtualMachine, pyclass};

#[derive(Debug, Clone)]
pub struct Opt {
    pub name: String,
    pub short: String,
    pub class: PyTypeRef,
    pub default: PyObjectRef,
}

impl Opt {
    pub fn new(name: String, short: String, class: PyTypeRef, default: PyObjectRef) -> Self {
        Self {
            name,
            short,
            class,
            default,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub class: PyTypeRef,
    pub default: PyObjectRef,
}

impl Arg {
    pub fn new(name: String, class: PyTypeRef, default: PyObjectRef) -> Self {
        Self {
            name,
            class,
            default,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub name: String,
    pub options: HashMap<String, Opt>,
    pub arguments: HashMap<String, Arguments>,
    pub function: Option<PyObjectRef>,
}

impl Command {
    pub fn named(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CLI {
    pub commands: HashMap<String, Command>,
}

impl CLI {}

#[pyclass(module = false, name = "umk::cli::CommandInfo")]
#[derive(Debug, PyPayload)]
pub struct CommandInitializer {
    pub details: PyRwLock<Command>,
}

impl basic::Registerable for CommandInitializer {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        basic::register::class::<Self>(vm, module)
    }
}

impl Default for CommandInitializer {
    fn default() -> Self {
        Self {
            details: PyRwLock::new(Default::default()),
        }
    }
}

impl DefaultConstructor for CommandInitializer {}

impl From<Command> for CommandInitializer {
    fn from(value: Command) -> Self {
        Self {
            details: PyRwLock::new(value),
        }
    }
}

impl Initializer for CommandInitializer {
    type Args = ();

    fn init(zelf: PyRef<Self>, _: Self::Args, _: &VirtualMachine) -> PyResult<()> {
        *zelf.details.write() = Command::default();
        Ok(())
    }
}

#[pyclass(with(Constructor, Initializer))]
impl CommandInitializer {}
