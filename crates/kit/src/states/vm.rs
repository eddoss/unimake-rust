use crate::basic;
use crate::states::object::{Details, State};
use global as G;
use rustpython_vm::convert::ToPyObject;
use rustpython_vm::{PyResult, VirtualMachine};

pub trait Entrypoint {
    fn umk(&self) -> impl Accessor;
}

impl Entrypoint for VirtualMachine {
    fn umk(&self) -> impl Accessor {
        EntryHolder::new(self)
    }
}

pub trait Accessor {
    fn read<F>(&self, func: F) -> PyResult<()>
    where
        F: FnOnce(Details) -> PyResult<()>;

    fn update<F>(&self, func: F) -> PyResult<()>
    where
        F: FnOnce(&mut Details) -> PyResult<()>;

    fn clone(&self) -> PyResult<Details>;
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
    fn read<F>(&self, func: F) -> PyResult<()>
    where
        F: FnOnce(Details) -> PyResult<()>,
    {
        func(self.clone()?)
    }

    fn update<F>(&self, func: F) -> PyResult<()>
    where
        F: FnOnce(&mut Details) -> PyResult<()>,
    {
        let mut states = self.clone()?;
        func(&mut states)?;
        self.vm.builtins.set_attr(
            G::STATES_OBJECT_NAME,
            State::from(states).to_pyobject(self.vm),
            self.vm,
        )
    }

    fn clone(&self) -> PyResult<Details> {
        let dict = self.vm.builtins.get_attr(G::STATES_OBJECT_NAME, self.vm)?;
        Ok(basic::cast::<State>(self.vm, dict)?.state.read().clone())
    }
}
