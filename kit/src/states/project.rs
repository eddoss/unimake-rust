use rustpython::vm::PyObjectRef;
use rustpython::vm::convert::ToPyObject;
use rustpython::vm::{PyResult, VirtualMachine};

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
