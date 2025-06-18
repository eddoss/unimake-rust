use crate::global;
use crate::kit::py;
use crate::kit::states::object::{Details, State};
use rustpython_vm::convert::ToPyObject;
use rustpython_vm::{PyResult, VirtualMachine};

pub trait VmExtension {
    fn umk(&self) -> impl VmExtensionAccess;
}

impl VmExtension for VirtualMachine {
    fn umk(&self) -> impl VmExtensionAccess {
        EntryHolder::new(self)
    }
}

pub trait VmExtensionAccess {
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

impl<'a> VmExtensionAccess for EntryHolder<'a> {
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
            global::state::CONTAINER,
            State::from(states).to_pyobject(self.vm),
            self.vm,
        )
    }

    fn clone(&self) -> PyResult<Details> {
        let dict = self
            .vm
            .builtins
            .get_attr(global::state::CONTAINER, self.vm)?;
        Ok(py::cast::<State>(self.vm, dict)?.state.read().clone())
    }
}
