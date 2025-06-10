use crate::kit::py;
use crate::umk;
use rustpython::vm::builtins::{PyDict, PyDictRef, PyListRef, PyModule, PyStr, PyStrRef};
use rustpython::vm::common::lock::PyRwLock;
use rustpython::vm::convert::ToPyObject;
use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer, Representable};
use rustpython::vm::{pyclass, AsObject, FromArgs, Py, PyPayload, PyResult};
use rustpython::vm::{PyRef, VirtualMachine};

#[pyclass(module = false, name = "Contributor")]
#[derive(Debug, PyPayload)]
pub struct Contributor {
    pub core: PyRwLock<umk::Contributor>,
}

impl py::Registerable for Contributor {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::class::<Self>(vm, module)
    }
}

impl Default for Contributor {
    fn default() -> Self {
        Contributor {
            core: PyRwLock::new(umk::Contributor::default()),
        }
    }
}

impl DefaultConstructor for Contributor {}

impl Representable for Contributor {
    #[inline]
    fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
        Ok(zelf.core.read().to_string())
    }
}

#[derive(FromArgs, Debug)]
pub struct InitArgs {
    #[pyarg(any, default)]
    pub name: Option<String>,

    #[pyarg(any, default)]
    pub emails: Option<Vec<String>>,

    #[pyarg(any, default = "None")]
    pub socials: Option<PyDictRef>,
}

impl Initializer for Contributor {
    type Args = InitArgs;

    fn init(zelf: PyRef<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult<()> {
        let mut target = umk::Contributor::default();
        target.name = args.name.unwrap_or_default();
        target.emails = args.emails.unwrap_or_default();
        match args.socials {
            None => {}
            Some(dict) => {
                for (key, value) in dict.into_iter() {
                    if let (Ok(k), Ok(v)) = (key.str(vm), value.str(vm)) {
                        target.socials.insert(k.to_string(), v.to_string());
                    }
                }
            }
        }
        *zelf.core.write() = target;
        Ok(())
    }
}

#[pyclass(with(Constructor, Initializer, Representable))]
impl Contributor {
    pub fn from_core(value: &umk::Contributor) -> Self {
        Self {
            core: PyRwLock::new(value.clone()),
        }
    }

    #[pygetset]
    pub fn name(&self) -> PyStr {
        let core = self.core.read();
        PyStr::from(core.name.as_str())
    }

    #[pygetset(setter)]
    pub fn set_name(&self, value: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
        self.core.write().name = value.to_string();
        Ok(())
    }

    #[pygetset]
    pub fn emails(&self, vm: &VirtualMachine) -> PyListRef {
        let refs = self
            .core
            .read()
            .emails
            .iter()
            .map(|v| PyStr::from(v.as_str()).into_ref(&vm.ctx).into())
            .collect();
        vm.ctx.new_list(refs)
    }

    #[pygetset(setter)]
    pub fn set_emails(&self, value: PyListRef, vm: &VirtualMachine) -> PyResult<()> {
        py::list::<PyStr, _>(vm, value.borrow_vec().iter(), "Expect list[str]")?;
        self.core.write().emails = value
            .borrow_vec()
            .iter()
            .filter_map(|v| v.downcast_ref::<PyStr>())
            .map(|v| v.to_string())
            .collect();
        Ok(())
    }

    #[pygetset]
    pub fn socials(&self, vm: &VirtualMachine) -> PyDictRef {
        let dict = PyDict::new_ref(&vm.ctx);
        for (key, value) in self.core.read().socials.iter() {
            let k = vm.ctx.new_str(key.as_str());
            let v = vm.ctx.new_str(value.as_str());
            dict.set_item(k.as_object(), v.to_pyobject(vm), vm).unwrap();
        }
        dict.into()
    }

    #[pygetset(setter)]
    pub fn set_socials(&self, value: PyDictRef, vm: &VirtualMachine) -> PyResult<()> {
        let mut core = self.core.write();
        core.socials.clear();
        for (key, val) in value {
            core.socials.insert(
                py::cast::<PyStr>(vm, key)?.to_string(),
                py::cast::<PyStr>(vm, val)?.to_string(),
            );
        }
        Ok(())
    }
}
