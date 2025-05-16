use crate::utils;

use rustpython::vm::{pymodule, VirtualMachine};
use rustpython_vm::builtins::PyModule;
use rustpython_vm::convert::ToPyObject;
use rustpython_vm::{PyRef, PyResult};

#[pymodule(name = "__umk__")]
pub mod py {
    use rustpython::vm::common::lock::PyRwLock;
    use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer, Representable};
    use rustpython::vm::PyObjectRef;
    use rustpython::vm::{pyclass, Py, PyPayload, PyRef, PyResult, VirtualMachine};
    use rustpython_vm::convert::ToPyObject;

    #[derive(Debug, Default, Clone)]
    pub struct Project {
        pub builder: Option<PyObjectRef>,
        pub instance: Option<PyObjectRef>,
        pub custom: bool,
    }

    impl Project {
        pub fn instantiate(&mut self, vm: &VirtualMachine) -> PyResult<()> {
            // let mut callable = object.clone();
            // if is_class {
            //     match object.get_attr("__init__", vm) {
            //         Ok(v) => callable = v,
            //         Err(e) => return Err(e),
            //     }
            // }
            //
            // // Get the code object (contains most signature info)
            // let code_obj = callable.get_attr("__code__", vm)?;
            // let code = code_obj.downcast_ref::<PyCode>().unwrap();
            //
            // // Get raw signature information from a code object
            // code.varnames
            //     .deref()
            //     .iter()
            //     .take(usize::try_from(code.arg_count + code.kwonlyarg_count).unwrap())
            //     .for_each(|&v| println!("{}", v.to_string()));
            //
            if self.builder.is_none() {
                return Err(vm.new_value_error("Could not create project info instance, project builder not found".to_string()));
            }
            let mut instance = crate::project::py::Info::default().to_pyobject(vm);
            if self.custom {
                instance = self.builder.clone().unwrap().call((), vm)?;
            } else {
                self.builder.clone().unwrap().call((instance.clone(),), vm)?;
            }
            self.instance = Some(instance);
            Ok(())
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct State {
        pub project: Project,
    }

    #[pyattr]
    #[pyclass(name = "Object")]
    #[derive(Debug, PyPayload)]
    pub struct Object {
        pub state: PyRwLock<State>,
    }

    impl Default for Object {
        fn default() -> Self {
            Self {
                state: PyRwLock::new(State::default()),
            }
        }
    }

    impl From<State> for Object {
        fn from(value: State) -> Self {
            Self {
                state: PyRwLock::new(value),
            }
        }
    }

    impl DefaultConstructor for Object {}

    impl Initializer for Object {
        type Args = ();

        fn init(zelf: PyRef<Self>, _: Self::Args, _: &VirtualMachine) -> PyResult<()> {
            *zelf.state.write() = State::default();
            Ok(())
        }
    }

    #[pyclass(with(Constructor, Initializer))]
    impl Object {}
}

pub fn module(vm: &VirtualMachine) -> PyRef<PyModule> {
    py::make_module(vm)
}

pub fn init(vm: &VirtualMachine) -> PyResult<()> {
    vm.builtins
        .set_attr("__unimake__", py::Object::default().to_pyobject(vm), vm)
}

pub fn get(vm: &VirtualMachine) -> PyResult<py::State> {
    let dict = vm.builtins.get_attr("__unimake__", vm)?;
    Ok(utils::cast::<py::Object>(vm, dict)?.state.read().clone())
}

pub fn update<F>(vm: &VirtualMachine, func: F) -> PyResult<()>
where
    F: FnOnce(&mut py::State) -> PyResult<()>,
{
    let mut states = get(vm)?;
    func(&mut states)?;
    vm.builtins
        .set_attr("__unimake__", py::Object::from(states).to_pyobject(vm), vm)
}
