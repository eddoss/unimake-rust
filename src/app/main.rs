mod cli;

use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use unisdk::prelude as sdk;

fn main() -> sdk::Result<()> {
    let cwd = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to get current working directory !");
            eprintln!("{}", e.to_string());
            std::process::exit(1);
        }
    };
    let workspace = {
        if !sdk::Workspace::exists(&cwd) {
            None
        } else {
            let keys = sdk::CacheKeys::default();
            let cache_dir = cwd.join(sdk::global::workspace::CACHE);
            let driver = sdk::FsCacheDriver::new(cache_dir);
            let cache = sdk::Cache::new(keys, Rc::new(RefCell::new(driver)));
            match sdk::Workspace::wrap(&cwd, cache) {
                Ok(v) => Some(v),
                Err(e) => {
                    eprintln!("Failed to create workspace instance");
                    eprintln!("{}", e.to_string());
                    std::process::exit(1);
                }
            }
        }
    };
    if let Some(w) = &workspace {
        w.load()?;
    }
    let app = cli::Interface::new(workspace)?;
    app.run()

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
