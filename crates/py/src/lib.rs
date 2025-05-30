use global as G;
use kit::states;
use kit::states::{Accessor, Entrypoint};
use rustpython::vm::{Interpreter, PyResult, VirtualMachine};
use rustpython::InterpreterConfig;
use umk::Error;
use umk::Result;

#[derive(Debug, Clone)]
pub enum WorkspaceMode {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    root: std::path::PathBuf,
    mode: WorkspaceMode,
}

impl Workspace {
    pub fn exists(&self) -> bool {
        match self.mode {
            WorkspaceMode::File => self.root.join(G::WORKSPACE_FILE).exists(),
            WorkspaceMode::Directory => self.root.join(G::WORKSPACE_DIRECTORY).exists(),
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self.mode, WorkspaceMode::File)
    }

    pub fn is_directory(&self) -> bool {
        matches!(self.mode, WorkspaceMode::Directory)
    }

    pub fn root(&self) -> std::path::PathBuf {
        self.root.clone()
    }

    pub fn path(&self) -> std::path::PathBuf {
        match self.mode {
            WorkspaceMode::File => self
                .root
                .join(G::WORKSPACE_FILE),
            WorkspaceMode::Directory => self
                .root
                .join(G::WORKSPACE_DIRECTORY)
        }
    }
}

pub struct Machine {
    interp: Interpreter,
    workspace: Workspace,
}

impl Machine {
    pub fn load(&self) -> Result {
        if self.workspace.is_directory() {
            self.exec(|vm| {
                vm.insert_sys_path(vm.new_pyobj(self.workspace.path().to_str().unwrap()))?;
                for (name, file) in G::SCRIPTS {
                    if self.workspace.root.join(file).exists() {
                        vm.import(name, 0)?;
                    }
                }
                Ok(())
            })
        } else {
            self.exec(|vm| {
                vm.insert_sys_path(vm.new_pyobj(self.workspace.root.to_str().unwrap()))?;
                vm.import(G::SINGLE_SCRIPT_NAME, 0)
            })
        }
    }

    pub fn instantiate(&self) -> Result {
        self.update(|state, vm| {
            state.project.instantiate(vm)
        })
    }

    pub fn read<F>(&self, f: F) -> Result
    where
        F: Fn(states::Details, &VirtualMachine) -> PyResult<()>,
    {
        self.exec(|vm| vm.umk().read(|details| f(details, vm)))
    }

    pub fn update<F>(&self, f: F) -> Result
    where
        F: Fn(&mut states::Details, &VirtualMachine) -> PyResult<()>,
    {
        self.exec(|vm| vm.umk().update(|details| f(details, vm)))
    }
}

impl Machine {
    fn exec<F, P>(&self, f: F) -> Result
    where
        F: FnOnce(&VirtualMachine) -> PyResult<P>,
    {
        let py_result = self.interp.enter(f);
        let mut text = String::default();
        if let Err(err) = py_result {
            self.interp.enter(|vm| {
                if let Err(err) = vm.write_exception(&mut text, &err) {
                    text = err.to_string();
                }
            });
        }
        if !text.is_empty() {
            Err(Error::new(text.as_str()))
        } else {
            Ok(())
        }
    }
}

impl Machine {
    pub fn from(root: std::path::PathBuf) -> Result<Machine> {
        let mut workspace = Workspace {
            mode: WorkspaceMode::File,
            root,
        };
        if !workspace.exists() {
            workspace.mode = WorkspaceMode::Directory;
        }
        if !workspace.exists() {
            return Err(Error::new(
                format!(
                    "Workspace does not contains '{}' nor '{}'.",
                    global::WORKSPACE_FILE,
                    global::WORKSPACE_DIRECTORY
                )
                    .as_str(),
            ));
        }
        let machine = Self {
            workspace,
            interp: InterpreterConfig::new()
                .init_stdlib()
                .init_hook(Box::new(|vm| vm.add_native_modules(kit::stdlib())))
                .interpreter(),
        };
        machine.exec(|vm| {
            kit::init(vm)?;
            kit::states::setup(vm)
        })?;
        Ok(machine)
    }
}