mod cli;

use kit;
use kit::states::Accessor;
use kit::states::Entrypoint;
use kit::Executable;
use std::sync::Arc;

fn main() {
    let interpreter = kit::interpreter();
    interpreter.exec(|vm| {
        vm.insert_sys_path(vm.new_pyobj(".unimake"))?;
        vm.import("project", 0)?;
        vm.umk().update(|s| s.project.instantiate(vm))?;
        vm.print((vm.umk().clone().unwrap().project.instance,))
    });

    let mut graph = Arc::new(cli::Graph::default());
    interpreter.exec(|vm| {
        vm.umk().read(|s| {
            graph.clone_from(&Arc::from(cli::Graph::from_py(&s.cli)));
            Ok(())
        })
    });

    let mut app = clap::Command::new("umk")
        .version("0.1.0")
        .about("Welcome to the Unimake CLI runner");

    for cmd in &graph.build() {
        app = app.subcommand(cmd)
    }

    if let Some((cmd_name, sub_matches)) = app.get_matches().subcommand() {
        let args = sub_matches.get_one::<String>("src");
        if let Some(args) = args {
            println!("Executing {}", args);
        }
    }

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

    interpreter.finalize(None);
}
