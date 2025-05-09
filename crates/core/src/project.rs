use std::collections::HashMap;
use std::fmt::Debug;

//////////////////////////////////////////////////////////////////
// Contributor
//////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct Contributor {
    pub name: String,
    pub emails: Vec<String>,
    pub socials: HashMap<String, String>,
}

impl Contributor {
    pub fn new(name: String, emails: Vec<String>, socials: HashMap<String, String>) -> Self {
        Self {
            name,
            emails,
            socials,
        }
    }
}

//////////////////////////////////////////////////////////////////
// Info
//////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub contributors: Vec<Contributor>,
}

impl Info {
    pub fn contributor(&mut self, name: String, emails: Vec<String>, socials: HashMap<String, String>) {
        let member = Contributor::new(name, emails, socials);
        self.contributors.push(member);
    }
}
