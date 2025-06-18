use rustpython_vm::builtins::PyTypeRef;
use rustpython_vm::class::StaticType;
use rustpython_vm::function::{ArgumentError, FromArgs, FuncArgs};
use rustpython_vm::{PyObjectRef, PyPayload, PyRef, PyResult, VirtualMachine};

//////////////////////////////////////////////////////////////////
// Casts
//////////////////////////////////////////////////////////////////

pub fn cast<T: PyPayload + StaticType>(
    vm: &VirtualMachine,
    object: impl Into<PyObjectRef>,
) -> PyResult<PyRef<T>> {
    let object = object.into();
    let target = T::static_type().name();
    match object.downcast::<T>() {
        Ok(res) => Ok(res),
        Err(_) => Err(vm.new_type_error(format!("Failed to cast object to {}", target))),
    }
}

pub fn args_to<T: FromArgs>(inputs: FuncArgs, vm: &VirtualMachine) -> PyResult<T> {
    match T::from_args(vm, &mut inputs.clone()) {
        Ok(value) => Ok(value),
        Err(error) => match error {
            ArgumentError::TooFewArgs => Err(vm.new_value_error("Too few args".to_string())),
            ArgumentError::TooManyArgs => Err(vm.new_value_error("Too many args".to_string())),
            ArgumentError::InvalidKeywordArgument(_) => {
                Err(vm.new_value_error("Invalid keyword arguments".to_string()))
            }
            ArgumentError::RequiredKeywordArgument(_) => {
                Err(vm.new_value_error("Required keyword arguments".to_string()))
            }
            ArgumentError::Exception(this) => Err(this),
        },
    }
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

//////////////////////////////////////////////////////////////////
// Checkers
//////////////////////////////////////////////////////////////////

pub fn is_same_type(vm: &VirtualMachine, a: &PyTypeRef, b: &PyTypeRef) -> bool {
    if a.name().to_string() == b.name().to_string() {
        let module_a = a.module(vm).str(vm).unwrap().to_string();
        let module_b = b.module(vm).str(vm).unwrap().to_string();
        return module_a == module_b;
    }
    false
}

pub fn is_list<'a, T, I>(vm: &VirtualMachine, iter: I, message: &str) -> PyResult<()>
where
    I: Iterator<Item = &'a PyObjectRef>,
    T: PyPayload,
{
    for object in iter {
        is::<T>(vm, &object, message)?;
    }
    Ok(())
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
