use std::collections::HashMap;
use rustpython::vm::{
    builtins,
    pymodule,
    pyclass,
    PyPayload,
    PyRef,
    PyResult,
    // PyObjectRef,
    VirtualMachine,
    common::lock::PyRwLock,
    function::PySetterValue,
    // function::FuncArgs,
    types::DefaultConstructor,
    types::Initializer,
    AsObject,
    FromArgs,
};
// use rustpython_vm::class::StaticType;
// use rustpython_vm::convert::ToPyObject;

#[pymodule(name="project")]
pub mod _module {
    use super::*;

    // #[pyattr]
    // #[pyclass(name = "Contributor")]
    // #[derive(Debug, PyPayload, Default)]
    // struct Contributor {
    //     name: PyRwLock<String>,
    //     emails: PyRwLock<Vec<String>>,
    //     socials: PyRwLock<HashMap<String, String>>,
    // }
    //
    // #[pyclass(with(DefaultConstructor, Initializer))]
    // impl Contributor {
    //     #[pygetset]
    //     fn name(&self) -> builtins::PyStr {
    //         builtins::PyStr::from(self.name.read().as_str())
    //     }
    //
    //     #[pygetset(setter)]
    //     pub fn set_name(&self, value: PySetterValue<builtins::PyStrRef>, vm: &VirtualMachine) {
    //         *self.name.write() = value.unwrap().as_str().to_string();
    //     }
    //
    //     // #[pygetset]
    //     // fn emails(&self) -> builtins::PyStr {
    //     //     builtins::PyStr::from(self.name.read().as_str())
    //     // }
    //     //
    //     // #[pygetset(setter)]
    //     // pub fn set_emails(&self, value: PySetterValue<Vec<String>>, vm: &VirtualMachine) {
    //     //     *self.name.write() = value.unwrap().as_str().to_string();
    //     // }
    // }
    //
    // impl DefaultConstructor for Contributor {}
    //
    // #[derive(FromArgs)]
    // struct ContributorInitArgs {
    //     #[pyarg(any, default)]
    //     name: Option<builtins::PyStrRef>,
    //     #[pyarg(any, default)]
    //     emails: Option<builtins::PyListRef>,
    //     #[pyarg(any, default)]
    //     socials: Option<builtins::PyDictRef>,
    // }
    //
    // impl Initializer for Contributor {
    //     type Args = ContributorInitArgs;
    //
    //     fn init(zelf: PyRef<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult<()> {
    //         let this = zelf.as_object().payload::<Self>().unwrap();
    //         *this.name.write() = args.name.unwrap().as_str().to_string();
    //         for item in args.emails.unwrap().borrow_vec().iter() {
    //             println!("{}", item.class().to_string())
    //         }
    //         // *this.emails.write() = args.emails.unwrap();
    //         Ok(())
    //     }
    // }

    // #[pyattr]
    // #[pyclass(name = "Info")]
    // #[derive(Debug, PyPayload)]
    // struct Info {
    //     id: String,
    //     version: String,
    //     name: String,
    //     description: Vec<String>,
    //     contributors: Vec<Contributor>,
    // }

    // #[pyclass]
    // impl Info {
    //     #[pygetset]
    //     fn name(&self) -> PyStr {
    //         PyStr::from(self.name.clone())
    //     }
    //
    //     #[pymethod]
    //     fn show(&self) {
    //         println!("backend: show project '{}'", self.name);
    //     }
    // }

    // #[pyattr]
    // #[pyclass(name = "Project")]
    // #[derive(Debug, PyPayload)]
    // struct Project {
    //     name: String,
    // }
    //
    // #[pyclass]
    // impl Project {
    //     #[pygetset]
    //     fn name(&self) -> builtins::PyStr {
    //         builtins::PyStr::from(self.name.clone())
    //     }
    //
    //     #[pymethod]
    //     fn show(&self) {
    //         println!("backend: show project '{}'", self.name);
    //     }
    // }
    //
    // #[pyfunction]
    // fn get() -> PyResult<Project> {
    //     println!("backend: request project");
    //     let res = Project{
    //         name: String::from("Golang"),
    //     };
    //     return PyResult::from(Ok(res));
    // }
    //
    // #[pyfunction]
    // fn empty(builder: PyObjectRef) -> PyObjectRef {
    //     println!("backend: register 'empty' project");
    //     runtime::enter(|c, v| {
    //         builder.call_with_args(Default::default(), v).expect("failed to call function wrapped by 'empty'");
    //        // c.decorators.project.golang = Option::from(builder.clone());
    //     });
    //     return builder;
    // }
}



#[pyclass(module = "project", name = "Contributor")]
#[derive(Debug, PyPayload, Default)]
struct Contributor {
    name: PyRwLock<String>,
    emails: PyRwLock<Vec<String>>,
    socials: PyRwLock<HashMap<String, String>>,
}

#[pyclass(with(DefaultConstructor, Initializer))]
impl Contributor {
    #[pygetset]
    fn name(&self) -> builtins::PyStr {
        builtins::PyStr::from(self.name.read().as_str())
    }

    #[pygetset(setter)]
    pub fn set_name(&self, value: PySetterValue<builtins::PyStrRef>, vm: &VirtualMachine) {
        *self.name.write() = value.unwrap().as_str().to_string();
    }
}

impl DefaultConstructor for Contributor {}

#[derive(FromArgs)]
struct ContributorInitArgs {
    #[pyarg(any, default)]
    name: Option<builtins::PyStrRef>,
    #[pyarg(any, default)]
    emails: Option<builtins::PyListRef>,
    #[pyarg(any, default)]
    socials: Option<builtins::PyDictRef>,
}

impl Initializer for Contributor {
    type Args = ContributorInitArgs;

    fn init(zelf: PyRef<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult<()> {
        let this = zelf.as_object().payload::<Self>().unwrap();
        *this.name.write() = args.name.unwrap().as_str().to_string();
        for item in args.emails.unwrap().borrow_vec().iter() {
            println!("{}", item.class().to_string())
        }
        // *this.emails.write() = args.emails.unwrap();
        Ok(())
    }
}
pub fn module(vm: &VirtualMachine) -> (&str, PyRef<builtins::PyModule>) {
    (_module::MODULE_NAME, _module::make_module(vm))
}