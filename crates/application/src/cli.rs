use clap;
use workspace::Workspace;

pub struct Interface {
    workspace: Option<Workspace>,
    root: clap::Command,
}

impl Interface {
    pub fn run(self) -> umk::Result {
        match self.root.clone().get_matches().subcommand() {
            Some(("cache", matches)) => {
                match &self.workspace {
                    None => Err(
                        umk::Error::new(
                            format!("Failed to generate, cache current working directory is not an {} project", global::project::NAME).as_str()
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

    fn call(&self, matches: clap::ArgMatches) -> umk::Result {
        if self.workspace.is_none() {
            return Err(umk::Error::new("Failed to call CLI, no workspace found"));
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
            println!("{:?}", matches.get_one::<String>("src").unwrap())
        }
        Ok(())
    }
}

impl Interface {
    pub fn new(workspace: Option<Workspace>) -> umk::Result<Self> {
        let mut root = clap::Command::new(global::project::NAME)
            .version(global::project::VERSION)
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

    pub fn cli(src: umk::Cli) -> clap::Command {
        let children: Vec<clap::Command> = src.nodes.into_iter().map(|c| cmd(c)).collect();
        clap::Command::new("cli")
            .about("CLI declared in workspace")
            .arg_required_else_help(true)
            .disable_help_subcommand(true)
            .subcommands(children)
    }

    pub fn cmd(src: umk::Cmd) -> clap::Command {
        let options: Vec<clap::Arg> = src.options.into_iter().map(|o| opt(o)).collect();
        let arguments: Vec<clap::Arg> = src.arguments.into_iter().map(|a| arg(a)).collect();
        clap::Command::new(src.name)
            .about(src.help.unwrap_or_default())
            .args(options.iter().chain(arguments.iter()))
    }

    pub fn opt(src: umk::Opt) -> clap::Arg {
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

    pub fn arg(src: umk::Arg) -> clap::Arg {
        clap::Arg::new(src.name)
            .required(true)
            .help(src.help.unwrap_or_default())
            .value_parser(parser(src.class))
    }

    fn parser(class: umk::ArgClass) -> clap::builder::ValueParser {
        match class {
            umk::ArgClass::String => value_parser!(String),
            umk::ArgClass::Integer => value_parser!(i64).into(),
            umk::ArgClass::Float => value_parser!(f64).into(),
            umk::ArgClass::Boolean => value_parser!(bool).into(),
            umk::ArgClass::Custom => value_parser!(String).into(),
        }
    }
}
