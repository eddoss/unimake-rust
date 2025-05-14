use std::borrow::Cow::Borrowed;

use ahash::HashMapExt;
use kit;
use rustpython::vm::stdlib::StdlibMap;
use rustpython::InterpreterConfig;

fn main() {
    // Create stdlib and emplace a framework to this.
    let mut stdlib = StdlibMap::new();
    stdlib.insert(Borrowed("umk"), Box::new(kit::package));

    // Create an interpreter with embedded stdlib.
    let interpreter = InterpreterConfig::new()
        .init_stdlib()
        .init_hook(Box::new(|vm| {
            for (name, module) in stdlib {
                vm.add_native_module(name, module);
            }
        }))
        .interpreter();

    // Append .unimake to sys.path
    interpreter.enter(|vm| {
        let unimake = vm.new_pyobj(".unimake");
        vm.insert_sys_path(unimake)
            .expect("failed to add '.unimake' to sys.path");
    });

    // Execute .unimake/project.py
    interpreter.enter(|vm| match vm.import("project", 0) {
        Ok(_) => {}
        Err(exception) => vm.print_exception(exception),
    });
}
