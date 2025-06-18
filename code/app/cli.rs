use clap;
use unisdk::prelude as sdk;

pub struct Interface {
    workspace: Option<sdk::Workspace>,
    root: clap::Command,
}

impl Interface {
    pub fn run(self) -> sdk::Result {
        match self.root.clone().get_matches().subcommand() {
            Some(("cache", matches)) => {
                match &self.workspace {
                    None => Err(
                        sdk::Error::new(
                            format!("Failed to generate, cache current working directory is not an {} project", sdk::global::project::NAME).as_str()
                        )),
                    Some(w) => {
                        w.load()?;
                        w.instantiate()?;
                        w.caches()
                    }
                }
            }
            Some(("cli", matches)) => {
                self.call(matches.clone())
            }
            _ => unreachable!("Unreachable subcommand"),
        }
    }

    fn call(&self, matches: clap::ArgMatches) -> sdk::Result {
        if self.workspace.is_none() {
            return Err(sdk::Error::new("Failed to call CLI, no workspace found"));
        }
        let mut matches = matches;
        let mut sequence = Vec::<(String, clap::ArgMatches)>::with_capacity(8);
        loop {
            match matches.subcommand() {
                None => break,
                Some((sub_cmd, sub_matches)) => {
                    sequence.push((sub_cmd.to_string(), sub_matches.clone()));
                    matches = sub_matches.clone();
                }
            }
        }
        let workspace = self.workspace.as_ref().unwrap();
        for (name, matches) in sequence {
            println!(
                "src={:?} dst={:?}",
                matches.get_one::<String>("src"),
                matches.get_one::<String>("dst")
            )
        }
        Ok(())
    }
}

impl Interface {
    pub fn new(workspace: Option<sdk::Workspace>) -> sdk::Result<Self> {
        let mut root = clap::Command::new(sdk::global::project::NAME)
            .version(sdk::global::project::VERSION)
            .author("Edward Sarkisyan <edw.sarkisyan@gmail.com>")
            .about("Unimake (umk) is a `make` alternative based on Python")
            .disable_help_subcommand(true)
            .arg_required_else_help(true)
            .subcommand(
                clap::Command::new("cache")
                    .about("Resolve project and cache all entries")
            );
        if let Some(w) = &workspace {
            let graph = w.cli()?;
            root = root.subcommand(build::cli(graph));
        }
        Ok(Self { workspace, root })
    }
}

mod build {
    use clap::value_parser;

    pub fn cli(src: unisdk::umk::Cli) -> clap::Command {
        let children: Vec<clap::Command> = src.nodes.into_iter().map(|c| cmd(c)).collect();
        clap::Command::new("cli")
            .about("CLI declared in workspace")
            .arg_required_else_help(true)
            .disable_help_subcommand(true)
            .subcommands(children)
    }

    pub fn cmd(src: unisdk::umk::Cmd) -> clap::Command {
        let options: Vec<clap::Arg> = src.options.into_iter().map(|o| opt(o)).collect();
        let arguments: Vec<clap::Arg> = src.arguments.into_iter().map(|a| arg(a)).collect();
        clap::Command::new(src.name)
            .about(src.help.unwrap_or_default())
            .args(options.iter().chain(arguments.iter()))
    }

    pub fn opt(src: unisdk::umk::Opt) -> clap::Arg {
        let name = if src.long != "" {
            src.long.clone()
        } else {
            String::from(src.short.unwrap())
        };
        let mut result = clap::Arg::new(name)
            .long(src.long)
            .required(src.required)
            .help(src.help.unwrap_or_default())
            .value_parser(parser(src.class));
        if let Some(v) = src.short {
            result = result.short(v);
        }
        result
    }

    pub fn arg(src: unisdk::umk::Arg) -> clap::Arg {
        clap::Arg::new(src.name)
            .required(true)
            .help(src.help.unwrap_or_default())
            .value_parser(parser(src.class))
    }

    fn parser(class: unisdk::umk::ArgClass) -> clap::builder::ValueParser {
        match class {
            unisdk::umk::ArgClass::String => value_parser!(String),
            unisdk::umk::ArgClass::Integer => value_parser!(i64).into(),
            unisdk::umk::ArgClass::Float => value_parser!(f64).into(),
            unisdk::umk::ArgClass::Boolean => value_parser!(bool).into(),
            unisdk::umk::ArgClass::Custom => value_parser!(String).into(),
        }
    }
}
