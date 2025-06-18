use std::path::PathBuf;

//////////////////////////////////////////////////////////////////
// Mode
//////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub enum Mode {
    Single,
    Tree,
}

impl Mode {
    pub fn from(root: &PathBuf) -> Option<Mode> {
        let path = root.join(global::workspace::FILE);
        if path.exists() {
            return Some(Mode::Single);
        }
        let path = root.join(global::workspace::DIRECTORY);
        if path.exists() {
            return Some(Mode::Tree);
        }
        None
    }
}

//////////////////////////////////////////////////////////////////
// Constructors
//////////////////////////////////////////////////////////////////

pub struct Workspace {
    root: PathBuf,
    mode: Mode,
}

impl Workspace {
    pub fn new(root: &PathBuf) -> sdk::Result<Self> {
        match Mode::from(root) {
            None => {
                let err = sdk::Error::Text(format!(
                    "Failed to wrap workspace. Given path is not an {} project: {}",
                    global::project::NAME,
                    root.display()
                ));
                Err(err)
            }
            Some(v) => Ok(Self {
                root: root.clone(),
                mode: v,
            }),
        }
    }
}

//////////////////////////////////////////////////////////////////
// Attributes
//////////////////////////////////////////////////////////////////

impl Workspace {
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn mode(&self) -> Mode {
        self.mode.clone()
    }
}

//////////////////////////////////////////////////////////////////
// Methods
//////////////////////////////////////////////////////////////////

impl Workspace {
    pub fn exists(root: &PathBuf) -> bool {
        Mode::from(root).is_some()
    }
}
