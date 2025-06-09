use kit::states::{VmExtension, VmExtensionAccess};
mod cache;
mod interpreter;
mod mode;

pub use cache::Cache;
pub use cache::Driver as CacheDriver;
pub use cache::Entry as CacheEntry;
pub use cache::FilesystemDriver as FsCacheDriver;
pub use cache::Keys as CacheKeys;

use crate::interpreter::Interpreter;
pub use crate::mode::Mode;
use rustpython::vm::{PyResult, VirtualMachine};
use std::path::PathBuf;

pub struct Workspace {
    root: PathBuf,
    mode: Mode,
    cache: Cache,
    interp: Interpreter,
}

//////////////////////////////////////////////////////////////////
// Constructors
//////////////////////////////////////////////////////////////////

impl Workspace {
    pub fn wrap(root: &PathBuf, cache: cache::Cache) -> umk::Result<Self> {
        let mode = Mode::from(root);
        if mode.is_none() {
            return Err(umk::Error::new(
                format!(
                    "Failed to wrap workspace. Given path is not an {} project: {}",
                    global::project::NAME,
                    root.display()
                )
                    .as_str(),
            ));
        }
        Ok(Self {
            root: root.clone(),
            interp: Interpreter::new()?,
            mode: mode.unwrap(),
            cache,
        })
    }
}

//////////////////////////////////////////////////////////////////
// Attributes
//////////////////////////////////////////////////////////////////

impl Workspace {
    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }

    pub fn is_single(&self) -> bool {
        matches!(self.mode, Mode::Single)
    }

    pub fn is_tree(&self) -> bool {
        matches!(self.mode, Mode::Tree)
    }
}

//////////////////////////////////////////////////////////////////
// Methods
//////////////////////////////////////////////////////////////////

impl Workspace {
    pub fn exists(root: &PathBuf) -> bool {
        Mode::from(root).is_some()
    }

    pub fn load(&self) -> umk::Result {
        match self.mode {
            Mode::Single => self.interp.exec(|vm| {
                vm.insert_sys_path(vm.new_pyobj(self.root.to_str().unwrap()))?;
                vm.import(global::script::SINGLE_NAME, 0)
            }),
            Mode::Tree => self.interp.exec(|vm| {
                let src = self.root.join(global::workspace::DIRECTORY);
                vm.insert_sys_path(vm.new_pyobj(src.to_str().unwrap()))?;
                for (name, file) in global::script::LIST {
                    if src.join(file).exists() {
                        vm.import(name, 0)?;
                    }
                }
                Ok(())
            }),
        }
    }

    pub fn instantiate(&self) -> umk::Result {
        self.set(|state, vm| state.project.instantiate(vm))
    }

    pub fn caches(&self) -> umk::Result {
        let mut cli = umk::Cli::default();
        self.get(|state, _| {
            cli = state.cli.to();
            Ok(())
        })?;
        self.cache.cli.set(cli)
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }

    pub fn cli(&self) -> umk::Result<umk::Cli> {
        let mut result = umk::Cli::default();
        self.get(|state, _| {
            result = state.cli.to();
            Ok(())
        })?;
        Ok(result)
    }

    pub fn call<F>(&self, command: Vec<String>, func: F) -> umk::Result
    where
        F: FnOnce(&kit::states::cli::Command, &VirtualMachine) -> PyResult<()>,
    {
        self.get(|state, vm| {
            Ok(())
        })
    }
}

//////////////////////////////////////////////////////////////////
// Global python states
//////////////////////////////////////////////////////////////////

impl Workspace {
    fn get<F>(&self, f: F) -> umk::Result
    where
        F: FnOnce(kit::states::Details, &VirtualMachine) -> PyResult<()>,
    {
        self.interp
            .exec(|vm| vm.umk().read(|details| f(details, vm)))
    }

    fn set<F>(&self, f: F) -> umk::Result
    where
        F: Fn(&mut kit::states::Details, &VirtualMachine) -> PyResult<()>,
    {
        self.interp
            .exec(|vm| vm.umk().update(|details| f(details, vm)))
    }
}
