use rustpython::vm::builtins::PyModule;
use rustpython::vm::{pymodule, PyRef, VirtualMachine};
use rustpython_vm::builtins::{PyListRef, PyStr};
use rustpython_vm::PyObjectRef;

#[pymodule(name = "project")]
mod _module {
    use super::{args, panic_with_trace};
    use core;
    use rustpython::vm::builtins::PyCode;
    use rustpython::vm::builtins::{PyDict, PyDictRef, PyListRef, PyStr, PyStrRef};
    use rustpython::vm::common::lock::PyRwLock;
    use rustpython::vm::convert::ToPyObject;
    use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer, Representable};
    use rustpython::vm::AsObject;
    use rustpython::vm::PyObjectRef;
    use rustpython::vm::{pyclass, Py, PyPayload, PyRef, PyResult, VirtualMachine};
    use rustpython_vm::convert::IntoObject;
    use std::ops::Deref;

    //////////////////////////////////////////////////////////////////
    // Global container
    //////////////////////////////////////////////////////////////////

    #[pyattr(name = "__unimake__")]
    fn __unimake__(vm: &VirtualMachine) -> PyDictRef {
        let dict = vm.ctx.new_dict();
        for key in ["builder", "is_builder_class", "instance"] {
            match dict.set_item(vm.ctx.new_str(key).as_object(), vm.ctx.none(), vm) {
                result => { vm.expect_pyresult(result, "") }
            }
        }
        dict
    }

    mod global {
        use super::*;
        pub fn get(vm: &VirtualMachine, key: &str) -> PyResult<PyObjectRef> {
            match vm.current_globals().get_item(key, vm) {
                Err(_) => Err(vm.new_value_error(format!("Failed to get {MODULE_NAME}.__unimake__[{key}]")))?,
                Ok(v) => Ok(v),
            }
        }

        pub fn set(vm: &VirtualMachine, key: &str, value: impl Into<PyObjectRef>) -> PyResult<()> {
            vm.current_globals().set_item(key, value.into_object(), vm)
        }
    }

    //////////////////////////////////////////////////////////////////
    // Decorators
    //////////////////////////////////////////////////////////////////

    #[pyfunction(name = "init")]
    fn init(object: PyObjectRef, vm: &VirtualMachine) -> PyResult<()> {
        if !object.is_callable() {
            return Err(vm.new_type_error("Failed to decorate not callable".to_string()));
        }

        let is_class = match object.class().name().as_ref() {
            v @ ("type" | "function") => v == "type",
            _ => {
                return Err(vm.new_type_error("Use 'empty' only with classes of functions".to_string()));
            }
        };

        global::set(vm, "builder", object.clone())?;
        global::set(vm, "is_builder_class", vm.ctx.new_bool(is_class))?;

        let mut callable = object.clone();
        if is_class {
            match object.get_attr("__init__", vm) {
                Ok(v) => callable = v,
                Err(e) => return Err(e),
            }
        }

        // Get the code object (contains most signature info)
        let code_obj = callable.get_attr("__code__", vm)?;
        let code = code_obj.downcast_ref::<PyCode>().unwrap();

        // Get raw signature information from a code object
        code.varnames.deref()
            .iter()
            .take(usize::try_from(code.arg_count + code.kwonlyarg_count).unwrap())
            .for_each(|&v| {
                println!("{}", v.to_string())
            });

        Ok(())
    }

    //////////////////////////////////////////////////////////////////
    // Contributor
    //////////////////////////////////////////////////////////////////

    #[pyattr]
    #[pyclass(name = "Contributor")]
    #[derive(Debug, PyPayload)]
    pub struct Contributor {
        core: PyRwLock<core::Contributor>,
    }

    impl Default for Contributor {
        fn default() -> Self {
            Contributor {
                core: PyRwLock::new(core::Contributor::default()),
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

    impl Initializer for Contributor {
        type Args = args::ContributorInit;

        fn init(zelf: PyRef<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult<()> {
            let mut target = core::Contributor::default();
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
        pub fn from_core(value: &core::Contributor) -> Self {
            Self {
                core: PyRwLock::new(value.clone())
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
            super::validate_string_list(vm, &value, "Contributor.emails: expect strings");
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
            value.into_iter().for_each(|(k, v)| {
                let key = super::validate_string(vm, &k, "Contributor.socials: expect string key");
                let value = super::validate_string(vm, &v, "Contributor.socials: expect string value");
                core.socials.insert(key, value);
            });
            Ok(())
        }
    }

    //////////////////////////////////////////////////////////////////
    // Info
    //////////////////////////////////////////////////////////////////

    #[pyattr]
    #[pyclass(name = "Info")]
    #[derive(Debug, PyPayload)]
    pub struct Info {
        core: PyRwLock<core::Info>,
    }

    impl Default for Info {
        fn default() -> Self {
            Info {
                core: PyRwLock::new(core::Info::default()),
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

    impl Initializer for Info {
        type Args = args::InfoInit;

        fn init(zelf: PyRef<Self>, args: Self::Args, _: &VirtualMachine) -> PyResult<()> {
            let mut target = core::Info::default();
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

    #[pyclass(with(Constructor, Initializer, Representable))]
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
            let refs = self.core.read().contributors.iter().map(|v| {
                Contributor::from_core(v).to_pyobject(&vm)
            }).collect();
            vm.ctx.new_list(refs)
        }

        #[pygetset(setter)]
        pub fn set_contributors(&self, value: PyListRef, vm: &VirtualMachine) -> PyResult<()> {
            self.core.write().contributors = value
                .borrow_vec()
                .iter()
                .map(|v| {
                    match v.downcast_ref::<Contributor>() {
                        Some(obj) => obj.core.read().clone(),
                        None => {
                            panic_with_trace(vm, format!("{MODULE_NAME}.Info: expect Contributors, got {}", v.class().name()).as_str());
                            core::Contributor::default()
                        }
                    }
                })
                .collect();
            Ok(())
        }

        // #[pygetset(setter)]
        // pub fn contrib(&self, name: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
        //     self.core.write().title = value.to_string();
        //     Ok(())
        // }
    }
}

pub fn module(vm: &VirtualMachine) -> PyRef<PyModule> {
    _module::make_module(vm)
}

mod args {
    use rustpython_vm::builtins::PyDictRef;
    use rustpython_vm::FromArgs;

    #[derive(FromArgs, Debug)]
    pub struct ContributorInit {
        #[pyarg(any, default)]
        pub name: Option<String>,

        #[pyarg(any, default)]
        pub emails: Option<Vec<String>>,

        #[pyarg(any, default = "None")]
        pub socials: Option<PyDictRef>,
    }

    #[derive(FromArgs, Debug)]
    pub struct InfoInit {
        #[pyarg(any, default)]
        pub name: Option<String>,

        #[pyarg(any, default)]
        pub version: Option<String>,

        #[pyarg(any, default)]
        pub title: Option<String>,

        #[pyarg(any, default)]
        pub description: Option<String>,

        // #[pyarg(any, default)]
        // pub contributors: Option<Vec<Contributor>>,
    }
}

fn validate_string_list(vm: &VirtualMachine, obj: &PyListRef, message: &str) {
    if let Some(index) = not_string_index(obj) {
        println!("[TRACE] index={} is not a 'str'", index);
        panic_with_trace(vm, message)
    }
}

fn validate_string(vm: &VirtualMachine, obj: &PyObjectRef, message: &str) -> String {
    match as_string(obj) {
        Some(res) => res,
        None => {
            panic_with_trace(vm, message);
            "".to_owned()
        }
    }
}

fn as_string(obj: &PyObjectRef) -> Option<String> {
    match obj.downcast_ref::<PyStr>() {
        Some(v) => Some(v.to_string()),
        None => None,
    }
}

fn not_string_index(obj: &PyListRef) -> Option<usize> {
    let mut res: Option<usize> = None;
    obj.borrow_vec()
        .iter()
        .enumerate()
        .for_each(|(index, object)| {
            if !as_string(object).is_some() {
                res = Some(index);
                return;
            }
        });
    res
}

fn panic_with_trace(vm: &VirtualMachine, message: &str) {
    let frame = vm.current_frame().unwrap();
    let location = frame.current_location();
    println!("[TRACE] {}", message);
    println!(
        "[TRACE] executing in {}:{} at {}",
        frame.code.source_path, location.row, location.column
    );
    panic!("")
}
