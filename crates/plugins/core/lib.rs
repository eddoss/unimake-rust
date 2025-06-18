use rustpython_vm::builtins::PyModule;
use rustpython_vm::{PyRef, VirtualMachine};

pub trait Interface: Drop + Send + Sync {
    fn initialize(&self, vm: &VirtualMachine);
    fn info(&self) -> Info;
    fn examples(&self) -> Vec<String>;
    fn cli(&self, driver: &Box<dyn sdk::CacheDriver>) -> Option<clap::Command>;
    fn cache(&self, driver: &Box<dyn sdk::CacheDriver>);
    fn register(&self, module: &PyRef<PyModule>, vm: &VirtualMachine);
}

pub struct Info {
    pub name: String,
    pub version: String,
    pub description: String,
}
