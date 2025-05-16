use rustpython_vm::object::PyObjectPayload;
use rustpython_vm::{AsObject, PyObjectRef, PyRef, PyResult, VirtualMachine};

pub fn cast<T: PyObjectPayload>(
    vm: &VirtualMachine,
    object: impl Into<PyObjectRef>,
) -> PyResult<PyRef<T>> {
    let object = object.into();
    let name = object.class().name().to_string();
    match object.downcast::<T>() {
        Ok(res) => Ok(res),
        Err(_) => Err(vm.new_type_error(format!("Failed to cast object to {}", name))),
    }
}
