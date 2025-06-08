use kit;
use rustpython::vm::Interpreter as PyInterpreter;
use rustpython::vm::{PyResult, VirtualMachine};
use rustpython::InterpreterConfig;
use umk::Error;
use umk::Result;

pub struct Interpreter {
    inner: PyInterpreter,
}

impl Interpreter {
    pub fn exec<F, P>(&self, f: F) -> Result
    where
        F: FnOnce(&VirtualMachine) -> PyResult<P>,
    {
        let py_result = self.inner.enter(f);
        let mut text = String::default();
        if let Err(err) = py_result {
            self.inner.enter(|vm| {
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

impl Interpreter {
    pub fn new() -> Result<Interpreter> {
        let result = Self {
            inner: InterpreterConfig::new()
                .init_stdlib()
                .init_hook(Box::new(|vm| vm.add_native_modules(kit::stdlib())))
                .interpreter(),
        };
        result.exec(|vm| {
            kit::init(vm)?;
            kit::states::setup(vm)
        })?;
        Ok(result)
    }
}
