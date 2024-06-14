use std::fmt::Debug;
use rustpython::InterpreterConfig;
use rustpython::vm::{
    Interpreter, PyObjectRef, stdlib, VirtualMachine
};
use rustpython_vm::convert::ToPyResult;
use rustpython_vm::PyResult;

// ////////////////////////////////////////////////////
// API
// ////////////////////////////////////////////////////

pub fn init(modules: stdlib::StdlibMap) {
    unsafe {
        match INTERPRETER {
            Some(_) => {return;}
            None => {
                CONTAINER = Some(Default::default());
                INTERPRETER = Some(
                    InterpreterConfig::new()
                        .init_stdlib()
                        .init_hook(Box::new(|vm| {
                            for (name, module) in modules {
                                vm.add_native_module(name, module);
                            }
                        }))
                        .interpreter(),
                );
            }
        }
    }
}

pub fn load(root: &str) {
    enter(|_, vm| {
        vm
            .insert_sys_path(vm.new_pyobj(root))
            .expect("failed to add sys.path");
        match vm.import("project", 0) {
            Ok(_) => {}
            Err(e) => {
                let text = e.to_pyresult(vm).unwrap().str(vm).unwrap().to_string();
                println!("{text}")
            }
        }

    });
    // enter(|cnt, vm| {
    //     let go = cnt.decorators.project.golang.as_mut().unwrap();
    //     go.call_with_args(Default::default(), vm).expect("failed to call function wrapped by '@umk.golang'");
    // });
}

#[derive(Default, Debug)]
pub struct ProjectDecorators {
    pub custom: Option<PyObjectRef>,
    pub empty: Option<PyObjectRef>,
    pub golang: Option<PyObjectRef>,
}

#[derive(Default, Debug)]
pub struct Decorators {
    pub project: ProjectDecorators,
}

#[derive(Default, Debug)]
pub struct Container {
    pub decorators: Decorators,
}

impl Container {}

static mut CONTAINER: Option<Container> = None;
static mut INTERPRETER: Option<Interpreter> = None;

pub fn enter(callback: impl Fn(&mut Container, &VirtualMachine)) {
    unsafe {
        INTERPRETER.as_mut().unwrap().enter(|vm|{
            // let frame = vm.current_frame().unwrap();
            // println!("backend: called at {}:{}", frame.code.source_path, frame.f_lineno().to_string());
            callback(CONTAINER.as_mut().unwrap(), vm);
        });
    }
}
