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
        result.links = HashMap::from_iter(result.nodes.iter().enumerate().map(|(i, _)| (i, None)));
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
