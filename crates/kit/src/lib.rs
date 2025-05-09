use rustpython::vm::{PyRef, VirtualMachine, builtins::PyModule, pymodule};
use rustpython_vm::convert::ToPyObject;

mod project;

#[pymodule(name = "umk")]
mod umk {}

pub fn package(vm: &VirtualMachine) -> PyRef<PyModule> {
    let root = umk::make_module(vm);
    {
        let module = project::module(vm);
        root.dict()
            .set_item(module.name.unwrap(), module.to_pyobject(vm), vm)
            .expect("Failed to insert 'project' to framework");
    }
    root
}

// fn emplace(vm: &VirtualMachine, root: &PyRef<PyModule>, child: PyRef<PyModule>, name: &str) {
//     let child = PySetterValue::Assign(child.into());
//     let name = vm.new_pyobj(name).as_interned_str(vm).unwrap();
//     root.as_object()
//         .generic_setattr(name, child, vm)
//         .expect("Failed to nest framework module");
// }
//
