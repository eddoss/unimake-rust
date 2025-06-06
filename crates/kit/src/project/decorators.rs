use crate::py;
use crate::states::Accessor;
use crate::states::Entrypoint;
use rustpython::vm::function::FuncArgs;
use rustpython::vm::function::{PyMethodDef, PyMethodFlags};
use rustpython::vm::{PyResult, VirtualMachine};
use rustpython_vm::PyRef;
use rustpython_vm::builtins::PyModule;

pub struct Init {}

impl py::Registerable for Init {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        py::register::function(vm, module, Self::NAME, &Self::METHOD)
    }
}

impl Init {
    const NAME: &'static str = "init";
    const METHOD: PyMethodDef =
        PyMethodDef::new_const(Self::NAME, Self::decorate, PyMethodFlags::empty(), None);

    fn decorate(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult {
        let callable = inputs.args[0].clone();
        vm.umk()
            .update(|s| {
                if s.project.builder.is_some() {
                    return Err(vm.new_type_error("Project 'init' must be used once".to_string()));
                }
                s.project.custom = match callable.class().name().as_ref() {
                    v @ ("type" | "function") => v == "type",
                    _ => {
                        return Err(vm.new_type_error(
                            "Use 'init' only with classes or functions".to_string(),
                        ));
                    }
                };
                s.project.builder = Some(callable);
                Ok(())
            })
            .map(|_| vm.ctx.none())
    }
}
