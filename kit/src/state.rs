use crate::basic;

use rustpython::vm::common::lock::PyRwLock;
use rustpython::vm::convert::ToPyObject;
use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer};
use rustpython::vm::PyObjectRef;
use rustpython::vm::{pyclass, PyPayload, PyRef, PyResult, VirtualMachine};

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
            return Err(vm.new_value_error(
                "Could not create project info instance, project builder not found".to_string(),
            ));
        }
        let mut instance = crate::project::Info::default().to_pyobject(vm);
        if self.custom {
            instance = self.builder.clone().unwrap().call((), vm)?;
        } else {
            self.builder
                .clone()
                .unwrap()
                .call((instance.clone(),), vm)?;
        }
        self.instance = Some(instance);
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct State {
    pub project: Project,
}

#[pyclass(module = false, name = "state::Object")]
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

pub fn init(vm: &VirtualMachine) -> PyResult<()> {
    vm.builtins
        .set_attr("__unimake__", Object::default().to_pyobject(vm), vm)
}

pub trait Entrypoint {
    fn umk(&self) -> impl Accessor;
}

impl Entrypoint for VirtualMachine {
    fn umk(&self) -> impl Accessor {
        EntryHolder::new(self)
    }
}

pub trait Accessor {
    fn read<F: FnOnce(&State) -> PyResult<()>>(&self, func: F) -> PyResult<()>;
    fn update<F: FnOnce(&mut State) -> PyResult<()>>(&self, func: F) -> PyResult<()>;
    fn clone(&self) -> PyResult<State>;
}

struct EntryHolder<'a> {
    vm: &'a VirtualMachine,
}

impl<'a> EntryHolder<'a> {
    fn new(vm: &'a VirtualMachine) -> Self {
        Self { vm }
    }
}

impl<'a> Accessor for EntryHolder<'a> {
    fn read<F: FnOnce(&State) -> PyResult<()>>(&self, func: F) -> PyResult<()> {
        let states = self.clone()?;
        func(&states)?;
        self.vm.builtins.set_attr(
            "__unimake__",
            Object::from(states).to_pyobject(self.vm),
            self.vm,
        )
    }

    fn update<F: FnOnce(&mut State) -> PyResult<()>>(&self, func: F) -> PyResult<()> {
        let mut states = self.clone()?;
        func(&mut states)?;
        self.vm.builtins.set_attr(
            "__unimake__",
            Object::from(states).to_pyobject(self.vm),
            self.vm,
        )
    }

    fn clone(&self) -> PyResult<State> {
        let dict = self.vm.builtins.get_attr("__unimake__", self.vm)?;
        Ok(basic::cast::<Object>(self.vm, dict)?.state.read().clone())
    }
}
