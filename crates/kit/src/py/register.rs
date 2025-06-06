use crate::py::Decorator;
use rustpython::vm::PyRef;
use rustpython::vm::VirtualMachine;
use rustpython::vm::builtins::PyModule;
use rustpython::vm::class::PyClassImpl;
use rustpython::vm::convert::ToPyObject;
use rustpython::vm::function::PyMethodDef;
use rustpython_vm::class::StaticType;

pub trait Registerer {
    fn register<T: Registerable>(&self, vm: &VirtualMachine);
}

impl Registerer for PyRef<PyModule> {
    fn register<T: Registerable>(&self, vm: &VirtualMachine) {
        T::register(vm, self)
    }
}

pub trait Registerable {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>);
}

pub fn submodule(vm: &VirtualMachine, module: &PyRef<PyModule>, sub: PyRef<PyModule>) {
    let module_name = module.name.unwrap().as_str();
    let submodule_name = sub.name.unwrap().as_str();
    module
        .dict()
        .set_item(submodule_name, sub.to_pyobject(vm), vm)
        .expect(
            format!("Failed to register submodule '{submodule_name}' in module '{module_name}'")
                .as_str(),
        );
}

pub fn class<T: PyClassImpl + StaticType>(vm: &VirtualMachine, module: &PyRef<PyModule>) {
    let target = T::make_class(&vm.ctx);
    let class_name = target.name().to_string();
    let module_name = module.name.unwrap().as_str();
    module
        .dict()
        .set_item(class_name.as_str(), target.to_pyobject(vm), vm)
        .expect(
            format!("Failed to register class '{class_name}' in module '{module_name}'").as_str(),
        );
}

pub fn decorator<T: Decorator>(vm: &VirtualMachine, module: &PyRef<PyModule>) {
    let module_name = module.name.unwrap();
    let name = T::NAME;
    module
        .dict()
        .set_item(T::NAME, T::METHOD.to_function().to_pyobject(vm), vm)
        .expect(
            format!("Failed to register decorator '{name}' in module '{module_name}'").as_str(),
        );
}

pub fn function(
    vm: &VirtualMachine,
    module: &PyRef<PyModule>,
    name: &str,
    method: &'static PyMethodDef,
) {
    let module_name = module.name.unwrap();
    module
        .dict()
        .set_item(name, method.to_function().to_pyobject(vm), vm)
        .expect(format!("Failed to register function '{name}' in module '{module_name}'").as_str());
}
