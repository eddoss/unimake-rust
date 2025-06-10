use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

//////////////////////////////////////////////////////////////////
// Contributor
//////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone)]
pub struct Contributor {
    pub name: String,
    pub emails: Vec<String>,
    pub socials: HashMap<String, String>,
}

impl fmt::Display for Contributor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = format!("Contributor name='{}'", self.name);
        if !self.emails.is_empty() {
            res = format!("{} emails=[{}]", res, self.emails.join(", "));
        }
        if !self.socials.is_empty() {
            res = format!("{} socials=[{}]", res, self.socials.values().join(", "));
        }
        write!(f, "{}", res)
    }
}

impl Contributor {
    pub fn new(name: &str, emails: Vec<String>, socials: HashMap<String, String>) -> Self {
        Self {
            name: name.to_string(),
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

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Project name='{}' version={}", self.name, self.version)
    }
}

impl Info {
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            title: "".to_string(),
            contributors: vec![],
        }
    }
}

impl Info {
    pub fn contributor(
        &mut self,
        name: String,
        emails: Vec<String>,
        socials: HashMap<String, String>,
    ) {
        let member = Contributor::new(name.as_str(), emails, socials);
        self.contributors.push(member);
    }
}
