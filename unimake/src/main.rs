use kit;
use kit::state;
use rustpython::vm::builtins::PyBaseExceptionRef;
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
            state::init(vm)
        })
        .map_err(|e| print_exception(&interpreter, e, Some(0)))
        .unwrap();

    interpreter
        .enter(|vm| -> PyResult {
            vm.insert_sys_path(vm.new_pyobj(".unimake"))?;
            vm.import("project", 0)?;
            state::update(vm, |s| s.project.instantiate(vm))?;
            vm.print((state::clone(vm).unwrap().project.instance,))?;
            Ok(vm.ctx.none())
        })
        .map_err(|e| print_exception(&interpreter, e, Some(0)))
        .unwrap();

    // // Execute .unimake
    // interpreter
    //     .enter(|vm| -> PyResult {
    //         let dict = vm.ctx.new_dict();
    //         for key in ["builder", "is_builder_class", "instance"] {
    //             dict.set_item(vm.ctx.new_str(key).as_object(), vm.ctx.none(), vm)?;
    //         }
    //         vm.builtins.set_attr("__unimake__", dict.clone(), vm)?;
    //
    //         vm.insert_sys_path(vm.new_pyobj(".unimake"))?;
    //         vm.import("project", 0)?;
    //
    //         let project = vm.import("umk", 0)?.get_attr("project", vm)?;
    //         let dict = project.get_attr("__unimake__", vm)?;
    //         vm.print((dict,))?;
    //         // let dict = dict.clone().downcast::<PyDict>();
    //         // if dict.is_err() {
    //         //     return Err(vm.new_value_error("Failed to cast umk.project.__unimake__ to dict".to_string()));
    //         // }
    //         // let object = kit::project::py::Info::default().to_pyobject(vm);
    //         // let dict = dict.unwrap();
    //         // let builder = dict.get_item("builder", vm).unwrap();
    //         // builder.call((object,), vm)
    //         Ok(vm.ctx.none())
    //     })
    //     .map_err(|e| print_exception(&interpreter, e, Some(0)))
    //     .unwrap();

    // Execute .unimake/project.py
    // if let Err(e) = interpreter.enter(|vm| -> PyResult {
    //     let result = vm.import("project", 0);
    //     result
    // }). {
    //
    //     return;
    // }

    // interpreter.enter(|vm| -> PyResult<()> {
    //     let globals = vm.current_globals().get_item("__unimake__", vm)?;
    //     for (k, v) in globals.deref().to_owned().deref() {
    //         let key =
    //         let value = super::validate_string(vm, &v, "Contributor.socials: expect string value");
    //     }
    //     if let Err(e) = vm.import("project", 0) {
    //         vm.print_exception(e)
    //     }
    // });

    interpreter.finalize(None);
}

fn print_exception(interpreter: &Interpreter, exc: PyBaseExceptionRef, exit: Option<i32>) {
    interpreter.enter(|vm| vm.print_exception(exc));
    if let Some(code) = exit {
        std::process::exit(code);
    }
}
