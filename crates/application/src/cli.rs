use crate::api;
use crate::cli::cmd::{PluginSub, ProjectSub};
use clap;
use std::rc::Rc;

pub fn new(api: Rc<dyn api::Interface>) -> umk::Result<Interface> {
    let command = <Cli as clap::CommandFactory>::command();
    let mut matches = command.clone().get_matches();
    let cli = <Cli as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
        .map_err(|e| umk::Error::new(e.to_string().as_str()))?;
    Ok(Interface { api, cli })
}

pub struct Interface {
    cli: Cli,
    api: Rc<dyn api::Interface>,
}

impl Interface {
    pub fn run(&self) -> umk::Result {
        match &self.cli.root {
            cmd::Root::Plugin(c) => {
                if let Some(sub) = &c.subcommand {
                    match sub {
                        PluginSub::List => {
                            println!("Hello from 'plugin list'")
                        }
                    }
                }
            }
            cmd::Root::Project(c) => {
                if let Some(sub) = &c.subcommand {
                    match sub {
                        ProjectSub::Inspect => println!("Hello from 'project inspect'"),
                        ProjectSub::Plugins => println!("Hello from 'project plugins'"),
                        ProjectSub::Authors => println!("Hello from 'project authors'"),
                    }
                }
            }
            cmd::Root::Cli => {
                println!("Hello from 'cli'");
            }
        }
        Ok(())
    }
}

#[derive(clap::Parser)]
#[command(
    author = "Edward Sarkisyan <edw.sarkisyan@gmail.com>",
    version = global::project::VERSION,
    about = "Unimake (umk) is a `make` alternative based on Python syntax."
)]
struct Cli {
    #[command(subcommand)]
    root: cmd::Root,
}

mod cmd {
    use clap;

    #[derive(clap::Subcommand, Debug)]
    pub enum Root {
        /// Project management commands
        #[command(arg_required_else_help(true))]
        Project(Project),

        /// Plugins management commands
        #[command(arg_required_else_help(true))]
        Plugin(Plugin),

        /// Project commands
        #[command(arg_required_else_help(true))]
        Cli,
    }

    #[derive(Debug, clap::Args)]
    pub struct Project {
        #[command(subcommand)]
        pub subcommand: Option<ProjectSub>,
    }

    #[derive(clap::Subcommand, Debug)]
    pub enum ProjectSub {
        /// Shows project information
        Inspect,

        /// Shows plugins required by project
        Plugins,

        /// Shows project authors (contributors)
        Authors,
    }

    #[derive(Debug, clap::Args)]
    pub struct Plugin {
        #[command(subcommand)]
        pub subcommand: Option<PluginSub>,
    }

    #[derive(clap::Subcommand, Debug)]
    pub enum PluginSub {
        /// Lists installed plugins
        List,
    }
}

pub mod user {
    use kit::states;
    use rustpython::vm::builtins::{PyBool, PyFloat, PyInt, PyStr, PyTypeRef};
    use rustpython::vm::class::StaticType;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    enum Class {
        String,
        Integer,
        Float,
        Boolean,
        Custom,
    }

    impl From<&PyTypeRef> for Class {
        fn from(t: &PyTypeRef) -> Self {
            if t.fast_issubclass(PyStr::static_type()) {
                Class::String
            } else if t.fast_issubclass(PyInt::static_type()) {
                Class::Integer
            } else if t.fast_issubclass(PyFloat::static_type()) {
                Class::Float
            } else if t.fast_issubclass(PyBool::static_type()) {
                Class::Boolean
            } else {
                Class::Custom
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Opt {
        class: Class,
        long: String,
        short: Option<char>,
        help: Option<String>,
        required: bool,
        multiple: bool,
    }

    impl From<&states::cli::Opt> for Opt {
        fn from(src: &states::cli::Opt) -> Opt {
            Self {
                class: Class::from(&src.class),
                long: src.long.clone(),
                short: src.short,
                help: src.help.clone(),
                required: false,
                multiple: false,
            }
        }
    }

    impl Opt {
        fn to(&self) -> clap::Arg {
            clap::Arg::new(&self.long)
                .long(self.long.clone())
                .short(self.short)
                .help(self.help.clone().unwrap_or_default())
                .required(self.required)
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Arg {
        #[serde(rename = "type")]
        class: Class,
        name: String,
        help: Option<String>,
    }

    impl From<&states::cli::Arg> for Arg {
        fn from(src: &states::cli::Arg) -> Arg {
            Self {
                class: Class::from(&src.class),
                name: src.name.clone(),
                help: src.help.clone(),
            }
        }
    }

    impl Arg {
        fn to(&self, index: usize) -> clap::Arg {
            clap::Arg::new(self.name.clone().to_uppercase())
                .help(self.help.clone().unwrap_or_default())
                .required(true)
                .index(index)
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Cmd {
        name: String,
        help: Option<String>,
        options: Vec<Opt>,
        arguments: Vec<Arg>,
    }

    impl From<&states::cli::Command> for Cmd {
        fn from(src: &states::cli::Command) -> Cmd {
            Self {
                name: src.name.clone(),
                help: src.help.clone(),
                options: src.options.values().map(|s| Opt::from(s)).collect(),
                arguments: src.arguments.values().map(|s| Arg::from(s)).collect(),
            }
        }
    }

    impl Cmd {
        fn to(&self) -> clap::Command {
            let opts = self.options.iter().map(|x| x.to());
            // let args = self.arguments.iter().enumerate().map(|(i, x)| x.to(i));
            clap::Command::new(self.name.clone())
                .args(opts)
                .about(self.help.clone().unwrap_or_default())
        }
    }

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Graph {
        nodes: Vec<Cmd>,
        links: HashMap<usize, Option<usize>>,
    }

    impl Graph {
        pub fn from_py(source: &states::cli::CLI) -> Graph {
            let mut result = Graph {
                nodes: vec![],
                links: Default::default(),
            };
            result.nodes = source.commands.values().map(|s| Cmd::from(s)).collect();
            result.links =
                HashMap::from_iter(result.nodes.iter().enumerate().map(|(i, _)| (i, None)));
            result
        }

        pub fn build(&self) -> Vec<clap::Command> {
            self.links
                .keys()
                .map(|x| self.nodes[*x].clone())
                .map(|x| x.to())
                .collect()
        }
    }
}
