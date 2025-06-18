use const_format::concatcp;
use rustpython_vm::common::lock::Lazy;

pub const PREFIX: &str = concatcp!(global::kit::NAME, "/plugin/cli/");
pub const GLOBAL_KEY: &str = concatcp!(global::kit::NAME, "/plugin/cli");
pub const FUNC_ATTR: &str = concatcp!(PREFIX, "cmd/builder");

pub const DECORATOR_CMD: &str = "cmd";
pub const DECORATOR_CMD_INNER: &str = concatcp!(PREFIX, DECORATOR_CMD, "/decorator");

pub const DECORATOR_OPT: &str = "opt";
pub const DECORATOR_OPT_INNER: &str = concatcp!(PREFIX, DECORATOR_OPT, "/decorator");

pub const DECORATOR_ARG: &str = "arg";
pub const DECORATOR_ARG_INNER: &str = concatcp!(PREFIX, DECORATOR_ARG, "/decorator");

pub const PLUGIN_INFO: Lazy<plugin::Info> = Lazy::new(|| plugin::Info {
    name: String::from("CLI"),
    version: "0.1.0".to_string(),
    description: "Allows users to declare its own command line interface".to_string(),
});
