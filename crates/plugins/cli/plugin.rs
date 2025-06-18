use crate::{argument, command, option};
use clap::Command;
use plugin::Interface as PluginInterface;
use rustpython::vm::pymodule;
use rustpython_vm::builtins::PyModule;
use rustpython_vm::class::PyClassImpl;
use rustpython_vm::{PyRef, VirtualMachine};
use sdk::py;
use sdk::py::{Registerable, Registerer};

#[derive(Default)]
pub struct Instance {}

impl Drop for Instance {
    fn drop(&mut self) {}
}

impl Registerable for Instance {
    fn register(vm: &VirtualMachine, module: &PyRef<PyModule>) {
        module.register::<option::Decorator>(vm);
        module.register::<argument::Decorator>(vm);
        module.register::<command::Decorator>(vm);
    }
}

impl PluginInterface for Instance {
    fn initialize(&self, vm: &VirtualMachine) {
        command::Binding::make_class(&vm.ctx);
    }

    fn info(&self) -> plugin::Info {
        plugin::Info {
            name: "CLI".to_string(),
            version: "0.1.0".to_string(),
            description: "Allows users to declare its own command line interface".to_string(),
        }
    }

    fn examples(&self) -> Vec<String> {
        todo!()
    }

    fn cli(&self, driver: &Box<dyn sdk::CacheDriver>) -> Option<Command> {
        todo!()
    }

    fn cache(&self, driver: &Box<dyn sdk::CacheDriver>) {
        todo!()
    }

    fn register(&self, root: &PyRef<PyModule>, vm: &VirtualMachine) {
        let module = _module::make_module(vm);
        module.register::<option::Decorator>(vm);
        module.register::<argument::Decorator>(vm);
        module.register::<command::Decorator>(vm);
        py::register::submodule(vm, root, module);
    }
}

#[pymodule(name = "cli")]
mod _module {}
