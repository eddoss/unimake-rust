use crate::basic;
use crate::states::object::{Details, State};
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
    fn read<F: FnOnce(&Details) -> PyResult<()>>(&self, func: F) -> PyResult<()>;
    fn update<F: FnOnce(&mut Details) -> PyResult<()>>(&self, func: F) -> PyResult<()>;
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
    fn read<F: FnOnce(&Details) -> PyResult<()>>(&self, func: F) -> PyResult<()> {
        let states = self.clone()?;
        func(&states)?;
        self.vm.builtins.set_attr(
            "__unimake__",
            State::from(states).to_pyobject(self.vm),
            self.vm,
        )
    }

    fn update<F: FnOnce(&mut Details) -> PyResult<()>>(&self, func: F) -> PyResult<()> {
        let mut states = self.clone()?;
        func(&mut states)?;
        self.vm.builtins.set_attr(
            "__unimake__",
            State::from(states).to_pyobject(self.vm),
            self.vm,
        )
    }

    fn clone(&self) -> PyResult<Details> {
        let dict = self.vm.builtins.get_attr("__unimake__", self.vm)?;
        Ok(basic::cast::<State>(self.vm, dict)?.state.read().clone())
    }
}
