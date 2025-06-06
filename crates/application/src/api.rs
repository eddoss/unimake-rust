use std::path::PathBuf;
use std::rc::Rc;
use umk;

pub fn new(cwd: PathBuf) -> Rc<impl Interface> {
    Rc::new(InterfaceImpl { cwd })
}

//////////////////////////////////////////////////////////////////
// Interfaces
//////////////////////////////////////////////////////////////////

pub trait Interface {
    fn project(&self) -> umk::Result<Box<dyn Project>>;
    fn plugin(&self) -> Box<dyn Plugin>;
}

pub trait Plugin {
    fn list(&self);
}

pub trait Project {
    fn inspect(&self) -> umk::Result;
}

//////////////////////////////////////////////////////////////////
// Implementation: Project
//////////////////////////////////////////////////////////////////

struct ProjectImpl {
    py: Box<executor::Interpreter>,
}

impl Project for ProjectImpl {
    fn inspect(&self) -> umk::Result {
        self.py
            .read(|state, vm| vm.print((state.project.instance,)))
    }
}

//////////////////////////////////////////////////////////////////
// Implementation: Plugin
//////////////////////////////////////////////////////////////////

struct PluginImpl {}

impl Plugin for PluginImpl {
    fn list(&self) {
        todo!()
    }
}

//////////////////////////////////////////////////////////////////
// Implementation: Interface
//////////////////////////////////////////////////////////////////

struct InterfaceImpl {
    cwd: PathBuf,
}

impl Interface for InterfaceImpl {
    fn project(&self) -> umk::Result<Box<dyn Project>> {
        let interpreter = executor::Interpreter::from(self.cwd.clone())?;
        Ok(Box::new(ProjectImpl {
            py: Box::new(interpreter),
        }))
    }

    fn plugin(&self) -> Box<dyn Plugin> {
        Box::new(PluginImpl {})
    }
}
