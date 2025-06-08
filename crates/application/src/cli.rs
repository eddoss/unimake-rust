use crate::cli::cmd::{PluginSub, ProjectSub, Root};
use clap;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub fn new(cwd: PathBuf) -> umk::Result<Interface> {
    let command = <Cli as clap::CommandFactory>::command();
    let mut matches = command.clone().get_matches();
    let cli = <Cli as clap::FromArgMatches>::from_arg_matches_mut(&mut matches)
        .map_err(|e| umk::Error::new(e.to_string().as_str()))?;
    Ok(Interface { cli, cwd })
}

pub struct Interface {
    cli: Cli,
    cwd: PathBuf,
}

impl Interface {
    pub fn run(&self) -> umk::Result {
        match &self.cli.root {
            Root::Plugin(c) => {
                if let Some(sub) = &c.subcommand {
                    match sub {
                        PluginSub::List => println!("plugin/list"),
                    }
                }
            }
            Root::Project(c) => {
                if let Some(sub) = &c.subcommand {
                    match sub {
                        ProjectSub::Inspect => println!("project/inspect"),
                        ProjectSub::Plugins => println!("project/plugins"),
                        ProjectSub::Authors => println!("project/authors"),
                    };
                    return Ok(());
                }
            }
            Root::Cli => {
                println!("cli");
            }
            Root::Cache => {
                let ws = self.workspace()?;
                ws.load()?;
                ws.instantiate()?;
                return ws.cache();
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

        /// Resolve project and cache CLI
        Cache,

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

impl Interface {
    fn workspace(&self) -> umk::Result<workspace::Workspace> {
        let keys = workspace::CacheKeys::default();
        let cache_dir = self.cwd.join(global::workspace::CACHE);
        let driver = workspace::FsCacheDriver::new(cache_dir);
        let cache = workspace::Cache::new(keys, Rc::new(RefCell::new(driver)));
        workspace::Workspace::wrap(&self.cwd, cache)
    }
}