use rustpython::vm::builtins::PyModule;
use rustpython::vm::{PyRef, VirtualMachine, pymodule};

#[pymodule(name = "project")]
mod _module {
    use super::args;
    use rustpython::vm::builtins::{PyDict, PyList, PyListRef, PyStr, PyStrRef};
    use rustpython::vm::common::lock::PyRwLock;
    use rustpython::vm::types::{Constructor, DefaultConstructor, Initializer, Representable};
    use rustpython::vm::{Py, PyPayload, PyRef, PyResult, VirtualMachine, pyclass};
    use std::collections::HashMap;

    //////////////////////////////////////////////////////////////////
    // Contributor
    //////////////////////////////////////////////////////////////////

    #[pyattr]
    #[pyclass(name = "Contributor")]
    #[derive(Debug, PyPayload)]
    pub struct Contributor {
        name: PyRwLock<PyStr>,
        emails: PyRwLock<PyList>,
        socials: PyRwLock<PyDict>,
    }

    impl Default for Contributor {
        fn default() -> Self {
            Contributor {
                name: PyRwLock::new(PyStr::from("")),
                emails: PyRwLock::new(PyList::default()),
                socials: PyRwLock::new(PyDict::default()),
            }
        }
    }

    impl DefaultConstructor for Contributor {}

    impl Representable for Contributor {
        #[inline]
        fn repr_str(zelf: &Py<Self>, _vm: &VirtualMachine) -> PyResult<String> {
            Ok(format!(
                "{MODULE_NAME}.Contributor({})",
                zelf.name.read().to_string()
            ))
        }
    }

    impl Initializer for Contributor {
        type Args = args::ContributorInit;

        fn init(zelf: PyRef<Self>, args: Self::Args, vm: &VirtualMachine) -> PyResult<()> {
            *zelf.name.write() = PyStr::from(args.name.unwrap_or_default());
            match args.emails {
                None => *zelf.emails.write() = PyList::default(),
                Some(strings) => {
                    let refs: Vec<_> = strings
                        .iter()
                        .map(|s| PyStr::from(s.as_str()).into_pyobject(vm))
                        .collect();
                    *zelf.emails.write() = PyList::from(refs);
                }
            }
            match args.socials {
                None => {
                    *zelf.socials.write() = PyDict::default();
                }
                Some(dict) => {
                    let mut values: HashMap<String, String> = HashMap::new();
                    for (key, value) in dict.into_iter() {
                        if let (Ok(k), Ok(v)) = (key.str(vm), value.str(vm)) {
                            values.insert(k.to_string(), v.to_string());
                        }
                    }
                    let dict = PyDict::default();
                    for (k, v) in values {
                        dict.get_or_insert(vm, PyStr::from(k).into_pyobject(vm), || {
                            PyStr::from(v).into_pyobject(vm)
                        })?;
                    }
                    *zelf.socials.write() = dict;
                }
            }
            Ok(())
        }
    }

    #[pyclass(with(Constructor, Initializer, Representable))]
    impl Contributor {
        #[pymethod]
        pub fn show(&self) {
            println!("Call Contributor.show");
        }

        #[pygetset]
        pub fn name(&self) -> PyStr {
            PyStr::from(self.name.read().to_string())
        }

        #[pygetset(setter)]
        pub fn set_name(&self, value: PyStrRef, _vm: &VirtualMachine) -> PyResult<()> {
            *self.name.write() = value.as_str().into();
            Ok(())
        }

        #[pygetset]
        pub fn emails(&self, vm: &VirtualMachine) -> PyListRef {
            println!("Call Contributor.emails(getter)");
            vm.ctx
                .new_list(self.emails.write().borrow_vec().to_vec())
                .into()
        }

        #[pygetset(setter)]
        pub fn set_emails(&self, value: PyListRef, vm: &VirtualMachine) -> PyResult<()> {
            println!("Call Contributor.emails(setter)");
            value
                .borrow_vec()
                .iter()
                .for_each(|v| match v.downcast_ref::<PyStr>() {
                    Some(_) => {}
                    None => {
                        super::tracepoint(vm);
                        panic!("Contributor.email: expect strings, but given {:?}", v);
                    }
                });
            *self.emails.write() = PyList::from(value.borrow_vec().to_vec());
            Ok(())
        }
    }

    //////////////////////////////////////////////////////////////////
    // Info
    //////////////////////////////////////////////////////////////////
}

pub fn module(vm: &VirtualMachine) -> PyRef<PyModule> {
    _module::make_module(vm)
}

fn tracepoint(vm: &VirtualMachine) {
    let frame = vm.current_frame().unwrap();
    let location = frame.current_location();
    println!(
        "[TRACE] executing in {}:{} at {}",
        frame.code.source_path, location.row, location.column
    );
}

mod args {
    use rustpython_vm::FromArgs;
    use rustpython_vm::builtins::PyDictRef;

    #[derive(FromArgs, Debug)]
    pub struct ContributorInit {
        #[pyarg(any, default)]
        pub name: Option<String>,

        #[pyarg(any, default)]
        pub emails: Option<Vec<String>>,

        #[pyarg(any, default = "None")]
        pub socials: Option<PyDictRef>,
    }
}
