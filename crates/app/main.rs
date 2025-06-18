mod cache;
mod interpreter;
mod workspace;

use crate::interpreter::Interpreter;
use crate::workspace::{Mode, Workspace};
use ahash::HashMapExt;
use rustpython::InterpreterConfig;
use rustpython_vm::builtins::PyModule;
use rustpython_vm::class::PyClassImpl;
use rustpython_vm::common::lock::Lazy;
use rustpython_vm::convert::ToPyObject;
use rustpython_vm::stdlib::StdlibMap;
use rustpython_vm::{pymodule, PyRef, VirtualMachine};
use sdk::Error;
use std::env;

type Plugins = Vec<Box<dyn plugin::Interface>>;

const PLUGINS: Lazy<Plugins> = Lazy::new(|| vec![Box::new(plugin_cli::Instance::default())]);
const VM: Lazy<Interpreter> = Lazy::new(|| {
    let kit = {
        let package = |vm: &VirtualMachine| -> PyRef<PyModule> {
            let framework = module::make_module(vm);
            PLUGINS.iter().for_each(|p| p.register(&framework, vm));
            framework
        };
        let mut result = StdlibMap::new();
        result.insert(global::kit::NAME.into(), Box::new(package));
        result
    };

    let result = InterpreterConfig::new()
        .init_stdlib()
        .init_hook(Box::new(|vm| {
            vm.add_native_modules(kit);
        }))
        .interpreter();

    result.enter(|vm| {
        sdk::py::Container::make_class(&vm.ctx);
        PLUGINS.iter().for_each(|p| p.initialize(vm));
        let _ = vm.builtins.set_attr(
            global::kit::CONTAINER,
            sdk::py::Container::default().to_pyobject(&vm),
            vm,
        );
    });

    result.into()
});

fn main() {
    let result = run();
    if result.is_ok() {
        std::process::exit(0);
    }
    match result.err().unwrap() {
        Error::Text(e) => {
            println!("{}", e);
        }
        Error::Io(e) => {
            println!("{}", e);
        }
        Error::Python(e) => {
            let _ = VM.exec(|vm| {
                vm.print_exception(e);
                Ok(())
            });
        }
        Error::Json(e) => {
            println!("{}", e);
        }
    }
}

fn run() -> sdk::Result {
    let cwd = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to get current working directory !");
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    // let cache = Arc::new(FilesystemDriver::new(cwd.join(global::workspace::CACHE)));

    let workspace = Workspace::new(&cwd)?;

    match workspace.mode() {
        Mode::Single => VM.exec(|vm| {
            vm.insert_sys_path(vm.new_pyobj(workspace.root().to_str().unwrap()))?;
            vm.import(global::workspace::SCRIPT, 0)?;
            Ok(())
        }),
        Mode::Tree => todo!("Workspace with tree mode does not support")
    }

    // Ok(())

    // let cwd = match env::current_dir() {
    //     Ok(v) => v,
    //     Err(e) => {
    //         eprintln!("Failed to get current working directory !");
    //         eprintln!("{}", e.to_string());
    //         std::process::exit(1);
    //     }
    // };
    // let workspace = {
    //     if !sdk::Workspace::exists(&cwd) {
    //         None
    //     } else {
    //         let keys = sdk::CacheKeys::default();
    //         let cache_dir = cwd.join(sdk::global::workspace::CACHE);
    //         let driver = sdk::FsCacheDriver::new(cache_dir);
    //         let cache = sdk::Cache::new(keys, Rc::new(RefCell::new(driver)));
    //         match sdk::Workspace::wrap(&cwd, cache) {
    //             Ok(v) => Some(v),
    //             Err(e) => {
    //                 eprintln!("Failed to create workspace instance");
    //                 eprintln!("{}", e.to_string());
    //                 std::process::exit(1);
    //             }
    //         }
    //     }
    // };
    // if let Some(w) = &workspace {
    //     w.load()?;
    // }
    // let app = cli::Interface::new(workspace)?;
    // app.run()

    // cli.run()

    // let interpreter = executor::Interpreter::from(cwd)?;
    // interpreter.load()?;
    // interpreter.instantiate()?;
    // interpreter.read(|state, vm| {
    //     vm.print((state.clone().project.instance,))
    // })

    // let mut graph = Arc::new(cli::Graph::default());
    // interpreter.exec(|vm| {
    //     vm.umk().read(|s| {
    //         graph.clone_from(&Arc::from(cli::Graph::from_py(&s.cli)));
    //         Ok(())
    //     })
    // });
    //
    // let mut app = clap::Command::new("umk")
    //     .version("0.1.0")
    //     .about("Welcome to the Unimake CLI runner");
    //
    // for cmd in &graph.build() {
    //     app = app.subcommand(cmd)
    // }
    //
    // if let Some((cmd_name, sub_matches)) = app.get_matches().subcommand() {
    //     let args = sub_matches.get_one::<String>("src");
    //     if let Some(args) = args {
    //         println!("Executing {}", args);
    //     }
    // }

    // interpreter
    //     .enter(|vm| -> PyResult {
    //         let _ = vm.umk().read(|s| {
    //             for cmd in s.cli.commands.values() {
    //                 let mut args = FuncArgs::default();
    //                 for opt in cmd.options.values() {
    //                     args.kwargs.insert(opt.name.clone(), opt.default.clone());
    //                 }
    //                 let _ = cmd
    //                     .function
    //                     .clone()
    //                     .expect("no user function")
    //                     .call(args, vm);
    //             }
    //             Ok(())
    //         });
    //         Ok(vm.ctx.none())
    //     })
    //     .map_err(|e| print_exception(&interpreter, e, Some(0)))
    //     .unwrap();

    // interpreter.finalize(None);
}

#[pymodule(name = "umk")]
mod module {}
