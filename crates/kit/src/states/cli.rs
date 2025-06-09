use crate::py;
use rustpython::vm::builtins::{PyBool, PyFloat, PyInt, PyModule, PyStr, PyTypeRef};
use rustpython::vm::class::StaticType;
use rustpython::vm::common::lock::PyRwLock;
use rustpython::vm::pyclass;
use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer};
use rustpython::vm::PyObjectRef;
use rustpython::vm::PyPayload;
use rustpython::vm::PyRef;
use rustpython::vm::PyResult;
use rustpython::vm::VirtualMachine;
use std::collections::HashMap;

fn class(t: &PyTypeRef) -> umk::ArgClass {
    if t.fast_issubclass(PyStr::static_type()) {
        umk::ArgClass::String
    } else if t.fast_issubclass(PyInt::static_type()) {
        umk::ArgClass::Integer
    } else if t.fast_issubclass(PyFloat::static_type()) {
        umk::ArgClass::Float
    } else if t.fast_issubclass(PyBool::static_type()) {
        umk::ArgClass::Boolean
    } else {
        umk::ArgClass::Custom
    }
}

#[derive(Debug, Clone)]
pub struct Opt {
    pub long: String,
    pub short: Option<char>,
    pub class: PyTypeRef,
    pub variable: Option<String>,
    pub default: PyObjectRef,
    pub help: Option<String>,
    pub required: bool,
}

impl Opt {
    pub fn new(
        class: PyTypeRef,
        long: String,
        short: Option<char>,
        variable: Option<String>,
        default: PyObjectRef,
        required: bool,
        help: Option<String>,
    ) -> Self {
        Self {
            long,
            short,
            class,
            default,
            variable,
            required,
            help,
        }
    }

    fn to(&self) -> umk::Opt {
        umk::Opt {
            class: class(&self.class),
            long: self.long.clone(),
            short: self.short,
            help: self.help.clone(),
            required: self.required,
            multiple: false,
            variable: self.variable.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub class: PyTypeRef,
    pub variable: Option<String>,
    pub default: PyObjectRef,
    pub help: Option<String>,
}

impl Arg {
    pub fn new(
        name: String,
        class: PyTypeRef,
        variable: Option<String>,
        default: PyObjectRef,
        help: Option<String>,
    ) -> Self {
        Self {
            name,
            class,
            variable,
            default,
            help,
        }
    }

    fn to(&self) -> umk::Arg {
        umk::Arg {
            class: class(&self.class),
            name: self.name.clone(),
            help: self.help.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Command {
    pub name: String,
    pub options: HashMap<String, Opt>,
    pub arguments: HashMap<String, Arg>,
    pub function: Option<PyObjectRef>,
    pub help: Option<String>,
    types: Vec<PyTypeRef>,
}

impl Command {
    fn to(&self) -> umk::Cmd {
        umk::Cmd {
            name: self.name.clone(),
            help: self.help.clone(),
            options: self.options.values().map(|o| o.to()).collect(),
            arguments: self.arguments.values().map(|a| a.to()).collect(),
        }
    }

    pub fn add_option(
        &mut self,
        vm: &VirtualMachine,
        long: String,
        short: Option<char>,
        class: PyTypeRef,
        variable: Option<String>,
        default: PyObjectRef,
        help: Option<String>,
        required: bool,
    ) -> PyResult<()> {
        if self.options.contains_key(&long) {
            let msg = format!("CLI command option already exists: '{}'", long);
            return Err(vm.new_value_error(msg));
        }
        let allowed = self
            .types
            .iter()
            .find(|&x| py::is_same_type(vm, x, &class))
            .is_some();
        if !allowed {
            let msg = format!("CLI command option has unsupported type '{}'", class.name());
            return Err(vm.new_type_error(msg));
        }
        self.options.insert(
            long.clone(),
            Opt::new(class, long, short, variable, default, required, help),
        );
        Ok(())
    }

    pub fn add_argument(
        &mut self,
        vm: &VirtualMachine,
        name: String,
        class: PyTypeRef,
        variable: Option<String>,
        default: PyObjectRef,
        help: Option<String>,
    ) -> PyResult<()> {
        if self.arguments.contains_key(&name) {
            let msg = format!("CLI command argument already exists: '{}'", name);
            return Err(vm.new_value_error(msg));
        }
        let allowed = self
            .types
            .iter()
            .find(|&x| py::is_same_type(vm, x, &class))
            .is_some();
        if !allowed {
            let msg = format!(
                "CLI command argument has unsupported type '{}'",
                class.name()
            );
            return Err(vm.new_type_error(msg));
        }
        self.arguments
            .insert(name.clone(), Arg::new(name, class, variable, default, help));
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CLI {
    pub commands: HashMap<String, Command>,
    pub types: Vec<PyTypeRef>,
}

impl Default for CLI {
    fn default() -> Self {
        Self {
            commands: HashMap::new(),
            types: vec![
                PyStr::create_static_type(),
                PyInt::create_static_type(),
                PyFloat::create_static_type(),
                PyBool::create_static_type(),
            ],
        }
    }
}

impl CLI {
    pub fn to(&self) -> umk::Cli {
        let mut result = umk::Cli::default();
        result.nodes = self.commands.values().map(|c| c.to()).collect();
        // TODO Links building is required
        result
    }

    pub fn cmd(&self, name: String) -> Command {
        Command {
            name,
            types: self.types.clone(),
            ..Default::default()
        }
    }

    pub fn find(sequence: Vec<String>) -> Option<Command> {
        None
    }
}

#[pyclass(module = false, name = "umk::cli::CommandInfo")]
#[derive(Debug, PyPayload)]
pub struct CommandInitializer {
    pub details: PyRwLock<Command>,
}

impl py::Registerable for CommandInitializer {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::class::<Self>(vm, module)
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
