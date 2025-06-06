use crate::project::Contributor;
use crate::py;
use rustpython::vm::FromArgs;
use rustpython::vm::builtins::{PyListRef, PyStr, PyStrRef};
use rustpython::vm::common::lock::PyRwLock;
use rustpython::vm::convert::ToPyObject;
use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer, Representable};
use rustpython::vm::{Py, PyPayload, PyRef, PyResult, VirtualMachine, pyclass};
use rustpython_vm::builtins::PyModule;

#[pyclass(module = false, name = "Info")]
#[derive(Debug, PyPayload)]
pub struct Info {
    core: PyRwLock<umk::Info>,
}

impl py::Registerable for Info {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::class::<Self>(vm, module)
    }
}

impl Default for Info {
    fn default() -> Self {
        Info {
            core: PyRwLock::new(umk::Info::default()),
        }
    }
}

impl DefaultConstructor for Info {}

impl Representable for Info {
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
    pub version: Option<String>,

    #[pyarg(any, default)]
    pub title: Option<String>,

    #[pyarg(any, default)]
    pub description: Option<String>,
}

impl Initializer for Info {
    type Args = InitArgs;

    fn init(zelf: PyRef<Self>, args: Self::Args, _: &VirtualMachine) -> PyResult<()> {
        let mut target = umk::Info::default();
        target.name = args.name.unwrap_or_default();
        target.version = args.version.unwrap_or_default();
        target.title = args.title.unwrap_or_default();
        target.description = args.description.unwrap_or_default();
        // match args.contributors {
        //     None => {}
        //     Some(dict) => {
        //         for (key, value) in dict.into_iter() {
        //             if let (Ok(k), Ok(v)) = (key.str(vm), value.str(vm)) {
        //                 target.socials.insert(k.to_string(), v.to_string());
        //             }
        //         }
        //     }
        // }
        *zelf.core.write() = target;
        Ok(())
    }
}

#[pyclass(flags(BASETYPE), with(Constructor, Initializer, Representable))]
impl Info {
    #[pymethod]
    pub fn show(&self) {
        println!("Call Contributor.show");
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
    pub fn version(&self) -> PyStr {
        let core = self.core.read();
        PyStr::from(core.version.as_str())
    }

    #[pygetset(setter)]
    pub fn set_version(&self, value: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
        self.core.write().version = value.to_string();
        Ok(())
    }

    #[pygetset]
    pub fn description(&self) -> PyStr {
        let core = self.core.read();
        PyStr::from(core.description.as_str())
    }

    #[pygetset(setter)]
    pub fn set_description(&self, value: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
        self.core.write().description = value.to_string();
        Ok(())
    }

    #[pygetset]
    pub fn title(&self) -> PyStr {
        let core = self.core.read();
        PyStr::from(core.title.as_str())
    }

    #[pygetset(setter)]
    pub fn set_title(&self, value: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
        self.core.write().title = value.to_string();
        Ok(())
    }

    #[pygetset]
    pub fn contributors(&self, vm: &VirtualMachine) -> PyListRef {
        let refs = self
            .core
            .read()
            .contributors
            .iter()
            .map(|v| Contributor::from_core(v).to_pyobject(&vm))
            .collect();
        vm.ctx.new_list(refs)
    }

    #[pygetset(setter)]
    pub fn set_contributors(&self, value: PyListRef, vm: &VirtualMachine) -> PyResult<()> {
        let mut core = self.core.write();
        core.contributors.clear();
        for object in value.borrow_vec().iter() {
            let entry = py::to::<Contributor>(vm, object, "Expect list[Contributor]")?;
            core.contributors.push(entry.core.read().clone());
        }
        Ok(())
    }

    // #[pygetset(setter)]
    // pub fn contrib(&self, name: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
    //     self.core.write().title = value.to_string();
    //     Ok(())
    // }
}
