use crate::basic;
use rustpython::vm::builtins::PyModule;
use rustpython::vm::common::lock::PyRwLock;
use rustpython::vm::function::FuncArgs;
use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer, Representable};
use rustpython::vm::{pyclass, AsObject, PyPayload, PyResult};
use rustpython::vm::{PyRef, VirtualMachine};
use rustpython_vm::types::Callable;
use rustpython_vm::{Py, PyObjectRef};

#[derive(Debug, Clone, Default)]
pub struct Hodler {
    pub command: String,
    pub group: String,
    pub options: Vec<String>,
    pub arguments: Vec<String>,
    pub callable: Option<PyObjectRef>,
}

#[pyclass(module = false, name = "cli::Holder")]
#[derive(Debug, PyPayload)]
pub struct Object {
    hodler: PyRwLock<Hodler>,
}

impl basic::Registerable for Object {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        basic::register::class::<Self>(vm, module)
    }
}

impl Default for Object {
    fn default() -> Self {
        Object {
            hodler: PyRwLock::new(Hodler::default()),
        }
    }
}

impl DefaultConstructor for Object {}

impl From<Hodler> for Object {
    fn from(value: Hodler) -> Self {
        Self {
            hodler: PyRwLock::new(value),
        }
    }
}

impl Initializer for Object {
    type Args = ();

    fn init(zelf: PyRef<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult<()> {
        *zelf.hodler.write() = Hodler::default();
        Ok(())
    }
}

impl Callable for Object {
    type Args = FuncArgs;
    fn call(zelf: &Py<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult {
        let callable = args.args[0].clone();
        zelf.hodler.write().callable = Some(callable.clone());
        Ok(callable)
    }
}

#[pyclass(with(Constructor, Initializer, Callable))]
impl Object {}
