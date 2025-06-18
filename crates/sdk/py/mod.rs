//////////////////////////////////////////////////////////////////
// Mod
//////////////////////////////////////////////////////////////////

pub mod register;
mod signature;
mod utils;

//////////////////////////////////////////////////////////////////
// Register
//////////////////////////////////////////////////////////////////

pub use register::Registerable;
pub use register::Registerer;

//////////////////////////////////////////////////////////////////
// Signature
//////////////////////////////////////////////////////////////////

pub use signature::Argument as SigArg;
pub use signature::Kind as SigKind;
pub use signature::Signature;

//////////////////////////////////////////////////////////////////
// Utils
//////////////////////////////////////////////////////////////////

pub use utils::*;

//////////////////////////////////////////////////////////////////
// Decorator trait
//////////////////////////////////////////////////////////////////

use rustpython::vm::function::PyMethodDef;

pub trait Function: 'static {
    const NAME: &'static str;
    const METHOD: PyMethodDef;
}

//////////////////////////////////////////////////////////////////
// Global objects
//////////////////////////////////////////////////////////////////

pub use state::Container;
pub use state::get;
pub use state::set;

mod state {
    use crate::py::utils;
    use rustpython_vm::common::lock::PyRwLock;
    use rustpython_vm::types::{Constructor, DefaultConstructor, Initializer};
    use rustpython_vm::{PyPayload, PyRef, PyResult, VirtualMachine, pyclass};
    use std::any::Any;
    use std::collections::HashMap;
    use std::sync::Arc;

    pub fn get<T: Send + Sync + 'static>(
        vm: &VirtualMachine,
        key: impl Into<String>,
    ) -> PyResult<Option<Arc<T>>> {
        let object = vm.builtins.get_attr(global::kit::CONTAINER, vm)?;
        let container = utils::cast::<Container>(vm, object)?;
        let key = key.into();
        match container
            .entries
            .read()
            .iter()
            .find(|x| x.0 == &key)
            .map(|x| x.1)
            .cloned()
        {
            None => Ok(None),
            Some(v) => match Arc::<dyn Any + Send + Sync>::downcast::<T>(v) {
                Ok(v) => Ok(Some(v)),
                Err(_) => {
                    let exc = format!("Failed to downcast global key: '{}'", key);
                    let exc = vm.new_runtime_error(exc);
                    Err(exc)
                }
            },
        }
    }

    pub fn set<T: Send + Sync + 'static>(
        vm: &VirtualMachine,
        key: impl Into<String>,
        value: T,
    ) -> PyResult<()> {
        let object = vm.builtins.get_attr(global::kit::CONTAINER, vm)?;
        let container = utils::cast::<Container>(vm, object)?;
        let key = key.into();
        container.entries.write().insert(key, Arc::new(value));
        Ok(())
    }

    #[pyclass(module = false, name = "UnimakeGlobals")]
    #[derive(Debug, PyPayload)]
    pub struct Container {
        entries: PyRwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
    }

    impl Default for Container {
        fn default() -> Self {
            Self {
                entries: PyRwLock::new(HashMap::default()),
            }
        }
    }

    impl DefaultConstructor for Container {}

    impl Initializer for Container {
        type Args = ();

        fn init(zelf: PyRef<Self>, _: Self::Args, _: &VirtualMachine) -> PyResult<()> {
            *zelf.entries.write() = HashMap::new();
            Ok(())
        }
    }

    #[pyclass(with(Constructor, Initializer))]
    impl Container {}
}
