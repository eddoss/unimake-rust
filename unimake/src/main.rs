use kit;
use kit::states;
use kit::states::Accessor;
use kit::states::Entrypoint;
use rustpython::vm::builtins::PyBaseExceptionRef;
use rustpython::vm::function::FuncArgs;
use rustpython::vm::{Interpreter, PyResult};
use rustpython::InterpreterConfig;

fn main() {
    // Create an interpreter with embedded stdlib.
    let interpreter = InterpreterConfig::new()
        .init_stdlib()
        .init_hook(Box::new(|vm| vm.add_native_modules(kit::stdlib())))
        .interpreter();

    interpreter
        .enter(|vm| -> PyResult<()> {
            kit::init(vm);
            states::init(vm)
        })
        .map_err(|e| print_exception(&interpreter, e, Some(0)))
        .unwrap();

    interpreter
        .enter(|vm| -> PyResult {
            vm.insert_sys_path(vm.new_pyobj(".unimake"))?;
            vm.import("project", 0)?;
            Ok(vm.ctx.none())
        })
        .map_err(|e| print_exception(&interpreter, e, Some(0)))
        .unwrap();

    interpreter
        .enter(|vm| -> PyResult {
            let _ = vm.umk().update(|s| s.project.instantiate(vm));
            vm.print((vm.umk().clone().unwrap().project.instance,))?;
            Ok(vm.ctx.none())
        })
        .map_err(|e| print_exception(&interpreter, e, Some(0)))
        .unwrap();

    interpreter
        .enter(|vm| -> PyResult {
            let _ = vm.umk().read(|s| {
                for cmd in s.cli.commands.values() {
                    let mut args = FuncArgs::default();
                    for opt in cmd.options.values() {
                        args.kwargs.insert(opt.name.clone(), opt.default.clone());
                    }
                    let _ = cmd
                        .function
                        .clone()
                        .expect("no user function")
                        .call(args, vm);
                }
                Ok(())
            });
            Ok(vm.ctx.none())
        })
        .map_err(|e| print_exception(&interpreter, e, Some(0)))
        .unwrap();

    interpreter.finalize(None);
}

fn print_exception(interpreter: &Interpreter, exc: PyBaseExceptionRef, exit: Option<i32>) {
    interpreter.enter(|vm| vm.print_exception(exc));
    if let Some(code) = exit {
        std::process::exit(code);
    }
}
