use rustpython_vm::class::StaticType;
use rustpython_vm::function::{ArgumentError, FromArgs, FuncArgs};
use rustpython_vm::{PyObjectRef, PyPayload, PyRef, PyResult, VirtualMachine};

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
            ArgumentError::InvalidKeywordArgument(_) => Err(vm.new_value_error("Invalid keyword arguments".to_string())),
            ArgumentError::RequiredKeywordArgument(_) => Err(vm.new_value_error("Required keyword arguments".to_string())),
            ArgumentError::Exception(this) => Err(this),
        }
    }
}
