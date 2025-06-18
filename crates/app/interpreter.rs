use rustpython::vm::Interpreter as PyInterpreter;
use rustpython::vm::{PyResult, VirtualMachine};
use sdk;

pub struct Interpreter {
    inner: PyInterpreter,
}

impl Interpreter {
    pub fn exec<F, P>(&self, f: F) -> sdk::Result<P>
    where
        F: FnOnce(&VirtualMachine) -> PyResult<P>,
    {
        match self.inner.enter(f) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }
}

impl From<PyInterpreter> for Interpreter {
    fn from(value: PyInterpreter) -> Self {
        Self { inner: value }
    }
}
