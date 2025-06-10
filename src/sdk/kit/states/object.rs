use crate::kit::states::cli::CLI;
use crate::kit::states::project::Project;
use rustpython_vm::common::lock::PyRwLock;
use rustpython_vm::types::{Constructor, DefaultConstructor, Initializer};
use rustpython_vm::{pyclass, PyPayload, PyRef, PyResult, VirtualMachine};

#[derive(Debug, Default, Clone)]
pub struct Details {
    pub project: Project,
    pub cli: CLI,
}

#[pyclass(module = false, name = "state::Object")]
#[derive(Debug, PyPayload)]
pub struct State {
    pub state: PyRwLock<Details>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            state: PyRwLock::new(Details::default()),
        }
    }
}

impl From<Details> for State {
    fn from(value: Details) -> Self {
        Self {
            state: PyRwLock::new(value),
        }
    }
}

impl DefaultConstructor for State {}

impl Initializer for State {
    type Args = ();

    fn init(zelf: PyRef<Self>, _: Self::Args, _: &VirtualMachine) -> PyResult<()> {
        *zelf.state.write() = Details::default();
        Ok(())
    }
}

#[pyclass(with(Constructor, Initializer))]
impl State {}
