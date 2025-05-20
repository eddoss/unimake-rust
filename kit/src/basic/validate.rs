use rustpython_vm::{AsObject, PyObjectRef, PyPayload, PyRef, PyResult, VirtualMachine};

pub fn list<'a, T, I>(vm: &VirtualMachine, iter: I, message: &str) -> PyResult<()>
where
    I: Iterator<Item = &'a PyObjectRef>,
    T: PyPayload,
{
    for object in iter {
        is::<T>(vm, &object, message)?;
    }
    Ok(())
}

pub fn to<T>(vm: &VirtualMachine, object: &PyObjectRef, message: &str) -> PyResult<PyRef<T>>
where
    T: PyPayload,
{
    match object.clone().downcast::<T>() {
        Ok(res) => Ok(res),
        Err(_) => Err(vm.new_type_error(message.to_string())),
    }
}

pub fn is<T>(vm: &VirtualMachine, object: &PyObjectRef, message: &str) -> PyResult<()>
where
    T: PyPayload,
{
    if object.downcast_ref::<T>().is_some() {
        Ok(())
    } else {
        Err(vm.new_type_error(message.to_string()))
    }
}

pub fn ok<T>(object: &PyObjectRef) -> Option<String>
where
    T: PyPayload,
{
    if !object.downcast_ref::<T>().is_some() {
        return Some(object.class().name().to_string());
    }
    None
}
