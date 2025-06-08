use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Class {
    String,
    Integer,
    Float,
    Boolean,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opt {
    pub class: Class,
    pub long: String,
    pub short: Option<char>,
    pub help: Option<String>,
    pub required: bool,
    pub multiple: bool,
    pub variable: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arg {
    #[serde(rename = "type")]
    pub class: Class,
    pub name: String,
    pub help: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cmd {
    pub name: String,
    pub help: Option<String>,
    pub options: Vec<Opt>,
    pub arguments: Vec<Arg>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: Vec<Cmd>,
    pub links: HashMap<usize, Option<usize>>,
}
